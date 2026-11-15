# Phase 1 WP-02：Java 方法清单与兼容矩阵

日期：2026-11-09

## Oracle 与构建证据

- WxJava Oracle：4.8.4.B
- Commit：`a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`
- 源码工作树：`%TEMP%\\wxjava-4.8.4.B-a49d6e1`，独立 detached worktree。
- 该临时 worktree 后续已被磁盘清理删除；2026-11-11 从同一 WxJava 仓库重建 `D:\\wxjava-phase1-oracle-20261111` detached worktree（同 SHA），用于重新核验九模块源码文件分母。此前方法清单来自该固定版本成功编译后的 `target/classes`，CSV 已持久化；重建的 D 盘 worktree 本轮未重新编译 Maven。
- Maven 命令：`mvn -pl weixin-java-common,weixin-java-mp,weixin-java-miniapp,weixin-java-pay,weixin-java-cp,weixin-java-open,weixin-java-channel,weixin-java-aispeech,weixin-java-qidian -am -DskipTests -Dcheckstyle.skip=true -Dspotless.check.skip=true package`
- 结果：BUILD SUCCESS；九个核心模块均成功编译。Surefire 测试按命令明确跳过，不能把这次构建描述为 Java 测试通过。

## 方法分母定义

清单来自固定 Oracle 编译后的九个模块 `target/classes`，使用 `javap -protected -s`：

- 纳入 public/protected 类或接口中声明的 public/protected 方法。
- 使用 JVM descriptor 区分重载，`method_id = SHA256(module, declaring class, method name, descriptor)`。
- 包含 Native 方法。
- 不把继承得到的方法重复计入声明方法分母；继承关系另由 class hierarchy 记录。
- 构造器不计入方法分母，单独写入 `inventory_java_constructors.csv`。
- 排除匿名/局部编译产物（`$数字`）及 package-info/module-info 编译产物。
- `class_header` 保存声明类的 extends/implements 行；`params`、`return_type` 保存 javap 展示的泛型类型文本；`generic_signature` 字段保存方法类型参数前缀。当前不是完整 JVM Signature 属性 dump，若后续需要字节码级泛型属性，应另加 `javap -v` 解析，不得把该字段夸大为完整 Signature attribute。

## 清点结果

| 模块 | 声明类数 | public/protected 方法 |
|---|---:|---:|
| common | 217 | 1,310 |
| mp | 549 | 5,752 |
| miniapp | 1,131 | 10,859 |
| pay | 1,034 | 14,156 |
| cp | 1,230 | 13,392 |
| open | 402 | 4,259 |
| channel | 693 | 6,992 |
| aispeech | 29 | 363 |
| qidian | 32 | 357 |
| **合计** | **5,317** | **57,440** |

- 独立构造器：4,856 条。
- Native 方法：17 条（全部模块）。
- Finance 类方法：21 条，其中 16 UNSUPPORTED、1 ADAPTED、4 PLATFORM_NA。

## 方法兼容矩阵

输出：`docs/migration/METHOD_COMPATIBILITY_MATRIX.csv`

| 自动候选/处置状态 | 数量 | 含义 |
|---|---:|---|
| CANDIDATE_MAPPED | 5,480 | 在候选 Rust 文件中找到同名 snake_case 方法；仅候选 |
| CANDIDATE_FILE_ONLY | 48,714 | 仅定位到文件候选，不能证明方法实现 |
| NEEDS_REVIEW | 3,225 | 无可靠映射或存在歧义 |
| ADAPTED | 1 | Finance.DecryptData 纯 Rust 替代，待差分验证 |
| UNSUPPORTED | 16 | Finance Native SDK 私有协议/句柄/拉取/媒体能力未支持 |
| PLATFORM_NA | 4 | Finance DLL/SO 加载及 Native 单例生命周期等平台机制 |
| **合计** | **57,440** | **自动分类闭合；VERIFIED=0** |

矩阵保留 Java 完整声明信息、JVM descriptor、声明类、类继承头、候选 Rust crate/type/file/method、迁移状态、语义状态、测试引用、差异说明和 Oracle commit。

自动匹配只生成候选状态，绝不生成 VERIFIED。当前矩阵不是人工逐方法验收结论。

## Finance Native 特殊项

`com.tencent.wework.Finance` 共 21 个 public/protected 方法：

- 16 个 Native SDK 初始化、句柄、聊天记录拉取、媒体拉取和 Slice/MediaData 操作标记 UNSUPPORTED。
- `DecryptData` 标记 ADAPTED_PENDING；Rust 纯 RSA/AES 解密实现尚需 Java/Rust 样本差分。
- 4 个 DLL/SO 加载与 Native 单例/平台机制标记 PLATFORM_NA。

不得将 Finance 对象或这些方法计入行为兼容完成数。

## 机器验收

执行：

```powershell
python scripts/inventory_java_objects_phase1.py --java-root <oracle> --csv docs/superpowers/data/inventory_java_objects.csv
python scripts/inventory_java_methods_phase1.py --java-root <oracle> --output docs/superpowers/data/inventory_java_methods_phase1.csv --constructors-output docs/superpowers/data/inventory_java_constructors.csv --jobs 8
python scripts/build_method_compatibility_matrix.py --project-root . --objects docs/superpowers/data/inventory_java_objects.csv --methods docs/superpowers/data/inventory_java_methods_phase1.csv --output docs/migration/METHOD_COMPATIBILITY_MATRIX.csv
python scripts/audit_phase1_migration.py --java-root <oracle> --objects docs/superpowers/data/inventory_java_objects.csv --methods docs/superpowers/data/inventory_java_methods_phase1.csv --matrix docs/migration/METHOD_COMPATIBILITY_MATRIX.csv --report-json docs/verification/phase1-migration-audit-2026-11-09.json
```

结果：`passed=true`、`errors=[]`、对象 3,288 行、方法 57,440 行、矩阵 57,440 行、唯一 method_id、Finance 21 行、状态汇总闭合、自动 VERIFIED=0。

## 验收边界

WP-02 的分母、方法清单、候选矩阵和自动算术审计通过。尚未完成逐方法人工映射、Java/Rust 双运行时行为差分和全模块语义验收。
