# Geyser Streaming Plugin — Build Plan

**Goal:** an accounts-only Solana Geyser plugin that streams account updates to
subscribers over gRPC (a small "Yellowstone"). Built to prove systems-Rust skill:
a non-blocking validator hot path, per-client backpressure, and real throughput numbers.

**Timeline:** ~5–6 weeks full-time. Week 6 is polish + buffer.

---

## Mental model: TWO separate repos (fixes the "where do I start" confusion)

- **`../ag` (your Agave clone)** — NOT where your plugin lives. Use it to (a) read the
  validator's code, (b) make OSS contributions / PRs. Keep it on `master`.
- **This repo (`geyser-stream-plugin`)** — YOUR project. A standalone Rust crate that
  depends on the published `agave-geyser-plugin-interface` library. This is the repo that
  goes on your GitHub as your portfolio piece.

You do NOT fork or edit Agave to build a plugin. You implement one trait from one
published library, compile it to a `.dylib`/`.so`, and the validator loads it.

---

## Setup — start here (Week 1, day 1)

**1. Install the Agave/Solana CLI** (gives you `solana-test-validator`):
```
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"
# docs: https://docs.anza.xyz/cli/install
solana-test-validator --version    # <-- note this version number; call it X.Y.Z
```

**2. Initialize the crate** (in this folder):
```
cd ~/Desktop/OpenSource/geyser-stream-plugin
cargo init --lib
```

**3. `Cargo.toml`** — make it a dynamic library, pin the interface crate to YOUR validator's version:
```toml
[package]
name = "geyser-stream-plugin"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
agave-geyser-plugin-interface = "=X.Y.Z"   # MUST equal `solana-test-validator --version`
bs58 = "0.5"
```

**4. `src/lib.rs`** — the minimal plugin (just prints account updates for now):
```rust
use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, ReplicaAccountInfoVersions, Result as PluginResult,
};

#[derive(Debug, Default)]
pub struct StreamPlugin;

impl GeyserPlugin for StreamPlugin {
    fn name(&self) -> &'static str {
        "geyser-stream-plugin"
    }

    fn on_load(&mut self, _config_file: &str, _is_reload: bool) -> PluginResult<()> {
        eprintln!("[geyser-stream-plugin] loaded");
        Ok(())
    }

    // Account callbacks are already ON by default (the trait's default impl
    // returns true), so this override is optional — it just states the intent.
    // Return false here if you only want transactions (speeds up startup).
    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn update_account(
        &self,
        account: ReplicaAccountInfoVersions,
        slot: u64,
        is_startup: bool,
    ) -> PluginResult<()> {
        if is_startup {
            return Ok(()); // skip the snapshot flood for now (handled in Week 5)
        }
        // If the compiler complains here, your pinned version has fewer/more
        // variants — adjust the arms to match.
        let pubkey = match account {
            ReplicaAccountInfoVersions::V0_0_3(a) => a.pubkey,
            ReplicaAccountInfoVersions::V0_0_2(a) => a.pubkey,
            ReplicaAccountInfoVersions::V0_0_1(a) => a.pubkey,
        };
        eprintln!(
            "[geyser-stream-plugin] account {} @ slot {}",
            bs58::encode(pubkey).into_string(),
            slot
        );
        Ok(())
    }
}

/// The validator loads the .dylib/.so and calls this to build the plugin.
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin: Box<dyn GeyserPlugin> = Box::new(StreamPlugin::default());
    Box::into_raw(plugin)
}
```

**5. Build:**
```
cargo build --release
# macOS -> target/release/libgeyser_stream_plugin.dylib
# Linux -> target/release/libgeyser_stream_plugin.so
```

**6. `config.json`** (absolute path to the built library):
```json
{ "libpath": "/Users/gurkiratsingh/Desktop/OpenSource/geyser-stream-plugin/target/release/libgeyser_stream_plugin.dylib" }
```

**7. Run the validator with your plugin, and generate activity:**
```
solana-test-validator --geyser-plugin-config config.json
# in another terminal:
solana airdrop 1        # creates account writes you'll see printed
```

If you see `account <pubkey> @ slot <n>` lines, the pipeline works end-to-end. **Week 1 done.**

---

## #1 gotcha: version / ABI match
The validator loads your plugin as a raw `dyn GeyserPlugin` pointer. If your plugin and the
validator were built against **different** `agave-geyser-plugin-interface` versions, the memory
layout can mismatch and the validator crashes on load. Rule: pin with `=` to exactly what
`solana-test-validator --version` prints, and rebuild the plugin whenever you change validator version.

Note: your `../ag` checkout is **master**, which has a newer method split (`update_account_for_bank`)
and marks `update_account` deprecated. Do NOT build your plugin against master — pin to a **released**
version, where `update_account(&self, account, slot, is_startup)` is the method every example uses.
Keep master only for reading code and OSS.

## macOS note
- Your library is `.dylib`, not `.so` — point `libpath` at the `.dylib`.
- `solana-test-validator` runs fine on macOS for development (Weeks 1–4).
- For **Week 5 load testing / real numbers**, use a cheap **Linux** cloud VM. Plugins run on Linux
  in production, and heavy load testing is far smoother there.

---

## Weekly milestones

**Week 1 — Load + print (setup above).** Skill: FFI / the `.dylib` boundary, `cdylib`, `_create_plugin`.
Done when: live accounts print from a running validator.

**Week 2 — Non-blocking hot path.** Put a bounded channel between the callback and the work.
`update_account` copies out an owned message, `try_send`s it, returns immediately; a background
thread drains it. Full channel -> drop + increment an atomic counter.
Skill: sync/async boundary, ownership across the handoff, backpressure (first taste).
Done when: validator unaffected under load, and the drop counter climbs when you slow the consumer.

**Week 3 — gRPC subscription service (single client).** Define a proto (account msg + server-streaming
`Subscribe`). Stand up a `tonic` server on its own tokio runtime in `on_load`; the worker forwards into it.
Write a tiny Rust client that subscribes and prints.
Skill: tonic/async, server-streaming gRPC, `prost`.
Done when: a separate client binary prints live accounts over gRPC.

**Week 4 — Many clients + filters + per-client backpressure (THE hard week — protect it).**
One producer -> N isolated per-subscriber queues. Filters in the request (by owner/program, by pubkey).
A slow client's full queue -> drop-for-that-client or disconnect; never block the shared producer.
Skill: concurrent fan-out, `Arc`/shared state, per-consumer isolation.
Done when: 3 clients (two fast, one with an artificial sleep) — the slow one only hurts itself.

**Week 5 — Startup flood + metrics + real numbers.** Handle the `is_startup` snapshot flood so it
doesn't blow your queues. Expose metrics (throughput, queue depth, per-client lag, drops). Benchmark
on a busy validator (Linux): sustained accounts/sec, p99 handoff latency, memory, drop rate at N clients.
Skill: perf measurement, profiling/flamegraphs.
Done when: you have a TABLE OF REAL NUMBERS (the résumé line).

**Week 6 — Polish + writeup + buffer.** README with an architecture diagram, the backpressure design,
the numbers, and a "v2 / next steps" section (fork handling, reconnect/replay). Tests for core logic,
a run script. Turn your dev-log into a short writeup on the hard part. OSS ramps up here.

---

## Cut list (if you fall behind — never cut backpressure)
1. Metrics dashboard polish — but KEEP the raw numbers.
2. Multiple filter types — ship filter-by-owner only.
3. Prometheus endpoint — structured log lines are enough to pull numbers from.

**Keep no matter what:** non-blocking hot path (W2), per-client backpressure (W4), real numbers (W5).
Those three ARE the project.

---

## Resources
- Interface crate docs: https://docs.rs/agave-geyser-plugin-interface
- The trait itself (in your clone): `../ag/geyser-plugin-interface/src/geyser_plugin_interface.rs`
- The hot-path loop that calls you: `../ag/geyser-plugin-manager/src/accounts_update_notifier.rs`
- Agave docs: https://docs.anza.xyz
- tonic (Week 3 gRPC): https://github.com/hyperium/tonic  (see its `examples/` for streaming)

## Reference plugins to study (read them — don't copy)
- **Yellowstone gRPC — github.com/rpcpool/yellowstone-grpc** (Triton One). THE blueprint: it is
  exactly what you're building (gRPC account streaming with filters + backpressure). Study its
  hot-path handoff and streaming design most. NOTE: it's AGPL-3.0 — learn from it, write your own.
- **Postgres plugin — github.com/solana-labs/solana-accountsdb-plugin-postgres.** The classic,
  simplest full trait implementation + config handling. Older, but clear.
- **solana-geyser-plugin-scaffold.** A minimal plugin that just logs every callback — good for
  seeing the bare skeleton. Older; may need version tweaks.
- **ckamm/solana-accountsdb-connector.** Another gRPC-style connector example.
