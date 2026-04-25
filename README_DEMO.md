# Zero-Trust Mesh Engine — Industry Demo & Usage Guide

This guide provides everything you need to run the software, verify its components, and deliver a **perfect, high-impact demonstration** to industry professionals.

---

## 1. Quick Start & Setup

To run the control plane (the desktop application):

```bash
# Install frontend dependencies
npm install

# Run the Tauri application in development mode
npm run tauri dev
```

> **Note:** The application uses SQLite for local persistence. If you ever need to completely wipe the state manually, you can delete the `zerotrust_mesh.db` file in your system's AppData/Local directory.

---

## 2. The "Perfect" Industry Demo Script

When showing this to an industry professional, you want to prove that this is a **real, working security engine**, not just a UI mockup. We will do this in two phases: first demonstrating the live cryptographic features on a blank slate, and second, loading a simulated "at-scale" environment to show the attack dashboards.

### Phase 1: Real-Time Binary Attestation & Identity (Live)
*Start with a completely empty state. If you have demo data loaded, go to Settings -> "Clear All Data" (or delete the DB).*

1. **Dashboard:** Point out that the dashboard is empty. We are starting from a zero-trust baseline.
2. **Services -> Register Service:**
   - Name: `Payment Gateway`
   - Port: `8443`
   - Binary Path: `/usr/bin/curl` (or any real binary on your system).
   - Click Register. Explain that the system just generated a cryptographic SPIFFE ID (`spiffe://...`) and X.509 certificate for this workload.
3. **Services -> Run Scan:**
   - Click the "Scan All Services" button.
   - Expand the `Payment Gateway` row. 
   - **TALKING POINT:** "The engine just went to the disk, read `/usr/bin/curl`, and calculated its SHA-256 hash in real-time. Because this is the first scan, it stored this hash as the secure baseline."
   - Run the scan again. It passes because the hash matches. The trust score is high (100%).

### Phase 2: Live Policy Engine (Live)
1. **Policies:**
   - Click "New Policy".
   - Name: `Block Compromised Services`
   - Action: `Deny`
   - Trust Threshold: `0.60` (Meaning: if trust score < 60%, deny).
   - Click Create Policy.
2. **Live Policy Evaluator:**
   - *Tip: You can get a real SPIFFE ID by going to the **Services** page and copying the `spiffe://...` string from the table.*
   - Paste that ID into the **Source SPIFFE ID** and **Destination SPIFFE ID** fields (or just leave them blank, as the trust score threshold will evaluate regardless of the ID).
   - Leave the Trust Score slider at `0.80`. Click "Evaluate". It should return **Allow** (or no match).
   - Move the Trust Score slider down to `0.30`. Click "Evaluate".
   - **TALKING POINT:** "Watch the evaluation time. In under 50 microseconds, the in-memory Rust policy engine evaluated the request, saw the trust score was below our threshold, and actively blocked the connection. This is the speed required for sidecar proxy enforcement."

### Phase 3: "At Scale" Simulation & Attacks (Simulated Workspace)
*Now, you want to show what the system looks like under load during a cyber attack.*

1. **Settings -> Development Tools:**
   - Click "Load Demo Workspace". 
   - **TALKING POINT:** "To show you the system under load, I'm going to load a simulated workspace. This injects background microservices and simulates a live DDoS and Port Scan attack."
2. **Dashboard:**
   - Show the populated metrics: active services, total attacks blocked in 24h, and open security alerts.
3. **Services -> Run Scan:**
   - Click "Scan All Services". 
   - Expand the services. Show that they mapped to real binaries (`/bin/sh`, `/bin/ls`) and were successfully measured.
4. **Attacks:**
   - Show the "Recent Attack Events" table (SYN Floods, Port Scans).
   - **Blacklist IP:** Enter an IP from the table (e.g., `192.168.1.50`), give a reason ("Repeated SYN Flood"), and click "Blacklist IP". Show it appearing in the Blacklist table.
   - **Acknowledge Alerts:** Go to Recent Alerts, click "Ack" on a critical alert to show SOC analyst workflow.

### Phase 4: Audit & Compliance
1. **Audit Log:**
   - Navigate to the Audit page.
   - **TALKING POINT:** "Every single action—from a binary measurement, to a policy evaluation, to an admin blacklisting an IP—is recorded in an immutable SQLite audit log."
   - Filter the dropdown by `attestation` or `policy`.
   - Click "Export" to show how logs can be sent to an external SIEM.

---

## 3. Addressing the "Live Attacks" Question

Industry professionals will ask: *"Is the attack detection actually intercepting live packets on my network right now?"*

**The exact, professional answer you should give:**

> "This desktop application is the **Control Plane**. It handles identity, policy distribution, binary attestation, and data aggregation. 
>
> The repository contains the **Data Plane** modules—specifically, an eBPF library for high-performance packet inspection (XDP/TC) and a WireGuard module for encrypted tunneling. 
> 
> However, to prevent accidentally breaking the host operating system's networking during development and demonstrations, this desktop build **does not auto-start the live eBPF packet ingestion loop**. The attack data you see in the UI is injected via our simulator to demonstrate how the Control Plane reacts (updating trust scores, triggering alerts, and adjusting policies). 
> 
> In a production deployment, the eBPF sidecar would run as a root daemon on the Linux nodes, intercepting real SYN floods and pushing those events via gRPC/API up to this exact Control Plane interface."

### How to verify the real capabilities yourself:
1. **Binary Attestation is 100% Real:** If you want to test it, register a service pointing to a binary you wrote. Scan it. Then re-compile or edit that binary and scan again. The hash will change, the scan will **Fail**, and the Trust Score will instantly drop.
2. **Policy Evaluation is 100% Real:** The `evaluate_policy` command executes actual logic against the SQLite ruleset.
3. **Data Persistence is 100% Real:** All tunnels, blacklists, and audit logs are genuinely written to local disk databases, not just mocked in Javascript.

## 4. Key Takeaways to Emphasize to the Industry Person

- **Rust Backend:** Emphasize that the core engine is written in Rust, ensuring memory safety and C-like performance for cryptographic operations.
- **Zero-Trust Principles:** It implements all 3 pillars of Zero Trust: 
  1. **Identity** (SPIFFE X.509 certs).
  2. **Device/Workload Integrity** (SHA-256 Binary Attestation).
  3. **Continuous Authorization** (Trust scores feeding dynamically into the Policy Evaluator).
- **Control Plane vs Data Plane Separation:** It uses modern architecture by decoupling the management UI from the underlying network enforcement.
