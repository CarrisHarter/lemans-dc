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
import os
import re
import subprocess
import time
from datetime import datetime, timedelta, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOTAL = 5211  # functions exported (analysis/export/pseudocode/functions/)
# Schedule: every unblocked function verified by the end of Fri 13 Nov JST, as a
# straight line from the first published snapshot (1,373 verified). The target
# count is TOTAL - blocked at snapshot time, the same "finished" the ETA uses.
PLAN_START = datetime(2026, 11, 12, 2, 11, 4, tzinfo=timezone.utc)
PLAN_START_VERIFIED = 1373
PLAN_END = datetime(2026, 11, 13, 15, 0, 0, tzinfo=timezone.utc)  # Sat 14 Nov 00:00 JST
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
_first_added = {"head": None, "map": {}, "checked": -60.0}


def _first_committed():
    """Evidence path (relative to ROOT) -> time it was first committed.

    Re-verification rewrites `verified_at` (a full re-run once moved 1,194
    proofs into three minutes), so the first commit bounds when a function was
    really first proven. HEAD is checked at most every 10 s; the log is re-read
    only when it moves.
    """
    if time.monotonic() - _first_added["checked"] < 10:
        return _first_added["map"]
    _first_added["checked"] = time.monotonic()
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
    """First-proof timestamps of canonical (non-branch-seed) passing evidence.

    A canonical evidence file is exactly `0x` + 8 hex digits. Branch seeds use
    `<addr>.<tag>.json`, so any other suffix — dot or hyphen — must not be
    counted as its own function (a hyphenated tag once inflated the count by 3).
    Each function is dated by the earliest of `first_verified_at` (kept across
    re-verification by verify-function.sh), `verified_at`, and its first commit.
    """
    stamps = []
    seen = set()
    committed = _first_committed()
    try:
        entries = list(os.scandir(VERIF))
    except OSError:
        return stamps
    for e in entries:
        name = e.name
        if not (name.endswith(".json") and CANONICAL_RE.match(name[:-5])):
            continue
        try:
            st = e.stat()
        except OSError:
            continue
        key = (st.st_mtime_ns, st.st_size)
        seen.add(name)
        hit = _evidence_cache.get(name)
        if hit and hit[0] == key:
            t = hit[1]
        else:
            try:
                with open(e.path, encoding="utf-8") as fh:
                    doc = json.load(fh)
            except (json.JSONDecodeError, OSError, UnicodeDecodeError):
                continue  # possibly mid-write; retry next scan
            t = None
            if doc.get("result") == "pass":
                t = _parse_ts(doc.get("first_verified_at") or doc.get("verified_at", ""))
            _evidence_cache[name] = (key, t)
        if t:
            c = committed.get(f"notes/verifications/{name}")
            stamps.append(min(t, c) if c else t)
    for gone in _evidence_cache.keys() - seen:
        del _evidence_cache[gone]
    return stamps


def _count(directory, prefix, suffix):
    try:
        return sum(1 for n in os.listdir(directory) if n.startswith(prefix) and n.endswith(suffix))
    except OSError:
        return 0


def _count_blocked(text):
    if "## Blocked" not in text:
        return 0
    section = text.split("## Blocked", 1)[1].split("## ", 1)[0]
    return len(re.findall(r"^\| `0x[0-9a-f]{8}` \|", section, re.M))


def blocked_count():
    try:
        return _count_blocked(LEDGER.read_text())
    except OSError:
        return 0


_blocked_at_rev = {}  # commit -> blocked count in its ledger
_blocked_then = {"at": 0.0, "value": None}


def blocked_then(hours):
    """Blocked count in the last commit at least `hours` old, or None if the
    repo has no commit that old. Re-resolved at most once a minute."""
    if time.monotonic() - _blocked_then["at"] < 60:
        return _blocked_then["value"]
    rev = sh("git", "rev-list", "-1", f"--before={hours} hours ago", "HEAD")
    if rev and rev not in _blocked_at_rev:
        _blocked_at_rev[rev] = _count_blocked(sh("git", "show", f"{rev}:notes/address-map.md"))
    _blocked_then.update(at=time.monotonic(), value=_blocked_at_rev.get(rev) if rev else None)
    return _blocked_then["value"]


def _history(stamps, now, points):
    """Cumulative verified count sampled at `points` even steps from the first
    proof to now: {"start", "step_s", "verified"}."""
    if not stamps:
        return None
    ordered = sorted(stamps)
    start = ordered[0]
    step = max((now - start).total_seconds() / (points - 1), 1.0)
    out, i = [], 0
    for k in range(points):
        edge = start + timedelta(seconds=step * k)
        while i < len(ordered) and ordered[i] <= edge:
            i += 1
        out.append(i)
    out[-1] = len(ordered)  # stamps a little in the future (clock skew) still count
    return {"start": start.isoformat(timespec="seconds"), "step_s": round(step, 1), "verified": out}


def _buckets(ages, width, count):
    """Histogram of ages (seconds) into `count` buckets of `width`, oldest first."""
    out = [0] * count
    for a in ages:
        i = int(a // width)
        if 0 <= i < count:
            out[count - 1 - i] += 1
    return out


def _rates(ages):
    """Proofs/hour over the last 1, RATE_WINDOW_H, 6 and 24 h, from proof ages
    in seconds.

    A window reaching back before the first proof is shortened to it, so a loop
    that started 2 h ago isn't diluted by idle time before it existed.
    """
    if not ages:
        return {}
    since = max(max(ages) / 3600, 1 / 6)
    out = {}
    for h in (1, RATE_WINDOW_H, 6, 24):
        n = sum(1 for a in ages if a < h * 3600)
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


def schedule(now, verified, target, finish):
    """Position against the plan: functions ahead (+) / behind (-) of the
    straight-line plan now, and hours the projected finish is early (+) / late (-)."""
    frac = min(max((now - PLAN_START) / (PLAN_END - PLAN_START), 0.0), 1.0)
    planned = round(PLAN_START_VERIFIED + (target - PLAN_START_VERIFIED) * frac)
    return {
        "planned": planned,
        "ahead": verified - planned,
        "early_hours": round((PLAN_END - finish).total_seconds() / 3600, 1) if finish else None,
        "plan_start": PLAN_START.isoformat(timespec="seconds"),
        "plan_start_verified": PLAN_START_VERIFIED,
        "plan_target": target,
        "plan_end": PLAN_END.isoformat(timespec="seconds"),
    }


def snapshot():
    """Return the public, aggregate-only progress document."""
    now = datetime.now(timezone.utc)
    stamps = _verified_timestamps()
    verified = len(stamps)
    blocked = blocked_count()
    ages = [max((now - t).total_seconds(), 0.0) for t in stamps]  # clock skew: future = now
    last24 = sum(1 for a in ages if a < 86400)
    last1h = sum(1 for a in ages if a < 3600)
    prev1h = sum(1 for a in ages if 3600 <= a < 7200)
    remaining = max(TOTAL - verified - blocked, 0)
    rates = _rates(ages)
    blocked_before = blocked_then(BLOCKED_TREND_H)
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
        "evidence_files": _count(VERIF, "0x", ".json"),
        "port_files": _count(PORTS, "p_", ".rs"),
        "blocked_change": {"hours": BLOCKED_TREND_H, "delta": blocked - blocked_before}
        if blocked_before is not None else None,
        "last_hour": last1h,
        "prev_hour": prev1h,  # proofs 1-2 h ago, for the hour-on-hour trend
        "last_day": last24,
        "pace_per_day": round(rate * 24, 1),
        "rate_window_h": RATE_WINDOW_H,
        "eta_days": round(eta_days, 1) if eta_days is not None else None,
        "eta_range": eta_range,  # [fastest, slowest] days from the 1 h / 3 h / 6 h paces
        "current_task": current_task(last1h, last24),
        "schedule": schedule(now, verified, TOTAL - blocked, finish),
        "latest": max(stamps).isoformat(timespec="seconds") if stamps else None,
        "hourly": _buckets(ages, 3600, 24),  # proofs per hour, last 24 h, oldest first
        "minutely": _buckets(ages, 60, 60),  # proofs per minute, last 60 min, oldest first
        "history": _history(stamps, now, HISTORY_POINTS),  # cumulative verified since the first proof
        # commit subjects are intentionally omitted: they contain addresses and
        # proposed names, which stay in the repo, not on a public endpoint.
    }
