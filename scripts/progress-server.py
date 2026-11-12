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
<meta name="theme-color" content="#06080c" media="(prefers-color-scheme: dark)">
<meta name="theme-color" content="#f3f5f8" media="(prefers-color-scheme: light)">
<meta name="description" content="Live progress of a function-by-function, machine-verified Rust port of Test Drive Le Mans (Sega Dreamcast).">
<title>Test Drive Le Mans — Rust port progress</title>
<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 4 4'%3E%3Cpath fill='%23fff' d='M0 0h4v4H0z'/%3E%3Cpath fill='%23111' d='M0 0h1v1H0zM2 0h1v1H2zM1 1h1v1H1zM3 1h1v1H3zM0 2h1v1H0zM2 2h1v1H2zM1 3h1v1H1zM3 3h1v1H3z'/%3E%3C/svg%3E">
<style>
  :root {
    --bg: #06080c; --panel: rgba(255,255,255,.032); --panel-hi: rgba(255,255,255,.06);
    --line: rgba(255,255,255,.085); --line-hi: rgba(255,255,255,.16);
    --fg: #f1f4f8; --muted: #8d97a7; --faint: #586172;
    --go: #22c55e; --go-hi: #86efac; --go-lo: #15803d; --go-glow: rgba(34,197,94,.38);
    --warn: #f5a524; --warn-lo: #a86b0c; --live: #ff3d3d; --track: #10141c;
    --sans: "Inter", "Inter var", ui-sans-serif, system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
    --mono: "JetBrains Mono", ui-monospace, "SF Mono", SFMono-Regular, Menlo, Consolas, monospace;
    --ease: cubic-bezier(.2,.8,.2,1);
  }
  @media (prefers-color-scheme: light) {
    :root { --bg: #f3f5f8; --panel: rgba(255,255,255,.82); --panel-hi: #ffffff;
            --line: rgba(15,23,42,.09); --line-hi: rgba(15,23,42,.18);
            --fg: #0d1526; --muted: #566175; --faint: #8b95a7; --track: #e2e7ee;
            --go: #16a34a; --go-hi: #4ade80; --go-lo: #166534; --go-glow: rgba(22,163,74,.25);
            --warn: #d98a07; --warn-lo: #a15f00; }
  }
  * { box-sizing: border-box; }
  html { background: var(--bg); -webkit-text-size-adjust: 100%; }
  body {
    margin: 0; min-height: 100dvh; color: var(--fg); font: 15px/1.55 var(--sans);
    -webkit-font-smoothing: antialiased; display: flex; justify-content: center;
    padding: max(env(safe-area-inset-top), clamp(1.25rem, 5vw, 3.75rem)) clamp(.9rem, 4vw, 2rem) 2.5rem;
    overflow-x: hidden;
  }
  body::before, body::after { content: ""; position: fixed; inset: -20%; z-index: -1; pointer-events: none; }
  body::before {
    background:
      radial-gradient(40% 35% at 18% 12%, rgba(34,197,94,.16), transparent 70%),
      radial-gradient(35% 30% at 88% 6%, rgba(255,61,61,.10), transparent 70%),
      radial-gradient(45% 40% at 60% 100%, rgba(56,132,255,.08), transparent 70%);
    animation: drift 28s ease-in-out infinite alternate;
  }
  body::after {
    inset: 0; opacity: .5;
    background-image: linear-gradient(var(--line) 1px, transparent 1px), linear-gradient(90deg, var(--line) 1px, transparent 1px);
    background-size: 56px 56px;
    -webkit-mask-image: radial-gradient(ellipse 80% 55% at 50% 0%, #000 20%, transparent 75%);
            mask-image: radial-gradient(ellipse 80% 55% at 50% 0%, #000 20%, transparent 75%);
  }
  @keyframes drift { to { transform: translate3d(3%, 2%, 0) scale(1.06); } }

  .wrap { width: min(1080px, 100%); }
  .num { font-family: var(--mono); font-variant-numeric: tabular-nums; letter-spacing: -.02em; }
  .eyebrow { font: 600 .7rem/1.2 var(--mono); letter-spacing: .17em; text-transform: uppercase; color: var(--muted); }
  .panel { position: relative; background: var(--panel); border: 1px solid var(--line); border-radius: 22px;
           -webkit-backdrop-filter: blur(14px); backdrop-filter: blur(14px);
           box-shadow: 0 1px 0 rgba(255,255,255,.04) inset, 0 24px 60px -30px rgba(0,0,0,.6); }
  @media (prefers-color-scheme: light) { .panel { box-shadow: 0 1px 0 #fff inset, 0 20px 50px -30px rgba(15,23,42,.25); } }

  /* header */
  header { display: flex; align-items: flex-start; justify-content: space-between; gap: 1.25rem;
           margin-bottom: clamp(1.5rem, 4vw, 2.5rem); }
  h1 { margin: .55rem 0 .4rem; font-size: clamp(1.7rem, 4vw, 2.6rem); line-height: 1.05; font-weight: 780;
       letter-spacing: -.03em; text-wrap: balance; }
  h1 span { color: var(--muted); font-weight: 500; }
  .lede { margin: 0; color: var(--muted); max-width: 62ch; text-wrap: pretty; }
  .status { flex: none; display: flex; flex-direction: column; align-items: flex-end; gap: .45rem; }
  .pill { display: inline-flex; align-items: center; gap: .55rem; padding: .5rem .85rem .5rem .75rem;
          border: 1px solid var(--line-hi); border-radius: 999px; background: var(--panel);
          font: 700 .7rem/1 var(--mono); letter-spacing: .16em; text-transform: uppercase; white-space: nowrap;
          transition: color .3s, border-color .3s; }
  .dot { position: relative; width: .55rem; height: .55rem; border-radius: 50%; background: var(--faint); transition: background .3s; }
  .pill[data-mode="live"] .dot { background: var(--live); }
  .pill[data-mode="live"] .dot::after { content: ""; position: absolute; inset: 0; border-radius: 50%;
                                        background: var(--live); animation: ping 1.6s var(--ease) infinite; }
  .pill[data-mode="stale"], .pill[data-mode="offline"] { color: var(--warn); border-color: color-mix(in srgb, var(--warn) 45%, transparent); }
  .pill[data-mode="stale"] .dot, .pill[data-mode="offline"] .dot { background: var(--warn); }
  .pill[data-mode="finished"] { color: var(--go); border-color: color-mix(in srgb, var(--go) 45%, transparent); }
  .pill[data-mode="finished"] .dot { background: var(--go); }
  .status small { color: var(--faint); font: .72rem/1.3 var(--mono); text-align: right; }
  @keyframes ping { 0% { transform: scale(1); opacity: .7; } 100% { transform: scale(3.2); opacity: 0; } }

  /* hero */
  .hero { display: grid; grid-template-columns: 1fr auto; gap: clamp(1.25rem, 4vw, 3rem); align-items: center;
          padding: clamp(1.25rem, 3.5vw, 2.4rem); overflow: hidden; }
  .hero::before { content: ""; position: absolute; inset: 0; border-radius: inherit; pointer-events: none;
                  background: radial-gradient(60% 90% at 0% 0%, rgba(34,197,94,.10), transparent 60%); }
  .pct { font-size: clamp(4.2rem, 14vw, 8.5rem); font-weight: 700; line-height: .82; letter-spacing: -.05em;
         background: linear-gradient(180deg, var(--fg) 30%, color-mix(in srgb, var(--fg) 55%, var(--go)));
         -webkit-background-clip: text; background-clip: text; color: transparent; }
  .pct small { font-size: .36em; letter-spacing: 0; margin-left: .06em; -webkit-text-fill-color: var(--muted); }
  .of { margin: 1rem 0 1.6rem; color: var(--muted); font-size: 1.02rem; }
  .of b { color: var(--fg); font-weight: 600; }

  .track-wrap { position: relative; padding-top: 1.6rem; }
  .track { position: relative; height: 32px; border-radius: 11px; overflow: hidden; background: var(--track);
           box-shadow: inset 0 1px 3px rgba(0,0,0,.45), inset 0 0 0 1px var(--line); }
  .track::before { content: ""; position: absolute; inset: 0; z-index: 1; pointer-events: none;
                   background: repeating-linear-gradient(90deg, transparent 0 calc(4.1667% - 1px), var(--line) calc(4.1667% - 1px) 4.1667%); }
  .seg { position: absolute; top: 0; bottom: 0; left: 0; width: 0;
         transition: width 1.2s var(--ease), left 1.2s var(--ease); }
  .seg.go { z-index: 2; background: linear-gradient(90deg, var(--go-lo), var(--go) 72%, var(--go-hi)); box-shadow: 0 0 28px var(--go-glow); }
  .seg.go::after { content: ""; position: absolute; inset: 0 0 52% 0; background: linear-gradient(rgba(255,255,255,.26), transparent); }
  .seg.go::before { content: ""; position: absolute; inset: 0; opacity: .5;
                    background: linear-gradient(90deg, transparent, rgba(255,255,255,.45), transparent) 0 0 / 40% 100% no-repeat;
                    animation: sheen 3.2s ease-in-out infinite; }
  .seg.bl { z-index: 2; background: repeating-linear-gradient(135deg, var(--warn) 0 6px, var(--warn-lo) 6px 12px); opacity: .92; }
  .flag { position: absolute; right: 0; top: 0; bottom: 0; width: 20px; z-index: 3; opacity: .9;
          background: conic-gradient(#fff 25%, #0d0d0d 0 50%, #fff 0 75%, #0d0d0d 0) 0 0 / 10px 10.67px;
          box-shadow: -6px 0 12px rgba(0,0,0,.35); }
  .car { position: absolute; top: 1.6rem; height: 32px; width: 0; left: 0; z-index: 4; transition: left 1.2s var(--ease); }
  .car::before { content: ""; position: absolute; left: -1.5px; top: -4px; bottom: -4px; width: 3px; border-radius: 2px;
                 background: #fff; box-shadow: 0 0 14px 3px var(--go-hi); }
  .pop { position: absolute; bottom: 100%; left: 0; transform: translateX(-50%); margin-bottom: .1rem;
         font: 700 .85rem/1 var(--mono); color: var(--go-hi); white-space: nowrap; text-shadow: 0 0 12px var(--go-glow);
         animation: pop 1.8s var(--ease) forwards; pointer-events: none; }
  .tag { position: absolute; bottom: 100%; left: 0; transform: translateX(-50%); margin-bottom: .45rem;
         font: 600 .66rem/1 var(--mono); letter-spacing: .12em; color: var(--muted); white-space: nowrap; }
  @keyframes pop { 0% { opacity: 0; transform: translate(-50%, 6px) scale(.8); } 15% { opacity: 1; transform: translate(-50%, 0) scale(1.08); }
                   70% { opacity: 1; } 100% { opacity: 0; transform: translate(-50%, -18px) scale(1); } }
  @keyframes sheen { 0% { background-position: -60% 0; } 60%, 100% { background-position: 160% 0; } }
  .legend { display: flex; flex-wrap: wrap; gap: .5rem 1.4rem; margin-top: .95rem; color: var(--muted); font-size: .82rem; }
  .legend i { display: inline-block; width: .7rem; height: .7rem; border-radius: 3px; margin-right: .45rem; vertical-align: -1px; }
  .legend b { color: var(--fg); font-weight: 600; font-family: var(--mono); font-variant-numeric: tabular-nums; margin-left: .3rem; }

  .gauge { position: relative; width: clamp(190px, 24vw, 250px); aspect-ratio: 1; }
  .gauge svg { width: 100%; height: 100%; overflow: visible; }
  .ring { fill: none; stroke-width: 11; transform: rotate(-90deg); transform-origin: 100px 100px;
          transition: stroke-dasharray 1.2s var(--ease), stroke-dashoffset 1.2s var(--ease); }
  .ring.bg { stroke: var(--track); }
  .ring.go { stroke: url(#g-go); filter: drop-shadow(0 0 6px var(--go-glow)); }
  .ring.bl { stroke: var(--warn); opacity: .85; }
  .tick { stroke: var(--faint); stroke-width: 1.2; stroke-linecap: round; }
  .tick.major { stroke: var(--muted); stroke-width: 2; }
  .tick.done { stroke: var(--go); }
  .knob { fill: #fff; filter: drop-shadow(0 0 5px var(--go-hi)); transition: cx 1.2s var(--ease), cy 1.2s var(--ease); }
  .gauge-c { position: absolute; inset: 0; display: grid; place-content: center; text-align: center; gap: .3rem; }
  .gauge-c b { font-size: clamp(1.9rem, 3.6vw, 2.45rem); font-weight: 650; line-height: 1; }
  .gauge-c span:last-child { color: var(--faint); font: .74rem/1 var(--mono); }

  /* now strip */
  .now { display: flex; align-items: center; gap: 1rem; margin: 1rem 0; padding: 1rem 1.25rem; }
  .now .eyebrow { flex: none; display: inline-flex; align-items: center; gap: .55rem; padding-right: 1rem; border-right: 1px solid var(--line); }
  .now p { margin: 0; flex: 1; min-width: 0; font-size: 1.04rem; overflow-wrap: anywhere; }
  .now small { flex: none; color: var(--faint); font: .78rem/1.2 var(--mono); }
  .now small b { color: var(--muted); font-weight: 600; }

  /* stats */
  .stats { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 1rem; }
  .stat { padding: 1.15rem 1.25rem 1.2rem; overflow: hidden; transition: border-color .4s; }
  .stat .num { display: block; margin-top: .6rem; font-size: clamp(1.55rem, 3vw, 2.1rem); font-weight: 650; line-height: 1.05;
               overflow-wrap: anywhere; }
  .stat p { margin: .4rem 0 0; color: var(--faint); font-size: .8rem; }
  .stat.go .num { color: var(--go); }
  .stat.bl .num { color: var(--warn); }
  .stat.bump { border-color: color-mix(in srgb, var(--go) 55%, transparent); }
  .up { color: var(--go); font-weight: 600; }

  /* charts */
  .charts { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-top: 1rem; }
  .chart { padding: 1.15rem 1.25rem 1rem; }
  .chart-h { display: flex; justify-content: space-between; align-items: baseline; gap: 1rem; margin-bottom: .85rem; }
  .chart-h small { color: var(--faint); font: .74rem/1 var(--mono); }
  .chart-h small b { color: var(--fg); font-weight: 600; }
  .chart svg { display: block; width: 100%; height: 86px; }
  .chart rect { fill: color-mix(in srgb, var(--go) 42%, transparent); transition: height .6s var(--ease), y .6s var(--ease); }
  .chart rect.hot { fill: var(--go); }
  .chart rect.zero { fill: var(--line); }
  .chart rect:hover { fill: var(--go-hi); }
  .axis { display: flex; justify-content: space-between; margin-top: .45rem; color: var(--faint); font: .68rem/1 var(--mono); }
  .empty { display: grid; place-items: center; height: 86px; color: var(--faint); font-size: .82rem; }

  .mini { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); margin-top: 1rem; overflow: hidden; }
  .mini div { padding: 1rem 1.25rem; border-left: 1px solid var(--line); min-width: 0; }
  .mini div:first-child { border-left: 0; }
  .mini .num { display: block; margin-top: .4rem; font-size: 1.15rem; font-weight: 600; }

  footer { margin-top: 2rem; padding-top: 1.25rem; border-top: 1px solid var(--line); color: var(--faint); font-size: .78rem; }
  footer .num { color: var(--muted); }
  .notice { margin: .6rem 0 0; max-width: 82ch; opacity: .85; }
  noscript { display: block; margin-bottom: 1rem; padding: .8rem 1rem; border: 1px solid var(--warn); border-radius: 12px; color: var(--warn); }

  /* finished */
  .finished .seg.go::before { animation: none; }
  .finished .flag { width: 34px; animation: wave 1.2s ease-in-out infinite alternate; }
  @keyframes wave { to { transform: skewY(-6deg) scaleX(1.1); } }

  @media (max-width: 860px) {
    .hero { grid-template-columns: 1fr; }
    .gauge { justify-self: center; order: -1; }
    .stats, .mini { grid-template-columns: repeat(2, minmax(0, 1fr)); }
    .charts { grid-template-columns: 1fr; }
    .mini div:nth-child(3) { border-left: 0; }
    .mini div:nth-child(n+3) { border-top: 1px solid var(--line); }
  }
  @media (max-width: 560px) {
    header { flex-direction: column; }
    .status { align-items: flex-start; }
    .status small { text-align: left; }
    .now { flex-direction: column; align-items: flex-start; gap: .55rem; }
    .now .eyebrow { border: 0; padding: 0; }
    .stat { padding: 1rem; }
  }
  @media (max-width: 360px) { .stats, .mini { grid-template-columns: 1fr; } .mini div { border-left: 0 !important; border-top: 1px solid var(--line); } .mini div:first-child { border-top: 0; } }
  @media (prefers-reduced-motion: reduce) {
    *, *::before, *::after { animation: none !important; transition: none !important; }
  }
  @media print {
    body::before, body::after { display: none; }
    .panel { box-shadow: none; -webkit-backdrop-filter: none; backdrop-filter: none; }
  }
</style>
<div class="wrap" id="app">
  <noscript>This dashboard needs JavaScript to load its numbers. Raw data: <a href="progress.json">progress.json</a></noscript>
  <header>
    <div>
      <div class="eyebrow">Sega Dreamcast &middot; 2001 &middot; SH-4 &rarr; Rust</div>
      <h1>Test Drive Le Mans <span>&mdash; port progress</span></h1>
      <p class="lede">Every function of the original game is being rewritten in Rust and machine-checked against the
        original code. This board shows, live, how many have crossed the finish line.</p>
    </div>
    <div class="status">
      <div class="pill" id="pill" data-mode="loading" role="status" aria-live="polite"><span class="dot"></span><span id="pill-t">Connecting</span></div>
      <small id="pill-sub">&nbsp;</small>
    </div>
  </header>

  <section class="panel hero">
    <div>
      <div class="pct num" aria-hidden="true"><span id="pct">0.0</span><small>%</small></div>
      <div class="of"><b class="num" id="verified-of">&mdash;</b> of <span class="num" id="total">&mdash;</span>
        functions proven identical to the original</div>
      <div class="track-wrap">
        <div class="track" id="track" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow="0" aria-label="Share of functions verified">
          <div class="seg go" id="seg-go"></div>
          <div class="seg bl" id="seg-bl"></div>
          <div class="flag" title="Finish line"></div>
        </div>
        <div class="car" id="car"><span class="tag" id="car-tag"></span></div>
      </div>
      <div class="legend">
        <span><i style="background:var(--go)"></i>Verified<b id="lg-go">&mdash;</b></span>
        <span><i style="background:var(--warn)"></i>Blocked<b id="lg-bl">&mdash;</b></span>
        <span><i style="background:var(--track);box-shadow:inset 0 0 0 1px var(--line-hi)"></i>Still to go<b id="lg-rest">&mdash;</b></span>
      </div>
    </div>
    <div class="gauge" aria-hidden="true">
      <svg viewBox="0 0 200 200">
        <defs><linearGradient id="g-go" x1="0" y1="0" x2="1" y2="1"><stop offset="0" stop-color="var(--go-hi)"/><stop offset="1" stop-color="var(--go-lo)"/></linearGradient></defs>
        <g id="ticks"></g>
        <circle class="ring bg" cx="100" cy="100" r="78" pathLength="1000"/>
        <circle class="ring bl" id="ring-bl" cx="100" cy="100" r="78" pathLength="1000" stroke-dasharray="0 1000"/>
        <circle class="ring go" id="ring-go" cx="100" cy="100" r="78" pathLength="1000" stroke-dasharray="0 1000"/>
        <circle class="knob" id="knob" cx="100" cy="22" r="5.5"/>
      </svg>
      <div class="gauge-c"><span class="eyebrow">Race clock</span><b class="num" id="clock">00:00</b><span>of 24:00</span></div>
    </div>
  </section>

  <section class="panel now">
    <span class="eyebrow"><span class="dot" id="now-dot"></span>Right now</span>
    <p id="task">Checking in with the pit wall&hellip;</p>
    <small id="last-proof-wrap" hidden>last proof <b class="num" id="last-proof">&mdash;</b></small>
  </section>

  <section class="stats">
    <div class="panel stat go" id="st-verified"><span class="eyebrow">Verified</span><span class="num" id="verified">&mdash;</span><p id="verified-sub">proven equivalent</p></div>
    <div class="panel stat"><span class="eyebrow">Still to go</span><span class="num" id="remaining">&mdash;</span><p>not yet proven</p></div>
    <div class="panel stat bl"><span class="eyebrow">Blocked</span><span class="num" id="blocked">&mdash;</span><p>parked for later</p></div>
    <div class="panel stat"><span class="eyebrow">Finish in</span><span class="num" id="eta">&mdash;</span><p id="eta-date">at current pace</p></div>
  </section>

  <section class="charts">
    <div class="panel chart">
      <div class="chart-h"><span class="eyebrow">Last 60 minutes</span><small>peak <b class="num" id="peak-m">&mdash;</b>/min</small></div>
      <div id="chart-m"><div class="empty">Waiting for activity data&hellip;</div></div>
      <div class="axis"><span>60 min ago</span><span>30</span><span>now</span></div>
    </div>
    <div class="panel chart">
      <div class="chart-h"><span class="eyebrow">Last 24 hours</span><small>peak <b class="num" id="peak-h">&mdash;</b>/hour</small></div>
      <div id="chart-h"><div class="empty">Waiting for activity data&hellip;</div></div>
      <div class="axis"><span>24 h ago</span><span>12 h</span><span>now</span></div>
    </div>
  </section>

  <section class="panel mini">
    <div><span class="eyebrow">Last hour</span><span class="num" id="last_hour">&mdash;</span></div>
    <div><span class="eyebrow">Pace / day</span><span class="num" id="pace">&mdash;</span></div>
    <div><span class="eyebrow">Avg / hour</span><span class="num" id="per-hour">&mdash;</span></div>
    <div><span class="eyebrow">Evidence files</span><span class="num" id="evidence">&mdash;</span></div>
  </section>

  <footer>
    <div>Data from <span id="src">&mdash;</span> &middot; updated <span class="num" id="ago">&mdash;</span></div>
    <p class="notice">This technical analysis is unofficial and is not endorsed by or affiliated with Atari SA or any of its licensors. All rights in <em>Le Mans 24 Hours</em> remain with their respective owners.</p>
  </footer>
</div>
<script>
"use strict";
const $ = id => document.getElementById(id);
const LIVE_MS = 1000, HIDDEN_MS = 15000, SNAP_POLL_MS = 5000, SNAP_REFETCH_MS = 60000;
const LIVE_RETRY_MS = 15000, LIVE_FAILS_MAX = 3, TIMEOUT_MS = 3000, STALE_MS = 2 * 3600e3;
const CACHE_KEY = "lemans-progress:v2";
const still = matchMedia("(prefers-reduced-motion: reduce)").matches;
const isNum = v => typeof v === "number" && Number.isFinite(v);
const clamp = (v, lo, hi) => Math.min(hi, Math.max(lo, v));
const fmt = (n, dp = 0) => Number(n).toLocaleString("en-US", {minimumFractionDigits: dp, maximumFractionDigits: dp});

const S = { doc: null, source: null, liveUrl: null, liveOk: false, liveFails: 0, lastLiveTry: 0,
            lastSnap: 0, inflight: false, timer: 0, everLoaded: false };

/* ---------- data ---------- */
function valid(d) {
  return !!d && typeof d === "object" && isNum(d.total) && d.total > 0 &&
    ["verified", "blocked", "remaining", "percent", "last_hour", "last_day", "pace_per_day", "evidence_files"]
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
    if (!valid(d)) return false;
    adoptLiveUrl(d.live_url);
    accept(d, "snapshot");
    return true;
  } catch (e) { return false; }
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
  const ms = document.hidden ? HIDDEN_MS : S.liveOk ? LIVE_MS : SNAP_POLL_MS;
  S.timer = setTimeout(poll, ms);
}

function accept(d, source) {
  const prev = S.doc;
  if (prev && Date.parse(d.updated) < Date.parse(prev.updated)) {
    S.source = source;
    return;
  }
  S.doc = d; S.source = source; S.everLoaded = true;
  render(d, prev);
  try { localStorage.setItem(CACHE_KEY, JSON.stringify(d)); } catch (e) {}
}

/* ---------- animation ---------- */
const shown = {}, token = {};
function count(id, to, dp = 0) {
  const node = $(id);
  const from = isNum(shown[id]) ? shown[id] : 0;
  const my = token[id] = (token[id] || 0) + 1;
  if (still || from === to) { shown[id] = to; node.textContent = fmt(to, dp); return; }
  const dur = S.firstPaintDone ? 650 : 1300, t0 = performance.now();
  const step = t => {
    if (token[id] !== my) return;
    const p = Math.min((t - t0) / dur, 1), e = 1 - Math.pow(1 - p, 3);
    shown[id] = from + (to - from) * e;
    node.textContent = fmt(shown[id], dp);
    if (p < 1) requestAnimationFrame(step);
  };
  requestAnimationFrame(step);
  setTimeout(() => { if (token[id] === my) { shown[id] = to; node.textContent = fmt(to, dp); } }, dur + 60);
}

function bump(el) {
  if (still) return;
  el.classList.remove("bump"); void el.offsetWidth; el.classList.add("bump");
  setTimeout(() => el.classList.remove("bump"), 1200);
}

function pop(n) {
  if (still || n <= 0) return;
  const p = document.createElement("span");
  p.className = "pop"; p.textContent = "+" + fmt(n);
  $("car").appendChild(p);
  p.addEventListener("animationend", () => p.remove());
  setTimeout(() => p.remove(), 2500);
}

/* ---------- formatting ---------- */
function ago(ms) {
  const s = Math.max(0, Math.round(ms / 1000));
  if (s < 5) return "just now";
  if (s < 60) return s + " s ago";
  const m = Math.floor(s / 60);
  if (m < 60) return m + " min " + String(s % 60).padStart(2, "0") + " s ago";
  const h = Math.floor(m / 60);
  if (h < 48) return h + " h " + (m % 60) + " min ago";
  return Math.floor(h / 24) + " days ago";
}

function eta(d) {
  if (d.percent >= 100 || d.remaining === 0) return ["Done", "every function proven"];
  const days = d.eta_days;
  if (!isNum(days) || days < 0) return ["\u2014", "waiting for pace data"];
  const h = days * 24;
  const big = h < 1 ? "< 1 h" : h < 48 ? Math.round(h) + " h" : days < 60 ? fmt(days, 1) + " days"
            : days < 730 ? fmt(days / 30.44, 1) + " months" : fmt(days / 365.25, 1) + " years";
  const when = new Date(Date.parse(d.updated) + days * 864e5);
  const opts = days < 7 ? {weekday: "short", day: "numeric", month: "short"}
             : days < 365 ? {day: "numeric", month: "short"} : {month: "short", year: "numeric"};
  const sub = days > 36500 ? "at current pace" : "around " + when.toLocaleDateString(undefined, opts);
  return [big, sub];
}

function clock(pct) {
  const mins = pct >= 100 ? 1440 : Math.min(1439, Math.floor(pct / 100 * 1440));
  return String(Math.floor(mins / 60)).padStart(2, "0") + ":" + String(mins % 60).padStart(2, "0");
}

/* ---------- rendering ---------- */
(function buildTicks() {
  const g = $("ticks"), NS = "http://www.w3.org/2000/svg";
  for (let i = 0; i < 24; i++) {
    const a = (i / 24) * 2 * Math.PI - Math.PI / 2, major = i % 6 === 0;
    const r1 = major ? 88 : 90, r2 = major ? 97 : 94;
    const l = document.createElementNS(NS, "line");
    l.setAttribute("x1", 100 + r1 * Math.cos(a)); l.setAttribute("y1", 100 + r1 * Math.sin(a));
    l.setAttribute("x2", 100 + r2 * Math.cos(a)); l.setAttribute("y2", 100 + r2 * Math.sin(a));
    l.setAttribute("class", "tick" + (major ? " major" : ""));
    l.dataset.h = i;
    g.appendChild(l);
  }
})();

function bars(hostId, data, unitSec, label) {
  const host = $(hostId);
  if (!Array.isArray(data) || !data.length || !data.every(v => isNum(v) && v >= 0)) {
    host.innerHTML = '<div class="empty">Activity chart appears with the next update</div>';
    return null;
  }
  const n = data.length, W = n * 10, H = 86, max = Math.max(1, ...data);
  let svg = host.querySelector("svg");
  if (!svg || svg.childElementCount !== n) {
    host.innerHTML = "";
    svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
    svg.setAttribute("viewBox", "0 0 " + W + " " + H);
    svg.setAttribute("preserveAspectRatio", "none");
    svg.setAttribute("role", "img");
    for (let i = 0; i < n; i++) {
      const r = document.createElementNS(svg.namespaceURI, "rect");
      r.setAttribute("x", i * 10 + 1.5); r.setAttribute("width", 7); r.setAttribute("rx", 1.5);
      r.setAttribute("y", H); r.setAttribute("height", 0);
      r.appendChild(document.createElementNS(svg.namespaceURI, "title"));
      svg.appendChild(r);
    }
    host.appendChild(svg);
  }
  const total = data.reduce((a, b) => a + b, 0);
  svg.setAttribute("aria-label", label + ": " + fmt(total) + " proofs, peak " + fmt(Math.max(...data)));
  [...svg.children].forEach((r, i) => {
    const v = data[i], h = v ? Math.max(3, (v / max) * (H - 2)) : 2;
    r.setAttribute("y", H - h); r.setAttribute("height", h);
    r.setAttribute("class", v === 0 ? "zero" : i === n - 1 ? "hot" : "");
    const back = n - 1 - i, unit = unitSec === 60 ? "min" : "h";
    r.firstChild.textContent = fmt(v) + (v === 1 ? " proof" : " proofs") + ", " +
      (back === 0 ? "this " + (unitSec === 60 ? "minute" : "hour") : back + "\u2013" + (back + 1) + " " + unit + " ago");
  });
  return Math.max(...data);
}

function render(d, prev) {
  const total = d.total;
  const pct = clamp(d.percent, 0, 100);
  const blPct = clamp(100 * d.blocked / total, 0, 100 - pct);
  const finished = pct >= 100;

  count("pct", pct, pct > 0 && pct < 0.1 ? 2 : 1);
  count("verified", d.verified);
  count("verified-of", d.verified);
  count("remaining", d.remaining);
  count("blocked", d.blocked);
  count("last_hour", d.last_hour);
  count("evidence", d.evidence_files);
  $("total").textContent = fmt(total);
  $("pace").textContent = fmt(d.pace_per_day);
  $("per-hour").textContent = fmt(d.last_day / 24, 1);
  $("lg-go").textContent = fmt(d.verified);
  $("lg-bl").textContent = fmt(d.blocked);
  $("lg-rest").textContent = fmt(d.remaining);
  $("verified-sub").innerHTML = d.last_hour > 0
    ? '<span class="up">+' + fmt(d.last_hour) + "</span> in the last hour" : "proven equivalent";

  $("seg-go").style.width = pct + "%";
  $("seg-bl").style.left = pct + "%";
  $("seg-bl").style.width = blPct + "%";
  $("car").style.left = pct + "%";
  $("car-tag").textContent = clock(pct);
  const track = $("track");
  track.setAttribute("aria-valuenow", pct.toFixed(2));
  track.setAttribute("aria-valuetext", fmt(d.verified) + " of " + fmt(total) + " functions verified");

  $("ring-go").setAttribute("stroke-dasharray", (pct * 10) + " 1000");
  $("ring-bl").setAttribute("stroke-dasharray", (blPct * 10) + " 1000");
  $("ring-bl").setAttribute("stroke-dashoffset", String(-pct * 10));
  const a = pct / 100 * 2 * Math.PI - Math.PI / 2;
  $("knob").setAttribute("cx", (100 + 78 * Math.cos(a)).toFixed(2));
  $("knob").setAttribute("cy", (100 + 78 * Math.sin(a)).toFixed(2));
  document.querySelectorAll("#ticks line").forEach(l => l.classList.toggle("done", Number(l.dataset.h) / 24 * 100 <= pct && pct > 0));
  $("clock").textContent = clock(pct);

  const task = finished ? "Chequered flag \u2014 every function has been proven" : String(d.current_task || "Between updates");
  $("task").textContent = task.charAt(0).toUpperCase() + task.slice(1);

  const [etaBig, etaSub] = eta(d);
  $("eta").textContent = etaBig;
  $("eta-date").textContent = etaSub;

  const pm = bars("chart-m", d.minutely, 60, "Proofs per minute, last hour");
  const ph = bars("chart-h", d.hourly, 3600, "Proofs per hour, last 24 hours");
  $("peak-m").textContent = pm == null ? "\u2014" : fmt(pm);
  $("peak-h").textContent = ph == null ? "\u2014" : fmt(ph);

  document.body.classList.toggle("finished", finished);

  if (prev && S.firstPaintDone && d.verified > prev.verified) { pop(d.verified - prev.verified); bump($("st-verified")); }
  S.firstPaintDone = true;
  status();
}

/* ---------- once-a-second status ---------- */
function status() {
  const d = S.doc, pill = $("pill");
  let mode, label, sub;
  if (!navigator.onLine) { mode = "offline"; label = "Offline"; sub = d ? "showing last known numbers" : "waiting for a connection"; }
  else if (!d) { mode = "loading"; label = "Connecting"; sub = "\u00a0"; }
  else {
    const age = Math.max(0, Date.now() - Date.parse(d.updated));
    if (d.percent >= 100) { mode = "finished"; label = "Finished"; sub = "chequered flag"; }
    else if (age > STALE_MS) { mode = "stale"; label = "Stale"; sub = "no fresh data for " + ago(age).replace(" ago", ""); }
    else if (S.source === "live" && S.liveOk) { mode = "live"; label = "Live"; sub = "updating every second"; }
    else if (S.source === "cache") { mode = "idle"; label = "Cached"; sub = "reconnecting\u2026"; }
    else { mode = "idle"; label = "Snapshot"; sub = S.liveUrl ? "live feed unreachable \u00b7 retrying" : "refreshed every 10 min"; }
    $("ago").textContent = ago(age);
    $("ago").title = new Date(Date.parse(d.updated)).toLocaleString();
    $("src").textContent = S.source === "live" ? "the live feed" : S.source === "cache" ? "this browser\u2019s cache" : "the published snapshot";
    const lp = Date.parse(d.latest || "");
    $("last-proof-wrap").hidden = Number.isNaN(lp);
    if (!Number.isNaN(lp)) $("last-proof").textContent = ago(Math.max(0, Date.now() - lp));
    document.title = fmt(clamp(d.percent, 0, 100), 1) + "% \u00b7 Test Drive Le Mans port";
  }
  if (pill.dataset.mode !== mode) pill.dataset.mode = mode;
  $("pill-t").textContent = label;
  $("pill-sub").textContent = sub;
  $("now-dot").style.background = mode === "live" ? "var(--live)" : mode === "finished" ? "var(--go)" : "";
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
            self._send(200, b"ok\\n", "text/plain; charset=utf-8")
        else:
            self._send(404, b"not found\\n", "text/plain; charset=utf-8")

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
