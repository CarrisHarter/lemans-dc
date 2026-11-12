#!/usr/bin/env python3
"""Collect one agent workspace's ports and specs into the canonical repo.

Usage: scripts/collect-batch.py <workspace> [--dry-run]

Copies `rust/lemans-verify/src/ports/p_*.rs` and `notes/specs/*.json`. Refuses
to overwrite an existing file with different content: a silent overwrite would
hide two agents claiming the same address.
"""
import filecmp
import subprocess
import shutil
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PORTS = ROOT / "rust/lemans-verify/src/ports"
SPECS = ROOT / "notes/specs"


def copy(src: Path, dst: Path, dry: bool, conflicts: list[str]):
    if dst.exists():
        if filecmp.cmp(src, dst, shallow=False):
            return False
        conflicts.append(f"{dst} exists with different content (from {src})")
        return False
    if not dry:
        dst.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dst)
    return True


def main(argv):
    if len(argv) < 2:
        print(__doc__)
        return 2
    ws = Path(argv[1])
    dry = "--dry-run" in argv
    if not ws.is_dir():
        print(f"no workspace at {ws}", file=sys.stderr)
        return 2

    copied, conflicts = [], []
    for src in sorted((ws / "rust/lemans-verify/src/ports").glob("p_*.rs")):
        if copy(src, PORTS / src.name, dry, conflicts):
            copied.append(src.stem[2:])
    for src in sorted((ws / "notes/specs").glob("*.json")):
        copy(src, SPECS / src.name, dry, conflicts)

    if not dry:
        subprocess.run([sys.executable, str(ROOT / "scripts" / "normalize-specs.py"),
                        *[str(s) for s in sorted(SPECS.glob("*.json"))]], check=False)

    for c in conflicts:
        print(f"CONFLICT {c}", file=sys.stderr)
    print(f"collected {len(copied)} port(s) from {ws}: {' '.join(copied)}")
    return 1 if conflicts else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
