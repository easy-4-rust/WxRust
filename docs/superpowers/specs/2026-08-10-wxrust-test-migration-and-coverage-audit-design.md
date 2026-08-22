# WxRust 测试迁移与覆盖审计规范设计

日期：2026-08-10
状态：进行中
来源：`docs/migration/weixin-java-*/迁移测试对照表.md`、`docs/migration/weixin-java-*/覆盖审计报告.md`

## 1. 背景

WxJava 有 354 个测试类（main 3288 文件中 test 379 文件），是理解 API 用法的捷径。WxRust 当前仅有 59 个测试文件，缺口 308 个。测试迁移需遵循三层规范（SOURCE_PARITY / RUST_OBLIGATION / VALUE_ADD），并以 cargo-llvm-cov 覆盖率作为补充信号。

## 2. 目标与非目标

### 2.1 目标

- 建立测试迁移三层规范：SOURCE_PARITY（镜像 Java 测试）+ RUST_OBLIGATION（Rust 特有关注点）+ VALUE_ADD（边界/并发/错误路径）。
- 定义覆盖审计规范：每个模块的测试对照表与覆盖审计报告。
- 补齐测试缺口（308 → 195 个 Rust 测试文件）。
- 覆盖率目标 >= 60% line。

### 2.2 非目标

- 不迁移 Java 中的 Demo/Infra 测试。
- 不追求 100% 行覆盖率。
- 覆盖率不作为完成权威（台账状态为权威）。

## 3. 方案比较

### 3.1 逐个 Java @Test 方法 1:1 翻译

每个 Java `@Test` 方法翻译为一个 Rust `#[test]`。优点是可追溯；缺点是 Java 测试的 setup/teardown 模式在 Rust 中不同（无 JUnit lifecycle），且部分 Java 测试依赖 Spring 上下文。

本方案不采用。

### 3.2 按 Java 测试类聚合，Rust 测试文件对齐

每个 Java 测试类对应一个 Rust 测试文件，文件内按功能分组。Java 的 `@BeforeEach` 用 Rust 的 setup 函数替代，Spring 上下文用 tokio::test 替代。

本方案作为最终方案。

## 4. 模块与依赖

### 4.1 测试编写三层规范

| 层次 | 要求 | 示例 |
|---|---|---|
| SOURCE_PARITY | 镜像对应 Java 测试的每个 `@Test` 方法 | `test_get_access_token` 对应 `WxMpServiceImplTest.testGetAccessToken` |
| RUST_OBLIGATION | 测试所有权、异步、错误、序列化、feature | `#[tokio::test]` / `Result<()>` 返回 / `serde_json::from_str` |
| VALUE_ADD | 边界、并发、错误路径 | 超时测试 / 并发锁竞争 / 错误码映射 |

### 4.2 中文来源注释规范

每个测试函数必须标注对应 Java 测试类和方法名：

```rust
/// 对应 Java: WxMpServiceImplTest.testGetAccessToken
/// 测试 access_token 获取的快速路径和强制刷新路径
#[tokio::test]
async fn test_get_access_token() -> Result<()> {
    // ...
}
```

### 4.3 分模块测试缺口

| 模块 | Java 测试类 | Rust 测试文件 | 缺口 | 需新增 |
|---|---|---|---|---|
| common | 29 | 7 | 22 | 8 |
| mp | 71 | 9 | 62 | 33 |
| miniapp | 68 | 9 | 59 | 32 |
| pay | 71 | 7 | 64 | 27 |
| cp | 88 | 8 | 80 | 33 |
| open | 13 | 5 | 8 | 2 |
| channel | 31 | 5 | 26 | 24 |
| aispeech | 2 | 3 | -1 | 0 |
| qidian | 6 | 6 | 0 | 0 |
| **合计** | **379** | **59** | **320** | **159** |

> 注：实际需新增 159 个测试文件（而非 308），因为部分 Java 测试是 Demo/Infra 不需要迁移，部分已通过综合测试覆盖。

### 4.4 四阶段执行计划

| Phase | 范围 | 测试文件数 | 预计工作量 |
|---|---|---|---|
| Phase 1 (P0) | 核心 Service 基础 CRUD + Token + 错误路径 | 42 | 9.5 天 |
| Phase 2 (P1) | 子域 Service + 重试 + 消息路由 | 86 | 15 天 |
| Phase 3 (P2) | 扩展 Service + 高级功能 | 67 | 18 天 |
| Phase 4 | CI/CD + 生产加固 | — | 4.5 天 |
| **合计** | | **195** | **47 天** |

### 4.5 覆盖审计规范

每个模块的覆盖审计报告（`docs/migration/<module>/覆盖审计报告.md`）包含：

1. 测试文件清单（Java ↔ Rust 对照）。
2. 每个 Java 测试的 Rust 覆盖状态（已覆盖 / 部分覆盖 / 缺失 / 不迁移）。
3. 缺失测试的优先级（P0/P1/P2/P3）。
4. 覆盖率信号（cargo-llvm-cov 输出）。

### 4.6 验收标准

- 所有 P0 测试文件编写完成并通过。
- 所有 P1 测试文件编写完成并通过。
- `cargo test --workspace` 全绿。
- `cargo-llvm-cov --workspace` 行覆盖率 >= 60%。
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` 零警告。
- `cargo audit` 无高危漏洞。
- 所有 crate `cargo publish --dry-run` 成功。

## 5. 验收标准

- 测试三层规范文档化。
- 9 模块测试缺口表完整。
- 四阶段执行计划有明确时间估算。
- 覆盖审计规范可执行。
