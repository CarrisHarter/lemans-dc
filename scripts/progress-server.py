#!/usr/bin/env python3
"""Serve the porting-progress dashboard over HTTP.

Usage:
  scripts/progress-server.py [--host 127.0.0.1] [--port 8787] [--interval 30]
  scripts/progress-server.py --static-out public/     # write index.html + progress.json

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
<meta name="viewport" content="width=device-width,initial-scale=1">
<meta name="color-scheme" content="dark light">
<title>Test Drive Le Mans — Rust port progress</title>
<link rel="icon" href="data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 4 4'%3E%3Cpath fill='%23fff' d='M0 0h4v4H0z'/%3E%3Cpath fill='%23111' d='M0 0h1v1H0zM2 0h1v1H2zM1 1h1v1H1zM3 1h1v1H3zM0 2h1v1H0zM2 2h1v1H2zM1 3h1v1H1zM3 3h1v1H3z'/%3E%3C/svg%3E">
<style>
  :root {
    --bg: #07090d; --panel: rgba(255,255,255,.035); --panel-2: rgba(255,255,255,.06);
    --line: rgba(255,255,255,.09); --fg: #eef2f7; --muted: #8b95a5; --faint: #5b6473;
    --go: #22c55e; --go-2: #86efac; --warn: #f5a524; --live: #ff3b3b; --track: #141922;
    --sans: "Inter", ui-sans-serif, system-ui, -apple-system, "Segoe UI", Roboto, sans-serif;
    --mono: ui-monospace, "JetBrains Mono", "SF Mono", SFMono-Regular, Menlo, Consolas, monospace;
  }
  @media (prefers-color-scheme: light) {
    :root { --bg: #f4f6f9; --panel: #ffffff; --panel-2: #f0f3f7; --line: rgba(15,23,42,.1);
            --fg: #0f172a; --muted: #5b6577; --faint: #8a94a6; --track: #e3e8ef;
            --go: #16a34a; --go-2: #4ade80; }
  }
  * { box-sizing: border-box; }
  html { background: var(--bg); }
  body {
    margin: 0; min-height: 100dvh; color: var(--fg); font: 15px/1.55 var(--sans);
    -webkit-font-smoothing: antialiased; display: flex; justify-content: center;
    padding: clamp(1.25rem, 5vw, 4rem) clamp(1rem, 4vw, 2rem);
    background:
      radial-gradient(1100px 520px at 12% -10%, rgba(34,197,94,.13), transparent 60%),
      radial-gradient(900px 480px at 110% 0%, rgba(255,59,59,.08), transparent 60%),
      var(--bg);
  }
  .wrap { width: min(1040px, 100%); }
  .num { font-family: var(--mono); font-variant-numeric: tabular-nums; letter-spacing: -.02em; }
  .eyebrow { font: 600 .72rem/1 var(--mono); letter-spacing: .18em; text-transform: uppercase;
             color: var(--muted); }

  header { display: flex; align-items: flex-start; justify-content: space-between; gap: 1rem;
           margin-bottom: clamp(1.75rem, 4vw, 2.75rem); }
  h1 { margin: .55rem 0 .35rem; font-size: clamp(1.6rem, 3.6vw, 2.35rem); line-height: 1.1;
       font-weight: 750; letter-spacing: -.025em; }
  h1 span { color: var(--muted); font-weight: 500; }
  .lede { margin: 0; color: var(--muted); max-width: 60ch; }

  .badge { flex: none; display: inline-flex; align-items: center; gap: .5rem; padding: .45rem .8rem;
           border: 1px solid var(--line); border-radius: 999px; background: var(--panel);
           font: 700 .72rem/1 var(--mono); letter-spacing: .16em; text-transform: uppercase; }
  .dot { width: .55rem; height: .55rem; border-radius: 50%; background: var(--faint); }
  .badge[data-s="live"] .dot { background: var(--live); box-shadow: 0 0 0 0 rgba(255,59,59,.6);
                              animation: pulse 1.8s infinite; }
  .badge[data-s="stale"] { color: var(--warn); }
  .badge[data-s="stale"] .dot { background: var(--warn); }
  @keyframes pulse { 70% { box-shadow: 0 0 0 .55rem rgba(255,59,59,0); }
                     100% { box-shadow: 0 0 0 0 rgba(255,59,59,0); } }

  .panel { background: var(--panel); border: 1px solid var(--line); border-radius: 20px; }
  .hero { padding: clamp(1.25rem, 3.5vw, 2.25rem); }
  .hero-top { display: flex; flex-wrap: wrap; align-items: flex-end; justify-content: space-between;
              gap: 1.25rem 2rem; margin-bottom: 1.6rem; }
  .pct { font-size: clamp(4rem, 13vw, 7.5rem); font-weight: 700; line-height: .85; }
  .pct small { font-size: .38em; color: var(--muted); margin-left: .08em; font-weight: 600; }
  .of { margin-top: .9rem; color: var(--muted); }
  .of b { color: var(--fg); font-weight: 600; }
  .clock { text-align: right; }
  .clock .num { font-size: clamp(1.9rem, 4.6vw, 2.7rem); font-weight: 600; line-height: 1; }
  .clock .num span { color: var(--faint); }
  .clock .eyebrow { display: block; margin-bottom: .55rem; }
  .clock p { margin: .45rem 0 0; color: var(--faint); font-size: .8rem; }

  .track { position: relative; height: 30px; border-radius: 10px; overflow: hidden;
           background: repeating-linear-gradient(90deg, transparent 0 46px, var(--line) 46px 47px),
                       var(--track); box-shadow: inset 0 1px 2px rgba(0,0,0,.35); }
  .seg { position: absolute; top: 0; bottom: 0; left: 0; width: 0;
         transition: width 1.1s cubic-bezier(.2,.8,.2,1), left 1.1s cubic-bezier(.2,.8,.2,1); }
  .seg.go { background: linear-gradient(90deg, #15803d, var(--go) 70%, var(--go-2));
            box-shadow: 0 0 24px rgba(34,197,94,.35); }
  .seg.go::after { content: ""; position: absolute; inset: 0 0 50% 0;
                   background: linear-gradient(rgba(255,255,255,.22), transparent); }
  .seg.go::before { content: ""; position: absolute; right: 0; top: -2px; bottom: -2px; width: 3px;
                    background: #fff; box-shadow: 0 0 12px 2px rgba(134,239,172,.9); }
  .seg.bl { background: repeating-linear-gradient(135deg, var(--warn) 0 6px, #b8790f 6px 12px);
            opacity: .9; }
  .flag { position: absolute; right: 0; top: 0; bottom: 0; width: 18px; opacity: .85;
          background: conic-gradient(#fff 25%, #111 0 50%, #fff 0 75%, #111 0) 0 0 / 10px 10px; }
  .legend { display: flex; flex-wrap: wrap; gap: .5rem 1.4rem; margin-top: .9rem;
            color: var(--muted); font-size: .82rem; }
  .legend i { display: inline-block; width: .7rem; height: .7rem; border-radius: 3px;
              margin-right: .45rem; vertical-align: -1px; }

  .now { display: flex; align-items: center; gap: .9rem; margin: 1rem 0;
         padding: 1rem 1.25rem; }
  .now .eyebrow { flex: none; padding-right: .9rem; border-right: 1px solid var(--line); }
  .now p { margin: 0; font-size: 1.02rem; }

  .stats { display: grid; grid-template-columns: repeat(4, 1fr); gap: 1rem; }
  .stat { padding: 1.1rem 1.2rem 1.15rem; }
  .stat .num { display: block; margin-top: .6rem; font-size: clamp(1.5rem, 3vw, 2rem);
               font-weight: 650; line-height: 1.05; }
  .stat p { margin: .35rem 0 0; color: var(--faint); font-size: .8rem; }
  .stat.go .num { color: var(--go); }
  .stat.bl .num { color: var(--warn); }
  .mini { display: grid; grid-template-columns: repeat(4, 1fr); margin-top: 1rem; overflow: hidden; }
  .mini div { padding: .95rem 1.2rem; border-left: 1px solid var(--line); }
  .mini div:first-child { border-left: 0; }
  .mini .num { display: block; margin-top: .4rem; font-size: 1.15rem; font-weight: 600; }

  footer { margin-top: 2rem; padding-top: 1.25rem; border-top: 1px solid var(--line);
           color: var(--faint); font-size: .78rem; }
  footer .num { color: var(--muted); }
  .notice { margin: .6rem 0 0; max-width: 80ch; opacity: .8; }

  @media (max-width: 760px) {
    .stats, .mini { grid-template-columns: repeat(2, 1fr); }
    .mini div:nth-child(3) { border-left: 0; }
    .mini div:nth-child(n+3) { border-top: 1px solid var(--line); }
    .clock { text-align: left; }
    .now { flex-direction: column; align-items: flex-start; gap: .5rem; }
    .now .eyebrow { border: 0; padding: 0; }
  }
  @media (prefers-reduced-motion: reduce) {
    .seg { transition: none; } .badge[data-s="live"] .dot { animation: none; }
  }
</style>
<div class="wrap">
  <header>
    <div>
      <div class="eyebrow">Sega Dreamcast &middot; 2001 &middot; SH-4 &rarr; Rust</div>
      <h1>Test Drive Le Mans <span>&mdash; port progress</span></h1>
      <p class="lede">Every function of the original game is rewritten in Rust and machine-checked
        against the original code. This board tracks how many have crossed the line.</p>
    </div>
    <div class="badge" id="badge" data-s="idle"><span class="dot"></span><span id="badge-t">Loading</span></div>
  </header>

  <section class="panel hero">
    <div class="hero-top">
      <div>
        <div class="pct num"><span id="pct">0.0</span><small>%</small></div>
        <div class="of"><b class="num" id="verified-of">&mdash;</b> of <span class="num" id="total">&mdash;</span>
          functions proven identical to the original</div>
      </div>
      <div class="clock">
        <span class="eyebrow">Race clock</span>
        <div class="num" id="clock">00:00<span> / 24:00</span></div>
        <p>progress, told as a 24-hour race</p>
      </div>
    </div>
    <div class="track" id="track" role="progressbar" aria-valuemin="0" aria-valuemax="100" aria-label="Functions verified">
      <div class="seg go" id="seg-go"></div>
      <div class="seg bl" id="seg-bl"></div>
      <div class="flag"></div>
    </div>
    <div class="legend">
      <span><i style="background:var(--go)"></i>Verified</span>
      <span><i style="background:var(--warn)"></i>Blocked</span>
      <span><i style="background:var(--track);box-shadow:inset 0 0 0 1px var(--line)"></i>Still to go</span>
    </div>
  </section>

  <section class="panel now">
    <span class="eyebrow">Right now</span>
    <p id="task">Checking in with the pit wall&hellip;</p>
  </section>

  <section class="stats">
    <div class="panel stat go"><span class="eyebrow">Verified</span><span class="num" id="verified">&mdash;</span><p>proven equivalent</p></div>
    <div class="panel stat"><span class="eyebrow">Still to go</span><span class="num" id="remaining">&mdash;</span><p>not yet attempted</p></div>
    <div class="panel stat bl"><span class="eyebrow">Blocked</span><span class="num" id="blocked">&mdash;</span><p>parked for later</p></div>
    <div class="panel stat"><span class="eyebrow">Finish in</span><span class="num" id="eta">&mdash;</span><p id="eta-date">at current pace</p></div>
  </section>

  <section class="panel mini">
    <div><span class="eyebrow">Last hour</span><span class="num" id="last_hour">&mdash;</span></div>
    <div><span class="eyebrow">Last 24 h</span><span class="num" id="last_day">&mdash;</span></div>
    <div><span class="eyebrow">Pace / day</span><span class="num" id="pace">&mdash;</span></div>
    <div><span class="eyebrow">Evidence files</span><span class="num" id="evidence">&mdash;</span></div>
  </section>

  <footer>
    <div>Updated <span class="num" id="ago">&mdash;</span> &middot; refreshes automatically</div>
    <p class="notice">This technical analysis is unofficial and is not endorsed by or affiliated with Atari SA or any of its licensors. All rights in <em>Le Mans 24 Hours</em> remain with their respective owners.</p>
  </footer>
</div>
<script>
const $ = id => document.getElementById(id);
const STALE_MS = 2 * 3600e3;
const still = matchMedia("(prefers-reduced-motion: reduce)").matches;
const shown = {};
let updatedAt = null;

const fmt = (n, dp = 0) => n.toLocaleString("en-US", {minimumFractionDigits: dp, maximumFractionDigits: dp});

function count(id, to, dp = 0) {
  const node = $(id), from = shown[id] ?? 0;
  shown[id] = to;
  if (still || from === to) { node.textContent = fmt(to, dp); return; }
  const t0 = performance.now(), dur = 1100;
  const step = t => {
    const p = Math.min((t - t0) / dur, 1), e = 1 - Math.pow(1 - p, 3);
    node.textContent = fmt(from + (to - from) * e, dp);
    if (p < 1) requestAnimationFrame(step);
  };
  requestAnimationFrame(step);
}

function ago(ms) {
  const s = Math.max(0, Math.round(ms / 1000));
  if (s < 60) return "just now";
  const m = Math.round(s / 60);
  if (m < 60) return m + " min ago";
  const h = Math.floor(m / 60);
  return h < 48 ? h + " h " + (m % 60) + " min ago" : Math.round(h / 24) + " days ago";
}

function badge() {
  if (!updatedAt) return;
  const age = Date.now() - updatedAt;
  $("ago").textContent = ago(age);
  $("ago").title = new Date(updatedAt).toLocaleString();
  const b = $("badge");
  if (age > STALE_MS) { b.dataset.s = "stale"; $("badge-t").textContent = "Stale"; }
  else if (b.dataset.live === "1") { b.dataset.s = "live"; $("badge-t").textContent = "Live"; }
  else { b.dataset.s = "idle"; $("badge-t").textContent = "Pit stop"; }
}

async function load() {
  for (const url of ["api/progress", "progress.json"]) {
    try {
      const r = await fetch(url, {cache: "no-store"});
      if (r.ok) return await r.json();
    } catch (e) {}
  }
  throw new Error("no data");
}

async function tick() {
  let d;
  try { d = await load(); } catch (e) { $("task").textContent = "Lost contact with the pit wall \\u2014 retrying\\u2026"; return; }

  count("pct", d.percent, 1);
  count("verified", d.verified);
  count("verified-of", d.verified);
  count("remaining", d.remaining);
  count("blocked", d.blocked);
  count("last_hour", d.last_hour);
  count("last_day", d.last_day);
  count("evidence", d.evidence_files);
  $("total").textContent = fmt(d.total);
  $("pace").textContent = fmt(d.pace_per_day);

  const blockedPct = 100 * d.blocked / d.total;
  $("seg-go").style.width = d.percent + "%";
  $("seg-bl").style.left = d.percent + "%";
  $("seg-bl").style.width = blockedPct + "%";
  $("track").setAttribute("aria-valuenow", d.percent);

  const mins = Math.round(d.percent / 100 * 1440);
  $("clock").firstChild.nodeValue =
    String(Math.floor(mins / 60)).padStart(2, "0") + ":" + String(mins % 60).padStart(2, "0");

  const task = d.current_task || "";
  $("task").textContent = task.charAt(0).toUpperCase() + task.slice(1);

  if (d.eta_days == null) { $("eta").textContent = "\\u2014"; $("eta-date").textContent = "waiting for pace data"; }
  else {
    const days = d.eta_days;
    $("eta").textContent = days < 1 ? Math.round(days * 24) + " h" : fmt(days, 1) + " days";
    const when = new Date(Date.now() + days * 864e5);
    $("eta-date").textContent = "around " + when.toLocaleDateString(undefined, {weekday: "short", day: "numeric", month: "short"});
  }

  updatedAt = Date.parse(d.updated);
  $("badge").dataset.live = d.last_hour > 0 ? "1" : "0";
  badge();
}

tick();
setInterval(tick, 30000);
setInterval(badge, 15000);
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


def write_static(out_dir: Path, state: State):
    out_dir.mkdir(parents=True, exist_ok=True)
    state.refresh()
    (out_dir / "index.html").write_text(PAGE)
    (out_dir / "progress.json").write_text(json.dumps(state.get(), indent=2) + "\n")
    return out_dir


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--host", default="127.0.0.1")
    ap.add_argument("--port", type=int, default=8787)
    ap.add_argument("--interval", type=float, default=30.0)
    ap.add_argument("--static-out", type=Path, default=None,
                    help="write index.html + progress.json here and exit")
    args = ap.parse_args()

    state = State(args.interval)
    if args.static_out:
        out = write_static(args.static_out, state)
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
