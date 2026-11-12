#!/usr/bin/env python3
"""Serve the porting-progress dashboard over HTTP.

Usage:
  scripts/progress-server.py [--host 127.0.0.1] [--port 8787] [--interval 30]
  scripts/progress-server.py --static-out public/ [--live-url https://…]
      # write index.html + progress.json; with --live-url the static page
      # polls that server's /api/progress every second and falls back to
      # progress.json when it is unreachable.

Endpoints:
  /                 dashboard (polls /api/progress)
  /api/progress     aggregate metrics JSON
  /healthz          "ok"

The payload is aggregates only (see scripts/progress_data.py): no addresses,
function names, code, or disc assets are served, per the Atari SA grant.

Stdlib only: ThreadingHTTPServer + a background refresher. Bind 127.0.0.1 by
default; use --host 0.0.0.0 for LAN, or put it behind a tunnel/reverse proxy
for public access.
"""
import argparse
import json
import threading
import time
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

import progress_data

PAGE = """<!doctype html>
<html lang="en">
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1,viewport-fit=cover">
<meta name="color-scheme" content="dark light">
<meta name="description" content="Test Drive Le Mans (Dreamcast, 2001): SH-4 to Rust port, functions verified equivalent to the original machine code. Live aggregate counters.">
<meta property="og:title" content="Test Drive Le Mans · SH-4 → Rust port progress">
<meta property="og:description" content="Functions verified equivalent to the original SH-4 machine code, updated live.">
<title>Test Drive Le Mans · SH-4 → Rust port progress</title>
<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 4 4'%3E%3Cpath fill='%23fff' d='M0 0h4v4H0z'/%3E%3Cpath fill='%23111' d='M0 0h1v1H0zM2 0h1v1H2zM1 1h1v1H1zM3 1h1v1H3zM0 2h1v1H0zM2 2h1v1H2zM1 3h1v1H1zM3 3h1v1H3z'/%3E%3C/svg%3E">
<style>
  :root { --bg: #07090d; --panel: #0d1118; --line: #1d2531; --fg: #e8edf4; --muted: #a3adbd; --faint: #7f899a;
          --go: #22c55e; --go-hi: #86efac; --warn: #f5a524; --live: #ff4545; --track: #161c26;
          --mono: ui-monospace, "JetBrains Mono", "SF Mono", Menlo, Consolas, monospace;
          --sans: ui-sans-serif, system-ui, -apple-system, "Segoe UI", Roboto, sans-serif; }
  @media (prefers-color-scheme: light) {
    :root { --bg: #f4f6f9; --panel: #fff; --line: #dce2ea; --fg: #0b1324; --muted: #445064; --faint: #5f6a7d;
            --go: #15803d; --go-hi: #22c55e; --warn: #b45309; --track: #e3e8ef; }
  }
  * { box-sizing: border-box; }
  html { background: var(--bg); -webkit-text-size-adjust: 100%; }
  body { margin: 0; min-height: 100dvh; display: grid; place-items: center; padding: clamp(.5rem, 2.5vw, 1.5rem);
         color: var(--fg); font: 14px/1.4 var(--sans); -webkit-font-smoothing: antialiased; }
  .app { width: min(1040px, 100%); display: grid; gap: .6rem; }
  .m { font-family: var(--mono); font-variant-numeric: tabular-nums; }
  .k { font: 600 .68rem/1.2 var(--mono); letter-spacing: .07em; text-transform: uppercase; color: var(--faint); }
  .box { background: var(--panel); border: 1px solid var(--line); border-radius: 10px; }

  header { display: flex; justify-content: space-between; align-items: center; gap: .4rem .75rem; flex-wrap: wrap; }
  h1 { margin: 0; font-size: 1rem; font-weight: 650; }
  h1 span { color: var(--muted); font-weight: 450; }
  .st { display: flex; align-items: center; gap: .6rem; color: var(--faint); font: .75rem var(--mono); }
  .pill { display: inline-flex; align-items: center; gap: .45rem; padding: .2rem .55rem; border: 1px solid var(--line);
          border-radius: 999px; color: var(--muted); font: 600 .7rem/1.2 var(--mono); letter-spacing: .06em; }
  .dot { width: .5rem; height: .5rem; border-radius: 50%; background: var(--faint); }
  .pill[data-mode=live] { color: var(--fg); }
  .pill[data-mode=live] .dot { background: var(--live); animation: ping 1.6s infinite; }
  .pill[data-mode=stale], .pill[data-mode=offline] { color: var(--warn); }
  .pill[data-mode=stale] .dot, .pill[data-mode=offline] .dot { background: var(--warn); }
  .pill[data-mode=done] { color: var(--go); }
  .pill[data-mode=done] .dot { background: var(--go); }
  @keyframes ping { 0% { box-shadow: 0 0 0 0 color-mix(in srgb, var(--live) 70%, transparent); } 70%, 100% { box-shadow: 0 0 0 6px transparent; } }

  .hero { display: grid; grid-template-columns: auto 1fr; gap: 1.4rem; align-items: center; padding: .9rem 1.1rem; }
  .pct { font: 700 clamp(2.4rem, 7vw, 4rem)/1 var(--mono); letter-spacing: -.04em; }
  .pct small { font-size: .45em; color: var(--muted); }
  .of { display: flex; justify-content: space-between; align-items: baseline; gap: .3rem 1rem; flex-wrap: wrap;
        margin-bottom: .45rem; color: var(--muted); font-size: .85rem; }
  .of b { color: var(--fg); font-weight: 600; }
  .lg { display: flex; gap: .8rem; font: .72rem var(--mono); }
  .lg i { display: inline-block; width: .6rem; height: .6rem; border-radius: 2px; margin-right: .3rem; vertical-align: -1px; }
  .bar { position: relative; height: 14px; border-radius: 4px; background: var(--track); overflow: hidden; }
  .bar::after { content: ""; position: absolute; inset: 0; pointer-events: none;
                background: repeating-linear-gradient(90deg, transparent 0 calc(10% - 1px), color-mix(in srgb, var(--bg) 65%, transparent) calc(10% - 1px) 10%); }
  .seg { position: absolute; top: 0; bottom: 0; left: 0; width: 0; transition: width .8s ease, left .8s ease; }
  .seg.v { background: var(--go); }
  .seg.b { background: repeating-linear-gradient(135deg, var(--warn) 0 4px, color-mix(in srgb, var(--warn) 55%, #000) 4px 8px); }
  .scale { display: flex; justify-content: space-between; margin-top: .25rem; color: var(--faint); font: .66rem var(--mono); }

  .grid { display: grid; grid-template-columns: repeat(5, minmax(0, 1fr)); gap: 1px; background: var(--line); overflow: hidden; }
  .grid > div { background: var(--panel); padding: .55rem .8rem; min-width: 0; }
  .grid .v { display: block; margin-top: .15rem; font: 600 1.15rem/1.25 var(--mono); font-variant-numeric: tabular-nums; white-space: nowrap; }
  .grid .s { display: block; color: var(--faint); font-size: .72rem; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .go { color: var(--go); }
  .wa { color: var(--warn); }
  .delta { margin-left: .35rem; font-size: .72rem; color: var(--go); opacity: 0; transition: opacity .3s; }
  .delta.on { opacity: 1; }
  .flash { animation: flash 1.2s; }
  @keyframes flash { from { color: var(--go-hi); text-shadow: 0 0 10px var(--go); } }

  .charts { display: grid; grid-template-columns: 1fr 1fr; gap: .6rem; }
  .chart { padding: .55rem .8rem .45rem; min-width: 0; }
  .ch { display: flex; justify-content: space-between; gap: .5rem; margin-bottom: .35rem; }
  .ch .m { font-size: .7rem; color: var(--muted); white-space: nowrap; }
  .chart svg, .empty { display: block; width: 100%; height: 64px; }
  .chart rect { fill: color-mix(in srgb, var(--go) 55%, transparent); transition: height .5s, y .5s; }
  .chart rect.z { fill: var(--line); }
  .chart rect.h, .chart rect:hover { fill: var(--go); }
  .empty { display: grid; place-items: center; color: var(--faint); font: .72rem var(--mono); }
  .ax { display: flex; justify-content: space-between; margin-top: .2rem; color: var(--faint); font: .64rem var(--mono); }

  .task { display: flex; gap: .55rem; padding: .5rem .8rem; font: .8rem var(--mono); min-width: 0; }
  .task b { color: var(--go); }
  #task { white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  footer { color: var(--faint); font-size: .7rem; }

  @media (max-height: 620px) {
    .grid .s { display: none; }
    .chart svg, .empty { height: 48px; }
  }
  @media (max-width: 720px) {
    .app { gap: .5rem; }
    .hero { grid-template-columns: 1fr; gap: .4rem; padding: .7rem .8rem; }
    .lg { display: none; }
    .grid { grid-template-columns: repeat(3, minmax(0, 1fr)); }
    .grid > div { padding: .4rem .6rem; }
    .grid > .dup, .grid .s { display: none; }
    .grid .v { font-size: .98rem; }
    .chart { padding: .45rem .6rem .35rem; }
    .chart svg, .empty { height: 44px; }
    .ch .m { display: none; }
  }
  @media (max-width: 720px) and (max-height: 600px) {
    .scale { display: none; }
    .chart svg, .empty { height: 34px; }
  }
  @media (prefers-reduced-motion: reduce) { *, *::before, *::after { animation: none !important; transition: none !important; } }
</style>
<div class="app">
  <header>
    <h1>Test Drive Le Mans <span>· Dreamcast 2001 · SH-4 → Rust port</span></h1>
    <div class="st"><span class="pill" id="pill" data-mode="loading" role="status"><span class="dot"></span><span id="pill-t">LOADING</span></span><span id="pill-sub"></span></div>
  </header>

  <section class="box hero">
    <div class="pct" id="pct-wrap"><span id="pct">0.00</span><small>%</small></div>
    <div>
      <div class="of"><span><b class="m" id="v-of">—</b> / <span class="m" id="total">—</span> functions verified equivalent to SH-4 original</span>
        <span class="lg"><span><i style="background:var(--go)"></i>verified</span><span><i style="background:var(--warn)"></i>blocked</span><span><i style="background:var(--track);box-shadow:inset 0 0 0 1px var(--line)"></i>remaining</span></span></div>
      <div class="bar" id="bar" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow="0" aria-label="Functions verified">
        <div class="seg v" id="seg-v"></div><div class="seg b" id="seg-b"></div>
      </div>
      <div class="scale"><span>0</span><span>25</span><span>50</span><span>75</span><span>100%</span></div>
    </div>
  </section>

  <section class="box grid">
    <div class="dup"><span class="k">verified</span><span class="v go"><span id="verified">—</span><span class="delta" id="delta"></span></span><span class="s">diff-tested ≡ original</span></div>
    <div><span class="k">blocked</span><span class="v wa" id="blocked">—</span><span class="s">parked, not in ETA</span></div>
    <div><span class="k">remaining</span><span class="v" id="remaining">—</span><span class="s">unported</span></div>
    <div><span class="k">Δ 1h</span><span class="v" id="last_hour">—</span><span class="s">proofs / 60 min</span></div>
    <div><span class="k">Δ 24h</span><span class="v" id="last_day">—</span><span class="s">proofs / 24 h</span></div>
    <div><span class="k">rate</span><span class="v" id="rate">—</span><span class="s">proofs/h, 24 h mean</span></div>
    <div><span class="k">eta</span><span class="v" id="eta">—</span><span class="s" id="eta-s">at 24 h rate</span></div>
    <div><span class="k">last proof</span><span class="v" id="last-proof">—</span><span class="s" id="last-proof-s">since last proof</span></div>
    <div><span class="k">evidence</span><span class="v" id="evidence">—</span><span class="s">oracle result files</span></div>
    <div><span class="k">port files</span><span class="v" id="ports">—</span><span class="s">Rust modules</span></div>
  </section>

  <section class="charts">
    <div class="box chart"><div class="ch"><span class="k">proofs / min · 60 min</span><span class="m" id="cm-s"></span></div>
      <div id="chart-m"><div class="empty">no data</div></div><div class="ax"><span>−60m</span><span>−30m</span><span>now</span></div></div>
    <div class="box chart"><div class="ch"><span class="k">proofs / h · 24 h</span><span class="m" id="ch-s"></span></div>
      <div id="chart-h"><div class="empty">no data</div></div><div class="ax"><span>−24h</span><span>−12h</span><span>now</span></div></div>
  </section>

  <section class="box task"><b>›</b><span id="task">connecting…</span></section>

  <footer>This technical analysis is unofficial and is not endorsed by or affiliated with Atari SA or any of its licensors. All rights in <em>Le Mans 24 Hours</em> remain with their respective owners.</footer>
</div>
<script>
"use strict";
const $ = id => document.getElementById(id);
const LIVE_MS = 1000, HIDDEN_MS = 15000, SNAP_POLL_MS = 5000, SNAP_REFETCH_MS = 60000;
const LIVE_RETRY_MS = 15000, LIVE_FAILS_MAX = 3, TIMEOUT_MS = 3000, STALE_MS = 2 * 3600e3;
const CACHE_KEY = "lemans-progress:v2";
const isNum = v => typeof v === "number" && Number.isFinite(v);
const clamp = (v, lo, hi) => Math.min(hi, Math.max(lo, v));
const fmt = (n, dp = 0) => Number(n).toLocaleString("en-US", {minimumFractionDigits: dp, maximumFractionDigits: dp});
const pad = n => String(n).padStart(2, "0");

const S = { doc: null, source: null, liveUrl: null, liveOk: false, liveFails: 0, lastLiveTry: 0,
            lastSnap: 0, inflight: false, timer: 0, everLoaded: false, loadFailed: false };

/* ---------- data ---------- */
function valid(d) {
  return !!d && typeof d === "object" && isNum(d.total) && d.total > 0 &&
    ["verified", "blocked", "remaining", "percent", "last_hour", "last_day", "pace_per_day"]
      .every(k => isNum(d[k]) && d[k] >= 0) &&
    typeof d.updated === "string" && !Number.isNaN(Date.parse(d.updated));
}

function safeLiveUrl(u) {
  if (typeof u !== "string" || !u) return null;
  try {
    const url = new URL(u);
    if (url.protocol === "https:" || (url.protocol === "http:" && location.protocol === "http:"))
      return url.origin + url.pathname.replace(/[/]+$/, "");
  } catch (e) {}
  return null;
}

async function getJSON(url) {
  const ac = new AbortController();
  const t = setTimeout(() => ac.abort(), TIMEOUT_MS);
  try {
    const r = await fetch(url, {cache: "no-store", signal: ac.signal, credentials: "omit"});
    if (!r.ok) throw new Error("HTTP " + r.status);
    return await r.json();
  } finally { clearTimeout(t); }
}

const liveEndpoint = () => (S.liveUrl === "" ? "api/progress" : S.liveUrl + "/api/progress") + "?t=" + Date.now();

function adoptLiveUrl(u) {
  const safe = safeLiveUrl(u);
  if (safe && safe !== S.liveUrl) { S.liveUrl = safe; S.liveFails = 0; S.lastLiveTry = 0; }
}

async function trySnapshot() {
  S.lastSnap = Date.now();
  try {
    const d = await getJSON("progress.json?t=" + Date.now());
    if (!valid(d)) throw new Error("invalid");
    S.loadFailed = false;
    adoptLiveUrl(d.live_url);
    accept(d, "snapshot");
    return true;
  } catch (e) { S.loadFailed = true; return false; }
}

async function tryLive() {
  S.lastLiveTry = Date.now();
  try {
    const d = await getJSON(liveEndpoint());
    if (!valid(d)) throw new Error("invalid");
    S.liveFails = 0; S.liveOk = true;
    accept(d, "live");
    return true;
  } catch (e) {
    S.liveFails++;
    if (S.liveFails >= LIVE_FAILS_MAX) S.liveOk = false;
    return false;
  }
}

async function poll() {
  if (S.inflight) return;
  S.inflight = true;
  try {
    if (!navigator.onLine) return;
    const now = Date.now();
    const liveDue = S.liveUrl !== null && (S.liveOk || S.liveFails < LIVE_FAILS_MAX || now - S.lastLiveTry >= LIVE_RETRY_MS);
    if (liveDue && await tryLive()) return;
    if (!S.liveOk && (!S.everLoaded || now - S.lastSnap >= SNAP_REFETCH_MS)) await trySnapshot();
  } finally {
    S.inflight = false;
    schedule();
    status();
  }
}

function schedule() {
  clearTimeout(S.timer);
  if (!navigator.onLine) return;
  S.timer = setTimeout(poll, document.hidden ? HIDDEN_MS : S.liveOk ? LIVE_MS : SNAP_POLL_MS);
}

function accept(d, source) {
  const prev = S.doc;
  if (prev && Date.parse(d.updated) < Date.parse(prev.updated)) { S.source = source; return; }
  S.doc = d; S.source = source; S.everLoaded = true;
  render(d, prev);
  try { localStorage.setItem(CACHE_KEY, JSON.stringify(d)); } catch (e) {}
}

/* ---------- formatting ---------- */
function ago(ms) {
  const s = Math.max(0, Math.round(ms / 1000));
  if (s < 60) return s + "s";
  const m = Math.floor(s / 60);
  if (m < 60) return m + "m " + pad(s % 60) + "s";
  const h = Math.floor(m / 60);
  if (h < 48) return h + "h " + pad(m % 60) + "m";
  return Math.floor(h / 24) + "d " + (h % 24) + "h";
}

function stamp(ms) {
  const t = new Date(ms);
  return t.getFullYear() + "-" + pad(t.getMonth() + 1) + "-" + pad(t.getDate()) + " " + pad(t.getHours()) + ":" + pad(t.getMinutes());
}

function bars(hostId, data, unit) {
  const host = $(hostId);
  if (!Array.isArray(data) || !data.length || !data.every(v => isNum(v) && v >= 0)) {
    host.innerHTML = '<div class="empty">no data</div>';
    return null;
  }
  const n = data.length, H = 64, max = Math.max(1, ...data), NS = "http://www.w3.org/2000/svg";
  let svg = host.querySelector("svg");
  if (!svg || svg.childElementCount !== n) {
    host.innerHTML = "";
    svg = document.createElementNS(NS, "svg");
    svg.setAttribute("viewBox", "0 0 " + n * 10 + " " + H);
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("role", "img");
    for (let i = 0; i < n; i++) {
      const r = document.createElementNS(NS, "rect");
      r.setAttribute("x", i * 10 + 1.5); r.setAttribute("width", 7); r.setAttribute("rx", 1);
      r.setAttribute("y", H); r.setAttribute("height", 0);
      r.appendChild(document.createElementNS(NS, "title"));
      svg.appendChild(r);
    }
    host.appendChild(svg);
  }
  const sum = data.reduce((a, b) => a + b, 0), peak = Math.max(...data);
  svg.setAttribute("aria-label", "proofs per " + unit + ": total " + sum + ", peak " + peak);
  [...svg.children].forEach((r, i) => {
    const v = data[i], h = v ? Math.max(3, v / max * (H - 2)) : 2;
    r.setAttribute("y", H - h); r.setAttribute("height", h);
    r.setAttribute("class", v === 0 ? "z" : i === n - 1 ? "h" : "");
    r.firstChild.textContent = v + " proofs · t−" + (n - 1 - i) + unit;
  });
  return "Σ " + fmt(sum) + " · peak " + fmt(peak);
}

/* ---------- rendering ---------- */
let deltaTimer = 0;
function render(d, prev) {
  const pct = clamp(d.percent, 0, 100);
  const blPct = clamp(100 * d.blocked / d.total, 0, 100 - pct);
  $("pct").textContent = pct.toFixed(2);
  $("v-of").textContent = fmt(d.verified);
  $("total").textContent = fmt(d.total);
  $("seg-v").style.width = pct + "%";
  $("seg-b").style.left = pct + "%";
  $("seg-b").style.width = blPct + "%";
  $("bar").setAttribute("aria-valuenow", pct.toFixed(2));

  $("verified").textContent = fmt(d.verified);
  $("blocked").textContent = fmt(d.blocked);
  $("remaining").textContent = fmt(d.remaining);
  $("last_hour").textContent = fmt(d.last_hour);
  $("last_day").textContent = fmt(d.last_day);
  const rate = d.last_day / 24;
  $("rate").textContent = fmt(rate, rate < 10 ? 1 : 0) + "/h";
  $("evidence").textContent = isNum(d.evidence_files) ? fmt(d.evidence_files) : "—";
  $("ports").textContent = isNum(d.port_files) ? fmt(d.port_files) : "—";

  let eta = "—", etaS = "no rate (0 in 24 h)";
  if (pct >= 100) { eta = "done"; etaS = "all verified"; }
  else if (d.remaining === 0) etaS = "only blocked left";
  else if (isNum(d.eta_days) && d.eta_days >= 0) {
    const days = d.eta_days;
    eta = days < 1 ? (days * 24).toFixed(1) + " h" : days < 1000 ? days.toFixed(1) + " d" : (days / 365.25).toFixed(1) + " y";
    etaS = "≈ " + stamp(Date.parse(d.updated) + days * 864e5);
  }
  $("eta").textContent = eta;
  $("eta-s").textContent = etaS;

  $("cm-s").textContent = bars("chart-m", d.minutely, "m") || "";
  $("ch-s").textContent = bars("chart-h", d.hourly, "h") || "";
  $("task").textContent = pct >= 100 ? "done — all functions verified" : String(d.current_task || "—");

  if (prev && d.verified > prev.verified) {
    const v = $("verified"), dl = $("delta");
    v.classList.remove("flash"); void v.offsetWidth; v.classList.add("flash");
    dl.textContent = "+" + fmt(d.verified - prev.verified);
    dl.classList.add("on");
    clearTimeout(deltaTimer);
    deltaTimer = setTimeout(() => dl.classList.remove("on"), 2500);
  }
  status();
}

function status() {
  const d = S.doc, pill = $("pill");
  let mode, label, sub = "";
  if (!navigator.onLine) { mode = "offline"; label = "OFFLINE"; }
  else if (!d && S.loadFailed) { mode = "offline"; label = "NO DATA"; sub = "retrying"; $("task").textContent = "progress.json unreachable or invalid — retrying"; }
  else if (!d) { mode = "loading"; label = "LOADING"; }
  else if (d.percent >= 100) mode = "done", label = "DONE";
  else if (Date.now() - Date.parse(d.updated) > STALE_MS) mode = "stale", label = "STALE";
  else if (S.source === "live" && S.liveOk) mode = "live", label = "LIVE · 1 Hz";
  else if (S.source === "cache") mode = "idle", label = "CACHED";
  else if (S.liveUrl !== null && !S.liveOk) mode = "idle", label = "RECONNECTING";
  else mode = "idle", label = "SNAPSHOT · 10 min";
  if (d) {
    sub = "upd " + ago(Date.now() - Date.parse(d.updated)) + " ago";
    const lp = Date.parse(d.latest || "");
    $("last-proof").textContent = Number.isNaN(lp) ? "—" : ago(Date.now() - lp);
    $("last-proof-s").textContent = Number.isNaN(lp) ? "no proofs yet" : "ago · " + stamp(lp).slice(11);
    document.title = clamp(d.percent, 0, 100).toFixed(1) + "% · TD Le Mans SH-4 → Rust";
  }
  if (pill.dataset.mode !== mode) pill.dataset.mode = mode;
  $("pill-t").textContent = label;
  $("pill-sub").textContent = sub;
}

/* ---------- lifecycle ---------- */
document.addEventListener("visibilitychange", () => { if (!document.hidden) { clearTimeout(S.timer); poll(); } else schedule(); });
addEventListener("online", () => { status(); poll(); });
addEventListener("offline", () => { clearTimeout(S.timer); status(); });
setInterval(status, 1000);

(async function start() {
  try {
    const cached = JSON.parse(localStorage.getItem(CACHE_KEY) || "null");
    if (valid(cached)) { S.doc = cached; S.source = "cache"; render(cached, null); }
  } catch (e) {}
  if (location.protocol.startsWith("http")) {
    S.liveUrl = "";
    if (await tryLive()) { schedule(); return; }
    S.liveUrl = null; S.liveFails = 0;
  }
  await trySnapshot();
  if (S.liveUrl !== null) await tryLive();
  schedule();
  status();
})();
</script>
</html>
"""


class State:
    """Latest snapshot, refreshed by a background thread."""

    def __init__(self, interval: float):
        self.interval = interval
        self.doc = progress_data.snapshot()
        self.lock = threading.Lock()
        self.stop = threading.Event()
        self.thread = threading.Thread(target=self._loop, daemon=True)
        self.thread.start()

    def _loop(self):
        while not self.stop.wait(self.interval):
            try:
                doc = progress_data.snapshot()
            except Exception:  # a scrape failure must not kill the server
                continue
            with self.lock:
                self.doc = doc

    def get(self):
        with self.lock:
            return self.doc

    def refresh(self):
        doc = progress_data.snapshot()
        with self.lock:
            self.doc = doc


class Handler(BaseHTTPRequestHandler):
    state: State = None  # set in main()

    def _send(self, code, body: bytes, ctype: str):
        self.send_response(code)
        self.send_header("Content-Type", ctype)
        self.send_header("Content-Length", str(len(body)))
        self.send_header("Cache-Control", "no-store")
        self.send_header("Access-Control-Allow-Origin", "*")  # aggregates only; polled cross-origin by the Pages copy
        self.send_header("X-Content-Type-Options", "nosniff")
        self.end_headers()
        if self.command != "HEAD":
            self.wfile.write(body)

    def do_GET(self):
        path = self.path.split("?", 1)[0]
        if path == "/":
            self._send(200, PAGE.encode(), "text/html; charset=utf-8")
        elif path == "/api/progress":
            body = json.dumps(self.state.get(), indent=2).encode()
            self._send(200, body, "application/json")
        elif path == "/healthz":
            self._send(200, b"ok\n", "text/plain; charset=utf-8")
        else:
            self._send(404, b"not found\n", "text/plain; charset=utf-8")

    do_HEAD = do_GET

    def log_message(self, fmt, *args):  # keep the terminal clean
        pass


def write_static(out_dir: Path, state: State, live_url: str | None = None):
    out_dir.mkdir(parents=True, exist_ok=True)
    state.refresh()
    doc = dict(state.get())
    if live_url:
        doc["live_url"] = live_url.rstrip("/")  # page polls <live_url>/api/progress every second
    (out_dir / "index.html").write_text(PAGE, encoding="utf-8")
    (out_dir / "progress.json").write_text(json.dumps(doc, indent=2) + "\n", encoding="utf-8")
    return out_dir


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=8787)
    ap.add_argument("--interval", type=float, default=30.0)
    ap.add_argument("--static-out", type=Path, default=None,
                    help="write index.html + progress.json here and exit")
    ap.add_argument("--live-url", default=None,
                    help="with --static-out: public base URL of a running server for per-second updates")
    args = ap.parse_args()

    state = State(args.interval)
    if args.static_out:
        out = write_static(args.static_out, state, args.live_url)
        print(f"wrote {out}/index.html and {out}/progress.json")
        return

    Handler.state = state
    srv = ThreadingHTTPServer((args.host, args.port), Handler)
    print(f"serving on http://{args.host}:{args.port}/  (aggregates only; Ctrl-C to stop)")
    try:
        srv.serve_forever()
    except KeyboardInterrupt:
        pass
    finally:
        state.stop.set()
        srv.server_close()


if __name__ == "__main__":
    main()
