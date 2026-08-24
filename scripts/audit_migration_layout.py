#!/usr/bin/env python3
"""V0 静态结构审计：逐行核对 inventory_java_objects.csv（3287 对象）与
crates/*/src 文件树的对应关系，输出处置分类。

处置规则（逐行判定，先文件存在性、后归类集）：
1. 预期路径（CSV rust_path 列）在对应 crate src 下存在            -> IMPLEMENTED
2. 预期路径不存在但按文件名在树内唯一定位到实际文件                -> IMPLEMENTED（路径差异）
3. 属于平台/外部依赖归类集                                       -> PLATFORM_NA / DEPENDENCY_REUSED
4. 其余                                                      -> MISSING

归类集（跨模块通用）：
- PLATFORM_NA：
  * HTTP 后端实现（*ApacheHttpClientImpl / *HttpComponentsImpl / *JoddHttpImpl / *OkHttpImpl）
    Java Apache HttpClient/HttpComponents/Jodd/OkHttp 客户端专属，Rust 以 reqwest 单一后端承载
  * Redis/Redisson/Jedis 外部存储配置实现（*RedisConfigImpl / *RedissonConfigImpl / *JedisConfigImpl
    / *RedisTemplateConfigImpl / Abstract*InRedis*ConfigImpl）
    Java 外部存储配置专属，Rust 以 Config trait + 默认内存配置承载
- DEPENDENCY_REUSED：
  * Gson builder / adapter（*GsonBuilder / *GsonAdapter）
    线格式内化于 bean serde rename/from_json
  * XML transformer（*XStreamTransformer）
    XML 线格式内化于 message 模块 quick-xml
  * JSON/XML 工具类（JsonUtils / XmlUtils / ResponseUtils）
    线格式/解码语义由 serde_json / quick-xml / 执行引擎内化

可重放：修复 crates/ 缺口后再次运行本脚本即可翻转对应行为 IMPLEMENTED。
"""
import argparse
import csv
import os
import sys
from collections import Counter, defaultdict

BASE = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
CSV_PATH = os.path.join(BASE, "docs", "superpowers", "data", "inventory_java_objects.csv")

# ---------- 模块 → crate 映射 ----------

MODULE_TO_CRATE = {
    "weixin-java-common": "wx-rust-common",
    "weixin-java-mp": "wx-rust-mp",
    "weixin-java-miniapp": "wx-rust-miniapp",
    "weixin-java-pay": "wx-rust-pay",
    "weixin-java-cp": "wx-rust-cp",
    "weixin-java-open": "wx-rust-open",
    "weixin-java-channel": "wx-rust-channel",
    "weixin-java-aispeech": "wx-rust-aispeech",
    "weixin-java-qidian": "wx-rust-qidian",
}

# ---------- PLATFORM_NA 归类集 ----------

# HTTP 后端实现后缀（Java HTTP 客户端专属）
HTTP_BACKEND_SUFFIXES = (
    "ApacheHttpClientImpl",
    "HttpComponentsImpl",
    "JoddHttpImpl",
    "OkHttpImpl",
    "HttpClientImpl",       # WxChannelServiceHttpClientImpl 等
    "HttpImpl",             # WxPayServiceApacheHttpImpl 等
)

# Redis/外部存储配置实现关键字
REDIS_CONFIG_KEYWORDS = (
    "RedisConfig",
    "RedissonConfig",
    "JedisConfig",
    "RedisTemplateConfig",
    "InRedis",
    "InRedisson",
)

# HTTP 客户端特定目录（出现在 java_file 路径中）
HTTP_CLIENT_DIRS = (
    "/apache/",
    "/hc/",
    "/jodd/",
    "/okhttp/",
    "/httpclient/",
    "/httpcomponents/",
)

# HTTP 客户端特定类名前缀
HTTP_CLIENT_PREFIXES = (
    "Apache",
    "HttpComponents",
    "Jodd",
    "JoddHttp",
    "OkHttp",
)

# HTTP 客户端特定类名后缀（出现在 executor/handler/builder/proxy 类中）
HTTP_CLIENT_SUFFIXES = (
    "Executor",
    "Handler",
    "Builder",
    "Proxy",
    "RetryStrategy",
    "ProxyInfo",
)

# ---------- DEPENDENCY_REUSED 归类集 ----------

# Gson builder / adapter / helper / parser 关键字
GSON_KEYWORDS = (
    "GsonBuilder",
    "GsonAdapter",
    "GsonHelper",
    "GsonParser",
    "Adapter",
)

# XML transformer / converter / initializer 关键字
XSTREAM_KEYWORDS = (
    "XStreamTransformer",
    "XStream",
    "XStreamCData",
    "XStreamInitializer",
    "XStreamMedia",
    "XStreamReplace",
)

# JSON/XML 工具类名
JSON_XML_UTIL_NAMES = {
    "JsonUtils",
    "XmlUtils",
    "ResponseUtils",
}

# ---------- 复合词规范化（Java camelCase -> Rust snake_case 名称差异） ----------

# Java 类名中的复合词在 Rust 命名中的规范化映射
# 键：Java 类名中需要特殊处理的子串；值：Rust 文件名中的等价形式
COMPOUND_WORD_NORMALIZATIONS = {
    "OAuth2": "oauth2",      # Java WxOAuth2Service -> wx_oauth2_service.rs (非 wx_o_auth2_service.rs)
    "OAuth": "oauth",        # 兜底
    "AiCrop": "ai_crop",     # 已按常规拆分
    "ImgProc": "img_proc",   # 已按常规拆分
    "XPay": "xpay",          # WxMaXPayService -> wx_ma_xpay_service (非 wx_ma_x_pay_service)
}

# ---------- 逐项显式处置（Task 12 V0 缺口清零，2026-08-24） ----------
# 依据：各模块台账脚本（audit_channel_ledger / audit_cp_ledger / audit_open_ledger /
# audit_miniapp_ledger / audit_pay_ledger）既有 SPECIAL/INLINED 归类 + 本次逐项核对
# WxJava 源码后的处置结论（详见 docs/verification/V0-gap-closure.md）。
# 三张表均以 (module, java_name) 为键；仅在文件存在性/复合词模糊匹配未命中时生效
# （新增文件落在预期路径的项由存在性判定优先，无需进表）。

# 已实现——符号改名/合并/跨 crate/生成器平铺（IMPLEMENTED，路径差异）
EXPLICIT_IMPLEMENTED = {
    # ---- weixin-java-channel（源自 audit_channel_ledger SPECIAL_SYMBOLS/UTIL_REUSED）----
    ("weixin-java-channel", "BaseWxChannelMessageService"): (
        "api/wx_channel_message_service.rs",
        "命名差异：Rust 文件去 Base 前缀（Java 接口 42 事件方法全量镜像）",
    ),
    ("weixin-java-channel", "BaseWxChannelService"): (
        "api/wx_channel_service.rs",
        "Java 三层继承链（Impl→HttpComponentsImpl→Base）在 Rust 以 trait 默认实现 + 组合合一，"
        "与 WxChannelService 同文件承载（doc 头注释声明）",
    ),
    ("weixin-java-channel", "BaseWxChannelMessageServiceImpl"): (
        "api/impl/wx_channel_message_service_impl.rs",
        "命名差异：Rust 文件去 Base 前缀；Java 抽象基类（addDefaultRule 39 条）"
        "Rust 以默认规则 + 消费者扩展点承载",
    ),
    ("weixin-java-channel", "MessageEventConstants"): (
        "constant/wx_channel_message_event_constants.rs",
        "命名差异：Rust 文件为 wx_channel_message_event_constants.rs",
    ),
    ("weixin-java-channel", "WxChannelApiUrlConstants"): (
        "enums/url_*.rs（URL 常量按子域拆分 25 文件）",
        "URL 常量按子域拆分至 enums/（url_core/url_order/url_product/url_funds/url_league 等）",
    ),
    ("weixin-java-channel", "WxChannelErrorMsgEnum"): (
        "wx-rust-common/src/error/（wx_channel_error_msg_enum）",
        "跨 crate 迁移：错误码→文案映射迁至 wx-rust-common（find_msg_by_code，"
        "WxType::Channel 分支接入）",
    ),
    # ---- weixin-java-cp（源自 audit_cp_ledger INLINED）----
    ("weixin-java-cp", "WxCpOaOaScheduleServiceImpl"): (
        "api/impl/wx_cp_oa_schedule_service_impl.rs",
        "命名差异：Rust 文件名去重（Java 类名含双 Oa）",
    ),
    ("weixin-java-cp", "WxCpApiPathConsts"): (
        "enums/url_*.rs（URL 常量按子域拆分 23 文件）",
        "URL 常量按子域拆分至 enums/（url_core/url_agent/url_chat/url_corp_group 等）",
    ),
    ("weixin-java-cp", "WxCpConsts"): (
        "constant/wx_cp_constants.rs",
        "命名差异：Rust 文件名为 wx_cp_constants.rs（模块文档注明对应 WxCpConsts 全量静态常量）",
    ),
    ("weixin-java-cp", "WxCpCryptUtil"): (
        "util/crypto/wx_cp_crypt_utils.rs::WxCpCryptUtils",
        "命名差异：Rust 符号名 WxCpCryptUtils（构造/decryptXml/encrypt/decryptPriKey 语义镜像；"
        "消息 XML 加解密由 common WxCryptUtil 承载）",
    ),
    # ---- weixin-java-miniapp（源自 audit_miniapp_ledger）----
    ("weixin-java-miniapp", "WxMaApiUrlConstants"): (
        "enums/url_*.rs（URL 常量按子域拆分）",
        "URL 常量按子域拆分至 enums/（url_core/url_business/g1_urls/g2_urls/g3_urls/g4_urls 等）",
    ),
    # ---- weixin-java-mp ----
    ("weixin-java-mp", "WxMpTemplateData"): (
        "bean/template/wx_mp_template_message.rs::WxMpTemplateData",
        "合并实现：模板消息数据节点与 WxMpTemplateMessage 同文件承载（mod.rs pub use 导出）",
    ),
    ("weixin-java-mp", "WxMpMapConfigImpl"): (
        "config/impl/wx_mp_default_config_impl.rs::WxMpDefaultConfigImpl",
        "Java 变体仅将 access_token 存储换为 ConcurrentHashMap（单机高并发）；"
        "Rust WxMpDefaultConfigImpl 的 Mutex/RwLock 字段原生并发安全，"
        "无 JVM 侧按存储结构分型的需求，语义合一承载",
    ),
    # ---- weixin-java-open（源自 audit_open_ledger SPECIAL_SYMBOLS）----
    ("weixin-java-open", "WxOpenFastMaService"): (
        "api/impl/wx_open_ma_service.rs::WxOpenMaService",
        "Java @Deprecated 接口（2021-06-23 起以 WxOpenMaService 替代），"
        "Rust 统一以 WxOpenMaService 承载，ADAPTED",
    ),
    ("weixin-java-open", "WxOpenFastMaServiceImpl"): (
        "api/impl/wx_open_ma_service.rs::WxOpenMaService",
        "Java @Deprecated（以 WxOpenMaService 替代），Rust 统一承载，ADAPTED",
    ),
    ("weixin-java-open", "WxOpenInMemoryConfigStorage"): (
        "config/impl/wx_open_default_config_impl.rs::WxOpenDefaultConfigImpl",
        "命名差异：Rust 以 WxOpenDefaultConfigImpl（与 mp/ma 的 DefaultConfigImpl 对齐）实现内存存储",
    ),
    ("weixin-java-open", "WxOpenMaServiceImpl"): (
        "api/impl/wx_open_ma_service.rs::WxOpenMaService",
        "代 ma 桥接（Java 继承 WxMaServiceImpl 表达代运营语义，"
        "Rust trait 默认实现 + 组合，ADAPTED）",
    ),
    ("weixin-java-open", "WxOpenMessageRouter"): (
        "api/impl/wx_open_component_service_impl.rs::route",
        "Java WxOpenMessageRouter 覆写 mp 路由为 component 回调；Rust 分发内联于 "
        "component service route()（verify_ticket/authorized/updateauthorized/"
        "notify_third_fasteregister）",
    ),
    ("weixin-java-open", "WxOpenMpServiceImpl"): (
        "api/impl/wx_open_mp_service.rs::WxOpenMpService",
        "代 mp 桥接（Java 继承 WxMpServiceImpl 覆写 token/config，"
        "Rust trait 默认实现 + 组合，ADAPTED）",
    ),
    ("weixin-java-open", "WxOpenServiceAbstractImpl"): (
        "api/impl/base_wx_open_service_impl.rs（执行引擎自由函数）",
        "trait 无法携带泛型方法，执行引擎抽为自由函数（execute_with_retry/"
        "execute_internal/get_component_access_token_with_lock 等）",
    ),
    ("weixin-java-open", "WxOpenCryptUtil"): (
        "util/crypto/wx_open_crypt_utils.rs",
        "路径差异：实际位于 util/crypto/ 且文件名复数（wx_open_crypt_utils.rs）",
    ),
    # ---- weixin-java-common ----
    ("weixin-java-common", "Required"): (
        "annotation/mod.rs::RequiredField",
        "合并实现：Java @Required 运行时注解的 Rust 等价物为 RequiredField 特性"
        "（validate_required 返回缺失字段名），位于 annotation/mod.rs（无单独 required.rs）",
    ),
    ("weixin-java-common", "WxOAuth2ServiceDecorator"): (
        "service/wx_oauth2_service.rs（trait 对象组合）",
        "Java 类为 lombok @Delegate 纯委托装饰器（零自有行为）；Rust 以 "
        "Arc<dyn WxOAuth2Service> 直接组合承载同一替代能力，无继承装饰样板需求",
    ),
    # ---- weixin-java-pay ----
    ("weixin-java-pay", "WxPayServiceImpl"): (
        "api/impl/base_wx_pay_service_impl.rs",
        "Java 继承链 BaseWxPayServiceImpl→HttpComponentsImpl→WxPayServiceImpl 的末层空壳"
        "（WxPayServiceImpl 仅选择 Apache HttpClient 后端）；Rust 以 reqwest 单一后端，"
        "api/impl/base_wx_pay_service_impl.rs 即默认完整实现",
    ),
    ("weixin-java-pay", "GlobalTradeTypeEnum"): (
        "enums/global_trade_type.rs::GlobalTradeTypeEnum",
        "命名差异：Rust 文件去 Enum 后缀，位于 enums/（doc 注释标对应 Java 类）",
    ),
    ("weixin-java-pay", "BaseWxPayRequest"): (
        "bean/request/*.rs（生成器平铺）",
        "Java 抽象基类字段（appid/mch_id/sub_appid/sub_mch_id/nonce_str/sign/sign_type 等）"
        "由 gen_pay_bean_structs.py 平铺进每个请求 bean（serde 字段带 XStreamAlias rename）；"
        "签名/校验语义位于 base_wx_pay_service_impl 与 util/sign_utils",
    ),
    ("weixin-java-pay", "BaseWxPayResult"): (
        "bean/result/*.rs（生成器平铺）",
        "Java 抽象基类字段（return_code/return_msg/result_code/err_code/err_code_des）"
        "平铺进每个结果 bean；fromXML/校验语义由 quick-xml serde + 服务 impl 承载",
    ),
    ("weixin-java-pay", "BaseWxPayV3Result"): (
        "bean/**/*_result.rs（rawJsonString 字段内联）",
        "Java 基类仅持有 rawJsonString；Rust v3 结果 bean（如 favor_stocks_get_result）"
        "内联同名字段（serde rename rawJsonString）",
    ),
    ("weixin-java-pay", "WxPayBaseNotifyV3Result"): (
        "bean/notify/wx_pay_notify_v3_result.rs 等（raw_data + 解密模式内联）",
        "Java 泛型接口（setRawData/setResult）；Rust notify v3 结果 bean 内联 "
        "raw_data: OriginNotifyResponse 字段 + parse/decrypt 关联函数承载同一模式",
    ),
}

# 平台/机制不可达——逐项依据（PLATFORM_NA）
EXPLICIT_PLATFORM_NA = {
    ("weixin-java-pay", "HttpClientBuilderCustomizer"): (
        "Apache HttpClientBuilder 定制钩子；Rust 以 reqwest Client 构建承载"
        "（代理/超时由宿主配置 reqwest，见 config/wx_pay_http_proxy.rs 说明）"
    ),
    ("weixin-java-pay", "SignatureExec"): (
        "Apache HttpClient 请求拦截器（签名注入机构）；Rust 以 reqwest 单一后端承载，"
        "签名内联于请求构造（util/crypto/create_authorization_header 与 "
        "v3/auth/wx_pay_credentials.rs::get_token）"
    ),
    ("weixin-java-pay", "WechatPayUploadHttpPost"): (
        "Apache HttpPost 变体（图片上传 meta 参与签名）；上传语义由服务 impl 上传方法 + "
        "wx-rust-common util::http 执行引擎承载，meta 签名内化于构造请求体处"
    ),
    ("weixin-java-pay", "WxPayV3DownloadHttpGet"): (
        "Apache HttpGet 变体（对账单流式下载）；Rust 由 reqwest execute_stream 流式下载承载"
        "（download_bill_stream，见 tests/download_stream_test.rs）"
    ),
    ("weixin-java-pay", "WxPayV3HttpClientBuilder"): (
        "Apache CloseableHttpClient 构建器（挂载 SignatureExec/Validator）；"
        "Rust 以 reqwest Client + v3 认证族（v3/auth）组合承载"
    ),
    ("weixin-java-pay", "HttpProxyUtils"): (
        "Apache HttpClientBuilder 代理/凭据注入；Rust 代理由宿主配置 reqwest Client"
        "（配置数据由 config/wx_pay_http_proxy.rs::WxPayHttpProxy 承载）"
    ),
    ("weixin-java-pay", "RequestUtils"): (
        "javax.servlet HttpServletRequest 读取工具（V3 回调读体）；Rust 无 Servlet 容器，"
        "回调 body 由宿主 Web 框架直接给出（wx_pay_notify_utils 解析入口）"
    ),
    ("weixin-java-pay", "ResourcesUtils"): (
        "Java classpath 资源加载（jodd ClassUtil 改造）；Rust 以文件路径/std::include_bytes "
        "或宿主资源机制承载，无类加载器链"
    ),
    ("weixin-java-pay", "XmlConfig"): (
        "Java 反射式 XML 序列化的 fastMode 开关（graalvm native 优化）；"
        "Rust quick-xml serde 常态即为无反射实现（等价 fastMode=true），无切换需求"
    ),
    ("weixin-java-pay", "BusinessOperationTransferExample"): (
        "示例/演示代码（example 包，非 SDK 运行时类）；对应业务方法已实现于 "
        "api/business_operation_transfer_service*.rs"
    ),
    ("weixin-java-pay", "NewTransferApiExample"): (
        "示例/演示代码（example 包，非 SDK 运行时类）；对应业务方法已实现于 "
        "api/transfer_service*.rs"
    ),
    ("weixin-java-mp", "MediaImgUploadHttpRequestExecutor"): (
        "Jodd HTTP 客户端专属变体（import jodd.http.*，同族 Apache/HttpComponents/Okhttp "
        "变体均已 PLATFORM_NA）；上传语义由 api/impl material service 的 media_img_upload + "
        "wx-rust-common util::http 执行引擎承载"
    ),
    ("weixin-java-common", "StringManager"): (
        "Apache Tomcat 式 ResourceBundle i18n 消息管理器（util/res 包，Java 资源束机制专属）；"
        "错误文案在 Rust 侧直接内联于各错误类型（WxErrorException/WxPayException 等）"
    ),
}

# 依赖内化——逐项依据（DEPENDENCY_REUSED）
EXPLICIT_DEPENDENCY_REUSED = {
    ("weixin-java-channel", "ChannelWxError"): (
        "wx-rust-common::error::WxError",
        "跨 crate 复用：Java 类为 @Deprecated 的 WxError 子类（无 channel 特有字段），"
        "构造器仅按 WxChannelErrorMsgEnum 翻译中文文案；该语义由 "
        "WxError::from_json_with_type(WxType::Channel)（translate_error_msg 已接入 "
        "wx_channel_error_msg_enum::find_msg_by_code 分支）完整承载",
    ),
    ("weixin-java-pay", "WxPayOrderNotifyResultConverter"): (
        "bean/notify/wx_pay_order_notify_result.rs 的 serde 派生",
        "XStream 反射 converter（couponList 节点展开/收起 + 字段映射）语义内化于 "
        "quick-xml serde 派生与 bean 字段 rename（coupon_list/coupon_type/coupon_id 等）",
    ),
}

# ---------- 证据文案 ----------

EVIDENCE_IMPLEMENTED = "文件存在：预期路径或同名文件在 crate src 树内定位成功"
EVIDENCE_IMPLEMENTED_PATH_DIFF = "文件存在（路径差异）：预期路径 {exp} 不存在，同名文件定位至 {actual}"
EVIDENCE_NA_HTTP = (
    "Java HTTP 客户端专属（Apache HttpClient / HttpComponents / Jodd / OkHttp）；"
    "Rust 以 reqwest 单一后端 + wx-rust-common util::http 执行引擎承载"
)
EVIDENCE_NA_REDIS = (
    "Java 外部存储（Redis / Redisson / Jedis）配置专属；"
    "Rust 以 Config trait + 默认内存配置承载，外部存储接入留待宿主集成"
)
EVIDENCE_REUSED_GSON = (
    "Gson 手写 adapter 线格式已内化于 bean serde rename/from_json 与服务实现"
)
EVIDENCE_REUSED_XSTREAM = (
    "XML 线格式已内化于 message 模块 quick-xml 解析与 out 消息序列化"
)
EVIDENCE_REUSED_JSON_XML = (
    "线格式/解码语义由 serde_json / quick-xml / 执行引擎内化"
)
EVIDENCE_NA_SESSION = (
    "Java Servlet Session 管理模型专属；Rust 以 wx_session / wx_session_manager trait 承载，"
    "无需 InternalSession/Constants/TooManyActiveSessionsException 等容器类"
)
EVIDENCE_NA_BASE_EXECUTOR = (
    "Java 请求执行器抽象基类；Rust 以 wx-rust-common util::http::request_executor trait + "
    "reqwest 单一后端承载，无需多 HTTP 客户端变体"
)
EVIDENCE_NA_ABSTRACT = (
    "Java 抽象基类（继承链）；Rust 以 trait 默认实现 / 组合模式承载"
)
EVIDENCE_MISSING = "未实现：crate src 树内无对应 .rs 文件"
EVIDENCE_NA_PACKAGE_INFO = (
    "Java 包级文档文件（package-info.java，仅承载 javadoc/包注解，无运行时类）；"
    "Rust 以模块 mod.rs 的模块级 doc 注释承载同一信息"
)


def is_http_backend(java_name, java_file):
    """判定是否为 Java HTTP 后端实现（类名 + 文件路径双重检测）。"""
    # 1) 明确的 HTTP 后端实现后缀
    if any(java_name.endswith(sfx) for sfx in HTTP_BACKEND_SUFFIXES):
        return True
    # 2) 文件路径位于 HTTP 客户端特定目录
    java_file_lower = java_file.lower().replace("\\", "/")
    if any(d in java_file_lower for d in HTTP_CLIENT_DIRS):
        return True
    # 3) 类名以 HTTP 客户端前缀开头 + 以特定后缀结尾（executor/handler/builder/proxy）
    java_name_lower = java_name.lower()
    if any(java_name_lower.startswith(pfx.lower()) for pfx in HTTP_CLIENT_PREFIXES):
        if any(java_name.endswith(sfx) for sfx in HTTP_CLIENT_SUFFIXES):
            return True
    # 4) 类名包含 HTTP 客户端关键字 + 特定后缀（大小写不敏感）
    http_kws_lower = ("apache", "httpcomponents", "jodd", "okhttp")
    if any(kw in java_name_lower for kw in http_kws_lower):
        if any(java_name.endswith(sfx) for sfx in HTTP_CLIENT_SUFFIXES):
            return True
    # 5) 文件路径在 requestexecuter/ 目录下的 Okhttp/Apache/HttpComponents/Jodd 变体
    if "/requestexecuter/" in java_file_lower or "/requestexecutor/" in java_file_lower:
        if any(kw in java_name_lower for kw in http_kws_lower):
            return True
    return False


def is_java_platform_specific(java_name, java_file, kind):
    """判定是否为 Java 平台特定类（session 管理、基础 executor 抽象等）。"""
    # Session 管理类（Java servlet session 模型，Rust 不需要）
    session_classes = {
        "Constants", "InternalSession", "InternalSessionManager",
        "StandardSessionFacade", "TooManyActiveSessionsException",
        "StringManager",
    }
    if java_name in session_classes and "session" in java_file.lower():
        return True

    # 基础 executor 抽象（非 HTTP 客户端特定，但属于 Java 请求执行模型）
    base_executor_classes = {
        "CommonUploadRequestExecutor",
        "CommonUploadRequestExecutorApacheImpl",
        "OcrDiscernRequestExecutor",
        "BaseMediaDownloadRequestExecutor",
        "MediaInputStreamUploadRequestExecutor",
        "MinishopUploadRequestExecutor",
        "MinishopUploadRequestCustomizeExecutor",
        "ResponseHandler",
        "ApiSignaturePostRequestExecutor",
        "QrcodeBytesRequestExecutor",
        "QrcodeRequestExecutor",
        "UploadAuthMaterialRequestExecutor",
        "VodSingleUploadRequestExecutor",
        "VodUploadPartRequestExecutor",
        "CommonUploadMultiRequestExecutor",
        "CommonUploadMultiRequestExecutorApacheImpl",
        "GenericUploadRequestExecutor",
        "MaQrCodeRequestExecutor",
        "ChannelFileUploadRequestExecutor",
        "ChannelMediaDownloadRequestExecutor",
        "MaterialDeleteRequestExecutor",
        "MaterialNewsInfoRequestExecutor",
        "MaterialUploadRequestExecutor",
        "MaterialVideoInfoRequestExecutor",
        "MaterialVoiceAndImageDownloadRequestExecutor",
        "MediaImgUploadRequestExecutor",
        "QrCodeRequestExecutor",
        "VoiceUploadRequestExecutor",
    }
    if java_name in base_executor_classes:
        return True

    # Abstract 抽象类（Java 继承链基类，Rust 以 trait 承载）
    if java_name.startswith("Abstract") and kind == "class":
        return True

    return False


def is_redis_config(java_name, java_file):
    """判定是否为 Java Redis/外部存储配置实现。"""
    name_lower = java_name.lower()
    file_lower = java_file.lower().replace("\\", "/")
    # 类名或文件路径包含 Redis/Redisson/Jedis 关键字
    redis_kws = ("redis", "redisson", "jedis")
    if any(kw in name_lower for kw in redis_kws):
        return True
    if any(kw in file_lower for kw in redis_kws):
        # 文件路径在 redis/ 目录下且类名含 Config/Storage/Holder
        if any(sfx in java_name for sfx in ("Config", "Storage", "Holder", "Ops")):
            return True
    return False


def is_gson_related(java_name, java_file):
    """判定是否为 Gson builder / adapter / helper / parser。"""
    if any(kw in java_name for kw in GSON_KEYWORDS):
        return True
    # 文件路径在 json/ 或 util/json/ 目录下且类名含 Adapter/Gson
    file_lower = java_file.lower().replace("\\", "/")
    if "/json/" in file_lower and ("adapter" in java_name.lower() or "gson" in java_name.lower()):
        return True
    return False


def is_xstream(java_name, java_file):
    """判定是否为 XStream transformer / converter / initializer。"""
    if any(kw in java_name for kw in XSTREAM_KEYWORDS):
        return True
    # 文件路径在 xml/ 目录下且类名含 Converter/Transformer
    file_lower = java_file.lower().replace("\\", "/")
    if "/xml/" in file_lower and any(kw in java_name for kw in ("Converter", "Transformer", "Initializer")):
        return True
    return False


def is_json_xml_util(java_name):
    """判定是否为 JSON/XML 工具类。"""
    return java_name in JSON_XML_UTIL_NAMES


def normalize_compound_words(java_name):
    """将 Java 类名中的复合词规范化为 Rust 文件名等价形式。

    例如 WxOAuth2Service -> [wx_cp_oauth2_service 形态候选]
    返回多个候选 snake_case 文件名（用于模糊匹配）。
    """
    import re
    # 基础转换：驼峰 -> snake_case
    base = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", java_name)
    base = re.sub(r"(?<=[A-Z])(?=[A-Z][a-z])", "_", base)
    base = base.lower().replace("__", "_")

    candidates = [base]

    def snake_of(word):
        s = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", word)
        s = re.sub(r"(?<=[A-Z])(?=[A-Z][a-z])", "_", s)
        return s.lower()

    # 对每个复合词规范化，生成额外候选
    for compound, normalized in COMPOUND_WORD_NORMALIZATIONS.items():
        if compound.lower() in java_name.lower():
            # 原始拆分形式（如 o_auth2）
            # 规范化形式（如 oauth2）
            alt = base.replace(compound.lower().replace("_", ""), normalized)
            if alt != base:
                candidates.append(alt)
            # 也尝试将规范化形式还原为拆分形式
            alt2 = base.replace(normalized, compound.lower())
            if alt2 != base and alt2 not in candidates:
                candidates.append(alt2)
            # 拆分形式 -> 规范化形式（Task 12 修复）：
            # 基础转换把 OAuth2 拆成 o_auth2、XPay 拆成 x_pay，而实际 Rust
            # 文件名为连续拼写（wx_cp_oauth2_service.rs / wx_ma_xpay_service.rs），
            # 此前缺该方向导致明明存在的文件被判 MISSING
            split_form = snake_of(compound)
            alt3 = base.replace(split_form, normalized)
            if alt3 != base and alt3 not in candidates:
                candidates.append(alt3)

    return candidates


def classify(java_name, java_file, rust_path, found_paths, extra_found_paths, kind="?", module="?"):
    """逐行处置。返回 (status, actual_path, evidence)。

    found_paths: 按预期文件名精确匹配的路径列表
    extra_found_paths: 按复合词规范化模糊匹配的路径列表
    """
    # 1) 文件存在性优先（精确路径匹配）
    if rust_path in found_paths:
        return "IMPLEMENTED", rust_path, EVIDENCE_IMPLEMENTED

    # 2) 文件名在树内唯一定位（路径差异容忍）
    if found_paths:
        actual = found_paths[0]
        note = EVIDENCE_IMPLEMENTED_PATH_DIFF.format(exp=rust_path, actual=actual)
        if len(found_paths) > 1:
            note += f"（同名文件多处命中 {found_paths}，需人工复核）"
        return "IMPLEMENTED", actual, note

    # 2b) 复合词规范化模糊匹配（如 OAuth2 vs O_Auth2）
    if extra_found_paths:
        actual = extra_found_paths[0]
        note = EVIDENCE_IMPLEMENTED_PATH_DIFF.format(exp=rust_path, actual=actual)
        note += "（复合词命名差异：Java camelCase 拆分与 Rust 文件名规范化不同）"
        if len(extra_found_paths) > 1:
            note += f"（多处命中 {extra_found_paths}，需人工复核）"
        return "IMPLEMENTED", actual, note

    # 2c) Java 包文档文件（package-info.java 仅承载包级 javadoc/注解，
    #     CSV 行的 java_name/kind/rust_path 均为 "?"）
    if java_file.replace("\\", "/").endswith("package-info.java"):
        return "PLATFORM_NA", rust_path, EVIDENCE_NA_PACKAGE_INFO

    # 2d) 逐项显式处置表（Task 12：改名/合并/跨 crate/生成器平铺的已实现项）
    explicit_impl = EXPLICIT_IMPLEMENTED.get((module, java_name))
    if explicit_impl:
        actual, reason = explicit_impl
        return "IMPLEMENTED", actual, f"已实现（符号改名/合并/跨 crate/平铺）：{reason}"

    explicit_reused = EXPLICIT_DEPENDENCY_REUSED.get((module, java_name))
    if explicit_reused:
        actual, reason = explicit_reused
        return "DEPENDENCY_REUSED", actual, f"依赖内化：{reason}"

    explicit_na = EXPLICIT_PLATFORM_NA.get((module, java_name))
    if explicit_na:
        return "PLATFORM_NA", rust_path, f"平台/机制不可达：{explicit_na}"

    # 3) PLATFORM_NA 归类
    if is_http_backend(java_name, java_file):
        return "PLATFORM_NA", rust_path, EVIDENCE_NA_HTTP
    if is_redis_config(java_name, java_file):
        return "PLATFORM_NA", rust_path, EVIDENCE_NA_REDIS
    if is_java_platform_specific(java_name, java_file, kind):
        # 根据具体类型返回不同证据
        session_classes = {
            "Constants", "InternalSession", "InternalSessionManager",
            "StandardSessionFacade", "TooManyActiveSessionsException",
            "StringManager",
        }
        if java_name in session_classes:
            return "PLATFORM_NA", rust_path, EVIDENCE_NA_SESSION
        if java_name.startswith("Abstract"):
            return "PLATFORM_NA", rust_path, EVIDENCE_NA_ABSTRACT
        return "PLATFORM_NA", rust_path, EVIDENCE_NA_BASE_EXECUTOR

    # 4) DEPENDENCY_REUSED 归类
    if is_gson_related(java_name, java_file):
        return "DEPENDENCY_REUSED", rust_path, EVIDENCE_REUSED_GSON
    if is_xstream(java_name, java_file):
        return "DEPENDENCY_REUSED", rust_path, EVIDENCE_REUSED_XSTREAM
    if is_json_xml_util(java_name):
        return "DEPENDENCY_REUSED", rust_path, EVIDENCE_REUSED_JSON_XML

    # 5) MISSING
    return "MISSING", rust_path, EVIDENCE_MISSING


def build_crate_index(crate_src):
    """构建文件名（不含 mod.rs）-> 相对路径列表 的索引。

    同时构建一个规范化文件名索引，用于复合词模糊匹配。
    """
    index = {}
    normalized_index = {}  # 规范化文件名 -> 相对路径列表
    if not os.path.isdir(crate_src):
        return index, normalized_index
    for dirpath, _dirnames, filenames in os.walk(crate_src):
        for fn in filenames:
            if fn.endswith(".rs") and fn != "mod.rs":
                rel = os.path.relpath(os.path.join(dirpath, fn), crate_src)
                index.setdefault(fn, []).append(rel)
                # 规范化文件名（去掉 .rs 后缀，统一为小写）
                norm = fn[:-3].lower().replace("-", "_")
                normalized_index.setdefault(norm, []).append(rel)
    return index, normalized_index


def find_in_index(index, normalized_index, rust_path, java_name):
    """按文件名在索引中查找，返回 (精确匹配列表, 模糊匹配列表)。"""
    basename = os.path.basename(rust_path)
    exact = index.get(basename, [])

    # 模糊匹配：按复合词规范化候选查找
    if not exact:
        candidates = normalize_compound_words(java_name)
        fuzzy = []
        for candidate in candidates:
            candidate_file = candidate + ".rs"
            if candidate_file in index:
                fuzzy.extend(index[candidate_file])
            # 也在规范化索引中查找
            if candidate in normalized_index:
                fuzzy.extend(normalized_index[candidate])
        # 去重
        fuzzy = list(dict.fromkeys(fuzzy))
        return exact, fuzzy

    return exact, []


def main():
    parser = argparse.ArgumentParser(description="V0 静态结构审计：inventory CSV vs crates 文件树")
    parser.add_argument("--verbose", action="store_true", help="打印逐行分类明细")
    parser.add_argument("--csv", default=CSV_PATH, help="inventory CSV 路径")
    args = parser.parse_args()

    csv_path = args.csv
    if not os.path.isfile(csv_path):
        print(f"[fatal] CSV 文件不存在：{csv_path}", file=sys.stderr)
        return 1

    # ---------- 读取 CSV ----------
    rows = []
    with open(csv_path, encoding="utf-8", newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            rows.append(row)
    print(f"读取 CSV：{len(rows)} 行（含表头 {reader.fieldnames}）")

    # ---------- 构建各 crate 文件索引 ----------
    crate_indices = {}
    crate_norm_indices = {}
    for module, crate_name in MODULE_TO_CRATE.items():
        crate_src = os.path.join(BASE, "crates", crate_name, "src")
        idx, norm_idx = build_crate_index(crate_src)
        crate_indices[module] = idx
        crate_norm_indices[module] = norm_idx

    # ---------- 逐行审计 ----------
    stats = Counter()
    per_module = defaultdict(lambda: Counter())
    per_kind = defaultdict(lambda: Counter())
    missing_rows = defaultdict(list)  # module -> [(java_name, kind, rust_path)]
    unexpected_modules = Counter()

    results = []  # (module, java_name, kind, rust_path, status, actual_path, evidence)

    for row in rows:
        module = row["module"]
        java_file = row["java_file"]
        kind = row["kind"]
        java_name = row["java_name"]
        rust_path = row["rust_path"]

        # 检查模块映射
        if module not in MODULE_TO_CRATE:
            unexpected_modules[module] += 1
            stats["UNEXPECTED_MODULE"] += 1
            results.append((module, java_name, kind, rust_path, "UNEXPECTED_MODULE", "", f"未知模块：{module}"))
            continue

        # 在对应 crate 索引中查找
        index = crate_indices[module]
        norm_index = crate_norm_indices[module]
        found, extra_found = find_in_index(index, norm_index, rust_path, java_name)

        # 精确路径检查
        crate_src = os.path.join(BASE, "crates", MODULE_TO_CRATE[module], "src")
        exact_exists = os.path.isfile(os.path.join(crate_src, rust_path))

        # 如果精确路径存在，加入 found 列表（确保 classify 优先匹配精确路径）
        if exact_exists and rust_path not in found:
            found.insert(0, rust_path)

        status, actual, evidence = classify(
            java_name, java_file, rust_path, found, extra_found, kind, module
        )

        stats[status] += 1
        per_module[module][status] += 1
        per_kind[kind][status] += 1

        if status == "MISSING":
            missing_rows[module].append((java_name, kind, rust_path))

        results.append((module, java_name, kind, rust_path, status, actual, evidence))

        if args.verbose:
            marker = {"IMPLEMENTED": "OK", "PLATFORM_NA": "NA", "DEPENDENCY_REUSED": "RE", "MISSING": "XX"}
            print(f"  [{marker.get(status, '??')}] {module}::{java_name} ({kind}) -> {rust_path} | {status}")

    # ---------- 汇总输出 ----------
    total = len(rows)
    print()
    print("=" * 72)
    print("V0 静态结构审计汇总")
    print("=" * 72)
    print(f"总行数：{total}")
    print()

    # 分类统计
    print("## 分类统计")
    print(f"| 分类 | 数量 | 占比 |")
    print(f"|---|---:|---:|")
    for cat in ("IMPLEMENTED", "PLATFORM_NA", "DEPENDENCY_REUSED", "MISSING", "UNEXPECTED_MODULE"):
        cnt = stats.get(cat, 0)
        pct = f"{cnt / total * 100:.1f}%" if total > 0 else "0%"
        print(f"| {cat} | {cnt} | {pct} |")
    handled = stats["IMPLEMENTED"] + stats.get("PLATFORM_NA", 0) + stats.get("DEPENDENCY_REUSED", 0)
    print(f"| 合计已处置 | {handled} | {handled / total * 100:.1f}% |")
    print()

    # 模块明细
    print("## 模块明细")
    print(f"| 模块 | crate | 总行数 | IMPLEMENTED | PLATFORM_NA | DEP_REUSED | MISSING |")
    print(f"|---|---|---:|---:|---:|---:|---:|")
    for module in sorted(MODULE_TO_CRATE.keys()):
        crate = MODULE_TO_CRATE[module]
        m = per_module[module]
        t = sum(m.values())
        if t == 0:
            continue
        print(f"| {module} | {crate} | {t} | {m.get('IMPLEMENTED', 0)} | {m.get('PLATFORM_NA', 0)} | {m.get('DEPENDENCY_REUSED', 0)} | {m.get('MISSING', 0)} |")
    # 未知模块
    if unexpected_modules:
        for mod, cnt in unexpected_modules.items():
            print(f"| {mod} | (无映射) | {cnt} | 0 | 0 | 0 | 0 |")
    print()

    # 类型明细
    print("## 类型明细")
    print(f"| kind | 总行数 | IMPLEMENTED | PLATFORM_NA | DEP_REUSED | MISSING |")
    print(f"|---|---:|---:|---:|---:|---:|")
    for kind in sorted(per_kind.keys()):
        k = per_kind[kind]
        t = sum(k.values())
        print(f"| {kind} | {t} | {k.get('IMPLEMENTED', 0)} | {k.get('PLATFORM_NA', 0)} | {k.get('DEPENDENCY_REUSED', 0)} | {k.get('MISSING', 0)} |")
    print()

    # MISSING 清单
    total_missing = sum(len(v) for v in missing_rows.values())
    print(f"## MISSING 清单（共 {total_missing} 项）")
    if total_missing == 0:
        print("无 MISSING 项。")
    else:
        for module in sorted(missing_rows.keys()):
            items = missing_rows[module]
            crate = MODULE_TO_CRATE[module]
            print(f"\n### {module} ({crate}) — {len(items)} 项 MISSING")
            print(f"| # | Java 名称 | kind | 预期 Rust 路径 |")
            print(f"|---:|---|---|---|")
            for i, (jname, jkind, rpath) in enumerate(items, 1):
                print(f"| {i} | `{jname}` | {jkind} | `{rpath}` |")
    print()

    # 结论
    impl_rate = (stats["IMPLEMENTED"] + stats.get("PLATFORM_NA", 0) + stats.get("DEPENDENCY_REUSED", 0)) / total * 100 if total > 0 else 0
    print("=" * 72)
    print(f"结论：{total} 对象中 {handled} 已处置（{impl_rate:.1f}%），{stats.get('MISSING', 0)} MISSING。")
    if stats.get("MISSING", 0) == 0:
        print("MISSING 清零，静态结构审计通过。")
    else:
        print(f"MISSING {stats['MISSING']} 项需补齐后方可通过静态结构审计。")
    print("=" * 72)

    return 0


if __name__ == "__main__":
    sys.exit(main())
