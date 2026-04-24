import{G as ee,b as Ie,aS as Oe,h as N,c as G,aB as Le,a as ae,q as z,r as Ye,am as je,s as $e,d as B,e as V,aJ as Ke,aQ as Xe,au as Ae,J as re,aT as m,I as J,aU as Ze,L as Je,_ as Qe,aV as Pe,aP as de,aW as xe,aX as er,y as rr,N as Ne,aY as ar,T as tr,C as ze,F as He,aZ as fe,ak as Re,a_ as ir,a$ as nr,aN as sr,K as fr,D as te,aK as or,b0 as lr,E as ur,aA as cr,b1 as dr,M as vr,v as De,b2 as Fe,ad as ve,b3 as hr,b4 as gr,a3 as _r,b5 as pr,b6 as br,b7 as se,b8 as $r,b9 as Ar,ba as Nr,bb as Er,bc as Sr,bd as Tr,be as wr,f as kr,ag as yr,u as Ee,g as ue,bf as Se,bg as mr,bh as F,Z as Mr,bi as Cr,bj as Ir,p as Or,j as Lr,k as Pr,i as H,o as zr,m as Hr,ab as Rr,ac as Dr}from"./DD_Wr0xw.js";import{b as Fr,i as Vr,d as Br,g as Wr,j as qr,k as Ur,n as Gr,l as Yr,a as I,o as jr,c as R}from"./B3JMgGaj.js";import{B as Kr,l as O,p as U,s as W}from"./CNXvGvbm.js";function Xr(e,r){return r}function Zr(e,r,a){for(var t=[],i=r.length,s,n=r.length,f=0;f<i;f++){let g=r[f];He(g,()=>{if(s){if(s.pending.delete(g),s.done.add(g),s.pending.size===0){var c=e.outrogroups;ce(e,de(s.done)),c.delete(s),c.size===0&&(e.outrogroups=null)}}else n-=1},!1)}if(n===0){var o=t.length===0&&a!==null;if(o){var d=a,u=d.parentNode;sr(u),u.append(d),e.items.clear()}ce(e,r,!o)}else s={pending:new Set(r),done:new Set},(e.outrogroups??(e.outrogroups=new Set)).add(s)}function ce(e,r,a=!0){var t;if(e.pending.size>0){t=new Set;for(const n of e.pending.values())for(const f of n)t.add(e.items.get(f).e)}for(var i=0;i<r.length;i++){var s=r[i];if(t!=null&&t.has(s)){s.f|=m;const n=document.createDocumentFragment();fr(s,n)}else te(r[i],a)}}var Te;function Jr(e,r,a,t,i,s=null){var n=e,f=new Map,o=(r&Oe)!==0;if(o){var d=e;n=N?G(Le(d)):d.appendChild(ee())}N&&ae();var u=null,g=Qe(()=>{var A=a();return Pe(A)?A:A==null?[]:de(A)}),c,b=new Map,$=!0;function S(A){k.effect.f&tr||(k.pending.delete(A),k.fallback=u,Qr(k,c,n,r,t),u!==null&&(c.length===0?u.f&m?(u.f^=m,X(u,null,n)):ze(u):He(u,()=>{u=null})))}function l(A){k.pending.delete(A)}var v=Ie(()=>{c=z(g);var A=c.length;let p=!1;if(N){var L=Ye(n)===je;L!==(A===0)&&(n=$e(),G(n),B(!1),p=!0)}for(var _=new Set,h=re,T=Je(),w=0;w<A;w+=1){N&&V.nodeType===Ke&&V.data===Xe&&(n=V,p=!0,B(!1));var E=c[w],M=t(E,w),y=$?null:f.get(M);y?(y.v&&Ae(y.v,E),y.i&&Ae(y.i,w),T&&h.unskip_effect(y.e)):(y=xr(f,$?n:Te??(Te=ee()),E,M,w,i,r,a),$||(y.e.f|=m),f.set(M,y)),_.add(M)}if(A===0&&s&&!u&&($?u=J(()=>s(n)):(u=J(()=>s(Te??(Te=ee()))),u.f|=m)),A>_.size&&Ze(),N&&A>0&&G($e()),!$)if(b.set(h,_),T){for(const[Q,x]of f)_.has(Q)||h.skip_effect(x.e);h.oncommit(S),h.ondiscard(l)}else S(h);p&&B(!0),z(g)}),k={effect:v,items:f,pending:b,outrogroups:null,fallback:u};$=!1,N&&(n=V)}function Y(e){for(;e!==null&&!(e.f&ir);)e=e.next;return e}function Qr(e,r,a,t,i){var E,M,y,Q,x,he,ge,_e,pe;var s=(t&nr)!==0,n=r.length,f=e.items,o=Y(e.effect.first),d,u=null,g,c=[],b=[],$,S,l,v;if(s)for(v=0;v<n;v+=1)$=r[v],S=i($,v),l=f.get(S).e,l.f&m||((M=(E=l.nodes)==null?void 0:E.a)==null||M.measure(),(g??(g=new Set)).add(l));for(v=0;v<n;v+=1){if($=r[v],S=i($,v),l=f.get(S).e,e.outrogroups!==null)for(const C of e.outrogroups)C.pending.delete(l),C.done.delete(l);if(l.f&fe&&(ze(l),s&&((Q=(y=l.nodes)==null?void 0:y.a)==null||Q.unfix(),(g??(g=new Set)).delete(l))),l.f&m)if(l.f^=m,l===o)X(l,null,a);else{var k=u?u.next:o;l===e.effect.last&&(e.effect.last=l.prev),l.prev&&(l.prev.next=l.next),l.next&&(l.next.prev=l.prev),P(e,u,l),P(e,l,k),X(l,k,a),u=l,c=[],b=[],o=Y(u.next);continue}if(l!==o){if(d!==void 0&&d.has(l)){if(c.length<b.length){var A=b[0],p;u=A.prev;var L=c[0],_=c[c.length-1];for(p=0;p<c.length;p+=1)X(c[p],A,a);for(p=0;p<b.length;p+=1)d.delete(b[p]);P(e,L.prev,_.next),P(e,u,L),P(e,_,A),o=A,u=_,v-=1,c=[],b=[]}else d.delete(l),X(l,o,a),P(e,l.prev,l.next),P(e,l,u===null?e.effect.first:u.next),P(e,u,l),u=l;continue}for(c=[],b=[];o!==null&&o!==l;)(d??(d=new Set)).add(o),b.push(o),o=Y(o.next);if(o===null)continue}l.f&m||c.push(l),u=l,o=Y(l.next)}if(e.outrogroups!==null){for(const C of e.outrogroups)C.pending.size===0&&(ce(e,de(C.done)),(x=e.outrogroups)==null||x.delete(C));e.outrogroups.size===0&&(e.outrogroups=null)}if(o!==null||d!==void 0){var h=[];if(d!==void 0)for(l of d)l.f&fe||h.push(l);for(;o!==null;)!(o.f&fe)&&o!==e.fallback&&h.push(o),o=Y(o.next);var T=h.length;if(T>0){var w=t&Oe&&n===0?a:null;if(s){for(v=0;v<T;v+=1)(ge=(he=h[v].nodes)==null?void 0:he.a)==null||ge.measure();for(v=0;v<T;v+=1)(pe=(_e=h[v].nodes)==null?void 0:_e.a)==null||pe.fix()}Zr(e,h,w)}}s&&Re(()=>{var C,be;if(g!==void 0)for(l of g)(be=(C=l.nodes)==null?void 0:C.a)==null||be.apply()})}function xr(e,r,a,t,i,s,n,f){var o=n&xe?n&er?Ne(a):rr(a,!1,!1):null,d=n&ar?Ne(i):null;return{v:o,i:d,e:J(()=>(s(r,o??a,d??i,f),()=>{e.delete(t)}))}}function X(e,r,a){if(e.nodes)for(var t=e.nodes.start,i=e.nodes.end,s=r&&!(r.f&m)?r.nodes.start:a;t!==null;){var n=or(t);if(s.before(t),t===i)return;t=n}}function P(e,r,a){r===null?e.effect.first=a:r.next=a,a===null?e.effect.last=r:a.prev=r}function D(e,r,a,t,i){var f;N&&ae();var s=(f=r.$$slots)==null?void 0:f[a],n=!1;s===!0&&(s=r.children,n=!0),s===void 0||s(e,n?()=>t:t)}function ea(e,r,a,t,i,s){let n=N;N&&ae();var f=null;N&&V.nodeType===lr&&(f=V,ae());var o=N?V:e,d=new Kr(o,!1);Ie(()=>{const u=r()||null;var g=dr;if(u===null){d.ensure(null,null);return}return d.ensure(u,c=>{if(u){if(f=N?f:cr(u,g),Fr(f,f),t){N&&Vr(u)&&f.append(document.createComment(""));var b=N?Le(f):f.appendChild(ee());N&&(b===null?B(!1):G(b)),t(f,b)}vr.nodes.end=f,c.before(f)}N&&G(c)}),()=>{}},ur),De(()=>{}),n&&(B(!0),G(o))}function ra(e,r){var a=void 0,t;Fe(()=>{a!==(a=r())&&(t&&(te(t),t=null),a&&(t=J(()=>{ve(()=>a(e))})))})}function Ve(e){var r,a,t="";if(typeof e=="string"||typeof e=="number")t+=e;else if(typeof e=="object")if(Array.isArray(e)){var i=e.length;for(r=0;r<i;r++)e[r]&&(a=Ve(e[r]))&&(t&&(t+=" "),t+=a)}else for(a in e)e[a]&&(t&&(t+=" "),t+=a);return t}function aa(){for(var e,r,a=0,t="",i=arguments.length;a<i;a++)(e=arguments[a])&&(r=Ve(e))&&(t&&(t+=" "),t+=r);return t}function ta(e){return typeof e=="object"?aa(e):e??""}const we=[...` 	
\r\f \v\uFEFF`];function ia(e,r,a){var t=e==null?"":""+e;if(a){for(var i of Object.keys(a))if(a[i])t=t?t+" "+i:i;else if(t.length)for(var s=i.length,n=0;(n=t.indexOf(i,n))>=0;){var f=n+s;(n===0||we.includes(t[n-1]))&&(f===t.length||we.includes(t[f]))?t=(n===0?"":t.substring(0,n))+t.substring(f+1):n=f}}return t===""?null:t}function ke(e,r=!1){var a=r?" !important;":";",t="";for(var i of Object.keys(e)){var s=e[i];s!=null&&s!==""&&(t+=" "+i+": "+s+a)}return t}function oe(e){return e[0]!=="-"||e[1]!=="-"?e.toLowerCase():e}function na(e,r){if(r){var a="",t,i;if(Array.isArray(r)?(t=r[0],i=r[1]):t=r,e){e=String(e).replaceAll(/\s*\/\*.*?\*\/\s*/g,"").trim();var s=!1,n=0,f=!1,o=[];t&&o.push(...Object.keys(t).map(oe)),i&&o.push(...Object.keys(i).map(oe));var d=0,u=-1;const S=e.length;for(var g=0;g<S;g++){var c=e[g];if(f?c==="/"&&e[g-1]==="*"&&(f=!1):s?s===c&&(s=!1):c==="/"&&e[g+1]==="*"?f=!0:c==='"'||c==="'"?s=c:c==="("?n++:c===")"&&n--,!f&&s===!1&&n===0){if(c===":"&&u===-1)u=g;else if(c===";"||g===S-1){if(u!==-1){var b=oe(e.substring(d,u).trim());if(!o.includes(b)){c!==";"&&g++;var $=e.substring(d,g).trim();a+=" "+$+";"}}d=g+1,u=-1}}}}return t&&(a+=ke(t)),i&&(a+=ke(i,!0)),a=a.trim(),a===""?null:a}return e==null?null:String(e)}function sa(e,r,a,t,i,s){var n=e.__className;if(N||n!==a||n===void 0){var f=ia(a,t,s);(!N||f!==e.getAttribute("class"))&&(f==null?e.removeAttribute("class"):r?e.className=f:e.setAttribute("class",f)),e.__className=a}else if(s&&i!==s)for(var o in s){var d=!!s[o];(i==null||d!==!!i[o])&&e.classList.toggle(o,d)}return s}function le(e,r={},a,t){for(var i in a){var s=a[i];r[i]!==s&&(a[i]==null?e.style.removeProperty(i):e.style.setProperty(i,s,t))}}function fa(e,r,a,t){var i=e.__style;if(N||i!==r){var s=na(r,t);(!N||s!==e.getAttribute("style"))&&(s==null?e.removeAttribute("style"):e.style.cssText=s),e.__style=r}else t&&(Array.isArray(t)?(le(e,a==null?void 0:a[0],t[0]),le(e,a==null?void 0:a[1],t[1],"important")):le(e,a,t));return t}function ie(e,r,a=!1){if(e.multiple){if(r==null)return;if(!Pe(r))return hr();for(var t of e.options)t.selected=r.includes(Z(t));return}for(t of e.options){var i=Z(t);if(gr(i,r)){t.selected=!0;return}}(!a||r!==void 0)&&(e.selectedIndex=-1)}function Be(e){var r=new MutationObserver(()=>{ie(e,e.__value)});r.observe(e,{childList:!0,subtree:!0,attributes:!0,attributeFilter:["value"]}),De(()=>{r.disconnect()})}function Ea(e,r,a=r){var t=new WeakSet,i=!0;_r(e,"change",s=>{var n=s?"[selected]":":checked",f;if(e.multiple)f=[].map.call(e.querySelectorAll(n),Z);else{var o=e.querySelector(n)??e.querySelector("option:not([disabled])");f=o&&Z(o)}a(f),e.__value=f,re!==null&&t.add(re)}),ve(()=>{var s=r();if(e===document.activeElement){var n=re;if(t.has(n))return}if(ie(e,s,i),i&&s===void 0){var f=e.querySelector(":checked");f!==null&&(s=Z(f),a(s))}e.__value=s,i=!1}),Be(e)}function Z(e){return"__value"in e?e.__value:e.value}const j=Symbol("class"),K=Symbol("style"),We=Symbol("is custom element"),qe=Symbol("is html"),oa=se?"link":"LINK",la=se?"input":"INPUT",ua=se?"option":"OPTION",ca=se?"select":"SELECT";function da(e){if(N){var r=!1,a=()=>{if(!r){if(r=!0,e.hasAttribute("value")){var t=e.value;ne(e,"value",null),e.value=t}if(e.hasAttribute("checked")){var i=e.checked;ne(e,"checked",null),e.checked=i}}};e.__on_r=a,Re(a),Nr()}}function va(e,r){r?e.hasAttribute("selected")||e.setAttribute("selected",""):e.removeAttribute("selected")}function ne(e,r,a,t){var i=Ue(e);N&&(i[r]=e.getAttribute(r),r==="src"||r==="srcset"||r==="href"&&e.nodeName===oa)||i[r]!==(i[r]=a)&&(r==="loading"&&(e[Tr]=a),a==null?e.removeAttribute(r):typeof a!="string"&&Ge(e).includes(r)?e[r]=a:e.setAttribute(r,a))}function ha(e,r,a,t,i=!1,s=!1){if(N&&i&&e.nodeName===la){var n=e,f=n.type==="checkbox"?"defaultChecked":"defaultValue";f in a||da(n)}var o=Ue(e),d=o[We],u=!o[qe];let g=N&&d;g&&B(!1);var c=r||{},b=e.nodeName===ua;for(var $ in r)$ in a||(a[$]=null);a.class?a.class=ta(a.class):a[j]&&(a.class=null),a[K]&&(a.style??(a.style=null));var S=Ge(e);for(const _ in a){let h=a[_];if(b&&_==="value"&&h==null){e.value=e.__value="",c[_]=h;continue}if(_==="class"){var l=e.namespaceURI==="http://www.w3.org/1999/xhtml";sa(e,l,h,t,r==null?void 0:r[j],a[j]),c[_]=h,c[j]=a[j];continue}if(_==="style"){fa(e,h,r==null?void 0:r[K],a[K]),c[_]=h,c[K]=a[K];continue}var v=c[_];if(!(h===v&&!(h===void 0&&e.hasAttribute(_)))){c[_]=h;var k=_[0]+_[1];if(k!=="$$")if(k==="on"){const T={},w="$$"+_;let E=_.slice(2);var A=Yr(E);if(Br(E)&&(E=E.slice(0,-7),T.capture=!0),!A&&v){if(h!=null)continue;e.removeEventListener(E,c[w],T),c[w]=null}if(A)Wr(E,e,h),qr([E]);else if(h!=null){let M=function(y){c[_].call(this,y)};c[w]=Ur(E,e,M,T)}}else if(_==="style")ne(e,_,h);else if(_==="autofocus")$r(e,!!h);else if(!d&&(_==="__value"||_==="value"&&h!=null))e.value=e.__value=h;else if(_==="selected"&&b)va(e,h);else{var p=_;u||(p=Gr(p));var L=p==="defaultValue"||p==="defaultChecked";if(h==null&&!d&&!L)if(o[_]=null,p==="value"||p==="checked"){let T=e;const w=r===void 0;if(p==="value"){let E=T.defaultValue;T.removeAttribute(p),T.defaultValue=E,T.value=T.__value=w?E:null}else{let E=T.defaultChecked;T.removeAttribute(p),T.defaultChecked=E,T.checked=w?E:!1}}else e.removeAttribute(_);else L||S.includes(p)&&(d||typeof h!="string")?(e[p]=h,p in o&&(o[p]=Ar)):typeof h!="function"&&ne(e,p,h)}}}return g&&B(!0),c}function ye(e,r,a=[],t=[],i=[],s,n=!1,f=!1){pr(i,a,t,o=>{var d=void 0,u={},g=e.nodeName===ca,c=!1;if(Fe(()=>{var $=r(...o.map(z)),S=ha(e,d,$,s,n,f);c&&g&&"value"in $&&ie(e,$.value);for(let v of Object.getOwnPropertySymbols(u))$[v]||te(u[v]);for(let v of Object.getOwnPropertySymbols($)){var l=$[v];v.description===br&&(!d||l!==d[v])&&(u[v]&&te(u[v]),u[v]=J(()=>ra(e,()=>l))),S[v]=l}d=S}),g){var b=e;ve(()=>{ie(b,d.value,!0),Be(b)})}c=!0})}function Ue(e){return e.__attributes??(e.__attributes={[We]:e.nodeName.includes("-"),[qe]:e.namespaceURI===Er})}var me=new Map;function Ge(e){var r=e.getAttribute("is")||e.nodeName,a=me.get(r);if(a)return a;me.set(r,a=[]);for(var t,i=e,s=Element.prototype;s!==i;){t=wr(i);for(var n in t)t[n].set&&a.push(n);i=Sr(i)}return a}function ga(e=!1){const r=kr,a=r.l.u;if(!a)return;let t=()=>F(r.s);if(e){let i=0,s={};const n=Mr(()=>{let f=!1;const o=r.s;for(const d in o)o[d]!==s[d]&&(s[d]=o[d],f=!0);return f&&i++,i});t=()=>z(n)}a.b.length&&yr(()=>{Me(r,t),Se(a.b)}),Ee(()=>{const i=ue(()=>a.m.map(mr));return()=>{for(const s of i)typeof s=="function"&&s()}}),a.a.length&&Ee(()=>{Me(r,t),Se(a.a)})}function Me(e,r){if(e.l.s)for(const a of e.l.s)z(a);r()}Cr();const Sa=Ir("dashboard");/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 * 
 * Copyright (c) 2026 Lucide Icons and Contributors
 * 
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 * 
 * ---
 * 
 * The following Lucide icons are derived from the Feather project:
 * 
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 * 
 * The MIT License (MIT) (for the icons listed above)
 * 
 * Copyright (c) 2013-present Cole Bemis
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 * 
 */const _a={xmlns:"http://www.w3.org/2000/svg",width:24,height:24,viewBox:"0 0 24 24",fill:"none",stroke:"currentColor","stroke-width":2,"stroke-linecap":"round","stroke-linejoin":"round"};/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 * 
 * Copyright (c) 2026 Lucide Icons and Contributors
 * 
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 * 
 * ---
 * 
 * The following Lucide icons are derived from the Feather project:
 * 
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 * 
 * The MIT License (MIT) (for the icons listed above)
 * 
 * Copyright (c) 2013-present Cole Bemis
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 * 
 */const pa=e=>{for(const r in e)if(r.startsWith("aria-")||r==="role"||r==="title")return!0;return!1};/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 * 
 * Copyright (c) 2026 Lucide Icons and Contributors
 * 
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 * 
 * ---
 * 
 * The following Lucide icons are derived from the Feather project:
 * 
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 * 
 * The MIT License (MIT) (for the icons listed above)
 * 
 * Copyright (c) 2013-present Cole Bemis
 * 
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 * 
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 * 
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 * 
 */const Ce=(...e)=>e.filter((r,a,t)=>!!r&&r.trim()!==""&&t.indexOf(r)===a).join(" ").trim();var ba=jr("<svg><!><!></svg>");function q(e,r){const a=O(r,["children","$$slots","$$events","$$legacy"]),t=O(a,["name","color","size","strokeWidth","absoluteStrokeWidth","iconNode"]);Or(r,!1);let i=U(r,"name",8,void 0),s=U(r,"color",8,"currentColor"),n=U(r,"size",8,24),f=U(r,"strokeWidth",8,2),o=U(r,"absoluteStrokeWidth",8,!1),d=U(r,"iconNode",24,()=>[]);ga();var u=ba();ye(u,(b,$,S)=>({..._a,...b,...t,width:n(),height:n(),stroke:s(),"stroke-width":$,class:S}),[()=>pa(t)?void 0:{"aria-hidden":"true"},()=>(F(o()),F(f()),F(n()),ue(()=>o()?Number(f())*24/Number(n()):f())),()=>(F(Ce),F(i()),F(a),ue(()=>Ce("lucide-icon","lucide",i()?`lucide-${i()}`:"",a.class)))]);var g=Pr(u);Jr(g,1,d,Xr,(b,$)=>{var S=Rr(()=>Dr(z($),2));let l=()=>z(S)[0],v=()=>z(S)[1];var k=R(),A=H(k);ea(A,l,!0,(p,L)=>{ye(p,()=>({...v()}))}),I(b,k)});var c=zr(g);D(c,r,"default",{}),Hr(u),I(e,u),Lr()}function Ta(e,r){const a=O(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const t=[["path",{d:"M22 12h-2.48a2 2 0 0 0-1.93 1.46l-2.35 8.36a.25.25 0 0 1-.48 0L9.24 2.18a.25.25 0 0 0-.48 0l-2.35 8.36A2 2 0 0 1 4.49 12H2"}]];q(e,W({name:"activity"},()=>a,{get iconNode(){return t},children:(i,s)=>{var n=R(),f=H(n);D(f,r,"default",{}),I(i,n)},$$slots:{default:!0}}))}function wa(e,r){const a=O(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const t=[["path",{d:"M6 22a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h8a2.4 2.4 0 0 1 1.704.706l3.588 3.588A2.4 2.4 0 0 1 20 8v12a2 2 0 0 1-2 2z"}],["path",{d:"M14 2v5a1 1 0 0 0 1 1h5"}],["path",{d:"M10 9H8"}],["path",{d:"M16 13H8"}],["path",{d:"M16 17H8"}]];q(e,W({name:"file-text"},()=>a,{get iconNode(){return t},children:(i,s)=>{var n=R(),f=H(n);D(f,r,"default",{}),I(i,n)},$$slots:{default:!0}}))}function ka(e,r){const a=O(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const t=[["rect",{width:"18",height:"11",x:"3",y:"11",rx:"2",ry:"2"}],["path",{d:"M7 11V7a5 5 0 0 1 10 0v4"}]];q(e,W({name:"lock"},()=>a,{get iconNode(){return t},children:(i,s)=>{var n=R(),f=H(n);D(f,r,"default",{}),I(i,n)},$$slots:{default:!0}}))}function ya(e,r){const a=O(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const t=[["rect",{x:"16",y:"16",width:"6",height:"6",rx:"1"}],["rect",{x:"2",y:"16",width:"6",height:"6",rx:"1"}],["rect",{x:"9",y:"2",width:"6",height:"6",rx:"1"}],["path",{d:"M5 16v-3a1 1 0 0 1 1-1h12a1 1 0 0 1 1 1v3"}],["path",{d:"M12 12V8"}]];q(e,W({name:"network"},()=>a,{get iconNode(){return t},children:(i,s)=>{var n=R(),f=H(n);D(f,r,"default",{}),I(i,n)},$$slots:{default:!0}}))}function ma(e,r){const a=O(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const t=[["rect",{width:"20",height:"8",x:"2",y:"2",rx:"2",ry:"2"}],["rect",{width:"20",height:"8",x:"2",y:"14",rx:"2",ry:"2"}],["line",{x1:"6",x2:"6.01",y1:"6",y2:"6"}],["line",{x1:"6",x2:"6.01",y1:"18",y2:"18"}]];q(e,W({name:"server"},()=>a,{get iconNode(){return t},children:(i,s)=>{var n=R(),f=H(n);D(f,r,"default",{}),I(i,n)},$$slots:{default:!0}}))}function Ma(e,r){const a=O(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const t=[["path",{d:"M9.671 4.136a2.34 2.34 0 0 1 4.659 0 2.34 2.34 0 0 0 3.319 1.915 2.34 2.34 0 0 1 2.33 4.033 2.34 2.34 0 0 0 0 3.831 2.34 2.34 0 0 1-2.33 4.033 2.34 2.34 0 0 0-3.319 1.915 2.34 2.34 0 0 1-4.659 0 2.34 2.34 0 0 0-3.32-1.915 2.34 2.34 0 0 1-2.33-4.033 2.34 2.34 0 0 0 0-3.831A2.34 2.34 0 0 1 6.35 6.051a2.34 2.34 0 0 0 3.319-1.915"}],["circle",{cx:"12",cy:"12",r:"3"}]];q(e,W({name:"settings"},()=>a,{get iconNode(){return t},children:(i,s)=>{var n=R(),f=H(n);D(f,r,"default",{}),I(i,n)},$$slots:{default:!0}}))}function Ca(e,r){const a=O(r,["children","$$slots","$$events","$$legacy"]);/**
 * @license lucide-svelte v1.0.1 - ISC
 *
 * ISC License
 *
 * Copyright (c) 2026 Lucide Icons and Contributors
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 *
 * ---
 *
 * The following Lucide icons are derived from the Feather project:
 *
 * airplay, alert-circle, alert-octagon, alert-triangle, aperture, arrow-down-circle, arrow-down-left, arrow-down-right, arrow-down, arrow-left-circle, arrow-left, arrow-right-circle, arrow-right, arrow-up-circle, arrow-up-left, arrow-up-right, arrow-up, at-sign, calendar, cast, check, chevron-down, chevron-left, chevron-right, chevron-up, chevrons-down, chevrons-left, chevrons-right, chevrons-up, circle, clipboard, clock, code, columns, command, compass, corner-down-left, corner-down-right, corner-left-down, corner-left-up, corner-right-down, corner-right-up, corner-up-left, corner-up-right, crosshair, database, divide-circle, divide-square, dollar-sign, download, external-link, feather, frown, hash, headphones, help-circle, info, italic, key, layout, life-buoy, link-2, link, loader, lock, log-in, log-out, maximize, meh, minimize, minimize-2, minus-circle, minus-square, minus, monitor, moon, more-horizontal, more-vertical, move, music, navigation-2, navigation, octagon, pause-circle, percent, plus-circle, plus-square, plus, power, radio, rss, search, server, share, shopping-bag, sidebar, smartphone, smile, square, table-2, tablet, target, terminal, trash-2, trash, triangle, tv, type, upload, x-circle, x-octagon, x-square, x, zoom-in, zoom-out
 *
 * The MIT License (MIT) (for the icons listed above)
 *
 * Copyright (c) 2013-present Cole Bemis
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 */const t=[["path",{d:"m21.73 18-8-14a2 2 0 0 0-3.48 0l-8 14A2 2 0 0 0 4 21h16a2 2 0 0 0 1.73-3"}],["path",{d:"M12 9v4"}],["path",{d:"M12 17h.01"}]];q(e,W({name:"triangle-alert"},()=>a,{get iconNode(){return t},children:(i,s)=>{var n=R(),f=H(n);D(f,r,"default",{}),I(i,n)},$$slots:{default:!0}}))}export{Ta as A,wa as F,q as I,ka as L,ya as N,ma as S,Ca as T,sa as a,Xr as b,Sa as c,Ma as d,Jr as e,Ea as f,ta as g,fa as h,ga as i,da as r,D as s};
