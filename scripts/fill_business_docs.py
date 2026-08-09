#!/usr/bin/env python3
"""批量填充 8 个业务模块的《迁移路线图》和《语义迁移对照表》。

对每个模块：
- 路线图：总目标/纳入范围/基线盘点/阶段任务（注入核心对象）/阶段总览状态/风险
- 语义表：核心能力/错误/序列化/组件替换等公共行为族 + 模块特有服务

模块特有数据在 MODULE_SPEC 中定义。
"""
import os
import re
import sys

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
MIG_DIR = os.path.join(BASE, "docs", "migration")

MODULE_SPEC = {
    "weixin-java-mp": {
        "crate": "wx-rust-mp", "n_obj": 428, "n_test": 71,
        "pkg": "me/chanjar/weixin/mp", "prefix": "WxMp",
        "services": "WxMpService + Kefu/Menu/User/UserTag/Material/MassMessage/TemplateMsg/Qrcode/DataCube/Card/Store/SubscribeMsg/Shake/Wifi/Comment/Device/Draft/FreePublish/Guide*/Marketing/MerchantInvoice/ReimburseInvoice/AiOpen/MemberCard",
        "key": "access_token(双检锁)+jsapi_ticket(票据缓存)+消息路由 WxMpMessageRouter(规则链+异步执行)+XML 消息收发加解密",
        "bean": "WxMpXmlMessage/WxMpXmlOutMessage(消息 XML) WxMpMenu(菜单树) WxMpUserInfo(用户) WxMpMaterial(素材) WxMpQrcodeTicket(二维码)",
        "unique": "消息 XML 加解密(AES-CBC+SHA1 签名+XML 格式)；jsapi_ticket 双票据缓存；消息路由 Router/Rule/Handler/Interceptor",
    },
    "weixin-java-miniapp": {
        "crate": "wx-rust-miniapp", "n_obj": 611, "n_test": 68,
        "pkg": "cn/binarywang/wx/miniapp", "prefix": "WxMa",
        "services": "WxMaService + User/Msg/Media/Kefu/Analysis/Cloud/Code/Express/Link/Scheme/Live*/Marketing/Plugin/Product*/Promotion/Qrcode*/Run/Security/Setting/Share/Shop*/Subscribe/Vod/XPay/OpenApi 等 50+ 子服务",
        "key": "code2Session(登录换 openid)+access_token 缓存+WxMaMessageRouter+微信支付关联(小程序支付)",
        "bean": "WxMaJscode2SessionResult(登录态) WxMaUserInfo(用户信息) WxMaTemplateMessage(模板消息) shop/xpay/express 子域 bean",
        "unique": "小程序 code2Session 登录；子域最细（shop/xpay/express/product）；云开发 Cloud 能力",
    },
    "weixin-java-pay": {
        "crate": "wx-rust-pay", "n_obj": 570, "n_test": 71,
        "pkg": "com/github/binarywang/wxpay", "prefix": "WxPay",
        "services": "WxPayService(v2+v3) + PayScore/ProfitSharing/Redpack/Transfer/EntPay/Ecommerce/Complaint/MarketingFavor/Bank/CustomDeclaration/MiPay 等 30+ 子服务",
        "key": "支付签名(MD5/HMAC-SHA256)+v3 平台证书验签(RSA)+XML 报文解析(支付回调)+多商户切换",
        "bean": "WxPayUnifiedOrderRequest/Result(下单) WxPayOrderQueryResult(查单) WxPayNotifyResult(回调) BaseWxPayResult(XML 基类) WxPayRefundResult(退款)",
        "unique": "v2 XML 报文+MD5/HMAC 签名；v3 JSON+RSA 证书验签+敏感字段加密；商户证书管理；多 AppId 切换",
    },
    "weixin-java-cp": {
        "crate": "wx-rust-cp", "n_obj": 594, "n_test": 88,
        "pkg": "me/chanjar/weixin/cp", "prefix": "WxCp",
        "services": "WxCpService + Agent/Department/User/Tag/Message/Media/ExternalContact/Oa*/School*/Kf/Living/Meeting/WeDoc/WeDrive 等 30+ 子服务",
        "key": "企业 access_token+WxCpMessageRouter+企微 JSAPI 签名+会话存档(消息审计)+第三方代开发",
        "bean": "WxCpMessage(消息) WxCpUser(成员) WxCpDepartment(部门) WxCpTag(标签) WxCpExternalContact(外部联系人) WxCpXmlMessage(回调 XML)",
        "unique": "会话存档 Finance SDK(JNI→Rust 重写解密)；企微回调加解密；第三方应用(代开发)；OA 审批/打卡/日程",
    },
    "weixin-java-open": {
        "crate": "wx-rust-open", "n_obj": 240, "n_test": 13,
        "pkg": "me/chanjar/weixin/open", "prefix": "WxOpen",
        "services": "WxOpenService(第三方平台) + ComponentService(组件) 代公众号/小程序管理(复用 mp/ma)",
        "key": "component_access_token(组件票据)+授权事件推送+代 mp/ma 服务桥接(WxOpenMpServiceImpl extends WxMpServiceImpl)",
        "bean": "WxOpenComponentAccessToken(组件 token) WxOpenAuthorizationInfo(授权信息) WxOpenMaAuthInfo(小程序授权)",
        "unique": "第三方平台授权流程；复用 mp/ma crate（Rust 用组合/trait 替代继承）",
    },
    "weixin-java-channel": {
        "crate": "wx-rust-channel", "n_obj": 618, "n_test": 31,
        "pkg": "me/chanjar/weixin/channel", "prefix": "WxChannel",
        "services": "WxChannelService + Order/Product/Coupon/AfterSale/Fund 等子域服务",
        "key": "视频号小店交易闭环+access_token 缓存+ResponseUtils.decode 统一响应解码",
        "bean": "WxChannelOrder(订单) WxChannelProduct(商品) WxChannelCoupon(优惠券) WxChannelAfterSale(售后) WxChannelFund(资金)",
        "unique": "视频号小店电商场景；ResponseUtils 统一解码（JSON 错误码处理）",
    },
    "weixin-java-aispeech": {
        "crate": "wx-rust-aispeech", "n_obj": 25, "n_test": 2,
        "pkg": "me/chanjar/weixin/aispeech", "prefix": "WxAispeech",
        "services": "WxAispeechService(AI 语音)",
        "key": "语音合成/识别接口",
        "bean": "语音请求/响应 bean",
        "unique": "最小模块；语音接口封装",
    },
    "weixin-java-qidian": {
        "crate": "wx-rust-qidian", "n_obj": 27, "n_test": 6,
        "pkg": "me/chanjar/weixin/qidian", "prefix": "WxQidian",
        "services": "WxQidianService + DialService(呼叫)/CallDataService(通话数据)",
        "key": "复用 mp 配置体系(WxQidianConfigStorage 同构)+IVR 呼叫",
        "bean": "IVRDialRequest/Response(呼叫) GetSwitchBoardListResponse(座席列表) QidianResponse(响应)",
        "unique": "企点呼叫中心能力；配置与 mp 高度同构",
    },
}


def fill_roadmap(module, spec):
    doc = os.path.join(MIG_DIR, module, "迁移路线图.md")
    t = open(doc, encoding="utf-8").read()
    p = spec["prefix"]

    repl = [
        ("<!-- TODO：说明公共 API、行为、示例、文档、测试和生产替代目标。 -->",
         f"""- **公共 API 对齐**：`{spec['pkg']}.*` 全部 {spec['n_obj']} 个 Java 对象迁移为 `{spec['crate']}` crate。
- **行为对齐**：{spec['key']}。
- **序列化对齐**：Gson TypeAdapter → serde 派生，JSON/XML 线格式逐字段对齐（XML 用 quick-xml）。
- **并发对齐**：`Lock` → `tokio::sync::Mutex`，`ExecutorService` → `tokio::task::spawn`。
- **生产替代目标**：纯 Rust + reqwest，`#![forbid(unsafe_code)]`，MSRV 1.85 / Edition 2024。"""),

        (f"| 生产对象 | <!-- TODO --> | <!-- TODO --> | 对象、方法、参数、逻辑可追溯 |",
         f"| 生产对象 | `{spec['pkg']}/**`（{spec['n_obj']} 对象） | `{spec['crate']}/src/**`（{spec['n_obj']} 主文件） | 对象、方法、参数、逻辑可追溯 |"),
        (f"| 示例/脚本 | <!-- TODO --> | <!-- TODO --> | 真实回放通过 |",
         f"| 示例/脚本 | 模块内 example/demo 类 | `examples/` + 集成测试 | 真实回放通过 |"),
        (f"| 测试/夹具 | <!-- TODO --> | <!-- TODO --> | 镜像、golden/live 差分与 Rust 原生证据分层通过 |",
         f"| 测试/夹具 | `src/test/java`（{spec['n_test']} 个测试对象） | `tests/` + 单元测试 | 镜像、golden/live 差分分层通过 |"),
        (f"| 宿主集成 | <!-- TODO --> | <!-- TODO --> | 真实依赖集成通过 |",
         f"| 宿主集成 | redis、微信沙箱 API | `{spec['crate']}` + feature gate | 真实依赖集成通过 |"),

        ("| <!-- TODO --> | `MISSING` / `STUB` / `PLATFORM_NA` | <!-- TODO；阻塞原因作为元数据，不改变事实状态 --> | <!-- TODO --> | 未完成状态不计入；平台不适用单列 | <!-- TODO --> |",
         f"""| HTTP 多后端适配类（如有） | `PLATFORM_NA` | reqwest 统一 HTTP | 单列 | `PLATFORM_NA` | reqwest 集成测试 |
| Gson 手写 TypeAdapter | `PLATFORM_NA` | serde 派生替代 | 单列 | `PLATFORM_NA` | golden 差分 |"""),

        ("| 模块/crate | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |",
         f"| 模块/crate | {module} | {spec['crate']} | 1:1 | `ls {module}` / `Cargo.toml` |"),
        ("| 对象/主文件 | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |",
         f"| 对象/主文件 | {spec['n_obj']} | 0 | {spec['n_obj']} | `scripts/inventory_java_objects.py` |"),
        ("| 公共方法/构造器 | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |",
         "| 公共方法/构造器 | 待 B0 以 javap 冻结 | 0 | 全量 | 方法清点 B0 冻结 |"),
        ("| 示例/脚本 | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |",
         "| 示例/脚本 | 待清点 | 0 | 全量 | B0 清点 |"),
        ("| 测试 | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |",
         f"| 测试 | {spec['n_test']} 个测试对象 | 0 | 全量处置 | 测试三本账 |"),

        # 阶段总览状态
        ("| B0 | 冻结全量基线、四文档和批次清单 | <!-- TODO --> | SHA、工具链、对象/签名/测试/例外完整分母 | 范围、合同、依赖顺序和口径锁定 |",
         "| B0 | 冻结全量基线、四文档和批次清单 | **规划完成**（四文档已生成） | SHA a49d6e14、对象分母、方法分母待 javap 冻结 | 范围锁定 |"),
        ("| B1 | 锁定架构与组件替换 | <!-- TODO --> | 共享错误、trait、registry、adapter、并发与依赖决策 | 阻断性决策已解决或获批 |",
         "| B1 | 锁定架构与组件替换 | 未开始 | trait + reqwest + serde 决策 | 阻断性决策解决 |"),
        ("| B2 | 一次性完成整批语义实现 | <!-- TODO --> | 全部对象、方法、重载、注释、示例、测试代码 | 所有非豁免行有真实实现；尚未运行验收 |",
         f"| B2 | 一次性完成整批语义实现 | 未开始 | {spec['n_obj']} 对象 + 方法/注释/测试 | 所有非豁免行有真实实现 |"),
        ("| V0 | 实现冻结与统一静态审计 | <!-- TODO --> | 全量 CodeGraph/静态差分、四文档批量回填 | 完整分母无遗漏、无 STUB |",
         "| V0 | 实现冻结与统一静态审计 | 未开始 | audit_migration_layout.py + 四文档回填 | 完整分母无遗漏 |"),
        ("| V1 | 统一工程验证 | <!-- TODO --> | fmt/check/test/doc/Clippy/feature/target 结果 | 整批工程门禁通过 |",
         "| V1 | 统一工程验证 | 未开始 | cargo fmt/check/test/clippy | 整批工程门禁通过 |"),
        ("| V2 | 统一行为验证 | <!-- TODO --> | 镜像、golden/live 差分、真实脚本回放 | 行为合同通过 |",
         "| V2 | 统一行为验证 | 未开始 | Java 测试镜像 + golden 差分 | 行为合同通过 |"),
        ("| V3 | 统一非功能验证 | <!-- TODO --> | 并发、负载、soak、property/fuzz、安全报告 | 阈值通过 |",
         "| V3 | 统一非功能验证 | 未开始 | 并发（token）、proptest | 阈值通过 |"),
        ("| V4 | 宿主、灰度与回滚 | <!-- TODO --> | 真实集成、灰度和回滚记录 | 生产替代门禁通过 |",
         "| V4 | 宿主、灰度与回滚 | 未开始 | 微信沙箱、redis | 生产替代门禁通过 |"),

        ("### 当前批次：<!-- TODO：必须是完整模块或用户明确授权的多模块范围 -->",
         f"### 当前批次：{module} → {spec['crate']}（完整模块）"),
    ]
    for old, new in repl:
        if old in t:
            t = t.replace(old, new)
        else:
            print(f"  [roadmap-warn] 未匹配: {old[:50]}")

    # 阶段任务表：注入核心对象行（表头后的 TODO 行）
    task_rows = f"""| `{p}Service`（门面） | `api/wx_{p.lower()}_service.rs`（async trait） | common `WxService` | 门面接口 + 子服务聚合 | V1 单测 | `MISSING` |
| `{p}ServiceImpl`/Base | `api/impl/`（trait 默认实现） | `wx-rust-common` | 继承链 → trait/组合 | V1 单测 | `MISSING` |
| ConfigStorage | `config/wx_*_config_storage.rs`（async trait） | `wx-rust-common` | token/ticket 缓存 + 锁 | V1 单测 + V3 并发 | `MISSING` |
| 子域 Service（{spec['services']}） | `api/*_service.rs`（每子域一个文件） | `{p}Service` | 子域接口 + 实现 | V1 单测 + V2 golden | `MISSING` |
| 消息路由 Router/Rule | `message/wx_*_message_router.rs`（builder 模式） | `wx-rust-common` | 规则链 + 异步执行 | V1 单测 + V2 golden | `MISSING` |
| 核心 bean（{spec['bean']}） | `bean/**`（serde 派生） | 无 | 序列化对齐 | V2 golden | `MISSING` |
| 工具/JSON/加解密 | `util/**` | 相关 crate | Gson→serde；XML→quick-xml | V2 golden | `MISSING` |"""
    m = re.search(r"(\| Java 对象/能力 \| Rust 文件/能力 \| 前置依赖 \| 工作项 \| 末端验收归属 \| 状态 \|\n\|-+\|.*\n)(\| <!-- TODO --> \|.*\n?)", t)
    if m:
        t = t.replace(m.group(2), task_rows + "\n", 1)
    else:
        print("  [roadmap-warn] 阶段任务表未找到")

    # 风险表
    m = re.search(r"(\| 风险 \| 影响 \| 触发信号 \| 对策 \| Owner \|\n\|-+\|.*\n)(\| <!-- TODO --> \|.*\n?)", t)
    if m:
        risk = f"""| {p} API 语义偏差 | 微信接口行为不一致 | golden 差分失败 | 逐接口 golden 夹具 | WxRust |
| 序列化差异（JSON/XML） | 字段名/格式不匹配 | 差分失败 | 线格式逐字段对齐 | WxRust |
| token/票据并发刷新 | 超限或重复请求 | 并发测试失败 | async 锁 + 单测 | WxRust |
| 消息路由行为差异 | 规则匹配/异步执行不一致 | 路由测试失败 | 镜像测试 | WxRust |"""
        t = t.replace(m.group(2), risk + "\n", 1)
    else:
        print("  [roadmap-warn] 风险表未找到")

    open(doc, "w", encoding="utf-8").write(t)
    print(f"[ok] {module} 路线图, 剩余 TODO: {t.count('<!-- TODO -->')}")


def fill_semantic(module, spec):
    doc = os.path.join(MIG_DIR, module, "语义迁移对照表.md")
    t = open(doc, encoding="utf-8").read()
    p = spec["prefix"]

    repl = [
        # 一、核心能力
        ("| <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | `MISSING` | <!-- TODO --> |",
         f"""| 门面 Service（{p}Service） | 顶层接口 + 子服务聚合 | async trait + 子服务持有 | 继承链 → trait/组合 | `MISSING` | 接口一致性测试 |
| access_token/ticket 缓存 | 双检锁 + 过期刷新 | async `Mutex` + 缓存 | Java `Lock` → async 锁 | `MISSING` | 并发刷新差分 |
| {spec['unique']} | 见各能力 | Rust 原生实现 | 一致 | `MISSING` | golden 差分 |
| 子域 Service | {spec['services']} | 每子域一个 trait + impl | 一致 | `MISSING` | 镜像测试 |"""),

        # 四、错误体系
        ("| <!-- TODO --> | `thiserror` / typed `Result` | <!-- TODO --> | <!-- TODO --> | `MISSING` | <!-- TODO --> |",
         f"""| `{p}ErrorException`（如存在） | `#[derive(Error)]` 变体 | 根因链 | 无重试 | `MISSING` | 抛错场景差分 |
| 微信错误码 | `WxError` + 分模块错误码表 | 错误码 → 中文 | — | `MISSING` | 错误码表差分 |"""),

        # 五、序列化
        ("| <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | `MISSING` | <!-- TODO --> |",
         f"""| JSON bean（{spec['bean']}） | serde 派生 + 自定义 `Serialize`/`Deserialize` | 字段名/默认/null 逐一对齐 | JSON 线格式兼容 | `MISSING` | golden 夹具 |
| XML 报文（如有） | quick-xml + serde | CDATA/嵌套/顺序 | XML 线格式兼容 | `MISSING` | golden 夹具 |"""),

        # 七、并发
        ("| <!-- TODO --> | `Mutex`/`RwLock`/`DashMap`/Tokio/Rayon | <!-- TODO --> | <!-- TODO --> | `MISSING` | <!-- TODO --> |",
         f"""| `Lock`（token 锁） | `tokio::sync::Mutex` | 超时抢锁 | Drop 释放 | `MISSING` | 并发测试 |
| `ExecutorService`（Router） | `tokio::task::spawn` | 取消 | JoinHandle | `MISSING` | 取消测试 |"""),

        # 八、SPI
        ("| <!-- TODO --> | trait registry / `inventory` / factory | <!-- TODO --> | <!-- TODO --> | `MISSING` | <!-- TODO --> |",
         "| 多实现选择（如 HTTP 后端） | feature 门控 / trait object | 编译期 | 无动态加载 | `MISSING` | feature 测试 |"),

        # 九、注解
        ("| <!-- TODO --> | compile/runtime/framework scan | derive/attribute/middleware/registry | <!-- TODO --> | `MISSING` | <!-- TODO --> |",
         "| Lombok `@Data`/`@Builder` | 编译期 | `#[derive(...)]` + builder | derive | `MISSING` | 生成 API 对比 |"),

        # 十、组件替换
        ("| <!-- TODO --> | <!-- TODO --> | <!-- 能力/协议/约束查询；crates.io/docs.rs/source/RustSec --> | std/direct/wrapper/trait+adapters/SPI/macro/redesign/host/exempt | <!-- TODO --> | contract/license/MSRV/target/runtime/security/maintenance | <!-- 分项证据与未知项 --> | `CANDIDATE` / `DEPENDENCY_DECLARED` / `SPIKE_VERIFIED` / `CONTRACT_VERIFIED` / `HOST_VERIFIED` / `PRODUCTION_READY` | <!-- TODO --> |",
         f"""| 微信 API HTTP | 签名/证书/超时 | `reqwest`（crates.io 2026-07 观察） | direct | 0.13.x | Apache-2.0/MSRV 1.85 | 适配高 | `CANDIDATE` | POC |
| JSON | 线格式 | `serde_json` | direct | 1.0.x | MIT/Apache-2.0 | 适配高 | `CANDIDATE` | round-trip |
| XML | CDATA/嵌套 | `quick-xml` | direct | 0.41.x | MIT | 适配中 | `CANDIDATE` | POC |
| 加解密/签名 | {p} 特有算法 | RustCrypto crates（aes/rsa/sha2/hmac） | direct | 锁定版本 | 各 license 合规 | 适配高 | `CANDIDATE` | 已知向量测试 |"""),

        # 十三、不迁移
        ("| <!-- TODO --> | `PLATFORM_NA` / `RUST_EXTENSION` | <!-- JVM/字节码/类加载器证据或 Rust 扩展原因 --> | <!-- TODO --> | 单列排除 | <!-- TODO --> |",
         f"""| Gson 手写 TypeAdapter | `PLATFORM_NA` | Gson 专属 | serde 派生替代 | 单列排除 | 规划批准 |
| 多 HTTP 后端（如有） | `PLATFORM_NA` | reqwest 统一 | 单列排除 | 规划批准 |
| reqwest feature 化 | `RUST_EXTENSION` | Rust 生态能力 | 不影响门面 | 单列 | 规划批准 |"""),
    ]
    for old, new in repl:
        if old in t:
            t = t.replace(old, new)
        else:
            print(f"  [semantic-warn] 未匹配: {old[:50]}")

    # 验证基线占位标记
    for lvl, name, note in [
        ("V0_STATIC", "静态结构/无 stub", "audit_migration_layout.py"),
        ("V1_RUST_LOCAL", "Rust 本地测试", "cargo test"),
        ("V2_MIRRORED", "Java 镜像合同", "镜像 Java 测试"),
        ("V3_GOLDEN_DIFF", "Java golden 差分", "golden 夹具"),
        ("V4_LIVE_DIFF", "Java/Rust live 差分", "同输入双实现"),
        ("V5_HOST", "真实宿主/依赖", "微信沙箱/redis"),
        ("V6_NONFUNCTIONAL", "并发/变异/负载/fuzz/安全", "并发+proptest"),
        ("V7_ROLLBACK", "灰度/回滚", "记录恢复时间"),
    ]:
        old = f"| `{lvl}` {name} | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |"
        if old not in t and lvl == "V2_MIRRORED":
            old = f"| `{lvl}` {name} | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> | 不等于差分 |"
        new = f"| `{lvl}` {name} | {note}（B2 冻结后） | — | — | 待执行 |"
        if old in t:
            t = t.replace(old, new)

    # 测试三本账
    old = "| `SOURCE_PARITY` | 每个 Java 测试及独立参数化/动态用例有处置 | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |"
    new = f"| `SOURCE_PARITY` | 每个 Java 测试及独立参数化/动态用例有处置 | 0/{spec['n_test']}（规划） | 待 B0 枚举 | 待执行 |"
    if old in t: t = t.replace(old, new)
    old = "| `RUST_OBLIGATION` | 目标机制引入的所有权、异步、错误、序列化、feature、macro、unsafe、adapter、组件风险 | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |"
    new = "| `RUST_OBLIGATION` | 目标机制引入的所有权、异步、错误、序列化、feature、macro、unsafe、adapter、组件风险 | 0/N（规划） | async 锁/取消/退避 | 待执行 |"
    if old in t: t = t.replace(old, new)
    old = "| `VALUE_ADD` | 分支/mutant/property/fuzz/事故/负载/安全驱动且能说明可捕获缺陷 | <!-- TODO --> | <!-- TODO --> | <!-- TODO --> |"
    new = "| `VALUE_ADD` | 分支/mutant/property/fuzz/事故/负载/安全驱动且能说明可捕获缺陷 | 0/N（规划） | token 并发/重试差分 | 待执行 |"
    if old in t: t = t.replace(old, new)

    open(doc, "w", encoding="utf-8").write(t)
    print(f"[ok] {module} 语义表, 剩余 TODO: {t.count('<!-- TODO -->')}")


def main():
    targets = sys.argv[1:] or list(MODULE_SPEC.keys())
    for m in targets:
        spec = MODULE_SPEC[m]
        fill_roadmap(m, spec)
        fill_semantic(m, spec)


if __name__ == "__main__":
    main()
