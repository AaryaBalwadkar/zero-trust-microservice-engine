/*
 * ZeroTrust Mesh - XDP Packet Filter
 * 
 * This eBPF program implements the packet inspection and filtering
 * requirements from Feature Group D of the SRS.
 * 
 * Requirements implemented:
 * - D1.1: XDP hook for ingress packets
 * - D1.2: TCP, UDP, ICMP inspection
 * - D1.3: 5-tuple extraction
 * - D2.1: SYN flood detection
 * - D2.2: Port scan detection
 * - D3.1: Drop packets matching deny policies
 * - D3.2: Rate limiting
 * - D3.5: Dynamic blacklisting
 * 
 * Compile with:
 *   clang -O2 -target bpf -c xdp_filter.bpf.c -o xdp_filter.bpf.o
 */

#include <linux/bpf.h>
#include <linux/if_ether.h>
#include <linux/ip.h>
#include <linux/tcp.h>
#include <linux/udp.h>
#include <linux/icmp.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_endian.h>

#ifndef IPPROTO_ICMP
#define IPPROTO_ICMP 1
#endif
#ifndef IPPROTO_TCP
#define IPPROTO_TCP 6
#endif
#ifndef IPPROTO_UDP
#define IPPROTO_UDP 17
#endif

#define MAX_ENTRIES 65536
#define RATE_LIMIT_WINDOW_NS 1000000000ULL  // 1 second in nanoseconds
#define BLACKLIST_TTL_NS     3600000000000ULL // 1 hour in nanoseconds
#define REPORT_DEDUP_NS      30000000000ULL  // 30 seconds dedup window

// Packet counters per IP
struct packet_counter {
    __u64 syn_count;
    __u64 total_count;
    __u64 last_reset;
};

// Port scan tracking — uses bitmap for DISTINCT port counting
// Each bit in the bitmap represents a hash bucket for a port.
// This prevents false positives from repeated connections to the same port.
struct port_scan_entry {
    __u64 port_bitmap_lo;  // Bitmap for ports hashed to bits 0-63
    __u64 port_bitmap_hi;  // Bitmap for ports hashed to bits 64-127
    __u64 window_start;
};

// Rate limit configuration
struct rate_config {
    __u32 syn_flood_threshold;    // D2.1: default 100
    __u32 port_scan_threshold;    // D2.2: default 50
    __u32 icmp_flood_threshold;   // D2.4: default 500
    __u32 http_flood_threshold;   // D2.3: default 1000
};

// BPF Maps

// Blacklist: blocked IPs (D3.5)
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, MAX_ENTRIES);
    __type(key, __u32);    // IPv4 address
    __type(value, __u64);  // Expiration timestamp (0 = permanent)
} blacklist SEC(".maps");

// Whitelist: bypassed IPs (D3.4)
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, __u32);    // IPv4 address
    __type(value, __u8);   // Just a flag
} whitelist SEC(".maps");

// Packet counters per source IP (D1.6)
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, MAX_ENTRIES);
    __type(key, __u32);    // IPv4 address
    __type(value, struct packet_counter);
} counters SEC(".maps");

// Port scan tracking
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, MAX_ENTRIES);
    __type(key, __u32);    // IPv4 address
    __type(value, struct port_scan_entry);
} port_scans SEC(".maps");

// Configuration (set from userspace)
struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, struct rate_config);
} config SEC(".maps");

// Track which IPs have been reported in current window (avoid event spam)
struct {
    __uint(type, BPF_MAP_TYPE_LRU_HASH);
    __uint(max_entries, MAX_ENTRIES);
    __type(key, __u32);    // IPv4 address
    __type(value, __u64);  // last reported timestamp
} reported SEC(".maps");

// Attack events ring buffer (send to userspace)
struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 256 * 1024);  // 256KB
} events SEC(".maps");

// Attack event structure
struct attack_event {
    __u32 src_ip;
    __u32 dst_ip;
    __u16 src_port;
    __u16 dst_port;
    __u8 protocol;
    __u8 attack_type;  // 1=SYN flood, 2=port scan, 3=ICMP flood
    __u64 packet_count;
    __u64 timestamp;
};

// Get rate limit configuration
static __always_inline struct rate_config *get_config(void) {
    __u32 key = 0;
    return bpf_map_lookup_elem(&config, &key);
}

// Check if IP is whitelisted (D3.4)
static __always_inline int is_whitelisted(__u32 ip) {
    return bpf_map_lookup_elem(&whitelist, &ip) != NULL;
}

// Check if IP is blacklisted (D3.5)
static __always_inline int is_blacklisted(__u32 ip) {
    __u64 *expiry = bpf_map_lookup_elem(&blacklist, &ip);
    if (!expiry)
        return 0;
    
    // Check if permanent blacklist (expiry = 0)
    if (*expiry == 0)
        return 1;
    
    // Check if expired
    __u64 now = bpf_ktime_get_ns();
    if (now > *expiry) {
        // Remove expired entry
        bpf_map_delete_elem(&blacklist, &ip);
        return 0;
    }
    
    return 1;
}

// Check if we already reported this IP recently (avoid event spam)
static __always_inline int already_reported(__u32 ip) {
    __u64 now = bpf_ktime_get_ns();
    __u64 *last = bpf_map_lookup_elem(&reported, &ip);
    if (last && (now - *last) < REPORT_DEDUP_NS) {
        // Already reported within 30 seconds
        return 1;
    }
    bpf_map_update_elem(&reported, &ip, &now, BPF_ANY);
    return 0;
}

// Auto-blacklist an IP for BLACKLIST_TTL_NS
static __always_inline void auto_blacklist(__u32 ip) {
    __u64 expiry = bpf_ktime_get_ns() + BLACKLIST_TTL_NS;
    bpf_map_update_elem(&blacklist, &ip, &expiry, BPF_ANY);
}

// Update packet counter and check for SYN flood
static __always_inline int check_syn_flood(__u32 ip, __u64 *out_count, struct rate_config *cfg) {
    __u64 now = bpf_ktime_get_ns();
    struct packet_counter *counter;
    struct packet_counter new_counter = {0};
    
    counter = bpf_map_lookup_elem(&counters, &ip);
    if (!counter) {
        new_counter.syn_count = 1;
        new_counter.total_count = 1;
        new_counter.last_reset = now;
        bpf_map_update_elem(&counters, &ip, &new_counter, BPF_ANY);
        *out_count = 1;
        return 0;
    }
    
    // Reset counter if window expired
    if (now - counter->last_reset > RATE_LIMIT_WINDOW_NS) {
        counter->syn_count = 1;
        counter->total_count = 1;
        counter->last_reset = now;
        *out_count = 1;
        return 0;
    }
    
    // Increment counter
    counter->syn_count++;
    counter->total_count++;
    *out_count = counter->syn_count;
    
    // Check threshold (D2.1)
    if (counter->syn_count > cfg->syn_flood_threshold) {
        return 1;  // SYN flood detected
    }
    
    return 0;
}

// Count set bits in a u64 (popcount)
static __always_inline __u64 popcount64(__u64 x) {
    x = x - ((x >> 1) & 0x5555555555555555ULL);
    x = (x & 0x3333333333333333ULL) + ((x >> 2) & 0x3333333333333333ULL);
    x = (x + (x >> 4)) & 0x0F0F0F0F0F0F0F0FULL;
    return (x * 0x0101010101010101ULL) >> 56;
}

// Check for port scan (D2.2) — tracks DISTINCT destination ports via bitmap
static __always_inline int check_port_scan(__u32 ip, __u16 port, __u64 *out_count, struct rate_config *cfg) {
    __u64 now = bpf_ktime_get_ns();
    struct port_scan_entry *entry;
    struct port_scan_entry new_entry = {0};
    
    // Hash port to a bit position (0-127)
    __u32 bit_pos = ((__u32)port * 2654435761U) & 127; // Knuth multiplicative hash
    
    entry = bpf_map_lookup_elem(&port_scans, &ip);
    if (!entry) {
        new_entry.window_start = now;
        if (bit_pos < 64)
            new_entry.port_bitmap_lo = 1ULL << bit_pos;
        else
            new_entry.port_bitmap_hi = 1ULL << (bit_pos - 64);
        bpf_map_update_elem(&port_scans, &ip, &new_entry, BPF_ANY);
        *out_count = 1;
        return 0;
    }
    
    // Reset if 10-second window expired
    if (now - entry->window_start > 10 * RATE_LIMIT_WINDOW_NS) {
        entry->port_bitmap_lo = 0;
        entry->port_bitmap_hi = 0;
        entry->window_start = now;
        if (bit_pos < 64)
            entry->port_bitmap_lo = 1ULL << bit_pos;
        else
            entry->port_bitmap_hi = 1ULL << (bit_pos - 64);
        *out_count = 1;
        return 0;
    }
    
    // Set the bit for this port (idempotent — same port won't increase count)
    if (bit_pos < 64)
        entry->port_bitmap_lo |= (1ULL << bit_pos);
    else
        entry->port_bitmap_hi |= (1ULL << (bit_pos - 64));
    
    // Count distinct ports (number of set bits)
    __u64 distinct = popcount64(entry->port_bitmap_lo) + popcount64(entry->port_bitmap_hi);
    *out_count = distinct;
    
    // Check threshold (D2.2: distinct ports in 10 seconds)
    if (distinct > cfg->port_scan_threshold) {
        return 1;  // Port scan detected
    }
    
    return 0;
}

// Send attack event to userspace
static __always_inline void report_attack(
    __u32 src_ip, __u32 dst_ip,
    __u16 src_port, __u16 dst_port,
    __u8 protocol, __u8 attack_type,
    __u64 packet_count
) {
    struct attack_event *event;
    
    event = bpf_ringbuf_reserve(&events, sizeof(*event), 0);
    if (!event)
        return;
    
    event->src_ip = src_ip;
    event->dst_ip = dst_ip;
    event->src_port = src_port;
    event->dst_port = dst_port;
    event->protocol = protocol;
    event->attack_type = attack_type;
    event->packet_count = packet_count;
    event->timestamp = bpf_ktime_get_ns();
    
    bpf_ringbuf_submit(event, 0);
}

// Main XDP program (D1.1)
SEC("xdp")
int xdp_filter(struct xdp_md *ctx) {
    void *data = (void *)(long)ctx->data;
    void *data_end = (void *)(long)ctx->data_end;
    
    // Parse Ethernet header
    struct ethhdr *eth = data;
    if ((void *)(eth + 1) > data_end)
        return XDP_PASS;
    
    // Only handle IPv4 (D1.2)
    if (eth->h_proto != bpf_htons(ETH_P_IP))
        return XDP_PASS;
    
    // Parse IP header
    struct iphdr *ip = (void *)(eth + 1);
    if ((void *)(ip + 1) > data_end)
        return XDP_PASS;
    
    __u32 src_ip = ip->saddr;
    __u32 dst_ip = ip->daddr;
    
    // Bypass loopback traffic — never block the machine talking to itself
    if (src_ip == dst_ip)
        return XDP_PASS;
    
    // Bypass link-local and loopback ranges (127.x.x.x = 0x7f in first byte)
    // Network byte order: 127.0.0.1 = 0x0100007f on little-endian
    if ((src_ip & 0x000000ff) == 0x0000007f)
        return XDP_PASS;
    
    // Get configuration
    struct rate_config *cfg = get_config();
    if (!cfg)
        return XDP_PASS;  // No config, pass all
    
    // Check whitelist first (D3.4)
    if (is_whitelisted(src_ip))
        return XDP_PASS;
    
    // Check blacklist (D3.5)
    if (is_blacklisted(src_ip))
        return XDP_DROP;
    
    // Handle TCP (D1.2)
    if (ip->protocol == IPPROTO_TCP) {
        struct tcphdr *tcp = (void *)ip + (ip->ihl * 4);
        if ((void *)(tcp + 1) > data_end)
            return XDP_PASS;
        
        __u16 src_port = bpf_ntohs(tcp->source);
        __u16 dst_port = bpf_ntohs(tcp->dest);
        __u64 pkt_count = 0;
        
        // Check for SYN flood (D2.1) — only pure SYN packets
        if (tcp->syn && !tcp->ack) {
            if (check_syn_flood(src_ip, &pkt_count, cfg)) {
                // Auto-blacklist attacker
                auto_blacklist(src_ip);
                // Only report once per window to avoid event spam
                if (!already_reported(src_ip)) {
                    report_attack(src_ip, dst_ip, src_port, dst_port, 
                                IPPROTO_TCP, 1, pkt_count);
                }
                return XDP_DROP;
            }
            
            // Port scan check (D2.2) — only on SYN packets that pass flood check
            // This detects an IP sending SYNs to many different destination ports
            if (check_port_scan(src_ip, dst_port, &pkt_count, cfg)) {
                auto_blacklist(src_ip);
                if (!already_reported(src_ip)) {
                    report_attack(src_ip, dst_ip, src_port, dst_port,
                                IPPROTO_TCP, 2, pkt_count);
                }
                return XDP_DROP;
            }
        }
        
        return XDP_PASS;
    }
    
    // Handle UDP (D1.2)
    if (ip->protocol == IPPROTO_UDP) {
        struct udphdr *udp = (void *)ip + (ip->ihl * 4);
        if ((void *)(udp + 1) > data_end)
            return XDP_PASS;
        
        __u64 udp_count = 0;
        // Check for port scan on UDP
        if (check_port_scan(src_ip, bpf_ntohs(udp->dest), &udp_count, cfg)) {
            auto_blacklist(src_ip);
            if (!already_reported(src_ip)) {
                report_attack(src_ip, dst_ip, 
                             bpf_ntohs(udp->source), bpf_ntohs(udp->dest),
                             IPPROTO_UDP, 2, udp_count);
            }
            return XDP_DROP;
        }
        
        return XDP_PASS;
    }
    
    // Handle ICMP (D1.2, D2.4)
    if (ip->protocol == IPPROTO_ICMP) {
        // Use counters for ICMP flood detection
        struct packet_counter *counter;
        struct packet_counter new_icmp = {0};
        __u64 now = bpf_ktime_get_ns();
        
        counter = bpf_map_lookup_elem(&counters, &src_ip);
        if (!counter) {
            new_icmp.syn_count = 0;
            new_icmp.total_count = 1;
            new_icmp.last_reset = now;
            bpf_map_update_elem(&counters, &src_ip, &new_icmp, BPF_ANY);
            return XDP_PASS;
        }
        
        // Reset if window expired
        if (now - counter->last_reset > RATE_LIMIT_WINDOW_NS) {
            counter->total_count = 1;
            counter->last_reset = now;
            return XDP_PASS;
        }
        
        counter->total_count++;
        
        if (counter->total_count > cfg->icmp_flood_threshold) {
            auto_blacklist(src_ip);
            if (!already_reported(src_ip)) {
                report_attack(src_ip, dst_ip, 0, 0, IPPROTO_ICMP, 3, 
                             counter->total_count);
            }
            return XDP_DROP;
        }
        
        return XDP_PASS;
    }
    
    return XDP_PASS;
}

char LICENSE[] SEC("license") = "GPL";
