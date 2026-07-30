#!/usr/bin/env python3
"""Find files that differ from upstream but have no intentional community changes.
Pattern: files with large diffs where changes are deletions of upstream code."""
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# Known intentional community change patterns
COMMUNITY_PATTERNS = [
    "ce-tools/",
    "patches/",
    "docs/",
    "COMMUNITY",
    "install.ps1",
    "install.sh",
    "build.bat",
    ".github/",
    "scripts/",
    "sync.ps1",
    "SOURCE_REV",
    "/calculator/",
    "/codec/",
    "/csv_ops/",
    "/json_query/",
    "/glob/",
    "/text/",
    "/web_search/",
    "/lang.rs",
    "xai-grok-telemetry/Cargo.toml",
    "xai-grok-telemetry/src/client.rs",
    "xai-grok-telemetry/src/session_ctx.rs",
    "xai-grok-telemetry/src/compat.rs",
    "grok-build-community",
]

def is_community_file(f):
    for p in COMMUNITY_PATTERNS:
        if p in f:
            return True
    return False

def main():
    # Get all files differing from upstream with line counts
    result = subprocess.run(
        ["git", "diff", "upstream/main", "--numstat"],
        capture_output=True, text=True, cwd=ROOT
    )
    
    noise_candidates = []
    for line in result.stdout.strip().split('\n'):
        if not line.strip():
            continue
        parts = line.split('\t')
        if len(parts) != 3:
            continue
        added, deleted, fname = parts
        added = int(added) if added != '-' else 0
        deleted = int(deleted) if deleted != '-' else 0
        
        if is_community_file(fname):
            continue
        
        # Files with large diffs (>200 lines total) that aren't community files
        if added + deleted > 200:
            noise_candidates.append((added + deleted, fname))
    
    noise_candidates.sort(reverse=True)
    
    print("=== Large diffs in non-community files (likely noise) ===")
    for total, fname in noise_candidates:
        print(f"  {total:>6}  {fname}")

if __name__ == "__main__":
    main()
