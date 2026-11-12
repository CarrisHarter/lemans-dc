#!/usr/bin/env python3
"""Live porting-progress dashboard for a second terminal.

Usage:  scripts/progress-tui.py [--once] [--interval 5]

Metrics come from scripts/progress_data.py (aggregates only). No curses, no
dependencies — ANSI redraw each tick, Ctrl-C to quit.
"""
import sys
import time
from datetime import datetime

import progress_data

BAR_W = 48


def fmt_eta(days):
    if days is None:
        return "—"
    if days < 1:
        return f"~{days * 24:.0f}h"
    if days < 60:
        return f"~{days:.0f}d"
    return f"~{days / 30:.1f}mo"


def render(s):
    pct = s["percent"]
    fill = int(BAR_W * s["verified"] / s["total"])
    bar = "█" * fill + "░" * (BAR_W - fill)
    return "\n".join([
        f"  Le Mans DC — verified Rust port   {datetime.now():%H:%M:%S}",
        f"  ┌{'─' * (BAR_W + 2)}┐",
        f"  │ {bar} │ {pct:5.1f}%",
        f"  └{'─' * (BAR_W + 2)}┘",
        f"  verified {s['verified']}/{s['total']}   "
        f"blocked {s['blocked']}   remaining {s['remaining']}",
        f"  evidence files {s['evidence_files']}   port files {s['port_files']}",
        f"  last hour {s['last_hour']}   last 24h {s['last_day']}   "
        f"pace {s['pace_per_day']:.0f}/day   ETA {fmt_eta(s['eta_days'])}",
        "  ── live view ── scripts/progress-server.py  (http://127.0.0.1:8787/)",
    ])


def main(argv):
    once = "--once" in argv
    interval = 5.0
    if "--interval" in argv:
        try:
            interval = float(argv[argv.index("--interval") + 1])
        except (ValueError, IndexError):
            pass
    if once:
        print(render(progress_data.snapshot()))
        return
    try:
        first = True
        while True:
            out = render(progress_data.snapshot())
            if not first:
                sys.stdout.write(f"\033[{out.count(chr(10)) + 1}A")
            sys.stdout.write(out + "\n")
            sys.stdout.flush()
            first = False
            time.sleep(interval)
    except KeyboardInterrupt:
        print()


if __name__ == "__main__":
    sys.exit(main(sys.argv))
