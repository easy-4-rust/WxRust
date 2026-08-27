# V2 行为镜像率复测报告（2026-08-27）

> 结论一句话：**当前总镜像率 46.1%（175/380），较上次 40.5%（138/341，2026-08-23、WxJava v4.8.4 基数）提升约 5.6 个百分点**；其中 WxJava 4.8.5/4.8.6 新增的 39 个测试类中仅 4 个已镜像。纯分析任务，本报告未改动任何代码或测试。
>
> **复测 #2（同日晚，g3/g4 深度补测合入后）：总镜像率升至 46.8%（178/380）**，miniapp 单模块 59.7%→64.2%。详见 §7。

---

## 1. 测量方法与口径

### 1.1 口径定义

| 项目 | 定义 |
| --- | --- |
| Java 测试类总体 | WxJava 仓库 HEAD（tag `v4.8.6`，commit `25423da46`）下 9 个目标模块 `*/src/test/**/*Test.java`，排除文件名含 `Abstract` 的抽象基类 |
| Rust 镜像 | `crates/wx-rust-*/tests/*.rs`（130 个集成测试文件；另有 1 个辅助模块 `wx-rust-qidian/tests/common/mod.rs` 不计入）头部/行内注释中**显式声明**对应的 Java 测试类 |
| 镜像判定（主口径，宽） | 注释中出现该 Java 测试类名（裸类名匹配），前文 30 字符内无否定词（无/没有/不存在/缺失/缺少/尚未/暂无）；且引用文件与 Java 类**同 crate 模块**。覆盖三种现存标注形态：①`// 对应 Java: XxxTest`；②`// 镜像 Java \`XxxTest\` / \…`（含多行列表、`SOURCE_PARITY 测试：对应 WxJava … 测试 util/crypto/SHA1Test…` 路径列表）；③golden 向量来源引用（如 `Java \`WxPayRefundNotifyResultTest\` 的 req_info 明文 golden`）。因此为**声明式宽口径**：含部分镜像与向量级引用 |
| 镜像判定（对照口径，严） | 仅承认 `\对应?[Jj]ava[:：\\\s]{0,7}XxxTest` 形态的紧邻标注（模拟上次测量所用的抽取方式），用于趋势对照 |
| 跨模块同名消歧 | 存在 3 组跨模块同名类：`XmlUtilsTest`（common/channel）、`WxMpBusyRetryTest`（mp/qidian）、`WxMpJsAPITest`（mp/qidian）。同名类只按**同模块**证据判定（qidian 两例两侧均有独立镜像成立；channel 的 `XmlUtilsTest` 仅被 wx-rust-common 引用，判未镜像） |

### 1.2 基数校准

在历史 tag 上逐点统计同类 Java 测试类总数：

```
v4.8.2: 326   v4.8.3: 330   v4.8.4: 341   v4.8.5: 353   HEAD(v4.8.6): 380
```

**v4.8.4 = 341，与上次测量基数完全一致**——即上次测量的分母就是 v4.8.4。故本轮"4.8.5/4.8.6 新增测试类"精确等于 `git diff --diff-filter=A v4.8.4 HEAD` 中新增的 39 个测试文件（380 − 341 = 39，两法互验一致）。

### 1.3 可复算命令

```bash
# Java 侧清单（380）
cd /Users/wandl/workspaces/workspace-github/WxJava && \
find weixin-java-{common,mp,miniapp,pay,cp,open,channel,aispeech,qidian} \
  -path "*/src/test/*" -name "*Test.java" | grep -v Abstract | sort > /tmp/java_tests.txt

# Rust 侧集成测试清单（131，含 1 个辅助 mod.rs；扣除后 130）
find crates -path "*/tests/*.rs" -type f | sort > /tmp/rust_tests.txt

# 抽取 Rust 文件声明的 Java 测试类（见 §1.1 宽口径规则）
# —— 本轮以脚本 /tmp/mirror_extract.py + /tmp/mirror_matrix.py 执行，
#    核心规则：注释文本中 [A-Z]\w*Test 令牌，负向前瞻过滤否定词，
#    按同名 crate-module 匹配 Java 模块。

# 4.8.5/4.8.6 新增测试类（39）
cd /Users/wandl/workspaces/workspace-github/WxJava && \
git diff --name-status --diff-filter=A v4.8.4 HEAD | grep "/src/test/" \
  | grep "Test.java$" | grep -v Abstract > /tmp/java_new_4856.txt

# 各 tag 基数复核示例
git ls-tree -r --name-only v4.8.4 | grep "/src/test/" | grep "Test.java$" \
  | grep -v Abstract | grep -E "^weixin-java-(common|mp|miniapp|pay|cp|open|channel|aispeech|qidian)/" | wc -l
```

---

## 2. 分模块矩阵（主口径：宽）

| 模块 | Java 类数 | Rust 文件数 | 已镜像 | 未镜像 | 镜像率 |
| --- | ---: | ---: | ---: | ---: | ---: |
| common | 29 | 16 | 15 | 14 | **51.7%** |
| mp | 59 | 16 | 35 | 24 | **59.3%** |
| miniapp | 67 | 20 | 40 | 27 | **59.7%** |
| pay | 74 | 22 | 29 | 45 | **39.2%** |
| cp | 83 | 21 | 22 | 61 | **26.5%** |
| open | 14 | 9 | 3 | 11 | **21.4%** |
| channel | 48 | 16 | 25 | 23 | **52.1%** |
| aispeech | 2 | 4 | 2 | 0 | **100%** |
| qidian | 4 | 6 | 4 | 0 | **100%** |
| **合计** | **380** | **130** | **175** | **205** | **46.1%** |

结构观察：
- mp / miniapp 为第一梯队（≈60%），其子域系列文件（`sub_domain_*` / `phase*_batch*`）是批量镜像主力。
- cp 是最大缺口（83 类仅镜像 22，占全部未镜像的 30%）：家校（School）、微文档（OaWeDoc）、第三方 TP 系、XML 消息等大块整域缺位。
- pay 未镜像数最多之一（45），但多为 bean/result 断言类；ServiceImpl 级缺口集中在沙箱、多商户切换、转账授权兼容层。
- open 与 channel 新增量（4.8.5/4.8.6 大增 channel）拉低了上期优势。
- aispeech、qidian 已全量镜像（aispeech 的 dialog service 属"Java 无对应测试、Rust 自建补充"情形，不计入分母变更）。

## 3. 总镜像率与上次对比

| 时点 | Java 基数 | 已镜像 | 镜像率 | 对应 WxJava 版本 |
| --- | ---: | ---: | ---: | --- |
| 2026-08-23（上次 V2） | 341 | 138 | 40.5% | v4.8.4 |
| **2026-08-27（本次，宽口径）** | **380** | **175** | **46.1%** | v4.8.6 HEAD |
| 本次（对照严格口径，仅 `对应 Java:` 紧邻标注） | 380 | 152 | 40.0% | v4.8.6 HEAD |

- **宽口径 46.1%，环比 +5.6pp**；新增镜像主要来自 4–8 月批量产出的 `sub_domain_*`、`phase1/phase2_batch*`、`coverage_boost_*`、`semantic_*` 系列 MockServer 测试。
- 注意口径漂移：近月新文件的标注格式已从单一 `对应 Java:` 扩散到 `镜像 Java`、SOURCE_PARITY 路径列表、golden 引用等多种写法，严格口径会漏计这 23 个（175−152），呈现 40.0%。据此判断**真实进展 = 46.1%；若沿用上次的窄抽取器则读数被低估**。建议后续统一采用宽口径并保留两种口径双报。
- 另一视角：v4.8.4 原 341 类中现有 171 个镜像（171−138 相当于老类补齐 + 口径放宽混合效应；因 Rust 仓无 git 历史，无法对旧快照用同一宽口径重算，故不直接宣称可比增量）。

## 4. 未镜像 Top 清单（15，按 Java 测试体量 LOC 排序，供后续补测排期）

| # | 模块 | Java 测试类 | LOC | 补测理由 |
| ---: | --- | --- | ---: | --- |
| 1 | cp | `WxCpSchoolUserTest` | 793 | 家校用户链路核心域，单类最大缺口 |
| 2 | cp | `WxCpXmlMessageTest` | 622 | CP XML 消息解析主干（对应 Rust `WxCpXmlMessage` 反序列化面） |
| 3 | miniapp | `WxMaMessageTest` | 576 | 小程序消息主链路 bean/router 断言 |
| 4 | cp | `WxCpTpLicenseServiceImplTest` | 551 | 第三方代应用许可证服务全方法 |
| 5 | pay | `MultiAppIdSwitchoverTest` | 546 | 多 appid 切换行为（Rust session/config 关键差异区） |
| 6 | cp | `WxCpOaWeDocServiceImplTest` | 541 | 微文档 OA 服务 HTTP 语义 |
| 7 | cp | `BaseWxCpTpServiceImplTest` | 519 | TP 基础服务基类（组件票据刷新等公共路径） |
| 8 | cp | `WxCpOaWeDocJsonTest` | 509 | 微文档 JSON 解析 golden |
| 9 | open | `WxOpenMaServiceImplTest` | 469 | 开放平台托管小程序服务（open 模块最大洞） |
| 10 | cp | `WxCpSchoolContactMessageTest` | 442 | 家校联系人消息 |
| 11 | cp | `WxCpLinkedCorpMessageTest` | 403 | 互联企业消息 |
| 12 | cp | `WxCpOaApprovalTemplateResultTest` | 390 | 审批模板详情/提交结果解析 |
| 13 | pay | `TransferAuthorizationApiCompatibilityTest` | 361 | **4.8.6 新增**转账授权兼容层（同族 user 版已有镜像 `transfer_authorization_test.rs`，商家版待补） |
| 14 | mp | `WxMpMaterialServiceImplTest` | 331 | 公众号素材服务 HTTP 语义（mp 剩余最大单项） |
| 15 | pay | `WxPayUnifiedOrderV3ResultTest` | 322 | V3 统一下单响应解析 golden |

说明：完整未镜像清单共 205 条，按 `wc -l` 排序即可复现本表头部队列；cp 模块占 Top12 中 9 席，建议下一波补测以「cp 整域批次」立项。

## 5. 专项：WxJava 4.8.5/4.8.6 新增测试类镜像状态

新增总计 **39**（v4.8.5 带 12 →累计 353；v4.8.6 再增 →380）。**已镜像 4，未镜像 35（10.3%）**。

### 5.1 已镜像（4）

| Java 测试类 | Rust 镜像文件 |
| --- | --- |
| channel `WxChannelKfServiceImplTest` | `crates/wx-rust-channel/tests/semantic_a_channel_5svc.rs` |
| channel `WxChannelQicServiceImplTest` | `crates/wx-rust-channel/tests/semantic_a_channel_5svc.rs` |
| pay `PartnerInvoiceServiceImplTest` | `crates/wx-rust-pay/tests/partner_invoice_test.rs` |
| pay `TransferUserAuthorizationApiCompatibilityTest` | `crates/wx-rust-pay/tests/transfer_authorization_test.rs` |

### 5.2 未镜像（35），按模块分组

- **channel ×17**（4.8.5/4.8.6 主战场）：`WxChannelAfterSaleServiceImplGuaranteeTest`、`WxChannelEwaybillServiceAccessorTest`、`WxChannelFavoriteServiceImplTest`、`WxChannelLimitedDiscountServiceImplTest`、`WxChannelProductAssistantServiceImplTest`、`WxChannelProductManagementServiceImplTest`、`WxChannelProductStockServiceImplTest`、`WxChannelServiceImplTest`、`WxChannelShopLinkServiceImplTest`、`WxTalentServiceImplTest`、`AfterSaleContractTest`、`PrintContentParamTest`、`WxChannelEwaybillBeanTest`、`WxChannelKfBeanTest`、`WxChannelSupplierBeanTest`、`WxChCryptUtilsTest`、`XmlUtilsTest`
- **cp ×5**：`BaseWxCpServiceImplLogTest`、`WxCpIntelligentRobotApiModeServiceTest`、`WxCpTodoServiceImplTest`、`WxCpDefaultConfigImplTest`、`WxCpIntelligentRobotCryptUtilTest`
- **miniapp ×5**：`WxMaInternetServiceImplSignatureTest`、`WxMaSubscribeServiceImplUrlTest`、`WxMaUserServiceImplPhoneNumberTest`、`WxMaGenerateNfcSchemeRequestTest`、`WxMaApiUrlConstantsXPayTest`
- **pay ×5**：`GeneralInvoiceRequestTest`、`WxMaEntrustRequestTest`、`LegacyEcommerceApiCompatibilityTest`、`TransferAuthorizationApiCompatibilityTest`、`WxPayServiceSandboxTest`
- **open ×3**：`WxOpenXmlMessageTest`、`WxOpenCryptUtilTest`、`WxOpenGsonBuilderTest`

专项结论：**新测试跟进率为 4/39 ≈ 10%**，显著落后存量镜像节奏。channel 是 4.8.5→4.8.6 功能迭代最重的模块（电商/视频号新接口），若不做定向追平，总镜像率将随上游版本继续被动稀释（每新增 ~13 个未镜像类约侵蚀 1pp）。

## 6. 附注

- 本轮依据的 Rust 标注快照为 2026-08-27 工作区状态；Rust 仓当前非 git 仓库，无法做时间轴回放复算，之后建议恢复版本管理以保证测量可追溯。
- 抽取中间产物（可直接复核）：`/tmp/java_tests.txt`、`/tmp/rust_tests.txt`、`/tmp/mirror_map.json`、`/tmp/final_matrix.json`、`/tmp/java_new_4856.txt`。

---

> 以上 §1–§6 为前次（当日早些时候）结论，保持原样留存。以下 §7 为 g3/g4 深度补测合入后的本次复测结果。

<a id="sec7"></a>

## 7. 复测 #2：g3/g4 深度补测后（2026-08-27 晚）

### 7.0 结论一句话

**整体镜像率 46.8%（178/380，was 46.1%）**；g3/g4 补测组对 25 个真实存在的 miniapp Java 测试类完成逐方法级镜像（其组内宽口径镜像率 88%→100%），其中**类级净新增镜像仅 3 个**（另 22 个此前已有声明式镜像、本轮升级为逐方法 MockServer 断言）；1 条声明（`WxMaComplaintServiceImplTest`）在 WxJava 中不存在同名类，实为 pay 模块 `ComplaintServiceImplTest` 的重复覆盖（该类早已由 `phase3_pay_complaint_bank_submerchant.rs` 镜像）。

### 7.1 复测方法与基线可复现性验证

- Java 分母不变：WxJava HEAD `25423da46393c8ddd170217ba5badec04253fca3`（=tag v4.8.6.B）下 9 模块共 **380** 个测试类（复验命令同 §1.3；分布 29/59/67/74/83/14/48/2/4 与 §2 完全一致）。
- Rust 侧本轮快照：`crates/*/tests/*.rs` 共 134 个 .rs（含 qidian 辅助 `common/mod.rs` 1 个，不计入），即 **133 个候选文件**，较前次测量的 130 增加 4 个新文件（commit bd49f14 `channel_legacy_completion_test.rs`、f370554 `legacy_audit_fixes_test.rs`、584e9f5 `g3_g4_depth_audit.rs`+`g3_g4_extra_mirror.rs`）。
- **基线复现**：将抽取脚本在前次 corpus 上重跑（仅排除两个 g3/g4 新文件），得到与前次 §2 矩阵**逐格完全一致**的结果——九个模块的宽口径已镜像数为 15/35/40/29/22/3/25/2/4，实体合计 175/380（46.1%），证明口径重建无误、两次读数可直接对比。
- 严格口径重构说明：前次"严=152"的抽取器中间产物已被清理，本轮按 §1.1 严口径定义重建（正则 `(?:对应)?[Jj]ava[:：\\\s]{0,7}\s*(类名)\b`）。该窄口径在前次 corpus 上复现值为 **133**；补回反引号形态（`` 镜像 Java `XxxTest` ``）可得 ≈151–152，与上次读数吻合——即上次"严 152"实际混入了反引号单行标注。本轮统一双报同一实现的两种变体：**窄严 133→136、含反引号严 151→152**，杜绝口径漂移误读。

### 7.2 总镜像率更新（主口径：宽）

| 时点 | Java 基数 | 已镜像 | 镜像率（宽） | 严格口径读数 |
| --- | ---: | ---: | ---: | --- |
| 前次（g3/g4 合入前） | 380 | 175 | 46.1% | 窄 133 / 含反引号 ≈151–152 |
| **本次复测 #2** | **380** | **178** | **46.8%（+0.7pp）** | 窄 136 / 含反引号 152 |

分模块变化（宽口径，仅列变动项，其余七模块数值与 §2 相同）：

| 模块 | Java 类数 | 前 → 后已镜像 | 前 → 后镜像率 |
| --- | ---: | ---: | --- |
| miniapp | 67 | 40 → **43** | 59.7% → **64.2%** |
| **合计** | **380** | 175 → **178** | 46.1% → **46.8%** |

Top-5 缺口模块（按未镜像绝对数，为本轮后最新队列，补测优先级从高到低）：

| # | 模块 | 未镜像/总数 | 模块镜像率 |
| ---: | --- | ---: | ---: |
| 1 | cp | 61/83 | 26.5% |
| 2 | pay | 45/74 | 39.2% |
| 3 | miniapp | 24/67 | 64.2% |
| 4 | mp | 24/59 | 59.3% |
| 5 | channel | 23/48 | 52.1% |

### 7.3 g3/g4 深度补测专项

本轮合入 commit 584e9f5（`test(miniapp): g3/g4 深度补测——镜像 Java 测试类 18→34`），新增两个集成测试文件：

- `crates/wx-rust-miniapp/tests/g3_g4_depth_audit.rs`（1033 行，`#[tokio::test]` ×45，G3 电商服务组 + G4 能力服务组）
- `crates/wx-rust-miniapp/tests/g3_g4_extra_mirror.rs`（676 行，`#[tokio::test]` ×25）

合计 **70 个新测试**（45+25，全部通过，复验命令 `cargo test -p wx-rust-miniapp --test g3_g4_depth_audit --test g3_g4_extra_mirror`）。每个测试函数带 `/// 对应 Java: <Class>.<method>` 逐方法标注，断言覆盖状态码/字段值/errcode!=0/请求体关键字段，质量高于早期批量文件的声明式镜像。

被镜像（声明）的 26 个 Java 测试类（按字母序；★=本轮类级净新增，☆=该声明在 WxJava 无同名类，见下注）：

| # | Java 测试类（miniapp） | 本轮前类级状态 |
| ---: | --- | --- |
| 1 | `WxMaAnalysisServiceImplTest` | 已有既有镜像（phase2_batch3 / sub_domain_g1_core） |
| 2 | `WxMaDeviceSubscribeServiceImplTest` | 已有既有镜像（sub_domain_g4_ability） |
| 3 | `WxMaEmployeeRelationServiceImplTest` | 已有既有镜像（coverage_boost_ma_sub_services） |
| 4 | `WxMaImmediateDeliveryServiceImplTest` | 已有既有镜像（sub_domain_g3_shop） |
| 5 | `WxMaJsapiServiceImplTest` | 已有既有镜像（sub_domain_g2_content） |
| 6 | `WxMaLinkServiceImplTest` | 已有既有镜像（sub_domain_g2_content） |
| 7 | `WxMaLiveGoodsServiceImplTest` | 已有既有镜像（coverage_boost_ma_product_live） |
| 8 | `WxMaLiveMemberServiceImplTest` | 已有既有镜像（coverage_boost_ma_product_live） |
| 9 | `WxMaLiveServiceImplTest` | 已有既有镜像（coverage_boost_ma_product_live） |
| 10 | `WxMaOpenApiServiceImplTest` | 已有既有镜像（sub_domain_g2_content） |
| 11 | `WxMaPluginServiceImplTest` | 已有既有镜像（sub_domain_g2_content） |
| 12 | `WxMaPromotionServiceTest` | 已有既有镜像（sub_domain_g4_ability） |
| 13 | `WxMaQrcodeJumpServiceImplTest` | 已有既有镜像（sub_domain_g4_ability） |
| 14 | `WxMaReimburseInvoiceServiceImplTest` | 已有既有镜像（sub_domain_g4_ability） |
| 15 | `WxMaSchemeServiceImplTest` | 已有既有镜像（phase2_batch3 / sub_domain_g2_content） |
| 16 | `WxMaSettingServiceImplTest` | 已有既有镜像（phase2_batch3 / sub_domain_g1_core） |
| 17 | `WxMaShopAccountServiceImplTest` | 已有既有镜像（sub_domain_g3_shop），本轮另写逐方法深化版 |
| 18 | `WxMaShopAfterSaleServiceImplTest` | 已有既有镜像（sub_domain_g3_shop） |
| 19 | `WxMaShopAuditServiceImplTest` | **本轮净新增**（仅 g3_g4_depth_audit.rs 声明） |
| 20 | `WxMaShopCatServiceImplTest` | 已有既有镜像（sub_domain_g3_shop） |
| 21 | `WxMaShopDeliveryServiceImplTest` | **本轮净新增**（仅 g3_g4_depth_audit.rs 声明） |
| 22 | `WxMaShopPayServiceImplTest` | **本轮净新增**（仅 g3_g4_depth_audit.rs 声明） |
| 23 | `WxMaShopRegisterServiceImplTest` | 已有既有镜像（sub_domain_g3_shop） |
| 24 | `WxMaVodServiceImplTest` | 已有既有镜像（sub_domain_g4_ability） |
| 25 | `WxMaXPayServiceImplTest` | 已有既有镜像（sub_domain_g4_ability） |
| 26 | `WxMaComplaintServiceImplTest` ☆ | 见下注 |

**幽灵声明注**：`WxMaComplaintServiceImplTest` 在 WxJava 全仓 9 模块的 src/test 下均不存在（complain 域的小程序侧只有 bean，无 ServiceImpl 测试）。nearest 对应物是 **pay** 模块的 `ComplaintServiceImplTest`——它早由 `crates/wx-rust-pay/tests/phase3_pay_complaint_bank_submerchant.rs`（20 处引用）完整镜像，本轮 miniapp 的投诉查询 errcode!=0 用例属第三重覆盖，不改变类级计数。commit 标题"18→34"按 Rust 文件声明数口径成立；按本报告类级净增口径为 **+3**。

#### g3/g4 组占比影响汇总

- 组内（25 个实存目标类）宽口径镜像率：**88%（22/25）→ 100%（25/25）**；缺口归零。
- 对整体：总镜像 175→178，**46.1%→46.8%（+0.7pp）**；g3/g4 目标占未存量大缺口的性质是"加深"而非"扩面"——扩面效率最高的仍是 Top-15 未镜像队列。
- 对 miniapp 模块：40→43，**59.7%→64.2%（+4.5pp）**，与 mp（59.3%）拉开身位，成为除 aispeech/qidian 全量模块外第一梯队首位。
- 定性收益（不进镜像率但值得记录）：70 个用例把 shop 三件套（Audit/Delivery/Pay）等此前只有粗粒度声明的类升级到逐方法对照，并在 xpay/promotion 等域补了 errcode!=0 负路径。

### 7.4 复测后 Top-15 未镜像清单（供下一波排期；与 §4 一致，无一出榜）

本轮 3 个净新增全部来自原已镜像类的加深，未触碰未镜像队列；经 LOC 重算（202 条未镜像），Top-15 名次与 §4 完全相同。**首批 5 个**：

1. cp `WxCpSchoolUserTest`（793 行）
2. cp `WxCpXmlMessageTest`（622）
3. miniapp `WxMaMessageTest`（576）
4. cp `WxCpTpLicenseServiceImplTest`（551）
5. pay `MultiAppIdSwitchoverTest`（546）

后续 10 名依次：`WxCpOaWeDocServiceImplTest`(541)、`BaseWxCpTpServiceImplTest`(519)、`WxCpOaWeDocJsonTest`(509)、`WxOpenMaServiceImplTest`(469)、`WxCpSchoolContactMessageTest`(442)、`WxCpLinkedCorpMessageTest`(403)、`WxCpOaApprovalTemplateResultTest`(390)、`TransferAuthorizationApiCompatibilityTest`(361)、`WxMpMaterialServiceImplTest`(331)、`WxPayUnifiedOrderV3ResultTest`(322)。cp 仍占半壁（7/15），维持「cp 整域批次」立项建议。

### 7.5 复测附注

- 复测中间产物：`/tmp/java_tests.txt`（380）、`/tmp/rust_tests.txt`（134）、`/tmp/mirror_map.json`、`/tmp/final_matrix.json`、抽取脚本 `/tmp/mirror_extract.py`。
- Rust 仓现已恢复 git 管理（本报告所在 commit 起），§6"非 git 仓库"的限制解除；此后镜像率测量可锚定 commit hash 回放。
- 建议：(a) 后续轮次继续宽/严双口径同脚本双报；(b) 将 g3/g4 的"逐方法 + 负路径"模板推广到 Top-15 补测；(c) miniapp 剩余 24 个未镜像多为 bean/XPay 常量类，适合一次性 batch 收口。
