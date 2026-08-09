# WxRust 语义迁移对照表（总览）

- Java 基线：WxJava `4.8.4.B`
- Rust 基线：`easy-4-rust/WxRust`
- 说明：本文为语义迁移总览索引；逐模块权威语义台账在：
- `docs/migration/<module>/语义迁移对照表.md`

## 语义覆盖范围

每张模块级语义表覆盖：
1. 核心调用链（token/执行引擎/重试/刷新/路由）
2. 类型、所有权、可空与默认语义
3. 方法重载与泛型适配
4. 错误体系与回滚语义
5. 序列化与线格式兼容
6. 并发与生命周期
7. SPI/动态分派与宏替代
8. 明确不迁移项与 Rust 扩展项

## 当前结论（节选，来源 docs/migration/README.md 与模块语义表）

- 公共基础层（common）与主要业务模块（mp/miniapp/pay/cp/open/channel/aispeech/qidian）均已有语义迁移文档。
- 组件替换决策（HTTP/JSON/XML/错误/并发）已在 ARCHITECTURE.md 冻结。
- 语义结论以模块语义表为权威；本总览仅做索引与口径统一。

> 如果总览与模块语义表冲突，以模块语义表为准。
