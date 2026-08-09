<a id="readme-top"></a>

<div align="center">

# WxRust

**A comprehensive Rust SDK for the WeChat ecosystem, ported from WxJava**

[![MSRV](https://img.shields.io/badge/MSRV-1.85-orange)](#3-rust-baseline)
[![License](https://img.shields.io/badge/license-Apache--2.0-green)](LICENSE)

[English](./README.md) | [简体中文](./README.zh-CN.md)

[Overview](#1-overview) · [Modules](#2-module-map) · [Architecture](#3-architecture) ·
[Quick Start](#5-quick-start) · [Features](#6-cargo-features) ·
[Quality](#8-quality-gates) · [Compatibility](#9-compatibility-with-wxjava) ·
[Contributing](#12-contributing--license)

</div>

---

> **Version**: `0.1.0`<br>
> **MSRV**: Rust `1.85`<br>
> **Edition**: `2024`<br>
> **Workspace Resolver**: `3`<br>
> **Maturity**: Experimental<br>
> **Last verified**: 2026-08-10

## 1. Overview

### 1.1 What is WxRust?

**WxRust is a Rust workspace providing a full-stack SDK for WeChat backend development, covering Official Accounts (MP), Mini Programs, WeChat Pay, WeChat Work (CP), Open Platform, Channels/Shop, and AI Speech.**

It is a semantic port of [WxJava](https://github.com/binarywang/WxJava) (v4.8.4.B), aiming for 100% functional behavior parity while adopting idiomatic Rust patterns.

| Dimension | Value |
|---|---|
| Version | `0.1.0` |
| MSRV / Edition | `1.85` / `2024` |
| unsafe policy | `#![forbid(unsafe_code)]` |
| Async runtime | tokio |
| HTTP client | reqwest (rustls) |
| License | `Apache-2.0` |

### 1.2 What WxRust is NOT

- Not a binary-compatible drop-in for WxJava; API names are preserved but signatures use Rust idioms (`Result`, `Option`, `async fn`).
- Not a thin FFI wrapper; it is a pure Rust reimplementation.
- Not production-ready yet; the project is in experimental/migration phase.

### 1.3 Status Evidence

| Claim | Value | Evidence |
|---|---|---|
| Workspace compiles | Pass | `cargo check --workspace` |
| Tests | All green, 0 failures | `cargo test --workspace` |
| Coverage (line) | 39.81% | `cargo llvm-cov --workspace` |
| Java objects mapped | 3287 / 3287 (0 MISSING) | `docs/migration/README.md` |
| Rust source files | 3406 | `find crates -name '*.rs'` |

## 2. Module Map

| Java Module | Rust Crate | Objects | Scope |
|---|---|---:|---|
| `weixin-java-common` | `wx-rust-common` | 174 | Error model, token engine, HTTP, session, dedup |
| `weixin-java-mp` | `wx-rust-mp` | 428 | Official Accounts API, message router, XML crypto |
| `weixin-java-miniapp` | `wx-rust-miniapp` | 611 | Mini Program code2Session, 50+ sub-services |
| `weixin-java-pay` | `wx-rust-pay` | 570 | WeChat Pay v2/v3, certs, signatures, XML |
| `weixin-java-cp` | `wx-rust-cp` | 594 | WeChat Work, session archive, OA |
| `weixin-java-open` | `wx-rust-open` | 240 | Third-party platform, proxy mp/ma |
| `weixin-java-channel` | `wx-rust-channel` | 618 | Channels / WeChat Shop e-commerce |
| `weixin-java-aispeech` | `wx-rust-aispeech` | 25 | AI Speech |
| `weixin-java-qidian` | `wx-rust-qidian` | 27 | Qidian call center |
| facade | `wx-rust` | — | Feature-gated re-exports |

## 3. Architecture

```text
┌─────────────────────────────────────────────────────────┐
│  Integration (future)  wx-rust-vernal / wx-rust-axum    │
├─────────────────────────────────────────────────────────┤
│  Business Layer  wx-rust-mp / miniapp / pay / cp /       │
│                  open / channel / aispeech / qidian      │
│                  Service traits + sub-domain services    │
├─────────────────────────────────────────────────────────┤
│  Foundation      wx-rust-common                          │
│                  error / config / http / bean / session  │
├─────────────────────────────────────────────────────────┤
│  Facade          wx-rust (feature-gated re-exports)      │
└─────────────────────────────────────────────────────────┘
```

### 3.1 Dependency Rules

- `wx-rust-common` has zero business-crate dependencies.
- Business crates depend only on `common`; no inter-business dependencies.
- Facade does re-exports only; no logic.

### 3.2 Key Java-to-Rust Mappings

| Java Mechanism | Rust Design | Reason |
|---|---|---|
| Inheritance chain (`ServiceImpl -> HttpComponentsImpl -> Base`) | trait + composition | No inheritance in Rust |
| `synchronized` / `ReentrantLock` | `tokio::sync::Mutex` / `RwLock` | Async-native |
| `Gson` | `serde` + `serde_json` | Compile-time zero-cost |
| `Apache HttpClient` / `OkHttp` / `Jodd` | `reqwest` (single backend) | Simplify to one HTTP stack |
| Checked exception | `thiserror` + `Result` | Explicit error propagation |
| `null` | `Option<T>` | Nullability visible in types |
| `ExecutorService` (Router) | `tokio::task::spawn` | Async task scheduling |

## 4. Rust Baseline

| Item | Value | Source |
|---|---|---|
| MSRV | `1.85` | `workspace.package.rust-version` |
| Edition | `2024` | `workspace.package.edition` |
| Resolver | `3` | `[workspace] resolver` |
| Clippy | `-D warnings` | CI |

## 5. Quick Start

### 5.1 Add dependency

```toml
[dependencies]
wx-rust-common = { git = "https://github.com/easy-4-rust/WxRust", branch = "main" }
```

### 5.2 Minimal example

```rust
use wx_rust_common::error::WxError;

fn main() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let err = WxError::from_json(json);
    println!("error_code={}", err.error_code);
}
```

### 5.3 Enable a business module

```toml
[dependencies]
wx-rust-mp = { git = "https://github.com/easy-4-rust/WxRust", branch = "main" }
```

## 6. Cargo Features

| Feature | Default | Description | Crate |
|---|:---:|---|---|
| `default` | Yes | Core types and traits | `wx-rust-common` |
| `redis` | No | Redis-backed storage (dedup, locks, config) | `wx-rust-common` |

```toml
[dependencies]
wx-rust-common = { version = "0.1.0", default-features = false, features = ["redis"] }
```

## 7. Project Structure

```text
WxRust/
├── Cargo.toml              # Workspace
├── crates/
│   ├── wx-rust/            # Facade (feature-gated)
│   ├── wx-rust-common/     # Foundation
│   ├── wx-rust-mp/         # Official Accounts
│   ├── wx-rust-miniapp/    # Mini Programs
│   ├── wx-rust-pay/        # WeChat Pay
│   ├── wx-rust-cp/         # WeChat Work
│   ├── wx-rust-open/       # Open Platform
│   ├── wx-rust-channel/    # Channels / Shop
│   ├── wx-rust-aispeech/   # AI Speech
│   └── wx-rust-qidian/     # Qidian
├── docs/                   # Architecture, migration, tech stack
├── scripts/                # Audit and generation scripts
├── integration/            # Framework integrations (future)
└── README.md
```

## 8. Quality Gates

### 8.1 Core gates

```bash
cargo check --workspace
cargo test --workspace
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### 8.2 Coverage

```bash
cargo llvm-cov --workspace --summary-only
```

| Metric | Value |
|---|---|
| Line coverage | 39.81% |
| Function coverage | 25.67% |
| Region coverage | 38.00% |

> Coverage is a signal for gap detection, not a completion authority. Completion authority is the per-module object ledger + semantic table + name consistency check.

## 9. Compatibility with WxJava

### 9.1 Upstream reference

| Item | Value |
|---|---|
| Upstream project | [WxJava](https://github.com/binarywang/WxJava) |
| Pinned version | `4.8.4.B` (commit `a49d6e1`) |
| Authority source | WxJava source, tests, fixtures |
| Rust target | Behavioral parity + idiomatic Rust |
| Non-targets | ABI compatibility, JVM/bytecode/platform-specific classes |

### 9.2 Object migration status

| Java Module | Objects | Implemented | PLATFORM_NA | DEPENDENCY_REUSED | MISSING |
|---|---:|---:|---:|---:|---:|
| common | 174 | 90 | 87 | 1 | 0 |
| mp | 428 | 284 | 44 | 39 | 0 |
| miniapp | 611 | 560 | 35 | 16 | 0 |
| pay | 570 | 556 | 12 | 2 | 0 |
| cp | 594 | 565 | 20 | 9 | 0 |
| open | 240 | 213 | 14 | 13 | 0 |
| channel | 618 | 603 | 9 | 6 | 0 |
| aispeech | 25 | 23 | 2 | 0 | 0 |
| qidian | 27 | 20 | 7 | 0 | 0 |
| **Total** | **3287** | **2914** | **230** | **86** | **0** |

### 9.3 Key semantic mappings

| Java Pattern | Rust Implementation |
|---|---|
| AccessToken double-check lock | `async fn` + `tokio::sync::Mutex` + 3s timeout |
| Execute engine with retry | `loop` + exponential backoff (`1s, 2s, 4s, 8s, 16s`) |
| Token auto-refresh (single retry) | `execute_internal` with `no_auto_refresh` flag |
| Message router (builder pattern) | `WxMpMessageRouter` + `Vec<Rule>` + async dispatch |
| ConfigStorage multi-backend | `trait WxConfigStorage` + memory/Redis implementations |

### 9.4 Explicit non-migration items

| Java Component | Reason | Rust Alternative |
|---|---|---|
| Apache/OkHttp/Jodd HTTP backends | Java HTTP client adapters | `reqwest` unified |
| Gson TypeAdapter | Gson-specific | `serde` derive |
| Native-image config | GraalVM-specific | Rust native binary |
| Solon plugins | JVM framework | No Rust equivalent |

## 10. Documentation

| Document | Path | Purpose |
|---|---|---|
| Migration Roadmap | `docs/MIGRATION_ROADMAP.md` | B0-V6 phase plan |
| Object Mapping | `docs/OBJECT_MAPPING_TABLE.md` | 3287 object index |
| Semantic Mapping | `docs/SEMANTIC_MAPPING_TABLE.md` | Core call chain / error / serialization |
| Name Consistency | `docs/NAME_CONSISTENCY_CHECK.md` | Java-to-Rust naming |
| Tech Stack | `docs/TECH_STACK_SELECTION.md` | Dependency selection |
| Architecture | `docs/ARCHITECTURE.md` | Component replacement decisions |
| Module Migrations | `docs/migration/<module>/` | Per-module 4-document sets |

## 11. Troubleshooting

| Symptom | Common Cause | Fix |
|---|---|---|
| `cargo check` fails | MSRV too low | Use Rust >= 1.85 |
| Feature compile error | Missing optional dep | Enable required feature |
| Test timeout | Async runtime conflict | Ensure `tokio` features match |

## 12. Contributing & License

Contributions welcome. Before submitting, run:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

This project is licensed under [Apache-2.0](LICENSE).

Upstream: [WxJava](https://github.com/binarywang/WxJava) (Apache-2.0).

---

<div align="center">

[Back to top](#readme-top) · [Issues](https://github.com/easy-4-rust/WxRust/issues)

</div>
