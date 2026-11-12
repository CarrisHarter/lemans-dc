#!/usr/bin/env python3
"""Aggregate progress metrics for the porting loop.

Deliberately publishes COUNTS ONLY: no addresses, no function names, no code,
no disc assets. The Atari SA grant (notes/permission/) covers publication of
notes and pseudocode; the port, the ROM and extracted assets must never be
published. Keeping the shared export to aggregates means anything built on
this module is safe to expose publicly.

Both the terminal dashboard (scripts/progress-tui.py) and the web server
(scripts/progress-server.py) read from here.
"""
import json
import re
import subprocess
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOTAL = 5211  # functions exported (analysis/export/pseudocode/functions/)
VERIF = ROOT / "notes/verifications"
PORTS = ROOT / "rust/lemans-verify/src/ports"
LEDGER = ROOT / "notes/address-map.md"
CANONICAL_RE = re.compile(r"^0x[0-9a-f]{8}$")  # 0xADDR only; seeds add .tag or -tag


def sh(*args):
    try:
        return subprocess.run(
            args, capture_output=True, text=True, cwd=ROOT, timeout=10
        ).stdout.strip()
    except (subprocess.SubprocessError, OSError):
        return ""


def _parse_ts(ts):
    try:
        return datetime.fromisoformat(ts)
    except (ValueError, TypeError):
        return None


def _verified_timestamps():
    """Verification timestamps of canonical (non-branch-seed) passing evidence.

    A canonical evidence file is exactly `0x` + 8 hex digits. Branch seeds use
    `<addr>.<tag>.json`, so any other suffix — dot or hyphen — must not be
    counted as its own function (a hyphenated tag once inflated the count by 3).
    """
    stamps = []
    for f in VERIF.glob("0x*.json"):
        if not CANONICAL_RE.match(f.stem):
            continue
        try:
            doc = json.loads(f.read_text())
        except (json.JSONDecodeError, OSError):
            continue
        if doc.get("result") == "pass":
            t = _parse_ts(doc.get("verified_at", ""))
            if t:
                stamps.append(t)
    return stamps


def blocked_count():
    try:
        text = LEDGER.read_text()
    except OSError:
        return 0
    section = text.split("## Blocked", 1)[1].split("## ", 1)[0]
    return len(re.findall(r"^\| `0x[0-9a-f]{8}` \|", section, re.M))


def current_task(last_hour: int, last_day: int) -> str:
    """One plain-language line describing what the porting loop is doing now.

    Derived from aggregate timestamps only: no addresses, names, or code.
    """
    if last_hour:
        return f"verifying rewritten functions — {last_hour} proven in the last hour"
    if last_day:
        return f"between verification runs — {last_day} proven in the last 24 hours"
    return "idle — no functions proven in the last 24 hours"


def snapshot():
    """Return the public, aggregate-only progress document."""
    now = datetime.now(timezone.utc)
    stamps = _verified_timestamps()
    verified = len(stamps)
    blocked = blocked_count()
    last24 = sum(1 for t in stamps if (now - t).total_seconds() < 86400)
    last1h = sum(1 for t in stamps if (now - t).total_seconds() < 3600)
    remaining = max(TOTAL - verified - blocked, 0)
    rate_day = float(last24)
    eta_days = (remaining / rate_day) if rate_day > 0 else None
    return {
        "updated": now.isoformat(timespec="seconds"),
        "total": TOTAL,
        "verified": verified,
        "blocked": blocked,
        "remaining": remaining,
        "percent": round(100.0 * verified / TOTAL, 2),
        "evidence_files": sum(1 for _ in VERIF.glob("0x*.json")),
        "port_files": sum(1 for _ in PORTS.glob("p_*.rs")),
        "last_hour": last1h,
        "last_day": last24,
        "pace_per_day": round(rate_day, 1),
        "eta_days": round(eta_days, 1) if eta_days is not None else None,
        "current_task": current_task(last1h, last24),
        # commit subjects are intentionally omitted: they contain addresses and
        # proposed names, which stay in the repo, not on a public endpoint.
    }
