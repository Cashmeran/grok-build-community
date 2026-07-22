# CE Patches

Single combined patch for Grok Build Community Edition.
Apply to a clean upstream/main checkout.

## Files

| Patch | Description |
|-------|-------------|
| `00-community-full.patch` | All community changes (generated via `git diff upstream/main`) |
| `01`-`18` (legacy) | Original granular patches (kept for reference, may not apply cleanly) |

## Upstream Update Workflow

```bash
git pull upstream main
# Regenerate the patch from the merged result:
git diff upstream/main > patches/00-community-full.patch
# Fix any conflicts, commit, then:
./build.bat
```

## Fresh Build

```bash
git clone https://github.com/xai-org/grok-build.git
cd grok-build
git apply patches/00-community-full.patch
cargo build -p xai-grok-pager-bin --release
```
