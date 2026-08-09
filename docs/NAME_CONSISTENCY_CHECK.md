# WxRust 对象名称一致性检查（总览）

- Java 基线：WxJava `4.8.4.B`
- Rust 基线：`easy-4-rust/WxRust`
- 逐模块权威台账：`docs/migration/<module>/对象名称一致性检查.md`

## 检查口径

1. 从 Java 包根起保留末 `2` 层包目录，转 `snake_case` 为预期 Rust 路径。
2. 每个 Java 对象应唯一对应一个主要 Rust 文件；内部类仅在主对象独占时合并，且必须写明理由。
3. `MISPLACED` 仅允许在“语义保留 + 明确批准 + 证据记录”前提下出现。

## 检查维度

- 对象名 ↔ 文件名一致性
- 目录层级一致性（保留末 2 层）
- 方法签名命名策略（重载 → `_with_*` 后缀等）
- 文档注释来源映射（JavaDoc 语义保留）

## 当前结论

根据 `docs/migration/README.md`，9 模块的名称一致性检查已生成并处于完成口径（0 MISSING）。

> 若总览与模块检查表冲突，以模块检查表为准。
