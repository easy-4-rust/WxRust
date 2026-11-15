# Phase 1 WP-01：WxJava 对象分母修复与验收

日期：2026-11-09

## 固定 Oracle

- WxJava 版本：4.8.4.B
- Commit：`a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`
- Oracle 工作树：Windows 临时目录 `%TEMP%\\wxjava-4.8.4.B-a49d6e1`，detached worktree；未切换原 WxJava 工作区分支。
- 2026-11-11 复核时，原临时 worktree 已随磁盘清理删除；已从同一 WxJava 仓库重新创建 `D:\\wxjava-phase1-oracle-20261111` detached worktree，HEAD 仍为上述 SHA，并用它重新执行源文件数量审计。
- WxRust 初始 Commit：`bc5dbb7ab7bfa7f4e236e9907411edde42e5f0bc`
- WxRust 工作分支：`feature/wxjava-1to1-phase1`

## 修复内容

旧对象清单仅扫描九个 Maven 模块各自的常规 package root，导致以下源文件漏出分母：

`weixin-java-cp/src/main/java/com/tencent/wework/Finance.java`

该文件属于 CP Maven 模块的 `src/main/java`，但 package 为 `com.tencent.wework`，不在 `me.chanjar.weixin.cp` 包根下。

新增 `scripts/inventory_java_objects_phase1.py`，以九个核心模块的整个 `src/main/java` 目录递归枚举 Java 文件，不再以 package root 作为发现边界。清单增加：

旧清单中的两条 MiniApp `package-info.java` 记录曾以 `?` 占位；本轮将其显式分类为 `package-info`，并保留包名、FQN、来源范围和 Oracle SHA。旧清单与新清单的净行数差异仍只有 Finance.java 一行。

- `java_package` / `java_fqn`
- `source_scope`（PACKAGE_ROOT / OUTSIDE_PACKAGE_ROOT）
- `rust_crate`
- `oracle_commit`
- 类型声明计数

对象清单仍以 Java 源文件为统计单位；它不是 Java 顶层/嵌套类型的总数。

## 新分母

| 模块 | Java 主源码文件 |
|---|---:|
| weixin-java-common | 174 |
| weixin-java-mp | 428 |
| weixin-java-miniapp | 611 |
| weixin-java-pay | 570 |
| weixin-java-cp | 595 |
| weixin-java-open | 240 |
| weixin-java-channel | 618 |
| weixin-java-aispeech | 25 |
| weixin-java-qidian | 27 |
| **合计** | **3,288** |

与旧清单 3,287 行相比，新增 Finance.java 一行。CP 从 594 修正为 595，其余八个模块不变。

## 当前静态对象处置结果

执行：

```powershell
python scripts/inventory_java_objects_phase1.py --java-root <WxJava-4.8.4.B-worktree> --csv docs/superpowers/data/inventory_java_objects.csv
python scripts/audit_migration_layout.py --csv docs/superpowers/data/inventory_java_objects.csv
```

结果：

| 状态 | 数量 | 解释 |
|---|---:|---|
| IMPLEMENTED | 2,966 | 旧静态对象审计规则定位到 Rust 文件/显式映射 |
| PARTIAL | 1 | Finance Native SDK：纯 Rust 解密替代存在，但 Native 句柄/拉取/媒体能力未实现 |
| PLATFORM_NA | 231 | Java 平台/容器/HTTP 后端等机制按现有处置规则排除 |
| DEPENDENCY_REUSED | 90 | 由 Rust serde/quick-xml/Common 等机制承载 |
| MISSING | 0 | 当前登记的 3,288 个源文件均有显式处置 |
| **合计** | **3,288** | **100% 已分类处置，不代表 100% 行为兼容** |

CP 模块的静态处置为 565 IMPLEMENTED + 1 PARTIAL + 20 PLATFORM_NA + 9 DEPENDENCY_REUSED = 595。

## Finance.java 的实际边界

- Java Finance Native SDK 包含 DLL/SO 加载、SDK 句柄、聊天记录拉取、媒体拉取、Slice/MediaData 管理及 DecryptData。
- WxRust 的 CP 消息审计实现对 Native SDK 私有协议拉取/媒体接口明确返回 -99。
- Rust 对 RSA + AES 消息解密有纯 Rust 替代实现。
- 因此对象级状态是 PARTIAL；方法级必须进一步拆分为 UNSUPPORTED、ADAPTED 和 PLATFORM_NA。不得把 Finance 整体标为 IMPLEMENTED 或 VERIFIED。

## 验收边界

本报告只证明：
1. 九个核心 Maven 模块的全部 Java 主源码文件已纳入分母。
2. 对象清单模块数量与固定 Oracle 文件树一致。
3. 静态对象审计状态算术闭合，MISSING=0。
4. Finance.java 的部分迁移边界已显式记录。

本报告不证明：
- Java/Rust 方法签名一一映射；
- 任何方法达到行为级 VERIFIED；
- WxRust 已达到生产就绪。

WP-01 分母修复通过。WP-02 已在 `phase1-method-matrix-2026-11-09.md` 完成方法清单与候选矩阵交付，但行为级差分仍未完成。
