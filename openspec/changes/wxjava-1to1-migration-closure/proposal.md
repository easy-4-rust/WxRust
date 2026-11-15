# WxJava 1:1 Migration Closure

## Why

WxRust 已完成大规模 Java 对象与方法清点，但现有分母遗漏了企业微信会话存档 Native SDK 的包外类型 `com.tencent.wework.Finance`，方法清单也缺少完整 JVM 签名、返回类型、修饰符和 Rust 对应关系。对象状态汇总还存在算术不闭合的问题。需要建立可重放、可审计的迁移闭环，避免把对象映射、代码存在和行为兼容混为一谈。

## What Changes

- 以 WxJava 4.8.4.B 固定提交作为原始迁移 Oracle，枚举九个核心 Maven 模块的全部 `src/main/java/**/*.java` 源文件。
- 将包外特殊源文件、Native 方法和 package-info 纳入明确分母或排除账本。
- 生成完整 Java 方法清单，包含 JVM descriptor、返回类型、修饰符、参数、throws、重载身份和声明类。
- 生成方法级兼容矩阵，自动匹配只产生候选，不自动判定行为 VERIFIED。
- 将 Finance Native SDK 的未支持能力显式标记为 UNSUPPORTED，将纯 Rust 替代能力标记为 ADAPTED/NEEDS_REVIEW。
- 建立对象、方法、状态汇总的机器校验和可复现报告。
- 为后续 Java/Rust Golden 差分测试建立公共框架，优先覆盖 common/pay/cp。
- 单独维护 WxJava 4.8.6.B 上游差异，不混入 4.8.4.B 原始迁移分母。

## Non-Goals

- 不重写已有 WxRust 业务实现。
- 不因自动候选匹配而批量改写 Service 或 Bean。
- 不将 PLATFORM_NA、DEPENDENCY_REUSED、UNSUPPORTED 计入行为兼容完成数。
- 不在本 Change 内实现企业微信官方 Native 私有协议。
- 不推送 GitHub、不发布 crates.io。

## Acceptance

- 九个核心模块的 Java 主源码文件逐文件进入对象清单，数量与固定 Oracle 文件树一致。
- Finance.java 被纳入对象清单，Native 能力的部分实现/不支持边界明确。
- Java 方法清单具有唯一稳定 method_id，覆盖 public/protected 声明方法及 Native 方法。
- 方法兼容矩阵行数与方法清单完全一致，所有状态统计闭合。
- 自动候选不产生 VERIFIED 状态。
- 脚本可在固定 Oracle Commit 上重复运行并生成相同结果。
- 关键命令、环境、失败原因和证据写入验证报告。
