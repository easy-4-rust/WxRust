# WxRust Phase 1：迁移闭环基线

## 执行基线

- WxRust 初始 HEAD：`bc5dbb7ab7bfa7f4e236e9907411edde42e5f0bc`
- 工作分支：`feature/wxjava-1to1-phase1`
- WxJava Oracle：`a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`（4.8.4.B）
- WxJava 当前本地 develop 检查点：`58af116149e4d89a12828cf22f10fee77a63295d`；它不是指定的 4.8.6.B 增量 SHA，不得替代用户指定的上游对照基线。
- 原工作区在创建分支前为 clean。
- 本地 WxJava 原工作区未切换分支；Oracle 使用独立 detached worktree。

## 已确认分母问题

固定 Oracle 的九个核心模块 `src/main/java` 文件数：

| 模块 | Java 文件数 |
|---|---:|
| common | 174 |
| mp | 428 |
| miniapp | 611 |
| pay | 570 |
| cp | 595 |
| open | 240 |
| channel | 618 |
| aispeech | 25 |
| qidian | 27 |
| **合计** | **3,288** |

其中 CP 有 1 个文件位于常规包根之外：

`weixin-java-cp/src/main/java/com/tencent/wework/Finance.java`

因此现有 3,287 对象分母少计 1 个 Java 源文件。当前旧清单模块数为 CP 594，其余模块与文件数一致。

## 原方法清单现状

现有 `inventory_java_methods.csv` 有 36,010 行，字段仅为：

`module,class,method,params,throws`

缺少返回类型、修饰符、JVM descriptor、泛型声明、稳定重载 ID 和继承关系；旧生成器仅扫描固定包根下的编译 class，漏掉 Finance。

## 执行环境与阻塞

- Windows Runner 在线，项目已接入。
- Python 3.12.14、Maven 3.9.9、JDK 17 可用。
- WxJava 4.8.4.B detached worktree：系统临时目录 `%TEMP%\\wxjava-4.8.4.B-a49d6e1`。
- Oracle Maven 编译正在尝试；首次命令因 PowerShell 原生参数拆分失败，已修正后重跑。最终构建状态以 Job 验证记录为准。

## 当前判断

WP-01/WP-02 尚未验收。当前仅完成基线固定和差异确认；不得宣称对象分母、方法矩阵或行为差分已修复。
