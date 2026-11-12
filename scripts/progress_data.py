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
        t = datetime.fromisoformat(ts)
    except (ValueError, TypeError):
        return None
    return t if t.tzinfo else t.replace(tzinfo=timezone.utc)


# path -> ((mtime_ns, size), pass timestamp or None); lets the live server
# re-scan every second without re-parsing unchanged evidence.
_evidence_cache = {}


def _verified_timestamps():
    """Verification timestamps of canonical (non-branch-seed) passing evidence.

    A canonical evidence file is exactly `0x` + 8 hex digits. Branch seeds use
    `<addr>.<tag>.json`, so any other suffix — dot or hyphen — must not be
    counted as its own function (a hyphenated tag once inflated the count by 3).
    """
    stamps = []
    seen = set()
    for f in VERIF.glob("0x*.json"):
        if not CANONICAL_RE.match(f.stem):
            continue
        try:
            st = f.stat()
        except OSError:
            continue
        key = (st.st_mtime_ns, st.st_size)
        seen.add(f)
        hit = _evidence_cache.get(f)
        if hit and hit[0] == key:
            t = hit[1]
        else:
            try:
                doc = json.loads(f.read_text())
            except (json.JSONDecodeError, OSError, UnicodeDecodeError):
                continue  # possibly mid-write; retry next scan
            t = _parse_ts(doc.get("verified_at", "")) if doc.get("result") == "pass" else None
            _evidence_cache[f] = (key, t)
        if t:
            stamps.append(t)
    for gone in _evidence_cache.keys() - seen:
        del _evidence_cache[gone]
    return stamps


def blocked_count():
    try:
        text = LEDGER.read_text()
    except OSError:
        return 0
    if "## Blocked" not in text:
        return 0
    section = text.split("## Blocked", 1)[1].split("## ", 1)[0]
    return len(re.findall(r"^\| `0x[0-9a-f]{8}` \|", section, re.M))


def _buckets(ages, width, count):
    """Histogram of ages (seconds) into `count` buckets of `width`, oldest first."""
    out = [0] * count
    for a in ages:
        i = int(a // width)
        if 0 <= i < count:
            out[count - 1 - i] += 1
    return out


def current_task(last_hour: int, last_day: int) -> str:
    """One line describing what the porting loop is doing now.

    Derived from aggregate timestamps only: no addresses, names, or code.
    """
    if last_hour:
        return f"verifying ports against SH-4 oracle — {last_hour:,} proven in last 1h"
    if last_day:
        return f"idle — {last_day:,} proven in last 24h"
    return "idle — 0 proven in last 24h"


def snapshot():
    """Return the public, aggregate-only progress document."""
    now = datetime.now(timezone.utc)
    stamps = _verified_timestamps()
    verified = len(stamps)
    blocked = blocked_count()
    ages = [max((now - t).total_seconds(), 0.0) for t in stamps]  # clock skew: future = now
    last24 = sum(1 for a in ages if a < 86400)
    last1h = sum(1 for a in ages if a < 3600)
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
        "latest": max(stamps).isoformat(timespec="seconds") if stamps else None,
        "hourly": _buckets(ages, 3600, 24),  # proofs per hour, last 24 h, oldest first
        "minutely": _buckets(ages, 60, 60),  # proofs per minute, last 60 min, oldest first
        # commit subjects are intentionally omitted: they contain addresses and
        # proposed names, which stay in the repo, not on a public endpoint.
    }
