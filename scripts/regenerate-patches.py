#!/usr/bin/env python3
"""Regenerate CE patches from git diff upstream/main."""
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
PATCHES = ROOT / "patches"
PATCHES.mkdir(exist_ok=True)

CATEGORIES = {
    "00-community-foundation.patch": [
        # Root
        "Cargo.toml", "Cargo.lock", "README.md", "README_CN.md",
        "COMMUNITY.md", "COMMUNITY_CN.md", "CONTRIBUTING.md", "SECURITY.md",
        "LICENSE", "THIRD-PARTY-NOTICES", "SOURCE_REV",
        "docs/CHANGELOG.md", "docs/CHANGELOG_CN.md",
        "docs/MAINTENANCE.md", "docs/MAINTENANCE_CN.md",
        "rust-toolchain.toml", "clippy.toml", "rustfmt.toml",
        # Telemetry
        "crates/codegen/xai-grok-telemetry/**",
        # Agent
        "crates/codegen/xai-grok-agent/src/config.rs",
        "crates/codegen/xai-grok-agent/src/builder.rs",
        "crates/codegen/xai-grok-agent/src/error.rs",
        "crates/codegen/xai-grok-agent/src/plugins/**",
        "crates/codegen/xai-grok-agent/src/prompt/context.rs",
        "crates/codegen/xai-grok-agent/src/prompt/user_message.rs",
        "crates/codegen/xai-grok-agent/Cargo.toml",
        # Pager bin
        "crates/codegen/xai-grok-pager-bin/src/main.rs",
        "crates/codegen/xai-grok-pager-bin/Cargo.toml",
        "crates/codegen/xai-grok-pager-bin/build.rs",
        # Shell
        "crates/codegen/xai-grok-shell/src/session/acp_session_impl/spawn.rs",
        "crates/codegen/xai-grok-shell/src/session/acp_session_impl/run_loop.rs",
        "crates/codegen/xai-grok-shell/src/agent/config.rs",
        "crates/codegen/xai-grok-shell/src/agent/config_model_override_parse.rs",
        "crates/codegen/xai-grok-shell/src/agent/app.rs",
        "crates/codegen/xai-grok-shell/src/agent/mvp_agent/**",
        "crates/codegen/xai-grok-shell/src/agent/relay.rs",
        "crates/codegen/xai-grok-shell/src/auth/**",
        "crates/codegen/xai-grok-shell/src/config/**",
        "crates/codegen/xai-grok-shell/src/extensions/**",
        "crates/codegen/xai-grok-shell/src/leader/**",
        "crates/codegen/xai-grok-shell/src/sampling/**",
        "crates/codegen/xai-grok-shell/src/session/**",
        "crates/codegen/xai-grok-shell/src/tools/**",
        "crates/codegen/xai-grok-shell/src/util/**",
        "crates/codegen/xai-grok-shell/src/lib.rs",
        "crates/codegen/xai-grok-shell/Cargo.toml",
        "crates/codegen/xai-grok-shell-base/**",
        "crates/codegen/xai-grok-shell-session-support/**",
        # Tools registry
        "crates/codegen/xai-grok-tools/src/registry/types.rs",
        "crates/codegen/xai-grok-tools/src/registry/proto_convert.rs",
        "crates/codegen/xai-grok-tools/Cargo.toml",
        # Shared / Env / Paths / Secrets
        "crates/codegen/xai-grok-shared/**",
        "crates/codegen/xai-grok-env/**",
        "crates/codegen/xai-grok-paths/**",
        "crates/codegen/xai-grok-secrets/**",
        # Models
        "crates/codegen/xai-grok-models/**",
        # Version
        "crates/codegen/xai-grok-version/**",
        # Announcements
        "crates/codegen/xai-grok-announcements/**",
        # Update
        "crates/codegen/xai-grok-update/**",
        # Config
        "crates/codegen/xai-grok-config/**",
        "crates/codegen/xai-grok-config-types/**",
        # MCP
        "crates/codegen/xai-grok-mcp/**",
        # HTTP
        "crates/codegen/xai-grok-http/**",
        # Hooks
        "crates/codegen/xai-grok-hooks/**",
        # Voice
        "crates/codegen/xai-grok-voice/**",
        # Workspace
        "crates/codegen/xai-grok-workspace/**",
        "crates/codegen/xai-grok-workspace-client/**",
        "crates/codegen/xai-grok-workspace-types/**",
        # Pager
        "crates/codegen/xai-grok-pager/Cargo.toml",
        "crates/codegen/xai-grok-pager/src/views/welcome/**",
        "crates/codegen/xai-grok-pager/src/views/**",
        "crates/codegen/xai-grok-pager/src/settings/**",
        "crates/codegen/xai-grok-pager/src/slash/**",
        "crates/codegen/xai-grok-pager/src/app/**",
        "crates/codegen/xai-grok-pager/src/acp/**",
        "crates/codegen/xai-grok-pager/src/headless.rs",
        "crates/codegen/xai-grok-pager/src/tracing.rs",
        "crates/codegen/xai-grok-pager-render/**",
        "crates/codegen/xai-grok-pager-minimal/**",
        # Markdown / Memory
        "crates/codegen/xai-grok-markdown/**",
        "crates/codegen/xai-grok-markdown-core/**",
        "crates/codegen/xai-grok-memory/**",
        # Plugin marketplace
        "crates/codegen/xai-grok-plugin-marketplace/**",
        # Sampling
        "crates/codegen/xai-grok-sampler/**",
        "crates/codegen/xai-grok-sampling-types/**",
        # Sandbox
        "crates/codegen/xai-grok-sandbox/**",
        # Chat state
        "crates/codegen/xai-chat-state/**",
        # File utils
        "crates/codegen/xai-file-utils/**",
        "crates/codegen/xai-fast-worktree/**",
        "crates/codegen/xai-gix-status/**",
        "crates/codegen/xai-fsnotify/**",
        # Hunk tracker
        "crates/codegen/xai-hunk-tracker/**",
        # TTY
        "crates/codegen/xai-tty-utils/**",
        # System power
        "crates/codegen/xai-system-power/**",
        # Subagent resolution
        "crates/codegen/xai-grok-subagent-resolution/**",
        # Token estimation
        "crates/codegen/xai-token-estimation/**",
        # Agent lifecycle
        "crates/codegen/xai-agent-lifecycle/**",
        # Codebase graph
        "crates/codegen/xai-codebase-graph/**",
        # Ratatui
        "crates/codegen/xai-ratatui-inline/**",
        "crates/codegen/xai-ratatui-textarea/**",
        # Crash handler
        "crates/codegen/xai-crash-handler/**",
        # Mermaid
        "crates/codegen/xai-grok-mermaid/**",
        # Compaction
        "crates/common/xai-grok-compaction/**",
        # Tracing
        "crates/codegen/xai-tracing-macros/**",
        "crates/common/xai-tracing/**",
        # Tool protocol / runtime / types
        "crates/common/xai-tool-protocol/**",
        "crates/common/xai-tool-runtime/**",
        "crates/common/xai-tool-types/**",
        # Test support
        "crates/codegen/xai-grok-test-support/**",
        "crates/common/xai-test-utils/**",
        # Circuit breaker
        "crates/common/xai-circuit-breaker/**",
        # Computer hub
        "crates/common/xai-computer-hub-core/**",
        "crates/common/xai-computer-hub-mcp-adapter/**",
        "crates/common/xai-computer-hub-sdk/**",
        # Interjection
        "crates/common/xai-interjection-core/**",
        # Workflow
        "crates/codegen/xai-workflow/**",
        # Prompt queue
        "crates/codegen/xai-prompt-queue/**",
        # SQLite journal
        "crates/codegen/xai-sqlite-journal/**",
        # PTY
        "crates/codegen/xai-grok-pager-pty-harness/**",
        "crates/codegen/ptyctl/**",
        "crates/codegen/ptyctl-cli/**",
        # ACP lib
        "crates/codegen/xai-acp-lib/**",
        # Proto build
        "crates/build/xai-proto-build/**",
        # Tools API
        "crates/codegen/xai-grok-tools-api/**",
        # Prod
        "prod/**",
        # .github issue templates
        ".github/ISSUE_TEMPLATE/**",
        ".github/pull_request_template.md",
        # Third party
        "third_party/**",
        # NPM
        "npm/**",
    ],
    "01-community-ci.patch": [
        ".github/workflows/**",
        "build.bat",
        "install.ps1",
        "install.sh",
    ],
    "02-community-prompt.patch": [
        "crates/codegen/xai-grok-agent/templates/**",
    ],
    "03-community-tools.patch": [
        "ce-tools/**",
        "crates/codegen/xai-grok-tools/src/implementations/web_search/**",
        "crates/codegen/xai-grok-tools/src/implementations/web_fetch/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/calculator/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/codec/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/csv_ops/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/glob/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/json_query/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/text/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/web_search/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/mod.rs",
        "crates/codegen/xai-grok-tools/src/implementations/mod.rs",
        "crates/codegen/xai-grok-tools/src/tool_taxonomy.rs",
        "crates/codegen/xai-grok-tools/src/types/resources.rs",
        "crates/codegen/xai-grok-tools/src/types/tool_metadata.rs",
        "crates/codegen/xai-grok-tools/src/types/output.rs",
        "crates/codegen/xai-grok-tools/src/types/tool_io.rs",
        "crates/codegen/xai-grok-tools/src/types/template_renderer.rs",
        "crates/codegen/xai-grok-tools/src/notification/**",
        "crates/codegen/xai-grok-tools/src/persistence.rs",
        "crates/codegen/xai-grok-tools/src/normalization.rs",
        "crates/codegen/xai-grok-tools/src/util/**",
        "crates/codegen/xai-grok-tools/src/implementations/lsp/**",
        "crates/codegen/xai-grok-tools/src/implementations/opencode/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build_hashline/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build_concise/**",
        "crates/codegen/xai-grok-tools/src/implementations/memory/**",
        "crates/codegen/xai-grok-tools/src/implementations/search_tool/**",
        "crates/codegen/xai-grok-tools/src/implementations/task_output/**",
        "crates/codegen/xai-grok-tools/src/implementations/use_tool/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/image_gen/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/video_gen/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/workflow/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/bash/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/read_file/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/task/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/scheduler/**",
        "crates/codegen/xai-grok-tools/src/computer/**",
        "crates/codegen/xai-grok-tools/tests/**",
        "crates/codegen/xai-grok-tools/src/implementations/grok_build/task_output/**",
    ],
    "04-community-i18n.patch": [
        "crates/codegen/xai-grok-i18n/**",
    ],
}


def run(cmd):
    r = subprocess.run(cmd, capture_output=True, text=True, cwd=ROOT)
    if r.returncode != 0:
        print(f"  Error: {' '.join(cmd)}")
        print(f"  {r.stderr}")
        return ""
    return r.stdout


def main():
    # Get all changed files vs upstream
    changed = run(["git", "diff", "upstream/main", "--name-only"]).strip().split("\n")
    changed = [f for f in changed if f]

    untracked = run(["git", "ls-files", "--others", "--exclude-standard"]).strip().split("\n")
    all_files = set(changed + [f for f in untracked if f])

    assigned = set()
    for patch_name, patterns in CATEGORIES.items():
        matched = set()
        for pat in patterns:
            for f in run(["git", "ls-files", "--", pat]).strip().split("\n"):
                f = f.strip()
                if f and f in all_files and f not in assigned:
                    matched.add(f)
        assigned |= matched

        if matched:
            diff = run(["git", "diff", "upstream/main", "--"] + sorted(matched))
            if diff.strip():
                (PATCHES / patch_name).write_text(diff, encoding="utf-8", newline="\n")
                print(f"  {patch_name}: {len(matched)} files, {len(diff)} bytes")
            else:
                print(f"  {patch_name}: 0 files (empty)")
        else:
            # Write empty patch header
            (PATCHES / patch_name).write_text(
                f"# {patch_name} — no changes in this category.\n", encoding="utf-8"
            )
            print(f"  {patch_name}: 0 files (empty)")

    # Remaining files go to foundation
    remaining = all_files - assigned
    if remaining:
        diff = run(["git", "diff", "upstream/main", "--"] + sorted(remaining))
        if diff.strip():
            existing = PATCHES / "00-community-foundation.patch"
            existing.write_text(existing.read_text(encoding="utf-8") + diff, encoding="utf-8", newline="\n")
            print(f"  + {len(remaining)} unassigned files appended to 00-community-foundation.patch")

    # SOURCE_REV
    rev = run(["git", "rev-parse", "upstream/main"]).strip()
    if rev:
        (ROOT / "SOURCE_REV").write_text(rev + "\n", encoding="utf-8", newline="\n")
        print(f"\n  SOURCE_REV: {rev[:12]}")

    print("\nDone.")


if __name__ == "__main__":
    main()
