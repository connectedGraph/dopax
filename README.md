<h1 align="center">

```
______ ___________  ___  __   __
|  _  \  _  | ___ \/ _ \ \ \ / /
| | | | | | | |_/ / /_\ \ \ V /
| | | | | | |  __/|  _  | /   \
| |/ /\ \_/ / |   | | | |/ /^\ \
|___/  \___/\_|   \_| |_/\/   \/

```

</h1>

<p align="center">
  <strong>A terminal agent tuned for how ADHD brains actually work.</strong><br>
  A fork of <a href="https://github.com/openai/codex">openai/codex</a> — rebranded, re-prompted, and re-wired so starting, continuing, and finishing work takes less friction.
</p>

<p align="center">
  <a href="#-why-dopax">Why</a> ·
  <a href="#-whats-different">What's different</a> ·
  <a href="#-install--build">Install</a> ·
  <a href="#-configuration">Config</a> ·
  <a href="#-upstream-sync">Upstream sync</a> ·
  <a href="#-license">License</a>
</p>

---

## 🧠 Why Dopax

General coding assistants assume a neurotypical operating mode: clean context, steady momentum, easy task initiation. Dopax assumes the opposite as a **silent, non-pathologizing default** — mild executive dysfunction, high startup friction, time blindness — and quietly builds the scaffolding in:

- **No labels, no check-ins.** It never says "ADHD" back at you or asks how your mood is. Care shows up as *structure*: smaller first steps, clearer options, lower activation energy.
- **Micro-chunking on demand.** When a task feels heavy, it gets shredded into two-line wins instead of delivered as one intimidating block.
- **5-minute rule baked in.** Proposing "just do the smallest slice for 5 minutes, permission to stop after" is a default move, not an intervention.
- **Time blindness compensation.** Artificial mini-deadlines, short focus sprints, and visible timelines instead of vague "later".
- **Stable, non-judging presence.** It doesn't get frustrated at abandoned threads or restarts. Come back, pick up, continue.

It's still a full coding agent underneath — the point is that the *relational layer* and the *tooling layer* both bend toward lower friction.

## ✨ What's different from upstream

Dopax tracks `openai/codex` closely (currently **0.151.0**) and layers a small, surgical delta on top:

| Area | Change |
|---|---|
| **System prompt** | A full ADHD-informed relational layer: invisible supportive default, task shredding, 5-minute rule, temptation bundling, time-blindness strategies. Ships in base / Claude / Codex prompt variants (`codex-rs/dopax_system_prompt*.md`). |
| **Experience manager** | New `dopax_experience_manager` tool: tracks ongoing projects, milestones, and personal-growth events with date ranges. Auto-injects `<current_time>` and `<active_experiences>` into context; expired/completed experiences purge on startup. (`core/src/experiences.rs`) |
| **Multi-select questions** | New `request_user_multi_select` tool so the agent can ask one structured question with several pickable answers instead of free-form back-and-forth. |
| **Own home directory** | `DOPAX_HOME` env var (falls back to `CODEX_HOME`), defaulting to `~/.dopax` — Dopax and upstream Codex can coexist on one machine. |
| **Custom providers** | `dopax login --api-key` offers an interactive choice: official OpenAI or any OpenAI-compatible Responses endpoint (relay/proxy/local), written to `config.toml` as the `dopax-custom` provider. |
| **Tolerant model listing** | `/models` parsing accepts the strict Codex backend shape, loose relay shapes, and the standard OpenAI `{"object":"list","data":[...]}` shape — so third-party relays drive the model picker too. |
| **Codex import** | One-click migration from an existing `~/.codex` install (settings, history, sessions, memories) into Dopax. *(Port to the new source-adapter architecture in progress.)* |
| **Branding** | TUI session header, `/app`, `/skills`, import flows — all Dopax. |

Everything else is upstream: sandboxing, MCP, plugins, hooks, agents dashboard, plan mode, memory.

## 📦 Install / Build

Prerequisites: Rust (stable, recent), Node not required.

```bash
git clone https://github.com/connectedGraph/dopax.git
cd dopax/codex-rs
cargo build --release -p codex-cli
```

The binary is `target/release/dopax` (aliased from the upstream `codex` CLI entrypoint — same commands, same flags).

```bash
# first run
dopax            # interactive TUI
dopax login      # ChatGPT, or `dopax login --api-key` for custom providers
```

> **Windows note:** build and tests are verified on Windows 11. For the test suite use `RUST_MIN_STACK=16777216 cargo test -p codex-tui --lib` — the default 1 MB test stack overflows on Windows (see upstream #… for the same behavior on vanilla codex).

## ⚙️ Configuration

Dopax reads `~/.dopax/config.toml` (same format as upstream Codex). Highlights:

```toml
# Point at any OpenAI-compatible endpoint
[model_providers.dopax-custom]
name = "My relay"
base_url = "https://my-relay.example.com/v1"
wire_api = "responses"

model_provider = "dopax-custom"
```

- `DOPAX_HOME` overrides the home directory; `CODEX_HOME` still works as a fallback.
- Existing Codex users: run `/import` inside the TUI to pull over settings, history, and sessions.
- The experience manager stores its data under `~/.dopax/experiences.json` and is gated by the `current_time_reminder` feature flag.

## 🔄 Upstream sync

Dopax is a **thin fork**: the entire product delta is a handful of files, so upstream releases are merged wholesale rather than cherry-picked.

- Current base: `openai/codex` **rust-v0.151.0** (2026-08-29)
- Sync cadence: every 1–2 upstream stable releases
- Merge history lives on the `dopax/merge-*` branches and in commit messages

## 🗺️ Roadmap

- [ ] Port the "import from Codex" migration to the new `external-agent-migration` source-adapter architecture
- [ ] Experience timeline visualization in the TUI
- [ ] Configurable prompt profile picker (supportive / neutral / focused)
- [ ] Publish signed release binaries

## 📄 License

Same as upstream: [Apache-2.0](LICENSE).

---

<p align="center">
<sub>
<code>dopax</code> — because the hardest commit is the first one.
</sub>
</p>
