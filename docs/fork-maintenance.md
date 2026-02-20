# Fork Maintenance — Brahmastra-Labs/zeroclaw

This document tracks which files in `Brahmastra-Labs/zeroclaw` intentionally diverge from the
upstream `zeroclaw-labs/zeroclaw` repository and why.  Use this as the conflict-resolution guide
during the weekly upstream rebase.

---

## Fork Identity

| Remote     | URL                                                       | Purpose                          |
|------------|-----------------------------------------------------------|----------------------------------|
| `origin`   | `git@github.com:Brahmastra-Labs/zeroclaw.git`             | Our fork — push here             |
| `upstream` | `https://github.com/zeroclaw-labs/zeroclaw.git`           | Original — sync from here only   |

Current base tag: `brahmastra/v0.1.0-base`

---

## Upstream Sync Workflow

Weekly GitHub Actions workflow (`.github/workflows/upstream-sync.yml`):

1. `git fetch upstream`
2. `git rebase upstream/main` onto a dated branch (`brahmastra/upstream-sync-YYYYMMDD`)
3. Opens a **draft PR** — never auto-merges
4. On conflict: workflow fails with a warning; manual resolution required

**Never push directly to `main`** in this fork.

---

## Diverging Files

These files are expected to conflict during upstream rebases.  Review each carefully.

### Candidate for upstream PR (general-purpose)

| File | Change | Status |
|------|--------|--------|
| `src/agent/router.rs` | Multi-agent routing (`AgentRouter`, `RoutingMode`) | New file — PR upstream when stable |
| `src/config/schema.rs` | `RoutingConfig`, `NamedAgentConfig`, `BindingConfig` additions | Partial upstream candidate |

### Fork-only (ProxApi-specific, will never go upstream)

| File | Change | Why fork-only |
|------|--------|---------------|
| `src/channels/imessage.rs` | `IMessageBackend::ProxApi` variant, `ProxApiIMessageChannel` | Calls ProxApi HTTP endpoints — not upstreamable |
| `src/channels/proxapi_imessage.rs` | New file — ProxApi iMessage HTTP client | ProxApi-specific |
| `src/config/schema.rs` | `IMessageBackend` enum, `proxapi_url`/`proxapi_token` fields | ProxApi-specific config |
| `src/mesh/mod.rs` | ZeroClaw peer-mesh discovery (cross-node via ProxApi mesh) | ProxApi infrastructure |
| `.github/workflows/upstream-sync.yml` | This workflow file | Fork infrastructure |
| `docs/fork-maintenance.md` | This document | Fork documentation |

---

## Rebase Conflict Resolution Guide

When `git rebase upstream/main` conflicts on the files above:

1. **`src/config/schema.rs`** — Our additions are at the end of the channel config section
   (search for `# brahmastra-fork` comments).  Keep our additions; accept upstream changes
   elsewhere.

2. **`src/channels/imessage.rs`** — We wrap the upstream type in an enum.  Accept upstream
   changes to the `AppleScript` variant; preserve our `ProxApi` variant and enum wrapper.

3. **`src/agent/router.rs`** — Entirely new file.  No upstream version exists; conflicts
   here indicate upstream added a similarly-named file.  Merge carefully.

4. **`src/mesh/mod.rs`** — Entirely new file.  Same as above.

---

## Adding New Fork-Specific Changes

When adding a ProxApi-specific feature to this fork:

1. Mark diverging config keys with a `# brahmastra-fork` comment.
2. Add the file to the **Fork-only** table above.
3. If the feature is general-purpose (not ProxApi-specific), add it to the
   **Candidate for upstream PR** table and open a PR upstream when it is stable.
