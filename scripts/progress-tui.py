#!/usr/bin/env python3
"""Live porting-progress dashboard for a second terminal.

Usage:  scripts/progress-tui.py [--once] [--interval 5]

Reads only: verification evidence on disk + git log. No curses, no
dependencies — ANSI redraw each tick, Ctrl-C to quit.
"""
import json
import re
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOTAL = 5211  # functions exported (notes/05-progress.md)
VERIF = ROOT / "notes/verifications"
PORTS = ROOT / "rust/lemans-verify/src/ports"
LEDGER = ROOT / "notes/address-map.md"
BAR_W = 48


def sh(*args):
    try:
        return subprocess.run(args, capture_output=True, text=True, cwd=ROOT,
                              timeout=10).stdout.strip()
    except (subprocess.SubprocessError, OSError):
        return ""


def verified_addrs():
    """Distinct addresses whose canonical evidence file says pass."""
    addrs = {}
    for f in VERIF.glob("0x*.json"):
        if "." in f.stem[2:]:  # branch seed (0xADDR.name.json), not canonical
            continue
        try:
            d = json.loads(f.read_text())
        except (json.JSONDecodeError, OSError):
            continue
        if d.get("result") == "pass":
            addrs[f.stem.lower()] = d.get("verified_at", "")
    return addrs


def blocked_count():
    """Rows in the ledger's Blocked table."""
    try:
        text = LEDGER.read_text()
    except OSError:
        return 0
    section = text.split("## Blocked", 1)[1].split("## ", 1)[0]
    return len(re.findall(r"^\| `0x[0-9a-f]{8}` \|", section, re.M))


def parse_ts(ts):
    try:
        return datetime.fromisoformat(ts)
    except (ValueError, TypeError):
        return None


def snapshot():
    addrs = verified_addrs()
    now = datetime.now(timezone.utc)
    stamps = sorted(t for t in (parse_ts(v) for v in addrs.values()) if t)
    last24 = sum(1 for t in stamps if (now - t).total_seconds() < 86400)
    last1h = [t for t in stamps if (now - t).total_seconds() < 3600]

    # Pace from the trailing 24h window: all evidence timestamps cluster in
    # the current session, so first-to-last span collapses and inflates rate.
    rate_day = float(last24)
    remaining = TOTAL - len(addrs) - blocked_count()
    eta_days = (remaining / rate_day) if rate_day > 0 else None

    log = sh("git", "log", "-5", "--format=%h %ar | %s")
    return {
        "verified": len(addrs),
        "files": sum(1 for _ in VERIF.glob("0x*.json")),
        "ports": sum(1 for _ in PORTS.glob("p_*.rs")),
        "blocked": blocked_count(),
        "remaining": remaining,
        "last24": last24,
        "last1h": len(last1h),
        "rate_day": rate_day,
        "eta_days": eta_days,
        "log": log.splitlines() if log else [],
    }


def fmt_eta(days):
    if days is None:
        return "—"
    if days < 1:
        return f"~{days * 24:.0f}h"
    if days < 60:
        return f"~{days:.0f}d"
    return f"~{days / 30:.1f}mo"


def render(s):
    pct = 100 * s["verified"] / TOTAL
    fill = int(BAR_W * s["verified"] / TOTAL)
    bar = "█" * fill + "░" * (BAR_W - fill)
    lines = [
        f"  Le Mans DC — verified Rust port   {datetime.now():%H:%M:%S}",
        f"  ┌{'─' * (BAR_W + 2)}┐",
        f"  │ {bar} │ {pct:5.1f}%",
        f"  └{'─' * (BAR_W + 2)}┘",
        f"  verified {s['verified']}/{TOTAL}   "
        f"blocked {s['blocked']}   remaining {s['remaining']}",
        f"  evidence files {s['files']}   port files {s['ports']}",
        f"  last hour {s['last1h']}   last 24h {s['last24']}   "
        f"pace {s['rate_day']:.0f}/day   ETA {fmt_eta(s['eta_days'])}",
        "  ── latest ──",
    ]
    lines += [f"  {l[:76]}" for l in s["log"][:5]] or ["  (no git log)"]
    return "\n".join(lines)


def main(argv):
    once = "--once" in argv
    interval = 5.0
    if "--interval" in argv:
        try:
            interval = float(argv[argv.index("--interval") + 1])
        except (ValueError, IndexError):
            pass
    if once:
        print(render(snapshot()))
        return
    try:
        first = True
        while True:
            out = render(snapshot())
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
