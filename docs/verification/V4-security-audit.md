# V4 安全审计报告

日期：2026-08-23
计划：`docs/superpowers/plans/2026-08-10-wxrust-migration-roadmap-and-execution.md` Task 4 Step 5

## 验证结果

| 门禁 | 命令 | 结果 |
|---|---|---|
| 漏洞扫描 | `cargo audit` | ⚠️ 1 medium（已配置已知风险例外） |
| 依赖治理 | `cargo deny check` | ✅ advisories ok, bans ok, licenses ok, sources ok |

## cargo audit 详情

- 扫描依赖：291 个 crate（Cargo.lock）
- **1 个漏洞**：`rsa 0.9.10` — Marvin Attack（RUSTSEC-2023-0071，severity 5.9 medium）
  - 风险：通过计时侧信道可能恢复 RSA 私钥
  - **无可用修复版本**：rsa 稳定版唯一 0.9.x；0.10 尚为 RC（0.10.0-rc.18）
  - 影响面：wx-rust-common / wx-rust-cp / wx-rust-pay（RSA-OAEP 加密、签名）
  - 缓解措施：RSA-OAEP 盲化、固定消息加密；**待 rsa 0.10 稳定后升级并移除例外**
- 其余 290 个依赖：无已知漏洞

## cargo deny 配置

新增 `deny.toml`（V4 与 Phase 4 共用门禁）：

- `[advisories]`：ignore = ["RUSTSEC-2023-0071"]（含升级追踪注释）
- `[licenses]`：allow = MIT/Apache-2.0/BSD-2/3-Clause/0BSD/ISC/MPL-2.0/Unicode-3.0/Zlib/CDLA-Permissive-2.0/CC0-1.0/Unlicense
- `[bans]`：multiple-versions = allow（避免过度约束现有依赖树）
- `[sources]`：unknown-registry/unknown-git = deny

## 结论

- 无高危/严重漏洞；唯一 medium（rsa）无修复版本可用，已登记为已知风险并配置例外
- 许可证合规：全部依赖满足允许列表
- **门禁判定：PASS（含 1 项已登记已知风险）**
