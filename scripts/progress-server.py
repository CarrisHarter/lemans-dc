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
<meta name="description" content="Live progress of a fan preservation project that is translating and checking every piece of the 2001 Dreamcast racing game Test Drive Le Mans. Unofficial; the game itself is not being released.">
<meta property="og:title" content="Test Drive Le Mans — preservation progress, live">
<meta property="og:description" content="Watch, live, as every piece of a 2001 Dreamcast racing game is translated and checked. A preservation project; the game itself is not being released.">
<meta property="og:type" content="website">
<meta name="twitter:card" content="summary">
<title>Test Drive Le Mans — preservation progress</title>
<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 4 4'%3E%3Cpath fill='%23fff' d='M0 0h4v4H0z'/%3E%3Cpath fill='%23111' d='M0 0h1v1H0zM2 0h1v1H2zM1 1h1v1H1zM3 1h1v1H3zM0 2h1v1H0zM2 2h1v1H2zM1 3h1v1H1zM3 3h1v1H3z'/%3E%3C/svg%3E">
<style>
  :root {
    --bg: #06080c; --panel: rgba(255,255,255,.035); --panel-hi: rgba(255,255,255,.065); --solid: #121821;
    --line: rgba(255,255,255,.09); --line-hi: rgba(255,255,255,.18);
    --fg: #f3f6fa; --muted: #a9b3c2; --faint: #7f899a;
    --go: #22c55e; --go-hi: #86efac; --go-lo: #15803d; --go-glow: rgba(34,197,94,.38);
    --warn: #f5a524; --warn-lo: #a86b0c; --live: #ff4545; --track: #10141c;
    --tip-bg: #eef2f7; --tip-fg: #0d1526;
    --sans: "Inter", "Inter var", ui-sans-serif, system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
    --mono: "JetBrains Mono", ui-monospace, "SF Mono", SFMono-Regular, Menlo, Consolas, monospace;
    --ease: cubic-bezier(.2,.8,.2,1);
  }
  @media (prefers-color-scheme: light) {
    :root { --bg: #f3f5f8; --panel: rgba(255,255,255,.85); --panel-hi: #ffffff; --solid: #ffffff;
            --line: rgba(15,23,42,.1); --line-hi: rgba(15,23,42,.2);
            --fg: #0b1324; --muted: #445064; --faint: #5f6a7d; --track: #e1e6ee;
            --go: #15803d; --go-hi: #22c55e; --go-lo: #14532d; --go-glow: rgba(21,128,61,.25);
            --warn: #b45309; --warn-lo: #7c3a06; --tip-bg: #0d1526; --tip-fg: #f3f6fa; }
  }
  * { box-sizing: border-box; }
  html { background: var(--bg); -webkit-text-size-adjust: 100%; }
  body {
    margin: 0; min-height: 100dvh; color: var(--fg); font: 16px/1.6 var(--sans);
    -webkit-font-smoothing: antialiased; display: flex; justify-content: center;
    padding: max(env(safe-area-inset-top), clamp(1.25rem, 5vw, 3.5rem)) clamp(.9rem, 4vw, 2rem) 2.5rem;
    overflow-x: hidden;
  }
  body::before, body::after { content: ""; position: fixed; inset: -20%; z-index: -1; pointer-events: none; }
  body::before {
    background:
      radial-gradient(40% 35% at 18% 12%, rgba(34,197,94,.16), transparent 70%),
      radial-gradient(35% 30% at 88% 6%, rgba(255,69,69,.10), transparent 70%),
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
  a { color: inherit; text-underline-offset: 3px; }

  .wrap { width: min(1080px, 100%); }
  .num { font-family: var(--mono); font-variant-numeric: tabular-nums; letter-spacing: -.02em; }
  .eyebrow { display: inline-flex; align-items: center; font: 600 .74rem/1.2 var(--mono); letter-spacing: .14em;
             text-transform: uppercase; color: var(--muted); }
  .panel { position: relative; background: var(--panel); border: 1px solid var(--line); border-radius: 22px;
           -webkit-backdrop-filter: blur(14px); backdrop-filter: blur(14px);
           box-shadow: 0 1px 0 rgba(255,255,255,.04) inset, 0 24px 60px -30px rgba(0,0,0,.6); }
  @media (prefers-color-scheme: light) { .panel { box-shadow: 0 1px 0 #fff inset, 0 20px 50px -30px rgba(15,23,42,.25); } }

  /* ? hints */
  .hint { position: relative; display: inline-grid; place-items: center; flex: none; width: 1.3rem; height: 1.3rem;
          margin-left: .45rem; padding: 0; border-radius: 50%; border: 1px solid var(--line-hi); background: transparent;
          color: var(--muted); font: 700 .72rem/1 var(--sans); letter-spacing: 0; text-transform: none; cursor: help; }
  .hint:hover, .hint:focus-visible { color: var(--fg); border-color: var(--muted); outline: none; }
  .hint::after { content: attr(data-tip); position: absolute; bottom: calc(100% + .55rem); left: -.6rem; z-index: 30;
                 width: max-content; max-width: min(270px, 78vw); padding: .65rem .8rem; border-radius: 12px;
                 background: var(--tip-bg); color: var(--tip-fg); font: 450 .84rem/1.45 var(--sans); text-align: left;
                 white-space: normal; box-shadow: 0 12px 30px -10px rgba(0,0,0,.5);
                 opacity: 0; transform: translateY(4px); pointer-events: none; transition: opacity .15s, transform .15s; }
  .hint[data-align="right"]::after { left: auto; right: -.6rem; }
  .hint:hover::after, .hint:focus::after { opacity: 1; transform: none; }

  /* header */
  header { display: flex; align-items: flex-start; justify-content: space-between; gap: 1.25rem; margin-bottom: 1.4rem; }
  h1 { margin: .55rem 0 .5rem; font-size: clamp(1.75rem, 4vw, 2.6rem); line-height: 1.08; font-weight: 780;
       letter-spacing: -.03em; text-wrap: balance; }
  h1 span { color: var(--muted); font-weight: 500; }
  .lede { margin: 0; color: var(--muted); max-width: 64ch; text-wrap: pretty; }
  .lede strong { color: var(--fg); font-weight: 600; }
  .status { flex: none; display: flex; flex-direction: column; align-items: flex-end; gap: .45rem; max-width: 18rem; }
  .pill { display: inline-flex; align-items: center; gap: .55rem; padding: .5rem .95rem .5rem .8rem;
          border: 1px solid var(--line-hi); border-radius: 999px; background: var(--panel);
          font: 650 .88rem/1 var(--sans); white-space: nowrap; transition: color .3s, border-color .3s; }
  .dot { position: relative; flex: none; width: .6rem; height: .6rem; border-radius: 50%; background: var(--faint); transition: background .3s; }
  .pill[data-mode="live"] .dot { background: var(--live); }
  .pill[data-mode="live"] .dot::after { content: ""; position: absolute; inset: 0; border-radius: 50%;
                                        background: var(--live); animation: ping 1.6s var(--ease) infinite; }
  .pill[data-mode="stale"], .pill[data-mode="offline"] { color: var(--warn); border-color: color-mix(in srgb, var(--warn) 50%, transparent); }
  .pill[data-mode="stale"] .dot, .pill[data-mode="offline"] .dot { background: var(--warn); }
  .pill[data-mode="finished"] { color: var(--go); border-color: color-mix(in srgb, var(--go) 50%, transparent); }
  .pill[data-mode="finished"] .dot { background: var(--go); }
  .status small { color: var(--faint); font-size: .82rem; line-height: 1.35; text-align: right; }
  @keyframes ping { 0% { transform: scale(1); opacity: .7; } 100% { transform: scale(3.2); opacity: 0; } }

  /* summary + explainer */
  .summary { margin: 0 0 1rem; padding: 1.15rem 1.35rem; font-size: clamp(1.08rem, 2.2vw, 1.3rem); line-height: 1.45;
             font-weight: 500; text-wrap: pretty; }
  .summary b { font-weight: 700; }
  .explain { margin: 0 0 1rem; padding: 0; }
  .explain summary { display: flex; align-items: center; gap: .7rem; padding: 1rem 1.35rem; cursor: pointer;
                     list-style: none; font-weight: 600; border-radius: 22px; }
  .explain summary::-webkit-details-marker { display: none; }
  .explain summary::before { content: "+"; display: grid; place-items: center; width: 1.6rem; height: 1.6rem; flex: none;
                             border-radius: 50%; border: 1px solid var(--line-hi); font: 600 1rem/1 var(--sans); color: var(--muted);
                             transition: transform .25s var(--ease); }
  .explain[open] summary::before { transform: rotate(45deg); }
  .explain summary:focus-visible { outline: 2px solid var(--go); outline-offset: 2px; }
  .explain summary small { margin-left: auto; color: var(--faint); font-weight: 400; font-size: .85rem; }
  .explain-body { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 1.1rem 2rem; padding: .25rem 1.35rem 1.35rem; }
  .explain h3 { margin: 0 0 .3rem; font-size: .98rem; }
  .explain p { margin: 0; color: var(--muted); font-size: .94rem; text-wrap: pretty; }
  .explain .wide { grid-column: 1 / -1; padding: .9rem 1rem; border-radius: 14px;
                   background: color-mix(in srgb, var(--warn) 10%, transparent); border: 1px solid color-mix(in srgb, var(--warn) 30%, transparent); }
  .explain .wide p { color: var(--fg); }

  /* hero */
  .hero { display: grid; grid-template-columns: 1fr auto; gap: clamp(1.25rem, 4vw, 3rem); align-items: center;
          padding: clamp(1.25rem, 3.5vw, 2.4rem); }
  .hero::before { content: ""; position: absolute; inset: 0; border-radius: inherit; pointer-events: none;
                  background: radial-gradient(60% 90% at 0% 0%, rgba(34,197,94,.10), transparent 60%); }
  .pct { font-size: clamp(4.2rem, 14vw, 8.5rem); font-weight: 700; line-height: .82; letter-spacing: -.05em;
         background: linear-gradient(180deg, var(--fg) 30%, color-mix(in srgb, var(--fg) 55%, var(--go)));
         -webkit-background-clip: text; background-clip: text; color: transparent; }
  .pct small { font-size: .36em; letter-spacing: 0; margin-left: .06em; -webkit-text-fill-color: var(--muted); }
  .of { margin: 1rem 0 1.4rem; color: var(--muted); font-size: 1.05rem; }
  .of b { color: var(--fg); font-weight: 600; }

  .track-wrap { position: relative; padding-top: 1.4rem; }
  .track { position: relative; height: 34px; border-radius: 11px; overflow: hidden; background: var(--track);
           box-shadow: inset 0 1px 3px rgba(0,0,0,.45), inset 0 0 0 1px var(--line); }
  .seg { position: absolute; top: 0; bottom: 0; left: 0; width: 0; transition: width 1.2s var(--ease), left 1.2s var(--ease); }
  .seg.go { z-index: 2; background: linear-gradient(90deg, var(--go-lo), var(--go) 72%, var(--go-hi)); box-shadow: 0 0 28px var(--go-glow); }
  .seg.go::after { content: ""; position: absolute; inset: 0 0 52% 0; background: linear-gradient(rgba(255,255,255,.26), transparent); }
  .seg.go::before { content: ""; position: absolute; inset: 0; opacity: .5;
                    background: linear-gradient(90deg, transparent, rgba(255,255,255,.45), transparent) 0 0 / 40% 100% no-repeat;
                    animation: sheen 3.2s ease-in-out infinite; }
  .seg.bl { z-index: 2; background: repeating-linear-gradient(135deg, var(--warn) 0 6px, var(--warn-lo) 6px 12px); opacity: .92; }
  .mline { position: absolute; top: 0; bottom: 0; width: 0; z-index: 3; border-left: 2px dashed rgba(255,255,255,.28); }
  .mline.passed { border-left-color: rgba(255,255,255,.7); }
  @media (prefers-color-scheme: light) { .mline { border-left-color: rgba(15,23,42,.25); } .mline.passed { border-left-color: rgba(255,255,255,.85); } }
  .flag { position: absolute; right: 0; top: 0; bottom: 0; width: 20px; z-index: 4; opacity: .9;
          background: conic-gradient(#fff 25%, #0d0d0d 0 50%, #fff 0 75%, #0d0d0d 0) 0 0 / 10px 11.33px;
          box-shadow: -6px 0 12px rgba(0,0,0,.35); }
  .car { position: absolute; top: 1.4rem; height: 34px; width: 0; left: 0; z-index: 5; transition: left 1.2s var(--ease); }
  .car::before { content: ""; position: absolute; left: -1.5px; top: -4px; bottom: -4px; width: 3px; border-radius: 2px;
                 background: #fff; box-shadow: 0 0 14px 3px var(--go-hi); }
  .pop { position: absolute; bottom: 100%; left: 0; transform: translateX(-50%); margin-bottom: .1rem;
         font: 700 .9rem/1 var(--mono); color: var(--go-hi); white-space: nowrap; text-shadow: 0 0 12px var(--go-glow);
         animation: pop 1.8s var(--ease) forwards; pointer-events: none; }
  @keyframes pop { 0% { opacity: 0; transform: translate(-50%, 6px) scale(.8); } 15% { opacity: 1; transform: translate(-50%, 0) scale(1.08); }
                   70% { opacity: 1; } 100% { opacity: 0; transform: translate(-50%, -18px) scale(1); } }
  @keyframes sheen { 0% { background-position: -60% 0; } 60%, 100% { background-position: 160% 0; } }
  .marks { position: relative; height: 1.3rem; margin-top: .4rem; }
  .mark { position: absolute; transform: translateX(-50%); color: var(--faint); font: 600 .74rem/1.3 var(--mono); }
  .mark.passed { color: var(--go); }
  .next { margin: .5rem 0 0; color: var(--muted); font-size: .95rem; }
  .next b { color: var(--fg); font-weight: 600; }
  .legend { display: flex; flex-wrap: wrap; gap: .5rem 1.4rem; margin-top: .9rem; color: var(--muted); font-size: .88rem; }
  .legend i { display: inline-block; width: .75rem; height: .75rem; border-radius: 3px; margin-right: .45rem; vertical-align: -1px; }

  .gauge-col { display: flex; flex-direction: column; align-items: center; gap: .8rem; }
  .gauge { position: relative; width: clamp(190px, 24vw, 240px); aspect-ratio: 1; }
  .gauge svg { width: 100%; height: 100%; overflow: visible; }
  .ring { fill: none; stroke-width: 11; transform: rotate(-90deg); transform-origin: 100px 100px;
          transition: stroke-dasharray 1.2s var(--ease), stroke-dashoffset 1.2s var(--ease); }
  .ring.bg { stroke: var(--track); }
  .ring.go { stroke: url(#g-go); filter: drop-shadow(0 0 6px var(--go-glow)); }
  .ring.bl { stroke: var(--warn); opacity: .85; }
  .tick { stroke: var(--faint); stroke-width: 1.2; stroke-linecap: round; }
  .tick.major { stroke: var(--muted); stroke-width: 2; }
  .tick.done { stroke: var(--go); }
  .knob { fill: #fff; filter: drop-shadow(0 0 5px var(--go-hi)); }
  .gauge-c { position: absolute; inset: 0; display: grid; place-content: center; text-align: center; gap: .3rem; }
  .gauge-c .eyebrow { justify-content: center; font-size: .66rem; }
  .gauge-c b { font-size: clamp(1.9rem, 3.6vw, 2.45rem); font-weight: 650; line-height: 1; }
  .gauge-c span:last-child { color: var(--faint); font: .78rem/1 var(--mono); }
  .gauge-cap { max-width: 17rem; margin: 0; text-align: center; color: var(--muted); font-size: .88rem; line-height: 1.45; }

  /* now strip */
  .now { display: flex; align-items: center; gap: 1rem; margin: 1rem 0; padding: 1rem 1.35rem; }
  .now .eyebrow { flex: none; gap: .55rem; padding-right: 1rem; border-right: 1px solid var(--line); }
  .now p { margin: 0; flex: 1; min-width: 0; font-size: 1.06rem; overflow-wrap: anywhere; }
  .now small { flex: none; color: var(--faint); font-size: .86rem; }
  .now small b { color: var(--muted); font-weight: 600; font-family: var(--mono); }

  /* stats */
  .stats { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 1rem; }
  .stat { padding: 1.15rem 1.25rem 1.2rem; transition: border-color .4s; }
  .stat .num { display: block; margin-top: .6rem; font-size: clamp(1.5rem, 2.8vw, 2.05rem); font-weight: 650; line-height: 1.1; overflow-wrap: anywhere; }
  .stat .num.words { font-family: var(--sans); letter-spacing: -.02em; }
  .stat p { margin: .4rem 0 0; color: var(--faint); font-size: .88rem; }
  .stat.go .num { color: var(--go); }
  .stat.bl .num { color: var(--warn); }
  .stat.bump { border-color: color-mix(in srgb, var(--go) 60%, transparent); }
  .up { color: var(--go); font-weight: 600; }

  /* charts */
  .charts { display: grid; grid-template-columns: 1fr 1fr; gap: 1rem; margin-top: 1rem; }
  .chart { padding: 1.15rem 1.25rem 1rem; }
  .chart-h { display: flex; justify-content: space-between; align-items: center; gap: 1rem; margin-bottom: .85rem; }
  .chart-h small { color: var(--faint); font-size: .84rem; }
  .chart-h small b { color: var(--fg); font-weight: 600; font-family: var(--mono); }
  .chart svg { display: block; width: 100%; height: 86px; }
  .chart rect { fill: color-mix(in srgb, var(--go) 45%, transparent); transition: height .6s var(--ease), y .6s var(--ease); }
  .chart rect.hot { fill: var(--go); }
  .chart rect.zero { fill: var(--line-hi); }
  .chart rect:hover { fill: var(--go-hi); }
  .axis { display: flex; justify-content: space-between; margin-top: .45rem; color: var(--faint); font-size: .76rem; }
  .empty { display: grid; place-items: center; height: 86px; color: var(--faint); font-size: .88rem; text-align: center; }

  .mini { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); margin-top: 1rem; }
  .mini div { padding: 1rem 1.25rem; border-left: 1px solid var(--line); min-width: 0; }
  .mini div:first-child { border-left: 0; }
  .mini .num { display: block; margin-top: .4rem; font-size: 1.2rem; font-weight: 600; }

  footer { margin-top: 2rem; padding-top: 1.25rem; border-top: 1px solid var(--line); color: var(--faint); font-size: .86rem; }
  footer .num { color: var(--muted); }
  .notice { margin: .6rem 0 0; max-width: 82ch; }
  noscript { display: block; margin-bottom: 1rem; padding: .8rem 1rem; border: 1px solid var(--warn); border-radius: 12px; color: var(--warn); }

  /* celebration */
  .toast { position: fixed; left: 50%; top: max(1rem, env(safe-area-inset-top)); z-index: 60; display: flex; align-items: center; gap: .9rem;
           max-width: calc(100vw - 2rem); padding: .85rem .9rem .85rem 1.15rem; border-radius: 16px; background: var(--solid);
           border: 1px solid color-mix(in srgb, var(--go) 55%, transparent); box-shadow: 0 20px 50px -15px rgba(0,0,0,.55), 0 0 40px -10px var(--go-glow);
           font-weight: 600; transform: translate(-50%, -160%); transition: transform .55s var(--ease); }
  .toast.show { transform: translate(-50%, 0); }
  .toast button { flex: none; width: 1.9rem; height: 1.9rem; border-radius: 50%; border: 1px solid var(--line-hi); background: transparent;
                  color: var(--muted); font: 600 1rem/1 var(--sans); cursor: pointer; }
  .confetti { position: fixed; inset: 0; z-index: 55; pointer-events: none; overflow: hidden; }
  .confetti i { position: absolute; top: -16px; width: 8px; height: 14px; border-radius: 2px;
                animation: fall var(--d) cubic-bezier(.25,.6,.45,1) var(--delay) forwards; }
  @keyframes fall { to { transform: translate3d(var(--x), 108vh, 0) rotate(var(--r)); } }

  /* finished */
  .finished .seg.go::before { animation: none; }
  .finished .flag { width: 34px; animation: wave 1.2s ease-in-out infinite alternate; }
  @keyframes wave { to { transform: skewY(-6deg) scaleX(1.1); } }

  @media (max-width: 860px) {
    .hero { grid-template-columns: 1fr; }
    .gauge-col { order: -1; }
    .stats, .mini { grid-template-columns: repeat(2, minmax(0, 1fr)); }
    .charts, .explain-body { grid-template-columns: 1fr; }
    .mini div:nth-child(3) { border-left: 0; }
    .mini div:nth-child(n+3) { border-top: 1px solid var(--line); }
    .hint[data-align="right"]::after { left: -.6rem; right: auto; }
    .stats .stat:nth-child(2n) .hint::after, .mini div:nth-child(2n) .hint::after { left: auto; right: -.6rem; }
  }
  @media (max-width: 560px) {
    header { flex-direction: column; }
    .status { align-items: flex-start; max-width: none; }
    .status small { text-align: left; }
    .now { flex-direction: column; align-items: flex-start; gap: .55rem; }
    .now .eyebrow { border: 0; padding: 0; }
    .stat { padding: 1rem; }
    .explain summary small { display: none; }
  }
  @media (max-width: 360px) {
    .stats, .mini { grid-template-columns: 1fr; }
    .mini div { border-left: 0 !important; border-top: 1px solid var(--line); }
    .mini div:first-child { border-top: 0; }
    .stats .stat .hint::after, .mini div .hint::after { left: -.6rem !important; right: auto !important; }
  }
  @media (prefers-reduced-motion: reduce) {
    *, *::before, *::after { animation: none !important; transition: none !important; }
  }
  @media print {
    body::before, body::after, .toast, .confetti, .hint { display: none; }
    .panel { box-shadow: none; -webkit-backdrop-filter: none; backdrop-filter: none; }
  }
</style>
<div class="wrap" id="app">
  <noscript>This page needs JavaScript to show its numbers. You can still see the raw figures in <a href="progress.json">progress.json</a>.</noscript>
  <header>
    <div>
      <div class="eyebrow">Sega Dreamcast &middot; 2001 &middot; fan preservation project</div>
      <h1>Test Drive Le Mans <span>&mdash; live progress</span></h1>
      <p class="lede">A project to fully understand and document the code of a classic racing game, one piece at a time.
        <strong>Reaching 100% doesn&rsquo;t mean a download</strong> &mdash; the game itself won&rsquo;t be released.</p>
    </div>
    <div class="status">
      <div class="pill" id="pill" data-mode="loading" role="status" aria-live="polite"><span class="dot"></span><span id="pill-t">Loading&hellip;</span></div>
      <small id="pill-sub">&nbsp;</small>
    </div>
  </header>

  <p class="panel summary" id="summary" aria-live="polite">Getting the latest numbers&hellip;</p>

  <details class="panel explain">
    <summary>What am I looking at?<small>a 30-second explanation</small></summary>
    <div class="explain-body">
      <div>
        <h3>The idea</h3>
        <p><em>Test Drive Le Mans</em> is a racing game released for the Sega Dreamcast in 2001. Its code is made of
          <span id="ex-total">5,211</span> small pieces &mdash; think of them as the sentences of a very long book.</p>
      </div>
      <div>
        <h3>What &ldquo;done&rdquo; means</h3>
        <p>Each piece is translated into a modern programming language. Then a computer checks the translation behaves
          <em>exactly</em> like the original. Only pieces that pass that check count as done.</p>
      </div>
      <div>
        <h3>&ldquo;Stuck for now&rdquo;</h3>
        <p>A few pieces need a different approach. They&rsquo;re set aside and revisited later, so they&rsquo;re not
          counted as done or to do.</p>
      </div>
      <div>
        <h3>The finish date</h3>
        <p>It&rsquo;s a rough guess based on how much was finished in the last 24 hours, so it moves as work speeds up
          or slows down. Treat it as a forecast, not a promise.</p>
      </div>
      <div class="wide">
        <h3>What happens at 100%?</h3>
        <p>Every piece of the game&rsquo;s code will have been understood and checked. This is a preservation and research
          project: <strong>the game itself will not be released or made available to download</strong>, and no game files
          are shared here.</p>
      </div>
    </div>
  </details>

  <section class="panel hero">
    <div>
      <div class="pct num" aria-hidden="true"><span id="pct">0.0</span><small>%</small></div>
      <div class="of"><b class="num" id="verified-of">&mdash;</b> of <span class="num" id="total">&mdash;</span>
        pieces of the game translated and checked</div>
      <div class="track-wrap">
        <div class="track" id="track" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-valuenow="0" aria-label="Share of the game checked">
          <div class="seg go" id="seg-go"></div>
          <div class="seg bl" id="seg-bl"></div>
          <div class="mline" style="left:25%" data-m="25"></div>
          <div class="mline" style="left:50%" data-m="50"></div>
          <div class="mline" style="left:75%" data-m="75"></div>
          <div class="flag" title="Finish line"></div>
        </div>
        <div class="car" id="car"></div>
      </div>
      <div class="marks" aria-hidden="true">
        <span class="mark" style="left:25%" data-m="25">25%</span>
        <span class="mark" style="left:50%" data-m="50">50%</span>
        <span class="mark" style="left:75%" data-m="75">75%</span>
      </div>
      <p class="next" id="next" hidden></p>
      <div class="legend">
        <span><i style="background:var(--go)"></i>Done</span>
        <span><i style="background:var(--warn)"></i>Stuck for now</span>
        <span><i style="background:var(--track);box-shadow:inset 0 0 0 1px var(--line-hi)"></i>To do</span>
      </div>
    </div>
    <div class="gauge-col">
      <div class="gauge" aria-hidden="true">
        <svg viewBox="0 0 200 200">
          <defs><linearGradient id="g-go" x1="0" y1="0" x2="1" y2="1"><stop offset="0" style="stop-color:var(--go-hi)"/><stop offset="1" style="stop-color:var(--go-lo)"/></linearGradient></defs>
          <g id="ticks"></g>
          <circle class="ring bg" cx="100" cy="100" r="78" pathLength="1000"/>
          <circle class="ring bl" id="ring-bl" cx="100" cy="100" r="78" pathLength="1000" stroke-dasharray="0 1000"/>
          <circle class="ring go" id="ring-go" cx="100" cy="100" r="78" pathLength="1000" stroke-dasharray="0 1000"/>
          <circle class="knob" id="knob" cx="100" cy="22" r="5.5"/>
        </svg>
        <div class="gauge-c"><span class="eyebrow">Le Mans clock</span><b class="num" id="clock">00:00</b><span>of 24:00</span></div>
      </div>
      <p class="gauge-cap" id="clock-cap">If this project were the 24-hour Le Mans race, this is how far in we&rsquo;d be.</p>
    </div>
  </section>

  <section class="panel now">
    <span class="eyebrow"><span class="dot" id="now-dot"></span>Right now</span>
    <p id="task">Checking in with the pit wall&hellip;</p>
    <small id="last-proof-wrap" hidden>last piece finished <b id="last-proof">&mdash;</b></small>
  </section>

  <section class="stats">
    <div class="panel stat go" id="st-verified">
      <span class="eyebrow">Done<button type="button" class="hint" data-tip="Pieces that have been translated and passed the automatic check against the original game.">?</button></span>
      <span class="num" id="verified">&mdash;</span><p id="verified-sub">translated and checked</p></div>
    <div class="panel stat">
      <span class="eyebrow">To do<button type="button" class="hint" data-tip="Pieces nobody has finished yet.">?</button></span>
      <span class="num" id="remaining">&mdash;</span><p>still waiting their turn</p></div>
    <div class="panel stat bl">
      <span class="eyebrow">Stuck for now<button type="button" class="hint" data-tip="Pieces that need a different approach. They&rsquo;re set aside and will be revisited later.">?</button></span>
      <span class="num" id="blocked">&mdash;</span><p>set aside for later</p></div>
    <div class="panel stat">
      <span class="eyebrow">Finish<button type="button" class="hint" data-align="right" data-tip="A rough forecast based on the last 24 hours of work, shown in your local time. It shifts as work speeds up or slows down.">?</button></span>
      <span class="num words" id="eta">&mdash;</span><p id="eta-date">at the current pace</p></div>
  </section>

  <section class="charts">
    <div class="panel chart">
      <div class="chart-h"><span class="eyebrow">The last hour<button type="button" class="hint" data-tip="Each bar is one minute. Taller bars mean more pieces were finished in that minute.">?</button></span>
        <small>busiest minute: <b id="peak-m">&mdash;</b></small></div>
      <div id="chart-m"><div class="empty">Waiting for activity&hellip;</div></div>
      <div class="axis"><span>1 hour ago</span><span>30 min ago</span><span>now</span></div>
    </div>
    <div class="panel chart">
      <div class="chart-h"><span class="eyebrow">The last day<button type="button" class="hint" data-align="right" data-tip="Each bar is one hour of work over the last 24 hours.">?</button></span>
        <small>busiest hour: <b id="peak-h">&mdash;</b></small></div>
      <div id="chart-h"><div class="empty">Waiting for activity&hellip;</div></div>
      <div class="axis"><span>yesterday</span><span>12 hours ago</span><span>now</span></div>
    </div>
  </section>

  <section class="panel mini">
    <div><span class="eyebrow">Last hour<button type="button" class="hint" data-tip="Pieces finished in the past 60 minutes.">?</button></span><span class="num" id="last_hour">&mdash;</span></div>
    <div><span class="eyebrow">Last 24 hours<button type="button" class="hint" data-tip="Pieces finished since this time yesterday.">?</button></span><span class="num" id="last_day">&mdash;</span></div>
    <div><span class="eyebrow">Per hour<button type="button" class="hint" data-tip="Average number of pieces finished per hour over the last day.">?</button></span><span class="num" id="per-hour">&mdash;</span></div>
    <div><span class="eyebrow">Whole game<button type="button" class="hint" data-align="right" data-tip="How many pieces make up the game&rsquo;s code in total.">?</button></span><span class="num" id="total-2">&mdash;</span></div>
  </section>

  <footer>
    <div>Numbers updated <span class="num" id="ago">&mdash;</span> &middot; <span id="src">&mdash;</span></div>
    <p class="notice">This technical analysis is unofficial and is not endorsed by or affiliated with Atari SA or any of its licensors. All rights in <em>Le Mans 24 Hours</em> remain with their respective owners.</p>
  </footer>
</div>
<div class="toast" id="toast" role="status" aria-live="polite" hidden><span id="toast-t"></span><button type="button" id="toast-x" aria-label="Dismiss">&times;</button></div>
<script>
"use strict";
const $ = id => document.getElementById(id);
const LIVE_MS = 1000, HIDDEN_MS = 15000, SNAP_POLL_MS = 5000, SNAP_REFETCH_MS = 60000;
const LIVE_RETRY_MS = 15000, LIVE_FAILS_MAX = 3, TIMEOUT_MS = 3000, STALE_MS = 2 * 3600e3;
const MILESTONES = [10, 20, 25, 30, 40, 50, 60, 70, 75, 80, 90, 100];
const CACHE_KEY = "lemans-progress:v2";
const still = matchMedia("(prefers-reduced-motion: reduce)").matches;
const isNum = v => typeof v === "number" && Number.isFinite(v);
const clamp = (v, lo, hi) => Math.min(hi, Math.max(lo, v));
const fmt = (n, dp = 0) => Number(n).toLocaleString("en-US", {minimumFractionDigits: dp, maximumFractionDigits: dp});
const plural = (n, one, many) => fmt(n) + " " + (n === 1 ? one : many);

const S = { doc: null, source: null, liveUrl: null, liveOk: false, liveFails: 0, lastLiveTry: 0,
            lastSnap: 0, inflight: false, timer: 0, everLoaded: false, loadFailed: false, firstPaintDone: false };

document.querySelectorAll(".hint").forEach(b => b.setAttribute("aria-label", "What does this mean? " + b.dataset.tip));

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
  const ms = document.hidden ? HIDDEN_MS : S.liveOk ? LIVE_MS : SNAP_POLL_MS;
  S.timer = setTimeout(poll, ms);
}

function accept(d, source) {
  const prev = S.doc, prevSource = S.source;
  if (prev && Date.parse(d.updated) < Date.parse(prev.updated)) { S.source = source; return; }
  S.doc = d; S.source = source; S.everLoaded = true;
  render(d, prev, prevSource);
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
  let done = false;
  const step = t => {
    if (done || token[id] !== my) return;
    const p = Math.min((t - t0) / dur, 1), e = 1 - Math.pow(1 - p, 3);
    shown[id] = from + (to - from) * e;
    node.textContent = fmt(shown[id], dp);
    if (p < 1) requestAnimationFrame(step);
  };
  requestAnimationFrame(step);
  // rAF stalls in background tabs; the timer guarantees the exact final value.
  setTimeout(() => { done = true; if (token[id] === my) { shown[id] = to; node.textContent = fmt(to, dp); } }, dur + 60);
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

let toastTimer = 0;
function celebrate(m, sinceLastVisit) {
  const t = $("toast");
  $("toast-t").textContent = m >= 100
    ? "Chequered flag! Every piece of the game has been checked."
    : sinceLastVisit ? "Since your last visit, the project passed " + m + "%!" : "Milestone reached: " + m + "% done!";
  t.hidden = false;
  requestAnimationFrame(() => requestAnimationFrame(() => t.classList.add("show")));
  clearTimeout(toastTimer);
  toastTimer = setTimeout(hideToast, 8000);
  if (still) return;
  const box = document.createElement("div");
  box.className = "confetti";
  const colors = ["var(--go)", "var(--go-hi)", "var(--warn)", "var(--live)", "#ffffff", "#60a5fa"];
  for (let i = 0; i < 90; i++) {
    const c = document.createElement("i");
    c.style.left = (Math.random() * 100) + "%";
    c.style.background = colors[i % colors.length];
    c.style.setProperty("--x", ((Math.random() - .5) * 240).toFixed(0) + "px");
    c.style.setProperty("--r", ((Math.random() - .5) * 1440).toFixed(0) + "deg");
    c.style.setProperty("--d", (2.2 + Math.random() * 1.8).toFixed(2) + "s");
    c.style.setProperty("--delay", (Math.random() * .5).toFixed(2) + "s");
    if (i % 3 === 0) { c.style.width = "10px"; c.style.height = "10px"; c.style.borderRadius = "50%"; }
    box.appendChild(c);
  }
  document.body.appendChild(box);
  setTimeout(() => box.remove(), 5000);
}
function hideToast() {
  const t = $("toast");
  t.classList.remove("show");
  setTimeout(() => { if (!t.classList.contains("show")) t.hidden = true; }, 600);
}
$("toast-x").addEventListener("click", hideToast);

/* ---------- words ---------- */
function ago(ms) {
  const s = Math.max(0, Math.round(ms / 1000));
  if (s < 5) return "just now";
  if (s < 60) return s + " seconds ago";
  const m = Math.floor(s / 60);
  if (m < 60) return m === 1 ? "1 minute ago" : m + " minutes ago";
  const h = Math.floor(m / 60);
  if (h < 48) return h + (h === 1 ? " hour" : " hours") + (m % 60 ? " " + (m % 60) + " min" : "") + " ago";
  return Math.floor(h / 24) + " days ago";
}

function duration(hours) {
  if (hours < 1) return "under an hour";
  if (hours < 1.5) return "about an hour";
  if (hours < 36) return "about " + Math.round(hours) + " hours";
  const days = hours / 24;
  if (days < 7) {
    const half = Math.round(days * 2) / 2, whole = Math.floor(half);
    return "about " + (half % 1 ? whole + "\u00bd" : String(whole)) + " days";
  }
  if (days < 60) return "about " + Math.round(days) + " days";
  if (days < 730) return "about " + Math.round(days / 30.44) + " months";
  return "about " + fmt(days / 365.25, 0) + " years";
}

function partOfDay(dt) {
  const h = dt.getHours();
  return h < 5 ? "night" : h < 12 ? "morning" : h < 17 ? "afternoon" : h < 21 ? "evening" : "night";
}

function whenText(ms) {
  const dt = new Date(ms), days = (ms - Date.now()) / 864e5;
  if (days < 0.5 && dt.getDate() === new Date().getDate()) return "later today (" + partOfDay(dt) + ")";
  if (days < 7) return dt.toLocaleDateString(undefined, {weekday: "long"}) + " " + partOfDay(dt);
  if (days < 365) return dt.toLocaleDateString(undefined, {day: "numeric", month: "long"});
  if (days < 36500) return dt.toLocaleDateString(undefined, {month: "long", year: "numeric"});
  return null;
}

function eta(d) {
  if (d.percent >= 100) return {big: "Finished", sub: "every piece checked", line: "Every piece has been checked."};
  if (d.remaining === 0) return {big: "\u2014", sub: "only stuck pieces are left", line: "Only the pieces that are stuck for now are left."};
  const days = d.eta_days;
  if (!isNum(days) || days < 0) return {big: "\u2014", sub: "not enough recent work to guess", line: "There hasn\u2019t been enough recent work to estimate a finish date."};
  const w = whenText(Date.parse(d.updated) + days * 864e5);
  const big = duration(days * 24).replace(/^about /, "About ");
  return {big, sub: w ? "around " + w : "at the current pace",
          line: w ? "At this pace, it\u2019ll be finished around " + w + "." : "At this pace, it\u2019s still a very long way off."};
}

function shareWords(p) {
  if (p >= 100) return "Finished";
  if (p >= 99) return "Almost there";
  if (p < 2) return "Just getting started";
  if (p < 8) return "Off the starting line";
  const F = [[12.5, "an eighth"], [20, "a fifth"], [25, "a quarter"], [33.3, "a third"], [40, "two fifths"], [50, "half"],
             [60, "three fifths"], [66.7, "two thirds"], [75, "three quarters"], [80, "four fifths"], [90, "nine tenths"]];
  let best = F[0];
  for (const f of F) if (Math.abs(f[0] - p) < Math.abs(best[0] - p)) best = f;
  const diff = p - best[0];
  const lead = Math.abs(diff) < 1.5 ? "About" : diff > 0 ? "Just over" : "Nearly";
  return lead + " " + best[1] + " done";
}

function clockMins(pct) { return pct >= 100 ? 1440 : Math.min(1439, Math.floor(pct / 100 * 1440)); }
function clock(pct) { const m = clockMins(pct); return String(Math.floor(m / 60)).padStart(2, "0") + ":" + String(m % 60).padStart(2, "0"); }

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

function bars(hostId, data, perMinute, label) {
  const host = $(hostId);
  if (!Array.isArray(data) || !data.length || !data.every(v => isNum(v) && v >= 0)) {
    host.innerHTML = '<div class="empty">This chart appears with the next update</div>';
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
  const total = data.reduce((a, b) => a + b, 0), peak = Math.max(...data);
  svg.setAttribute("aria-label", label + ": " + plural(total, "piece", "pieces") + " finished, busiest " + (perMinute ? "minute " : "hour ") + fmt(peak));
  [...svg.children].forEach((r, i) => {
    const v = data[i], h = v ? Math.max(3, (v / max) * (H - 2)) : 2;
    r.setAttribute("y", H - h); r.setAttribute("height", h);
    r.setAttribute("class", v === 0 ? "zero" : i === n - 1 ? "hot" : "");
    const back = n - 1 - i;
    const when = back === 0 ? (perMinute ? "this minute" : "this hour")
      : perMinute ? (back === 1 ? "1 minute ago" : back + " minutes ago") : (back === 1 ? "1 hour ago" : back + " hours ago");
    r.firstChild.textContent = plural(v, "piece", "pieces") + " finished, " + when;
  });
  return peak;
}

function render(d, prev, prevSource) {
  const total = d.total;
  const pct = clamp(d.percent, 0, 100);
  const blPct = clamp(100 * d.blocked / total, 0, 100 - pct);
  const finished = pct >= 100;
  const perHour = d.last_day / 24;

  count("pct", pct, pct > 0 && pct < 0.1 ? 2 : 1);
  count("verified", d.verified);
  count("verified-of", d.verified);
  count("remaining", d.remaining);
  count("blocked", d.blocked);
  count("last_hour", d.last_hour);
  count("last_day", d.last_day);
  $("total").textContent = fmt(total);
  $("total-2").textContent = fmt(total);
  $("ex-total").textContent = fmt(total);
  $("per-hour").textContent = fmt(perHour, perHour < 10 ? 1 : 0);
  $("verified-sub").innerHTML = d.last_hour > 0
    ? '<span class="up">+' + fmt(d.last_hour) + "</span> in the last hour" : "translated and checked";

  $("seg-go").style.width = pct + "%";
  $("seg-bl").style.left = pct + "%";
  $("seg-bl").style.width = blPct + "%";
  $("car").style.left = pct + "%";
  const track = $("track");
  track.setAttribute("aria-valuenow", pct.toFixed(2));
  track.setAttribute("aria-valuetext", fmt(d.verified) + " of " + fmt(total) + " pieces checked");
  document.querySelectorAll("[data-m]").forEach(el => el.classList.toggle("passed", pct >= Number(el.dataset.m)));

  const next = MILESTONES.find(m => m > pct);
  if (next && !finished) {
    const need = Math.max(1, Math.ceil(next / 100 * total) - d.verified);
    const away = perHour > 0 ? ", " + duration(need / perHour) + " away" : "";
    $("next").innerHTML = "Next milestone: <b>" + next + "%</b> \u2014 " + plural(need, "more piece", "more pieces") + away + ".";
    $("next").hidden = false;
  } else $("next").hidden = true;

  $("ring-go").setAttribute("stroke-dasharray", (pct * 10) + " 1000");
  $("ring-bl").setAttribute("stroke-dasharray", (blPct * 10) + " 1000");
  $("ring-bl").setAttribute("stroke-dashoffset", String(-pct * 10));
  const a = pct / 100 * 2 * Math.PI - Math.PI / 2;
  $("knob").setAttribute("cx", (100 + 78 * Math.cos(a)).toFixed(2));
  $("knob").setAttribute("cy", (100 + 78 * Math.sin(a)).toFixed(2));
  document.querySelectorAll("#ticks line").forEach(l => l.classList.toggle("done", pct > 0 && Number(l.dataset.h) / 24 * 100 <= pct));
  $("clock").textContent = clock(pct);
  const cm = clockMins(pct), ch = Math.floor(cm / 60), cmin = cm % 60;
  $("clock-cap").textContent = finished
    ? "If this project were the 24-hour Le Mans race, we\u2019d have just crossed the finish line."
    : "If this project were the 24-hour Le Mans race, we\u2019d be " +
      (ch ? ch + (ch === 1 ? " hour" : " hours") + (cmin ? " " + cmin + " min" : "") : cmin + " minutes") + " in.";

  const task = finished ? "Chequered flag \u2014 every piece has been checked" : String(d.current_task || "Between updates");
  $("task").textContent = task.charAt(0).toUpperCase() + task.slice(1);

  const e = eta(d);
  $("eta").textContent = e.big;
  $("eta-date").textContent = e.sub;
  const recent = finished ? "" : d.last_hour > 0
    ? " <b>" + plural(d.last_hour, "piece", "pieces") + "</b> finished in the last hour."
    : " No new pieces in the last hour.";
  $("summary").innerHTML = "<b>" + shareWords(pct) + ".</b>" + recent + " " + e.line;

  const pm = bars("chart-m", d.minutely, true, "Pieces finished per minute over the last hour");
  const ph = bars("chart-h", d.hourly, false, "Pieces finished per hour over the last day");
  $("peak-m").textContent = pm == null ? "\u2014" : plural(pm, "piece", "pieces");
  $("peak-h").textContent = ph == null ? "\u2014" : plural(ph, "piece", "pieces");

  document.body.classList.toggle("finished", finished);

  if (prev && S.firstPaintDone) {
    const prevPct = clamp(prev.percent, 0, 100);
    if (d.verified > prev.verified) { pop(d.verified - prev.verified); bump($("st-verified")); }
    const crossed = MILESTONES.filter(m => prevPct < m && pct >= m).pop();
    if (crossed) celebrate(crossed, prevSource === "cache");
  }
  S.firstPaintDone = true;
  status();
}

/* ---------- once-a-second status ---------- */
function status() {
  const d = S.doc, pill = $("pill");
  let mode, label, sub;
  if (!navigator.onLine) { mode = "offline"; label = "You\u2019re offline"; sub = d ? "showing the last numbers we had" : "waiting for a connection"; }
  else if (!d && S.loadFailed) {
    mode = "offline"; label = "Can\u2019t load numbers"; sub = "trying again every few seconds";
    $("task").textContent = "Lost contact with the pit wall \u2014 trying again every few seconds";
    $("summary").textContent = "We can\u2019t reach the latest numbers right now. This page will keep trying.";
  }
  else if (!d) { mode = "loading"; label = "Loading\u2026"; sub = "\u00a0"; }
  else {
    const age = Math.max(0, Date.now() - Date.parse(d.updated));
    if (d.percent >= 100) { mode = "finished"; label = "Finished!"; sub = "every piece checked"; }
    else if (age > STALE_MS) { mode = "stale"; label = "Not updated for a while"; sub = "last update " + ago(age); }
    else if (S.source === "live" && S.liveOk) { mode = "live"; label = "Updating live"; sub = "numbers refresh every second"; }
    else if (S.source === "cache") { mode = "idle"; label = "Loading latest\u2026"; sub = "showing numbers saved on this device"; }
    else if (S.liveUrl !== null && !S.liveOk) { mode = "idle"; label = "Briefly disconnected"; sub = "numbers may be a few minutes old"; }
    else { mode = "idle"; label = "Updated every 10 min"; sub = "last update " + ago(age); }
    $("ago").textContent = ago(age);
    $("ago").title = new Date(Date.parse(d.updated)).toLocaleString();
    $("src").textContent = S.source === "live" ? "live" : S.source === "cache" ? "saved copy on this device" : "10-minute snapshot";
    const lp = Date.parse(d.latest || "");
    $("last-proof-wrap").hidden = Number.isNaN(lp);
    if (!Number.isNaN(lp)) $("last-proof").textContent = ago(Math.max(0, Date.now() - lp));
    document.title = fmt(clamp(d.percent, 0, 100), 1) + "% done \u00b7 Test Drive Le Mans preservation";
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
    if (valid(cached)) { S.doc = cached; S.source = "cache"; render(cached, null, null); }
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
