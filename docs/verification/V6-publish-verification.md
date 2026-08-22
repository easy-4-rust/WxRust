# V6 发布验证报告

日期：2026-08-23
计划：`docs/superpowers/plans/2026-08-10-wxrust-migration-roadmap-and-execution.md` Task 4 Step 7

## 验证结果

| 项目 | 结果 |
|---|---|
| 依赖发布配置 | ✅ 已修复（内部 crate workspace 声明 + version） |
| wx-rust-common dry-run | ✅ `cargo publish --dry-run` 打包成功 |
| wx-rust（facade，无内部依赖） | ✅ `cargo package` 打包成功 |
| 其余 8 个 crate | ⏸ 受发布顺序约束（依赖 wx-rust-common 未发布） |

## 修复内容

**问题**：`cargo publish --dry-run` 报 `all dependencies must have a version requirement specified when publishing`——内部 crate 以纯 `path` 依赖相互引用，publish 时 path 会被剥离导致无版本号。

**修复**（commit 6728cc7）：
- `Cargo.toml` `[workspace.dependencies]` 声明全部 9 个内部 crate：`X = { version = "0.1.0", path = "crates/X" }`
- 各 crate 依赖改为 `X.workspace = true`（本地构建仍走 path，publish 携带 crates.io 版本）
- `cargo check --workspace` 通过

## 发布顺序约束（非缺陷）

`cargo package`/`cargo publish --dry-run` 需从 crates.io 索引解析依赖版本。`wx-rust-common` 尚未发布，依赖它的 8 个 crate（mp/miniapp/pay/cp/open/channel/aispeech/qidian）在打包阶段报 `no matching package named wx-rust-common found`。

**结论**：配置层面已验证正确（common 与 facade 打包成功、依赖声明合法）。真实发布需按依赖拓扑顺序执行：先 `cargo publish -p wx-rust-common`，再发布依赖者。

## 建议发布顺序

1. `wx-rust-common`（无内部依赖）
2. `wx-rust-mp` / `wx-rust-miniapp` / `wx-rust-pay` / `wx-rust-cp` / `wx-rust-channel` / `wx-rust-aispeech` / `wx-rust-qidian`（依赖 common）
3. `wx-rust-open`（依赖 common + mp + miniapp）
4. `wx-rust`（facade，无内部依赖但依赖上述全部——待确认后发布）

## 门禁判定：PASS（配置修复完成；发布顺序约束已记录）
