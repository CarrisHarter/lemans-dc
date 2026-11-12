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
<meta charset="utf-8">
<meta name="viewport" content="width=device-width,initial-scale=1">
<title>Le Mans 24h (Dreamcast) — port progress</title>
<style>
  :root { color-scheme: light dark; }
  body { margin: 0; min-height: 100dvh; display: grid; place-items: center;
         font: 16px/1.5 ui-monospace, SFMono-Regular, Menlo, monospace;
         background: #0d1117; color: #e6edf3; }
  @media (prefers-color-scheme: light) { body { background: #f6f8fa; color: #1f2328; } }
  .card { width: min(920px, 92vw); padding: 2rem; }
  h1 { font-size: 1.1rem; font-weight: 600; letter-spacing: .02em; margin: 0 0 1.5rem; }
  .bar { height: 22px; border-radius: 11px; overflow: hidden; background: #21262d; }
  @media (prefers-color-scheme: light) { .bar { background: #d0d7de; } }
  .fill { height: 100%; width: 0; background: linear-gradient(90deg,#2f81f7,#39d353);
          transition: width .6s ease; }
  .pct { font-size: 2.4rem; font-weight: 700; margin: 1rem 0 .25rem; }
  .sub { opacity: .75; margin-bottom: 1rem; }
  .task { margin: 0 0 1.75rem; padding: .55rem .8rem; border-left: 3px solid #2f81f7;
          background: #161b22; border-radius: 0 8px 8px 0; }
  @media (prefers-color-scheme: light) { .task { background: #eef1f4; } }
  .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(150px, 1fr)); gap: .75rem; }
  .cell { border: 1px solid #30363d; border-radius: 10px; padding: .8rem .9rem; }
  @media (prefers-color-scheme: light) { .cell { border-color: #d0d7de; } }
  .k { font-size: .72rem; text-transform: uppercase; letter-spacing: .08em; opacity: .6; }
  .v { font-size: 1.25rem; font-weight: 600; }
  footer { margin-top: 1.75rem; font-size: .78rem; opacity: .55; }
  .notice { margin-top: .4rem; font-size: .68rem; opacity: .4; max-width: 72ch; }
  a { color: inherit; }
</style>
<h1>Test Drive Le Mans (Dreamcast) — verified Rust port progress</h1>
<div class="card" style="padding:0">
  <div class="pct" id="pct">—</div>
  <div class="bar"><div class="fill" id="fill"></div></div>
  <div class="sub" id="sub">loading…</div>
  <div class="task"><span class="k">current task</span> <span id="task">—</span></div>
  <div class="grid">
    <div class="cell"><div class="k">verified</div><div class="v" id="verified">—</div></div>
    <div class="cell"><div class="k">blocked</div><div class="v" id="blocked">—</div></div>
    <div class="cell"><div class="k">remaining</div><div class="v" id="remaining">—</div></div>
    <div class="cell"><div class="k">last hour</div><div class="v" id="last_hour">—</div></div>
    <div class="cell"><div class="k">last 24h</div><div class="v" id="last_day">—</div></div>
    <div class="cell"><div class="k">pace / day</div><div class="v" id="pace">—</div></div>
    <div class="cell"><div class="k">ETA</div><div class="v" id="eta">—</div></div>
    <div class="cell"><div class="k">evidence files</div><div class="v" id="evidence">—</div></div>
  </div>
  <footer id="foot"></footer>
  <div class="notice">This technical analysis is unofficial and is not endorsed by or affiliated with Atari SA or any of its licensors. All rights in <em>Le Mans 24 Hours</em> remain with their respective owners.</div>
</div>
<script>
const el = id => document.getElementById(id);
function eta(d) { if (d == null) return "—";
  return d < 1 ? Math.round(d*24) + "h" : d < 60 ? Math.round(d) + "d" : (d/30).toFixed(1) + "mo"; }
async function load() {
  for (const url of ["api/progress", "progress.json"]) {
    try { return await (await fetch(url, {cache: "no-store"})).json(); } catch (e) {}
  }
  throw new Error("no data");
}
async function tick() {
  try {
    const d = await load();
    el("pct").textContent = d.percent.toFixed(1) + "%";
    el("fill").style.width = d.percent + "%";
    el("sub").textContent = d.verified.toLocaleString() + " of " +
      d.total.toLocaleString() + " functions proven equivalent to the original SH-4 machine code";
    el("task").textContent = d.current_task;
    for (const k of ["verified","blocked","remaining","last_hour","last_day","evidence_files"])
      el(k === "evidence_files" ? "evidence" : k).textContent = d[k].toLocaleString();
    el("pace").textContent = d.pace_per_day.toLocaleString();
    el("eta").textContent = eta(d.eta_days);
    el("foot").textContent = "updated " + d.updated;
  } catch (e) { el("sub").textContent = "reconnecting…"; }
}
tick(); setInterval(tick, 30000);
</script>
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
