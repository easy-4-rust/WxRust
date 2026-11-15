# Design: WxJava 1:1 Migration Closure

## 基线

- 原始迁移 Oracle：WxJava 4.8.4.B / `a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`
- 当前 WxRust 实现：启动时核实的 Git HEAD，后续报告必须记录确切 SHA。
- 上游增量参考：WxJava develop / `b650e5d2b80496816adb76b05dcc64865bff4feb`；若本地无法取得该对象，记录阻塞，不得用其他 HEAD 替代。

## 数据分层

1. Source inventory：Java 源文件/顶层类型清单，包含源文件路径、包名、FQN、类型、模块和 Oracle SHA。
2. Method inventory：Java 声明方法清单，以 JVM descriptor 识别重载，包含声明类、修饰符、返回类型、参数、throws、泛型声明、super types。
3. Compatibility matrix：每条 Java 方法关联 Rust crate/type/file/method 候选、迁移状态、语义状态、测试证据和差异。
4. Evidence ledger：差分测试、构建、覆盖率、安全和真实环境证据。

## 状态语义

- MAPPED：建立映射关系。
- CANDIDATE_MAPPED：自动工具找到 Rust 候选，尚未人工确认。
- IMPLEMENTED：Rust 侧实现已确认存在。
- ADAPTED：实现机制不同，需要行为证据。
- VERIFIED：指定场景的 Java/Rust 可观察行为差分通过。
- DEPENDENCY_REUSED：由 Rust 依赖或共享基础设施承载，需逐项证据。
- PLATFORM_NA：Java 平台专属能力，有明确不迁移理由。
- UNSUPPORTED：接口/行为明确未支持。
- PARTIAL：一个 Java 类型中部分能力迁移、部分不支持。
- MISSING：没有实现也没有合理处置。
- NEEDS_REVIEW：自动化无法可靠判定。

只有经人工审查并附测试证据后才允许 VERIFIED。文件名/方法名匹配不得自动升级状态。

## Finance 特殊处理

`com.tencent.wework.Finance` 不在 CP 普通包根下，必须单独纳入源文件分母。Native 拉取、媒体、SDK 句柄和内存 Slice 管理能力不等同于普通 HTTP API。已知 Rust 会话存档服务对拉取/媒体方法返回 -99；纯 Rust RSA/AES 解密路径是 ADAPTED，不能宣称完整等价。

## 方法清单生成

优先在固定 Oracle worktree 中编译九个 Maven 模块，然后使用 `javap -protected -s` 读取编译后 class。descriptor 是重载身份的稳定基础；Java 源码泛型/参数/throws 文本作为辅助语义。构造器单独统计，不混入业务方法分母。类继承关系单独记录，避免把继承方法重复计入声明方法分母。

## 差分测试

Java Adapter 与 Rust Adapter 使用相同输入和 Mock HTTP/固定时钟/固定随机源。比较请求 URL、HTTP method、headers、JSON/XML、字段缺省/null/空串、响应、错误码、异常传播、重试和副作用。比较器必须按协议语义规范化，不得仅做字符串比较。

## 不变量

- 对象分母 = 九个核心模块 `src/main/java` 下全部 Java 源文件数。
- 方法分母 = 固定范围内所有 public/protected 声明方法记录数（含 Native，不含构造器；构造器另表）。
- 每条方法记录具有唯一 method_id。
- 方法矩阵行数 = 方法清单行数。
- 每个对象/方法恰好一个主迁移状态。
- 状态分项之和必须等于总数。
- 未经证据不得将候选状态标为 VERIFIED。
