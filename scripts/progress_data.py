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
from collections import Counter
from datetime import datetime, timedelta, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOTAL = 5211  # functions exported (analysis/export/pseudocode/functions/)
# Schedule: the first forecast the public page published (1,373 verified,
# 3,751 remaining, ETA 2.7 d), as a straight line from that moment.
PLAN_START = datetime(2026, 11, 12, 2, 11, 4, tzinfo=timezone.utc)
PLAN_START_VERIFIED = 1373
PLAN_TARGET = 1373 + 3751
PLAN_DAYS = 2.7
RATE_WINDOW_H = 3  # primary pace window; 1 h and 6 h bound the ETA range
BLOCKED_TREND_H = 6  # blocked count is compared with the ledger this long ago
HISTORY_POINTS = 48  # samples in the cumulative verified series
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
_first_added = {"head": None, "map": {}}


def _first_committed():
    """Evidence path (relative to ROOT) -> time it was first committed.

    Re-verification rewrites `verified_at` (a full re-run once moved 1,194
    proofs into three minutes), so the first commit bounds when a function was
    really first proven. Recomputed only when HEAD moves.
    """
    head = sh("git", "rev-parse", "HEAD")
    if head and head != _first_added["head"]:
        out = sh("git", "log", "--diff-filter=A", "--format=@%cI", "--name-only", "--", "notes/verifications")
        first, when = {}, None
        for line in out.splitlines():  # newest first: later lines overwrite with older commits
            if line.startswith("@"):
                when = _parse_ts(line[1:])
            elif line and when:
                first[line] = when
        _first_added.update(head=head, map=first)
    return _first_added["map"]


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


def _split_bulk(stamps):
    """(organic stamps, bulk count): drop minutes with >= BULK_PER_MIN proofs."""
    per_min = Counter(t.replace(second=0, microsecond=0) for t in stamps)
    organic = [t for t in stamps if per_min[t.replace(second=0, microsecond=0)] < BULK_PER_MIN]
    return organic, len(stamps) - len(organic)


def _rates(now, organic):
    """Proofs/hour over the last 1, RATE_WINDOW_H, 6 and 24 h of organic work.

    A window reaching back before the first organic proof is shortened to it, so
    a loop that started 2 h ago isn't diluted by idle time before it existed.
    """
    if not organic:
        return {}
    since = max((now - min(organic)).total_seconds() / 3600, 1 / 6)
    out = {}
    for h in (1, RATE_WINDOW_H, 6, 24):
        n = sum(1 for t in organic if (now - t).total_seconds() < h * 3600)
        out[h] = n / min(h, since)
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


def schedule(now, verified, finish):
    """Position against the plan: functions ahead (+) / behind (-) of the
    straight-line plan now, and hours the projected finish is early (+) / late (-)."""
    plan_end = PLAN_START + timedelta(days=PLAN_DAYS)
    frac = min(max((now - PLAN_START) / (plan_end - PLAN_START), 0.0), 1.0)
    planned = round(PLAN_START_VERIFIED + (PLAN_TARGET - PLAN_START_VERIFIED) * frac)
    return {
        "planned": planned,
        "ahead": verified - planned,
        "early_hours": round((plan_end - finish).total_seconds() / 3600, 1) if finish else None,
        "plan_end": plan_end.isoformat(timespec="seconds"),
    }


def snapshot():
    """Return the public, aggregate-only progress document."""
    now = datetime.now(timezone.utc)
    stamps = _verified_timestamps()
    verified = len(stamps)
    blocked = blocked_count()
    organic, bulk = _split_bulk(stamps)
    ages = [max((now - t).total_seconds(), 0.0) for t in organic]  # clock skew: future = now
    last24 = sum(1 for a in ages if a < 86400)
    last1h = sum(1 for a in ages if a < 3600)
    remaining = max(TOTAL - verified - blocked, 0)
    rates = _rates(now, organic)
    rate = rates.get(RATE_WINDOW_H) or rates.get(24, 0.0)  # idle for 3 h: fall back to the day
    eta_days = remaining / (rate * 24) if rate > 0 else None
    bounds = [r for h, r in rates.items() if h != 24 and r > 0] + ([rate] if rate > 0 else [])
    eta_range = [round(remaining / (max(bounds) * 24), 1), round(remaining / (min(bounds) * 24), 1)] if bounds else None
    if remaining == 0 and stamps:
        finish = max(stamps)
    elif eta_days is not None:
        finish = now + timedelta(days=eta_days)
    else:
        finish = None
    return {
        "updated": now.isoformat(timespec="seconds"),
        "total": TOTAL,
        "verified": verified,
        "blocked": blocked,
        "remaining": remaining,
        "percent": round(100.0 * verified / TOTAL, 2),
        "evidence_files": sum(1 for _ in VERIF.glob("0x*.json")),
        "port_files": sum(1 for _ in PORTS.glob("p_*.rs")),
        "bulk_loaded": bulk,  # counted in verified; excluded from pace, charts and ETA
        "last_hour": last1h,  # organic proofs only, as are last_day/hourly/minutely
        "last_day": last24,
        "pace_per_day": round(rate * 24, 1),
        "rate_window_h": RATE_WINDOW_H,
        "eta_days": round(eta_days, 1) if eta_days is not None else None,
        "eta_range": eta_range,  # [fastest, slowest] days from the 1 h / 3 h / 6 h paces
        "current_task": current_task(last1h, last24),
        "schedule": schedule(now, verified, finish),
        "latest": max(stamps).isoformat(timespec="seconds") if stamps else None,
        "hourly": _buckets(ages, 3600, 24),  # proofs per hour, last 24 h, oldest first
        "minutely": _buckets(ages, 60, 60),  # proofs per minute, last 60 min, oldest first
        # commit subjects are intentionally omitted: they contain addresses and
        # proposed names, which stay in the repo, not on a public endpoint.
    }
