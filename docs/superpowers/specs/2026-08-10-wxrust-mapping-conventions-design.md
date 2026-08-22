# WxRust 对象/语义/命名映射规范设计

日期：2026-08-10
状态：已完成
来源：`docs/OBJECT_MAPPING_TABLE.md`、`docs/SEMANTIC_MAPPING_TABLE.md`、`docs/NAME_CONSISTENCY_CHECK.md`

## 1. 背景

WxJava 有 3287 个 main 对象和 36010 个 javap 公共方法需要迁移到 Rust。迁移过程中需要三套映射规范来保证一致性：对象级对照（每个 Java 对象对应哪个 Rust 文件）、语义迁移对照（每个 Java 机制如何在 Rust 中等价实现）、名称一致性检查（文件名/方法名/目录结构是否符合命名规则）。

## 2. 目标与非目标

### 2.1 目标

- 建立对象级对照规范：每个 Java 对象唯一对应一个 Rust 文件。
- 建立语义迁移规范：8 个维度覆盖全部 Java→Rust 映射。
- 建立名称一致性检查规范：4 个维度保证命名一致。
- 9 模块四件套（路线图/对象级对照表/语义迁移对照表/名称一致性检查）全部生成。

### 2.2 非目标

- 不涉及具体实现代码。
- 不修改已冻结的命名规则（以 ARCHITECTURE.md 为准）。

## 3. 方案比较

### 3.1 按 Java 包结构 1:1 映射目录

Java 包 `me.chanjar.weixin.mp.api.impl` 有 6 层，Rust 若 1:1 映射会导致目录过深。且 Rust 的模块系统（mod）与 Java 包语义不同。

本方案不采用。

### 3.2 保留末 2 层包目录，转 snake_case

`me.chanjar.weixin.mp.api.impl` → `api/impl/`。目录深度合理，与 Rust 惯例一致。内部类仅在主对象独占时合并，必须写明理由。

本方案作为最终方案。

## 4. 模块与依赖

### 4.1 对象级对照规范

| 维度 | 规则 |
|---|---|
| 映射粒度 | 每个 Java 对象唯一对应一个主要 Rust 文件 |
| 内部类 | 仅在主对象独占时合并，必须写明理由 |
| 状态口径 | `IMPLEMENTED / DEPENDENCY_REUSED / PLATFORM_NA` 计入已处置 |
| 阻断项 | `MISSING / MISPLACED / STUB / PARTIAL / UNVERIFIED` 为迁移阻断项 |
| 权威台账 | `docs/migration/<module>/对象级对照表.md` |

### 4.2 语义迁移规范（8 维度）

| 维度 | 覆盖内容 |
|---|---|
| 1. 核心调用链 | token / 执行引擎 / 重试 / 刷新 / 路由 |
| 2. 类型、所有权、可空与默认 | `Option<T>` / `Default` / 所有权转移 |
| 3. 方法重载与泛型适配 | `_with_*` 后缀 / 关联类型 |
| 4. 错误体系与回滚 | `Result<T, WxErrorException>` / thiserror |
| 5. 序列化与线格式兼容 | serde / quick-xml / 字段映射 |
| 6. 并发与生命周期 | `Arc<Mutex>` / `tokio::task` / RAII |
| 7. SPI/动态分派与宏替代 | trait object / enum dispatch / derive 宏 |
| 8. 明确不迁移项与 Rust 扩展 | `HttpClientType` 删除 / `GsonBuilder` 删除 |

### 4.3 名称一致性检查规范（4 维度）

| 维度 | 规则 |
|---|---|
| 对象名 ↔ 文件名 | PascalCase → snake_case |
| 目录层级 | 保留末 2 层包目录 |
| 方法签名命名策略 | getter 去 `get_` / 重载 `_with_*` / 布尔 `is_` |
| 文档注释来源 | JavaDoc 语义 100% 保留，禁止写"对应 Java" |

### 4.4 目录/文件映射示例

| Java 路径 | Rust 路径 |
|---|---|
| `me.chanjar.weixin.common.error.WxError` | `crates/wx-rust-common/src/error/wx_error.rs` |
| `me.chanjar.weixin.mp.api.impl.BaseWxMpServiceImpl` | `crates/wx-rust-mp/src/api/impl/base_wx_mp_service_impl.rs` |
| `cn.binarywang.wx.miniapp.bean.WxMaUserInfo` | `crates/wx-rust-miniapp/src/bean/wx_ma_user_info.rs` |

### 4.5 模块覆盖状态

| Java 模块 | Rust crate | 对象数 | 文档状态 |
|---|---|---|---|
| weixin-java-common | wx-rust-common | 174 | 四件套完成，0 MISSING |
| weixin-java-mp | wx-rust-mp | 428 | 四件套完成，0 MISSING |
| weixin-java-miniapp | wx-rust-miniapp | 611 | 四件套完成，0 MISSING |
| weixin-java-pay | wx-rust-pay | 570 | 四件套完成，0 MISSING |
| weixin-java-cp | wx-rust-cp | 594 | 四件套完成，0 MISSING |
| weixin-java-open | wx-rust-open | 240 | 四件套完成，0 MISSING |
| weixin-java-channel | wx-rust-channel | 618 | 四件套完成，0 MISSING |
| weixin-java-aispeech | wx-rust-aispeech | 25 | 四件套完成，0 MISSING |
| weixin-java-qidian | wx-rust-qidian | 27 | 四件套完成，0 MISSING |

## 5. 验收标准

- 9 模块四件套（路线图/对象级对照表/语义迁移对照表/名称一致性检查）全部生成。
- 所有模块记录为 0 MISSING。
- 命名规则与 ARCHITECTURE.md 一致。
- 目录映射示例覆盖 common/mp/miniapp 三个典型路径。
