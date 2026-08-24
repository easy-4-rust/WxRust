# V0 迁移缺口清零验证（Task 12）

日期：2026-08-24
审计脚本：`scripts/audit_migration_layout.py`（逐行核对 `docs/superpowers/data/inventory_java_objects.csv` 3287 对象与 `crates/*/src` 文件树）

## 终态数字

| 指标 | 上次审计（2026-08-23） | 本次终态 |
|---|---:|---:|
| CSV 总对象 | 3287 | 3287 |
| IMPLEMENTED | 2911（88.6%） | 2966（90.2%） |
| PLATFORM_NA | 216 | 231 |
| DEPENDENCY_REUSED | 88 | 90 |
| **MISSING** | **72** | **0** |
| 合计已处置 | 3215（97.8%） | **3287（100%）** |

模块明细（终态）：channel 618 / cp 594 / miniapp 611 / mp 428 / open 240 / pay 570 / common 174 / qidian 27 / aispeech 25，全部 MISSING=0。

72 项缺口处置统计：**新增实现 21（全部 pay）· 既有实现映射修正 34（显式表 29 + 复合词匹配修复 5：改名/合并/跨 crate/生成器平铺）· PLATFORM_NA 15（Apache HttpClient/JVM 机制/示例代码/包文档）· DEPENDENCY_REUSED 2**。

回归：`cargo test --workspace` 全绿 **1968**（基线 1942，+26：pay v3/config/exception 新增单测）；`cargo clippy --workspace --all-targets -- -D warnings` 干净；`cargo fmt --all` 无 diff。

## 一、pay 39 项

### 1.1 新增实现（21 项，文件落在预期路径）

新建 `crates/wx-rust-pay/src/v3/` 模块（对应 Java `com.github.binarywang.wxpay.v3` 包，按 Java 目录镜像；加密/验签引擎复用既有 `util/crypto/` 纯函数，零重复）与 config/exception/util 补齐：

| # | Java 类 | Rust 路径 | 实现范围 |
|---:|---|---|---|
| 1 | `v3/auth/Signer`（接口+SignatureResult） | `v3/auth/signer.rs` | trait + 结果类型（Java unchecked 异常 → `Result<_, WxV3CryptoError>`） |
| 2 | `v3/auth/Verifier` | `v3/auth/verifier.rs` | trait（verify/get_valid_certificate/set_other_verifier 默认空实现）+ `WxPayValidCertificate` 枚举承载 Java `X509Certificate` 双形态多态 |
| 3 | `v3/auth/PrivateKeySigner` | `v3/auth/private_key_signer.rs` | SHA256withRSA 签名 + 序列号绑定（复用 `sign_sha256_rsa`）；随机密钥往返测试 |
| 4 | `v3/auth/CertificatesVerifier` | `v3/auth/certificates_verifier.rs` | Java 命名镜像，实现 `Verifier`，引擎委托 `util::crypto::WxPayCertificatesVerifier` |
| 5 | `v3/auth/AutoUpdateCertificatesVerifier` | `v3/auth/auto_update_certificates_verifier.rs` | Java 构造器参数形态（credentials+apiV3Key+间隔）+ `TimeInterval` 枚举；引擎委托既有 `WxPayAutoUpdateCertificatesVerifier`；ADAPTED：构造不联网，下载延迟到 `check_and_auto_update`（异步） |
| 6 | `v3/auth/PublicCertificateVerifier` | `v3/auth/public_certificate_verifier.rs` | 公钥模式验签：序列号不含 `PUB_KEY_ID` 先走兜底证书验证器、失败公钥兜底（Java 语义逐行对齐）；往返+兜底测试 |
| 7 | `v3/auth/X509PublicCertificate` | `v3/auth/x509_public_certificate.rs` | 公钥+publicId 承载；`serial_number_id` 去 `PUB_KEY_ID_` 前缀（对应 Java BigInteger(hex)）；checkValidity 恒 Ok |
| 8 | `v3/auth/WxPayCredentials` | `v3/auth/wx_pay_credentials.rs` | getSchema（WECHATPAY2-SHA256-RSA2048）/getToken 五元组/buildMessage（METHOD\ncanonical_url\nts\nnonce\nbody\n）/setSignUriStripPrefix 规范化/stripPathPrefix；token 格式与前缀剥离测试 |
| 9 | `v3/auth/WxPayValidator` | `v3/auth/wx_pay_validator.rs` | Content-Type 非 JSON 放行、四头缺失 false、`timestamp\nnonce\nbody\n` 验签；非 JSON/缺头/篡改/公钥验证器组合测试 |
| 10 | `v3/Credentials`（接口） | `v3/credentials.rs` | trait + `CredentialsRequest` 值对象（ADAPTED：Java 以 Apache `HttpRequestWrapper` 为参 → method/path/query/body） |
| 11 | `v3/Validator`（接口） | `v3/validator.rs` | trait + `ValidationResponse`（ADAPTED：Java 以 Apache `CloseableHttpResponse` 为参 → 五头+body） |
| 12 | `v3/SpecEncrypt`（注解） | `v3/spec_encrypt.rs` | `SpecEncrypt` 标记 trait + `spec_encrypt_fields` 约定（ADAPTED：Java 注解/反射加密 → Rust 调用侧显式加密） |
| 13 | `v3/util/AesUtils` | `v3/util/aes_utils.rs` | `encrypt`/`decrypt_to_string`/`decrypt_to_bytes` Java 命名镜像（复用 `aes_gcm_*`）；往返测试 |
| 14 | `v3/util/PemUtils` | `v3/util/pem_utils.rs` | `load_private_key`/`load_public_key`/`load_certificate`（复用 `wx_pay_cert_utils`） |
| 15 | `v3/util/RsaCryptoUtil` | `v3/util/rsa_crypto_util.rs` | `encrypt_oaep`/`decrypt_oaep`（SHA-1 OAEP，复用 `rsa_oaep_*`）；往返测试；`encryptFields` 反射字段加密按 ADAPTED 以显式调用承载 |
| 16 | `config/VerifierBuilder` | `config/verifier_builder.rs` | `build_verifier`：证书模式（优先）→ 公钥模式（setOtherVerifier 注入兜底）→ None；`build_public_cert_verifier`；payBaseUrl rawPath 前缀提取；离线测试 5 项 |
| 17 | `config/WxPayConfigHolder` | `config/wx_pay_config_holder.rs` | thread_local 标签（get/set/remove，remove 重置 "default" 对应 Java ThreadLocal.remove 回落初始值）；线程隔离测试 |
| 18 | `config/WxPayHttpProxy` | `config/wx_pay_http_proxy.rs` | 四字段 DTO（serde rename 驼峰）+ `is_effective()`（对应 Java `initHttpProxy` 的 host/port 判断） |
| 19 | `exception/WxPayException` | `exception/wx_pay_exception.rs` | 七字段 + Builder（fluent 六 setter）+ `build_error_msg`「，」拼装（逐片段与 Java 一致）+ `from_base_result_fields`（对应 `from(BaseWxPayResult)` 含 errorCode/errorMessage 覆盖分支）+ `From → WxErrorException::Runtime` |
| 20 | `exception/WxSignTestException` | `exception/wx_sign_test_exception.rs` | 携带 `WxPayException` 的子类形态（`From` 向上转型），对应 Java 仅两构造器的空子类 |
| 21 | `util/ZipUtils` | `util/zip_utils.rs` | `un_gzip_file`（去扩展名，对应 `FilenameUtils.removeExtension` 仅剥文件名段内扩展名）+ `gunzip_bytes`；flate2 往返测试 |

另附引擎补齐 1 项（既有文件增强，非新增文件、不计入 72 项缺口）：`WxPayAutoUpdateCertificatesVerifier`（`util/crypto/wx_pay_cert_verifier.rs`）补 `minutes_interval()` getter（供 v3 镜像层暴露 Java 字段）。

### 1.2 既有实现映射修正（6 项 IMPLEMENTED + 1 项 DEPENDENCY_REUSED）

| Java 类 | 当前 Rust 承载 | 说明 |
|---|---|---|
| `service/impl/WxPayServiceImpl` | `api/impl/base_wx_pay_service_impl.rs` | Java 继承链末层空壳（仅选择 Apache 后端）；Rust reqwest 单一后端，base impl 即默认完整实现 |
| `bean/result/enums/GlobalTradeTypeEnum` | `enums/global_trade_type.rs::GlobalTradeTypeEnum` | 命名差异：去 Enum 后缀、位于 enums/（doc 注释标对应） |
| `bean/request/BaseWxPayRequest` | `bean/request/*.rs`（生成器平铺） | 基类字段（appid/mch_id/sub_appid/sub_mch_id/nonce_str/sign/sign_type）由 `gen_pay_bean_structs.py` 平铺进每个请求 bean；签名语义在 base_wx_pay_service_impl/util::sign_utils |
| `bean/result/BaseWxPayResult` | `bean/result/*.rs`（平铺） | return_code/return_msg/result_code/err_code/err_code_des 平铺；fromXML 由 quick-xml serde 承载 |
| `bean/result/BaseWxPayV3Result` | `bean/**/*_result.rs` | 基类仅持 rawJsonString；v3 结果 bean 内联同名字段（serde rename） |
| `bean/notify/WxPayBaseNotifyV3Result` | `bean/notify/wx_pay_notify_v3_result.rs` 等 | 泛型接口（setRawData/setResult）→ raw_data 字段 + parse/decrypt 关联函数模式 |
| `converter/WxPayOrderNotifyResultConverter` | `bean/notify/wx_pay_order_notify_result.rs` serde 派生 | **DEPENDENCY_REUSED**：XStream converter（couponList 节点展开）内化于 quick-xml serde 派生 |

### 1.3 PLATFORM_NA（11 项，逐项依据）

| Java 类 | 依据 |
|---|---|
| `config/HttpClientBuilderCustomizer` | Apache HttpClientBuilder 定制钩子；Rust 以 reqwest Client 构建承载 |
| `v3/SignatureExec` | Apache HttpClient 请求拦截器；签名内联于请求构造（`create_authorization_header` / `WxPayCredentials::get_token`） |
| `v3/WechatPayUploadHttpPost` | Apache HttpPost 变体（上传 meta 签名）；上传由服务 impl + common 执行引擎承载 |
| `v3/WxPayV3DownloadHttpGet` | Apache HttpGet 变体；Rust 由 reqwest 流式下载承载（`download_bill_stream`） |
| `v3/WxPayV3HttpClientBuilder` | Apache CloseableHttpClient 构建器；reqwest Client + v3 认证族组合承载 |
| `util/HttpProxyUtils` | Apache 代理/凭据注入；代理由宿主配置 reqwest（数据由 `WxPayHttpProxy` 承载） |
| `util/RequestUtils` | `javax.servlet HttpServletRequest` 读取；Rust 无 Servlet 容器，回调 body 由宿主 Web 框架给出 |
| `util/ResourcesUtils` | Java classpath 资源加载（jodd ClassUtil）；Rust 以文件路径/宿主资源机制承载 |
| `util/XmlConfig` | Java 反射 XML fastMode 开关（graalvm 优化）；quick-xml serde 常态即无反射（等价 fastMode=true） |
| `example/BusinessOperationTransferExample` | 示例/演示代码（非 SDK 运行时类）；业务方法已实现于 `api/business_operation_transfer_service*.rs` |
| `example/NewTransferApiExample` | 同上；业务方法已实现于 `api/transfer_service*.rs` |

## 二、open 8 项（全部既有实现映射修正）

沿用 `audit_open_ledger.py` SPECIAL_SYMBOLS 归类（逐项核对文件存在）：

| Java 类 | 当前 Rust 承载 | 说明 |
|---|---|---|
| `WxOpenInMemoryConfigStorage` | `config/impl/wx_open_default_config_impl.rs::WxOpenDefaultConfig` | 命名对齐 mp/ma DefaultConfig 家族 |
| `WxOpenServiceAbstractImpl` | `api/impl/base_wx_open_service_impl.rs` | 执行引擎自由函数（trait 无法携带泛型方法） |
| `WxOpenMaServiceImpl` | `api/impl/wx_open_ma_service.rs::WxOpenMaService` | 代 ma 桥接（trait 默认实现+组合，ADAPTED） |
| `WxOpenFastMaService` / `WxOpenFastMaServiceImpl` | 同上 | Java @Deprecated，统一以 WxOpenMaService 承载 |
| `WxOpenMpServiceImpl` | `api/impl/wx_open_mp_service.rs::WxOpenMpService` | 代 mp 桥接 |
| `WxOpenMessageRouter` | `api/impl/wx_open_component_service_impl.rs::route` | component 回调分发内联（verify_ticket/authorized/updateauthorized/notify_third_fasteregister） |
| `WxOpenCryptUtil` | `util/crypto/wx_open_crypt_utils.rs` | 路径/文件名复数差异 |

## 三、channel 7 项 / cp 6 项 / miniapp 5 项 / mp 3 项 / common 4 项

### channel（7：5 映射 + 1 跨 crate + 1 内化）
- `BaseWxChannelMessageService` → `api/wx_channel_message_service.rs`（去 Base 前缀，42 事件方法全量镜像）
- `BaseWxChannelService` → `api/wx_channel_service.rs`（三层继承链合一）
- `BaseWxChannelMessageServiceImpl` → `api/impl/wx_channel_message_service_impl.rs`
- `MessageEventConstants` → `constant/wx_channel_message_event_constants.rs`
- `WxChannelApiUrlConstants` → `enums/url_*.rs`（按子域拆分 25 文件）
- `WxChannelErrorMsgEnum` → `wx-rust-common/src/error/wx_channel_error_msg_enum`（跨 crate，find_msg_by_code）
- `ChannelWxError` → **DEPENDENCY_REUSED** `wx-rust-common::error::WxError`（@Deprecated 空子类，文案翻译由 `from_json_with_type(WxType::Channel)` 承载）

### cp（6：复合词匹配修复 2 + 映射 4）
- `WxCpOAuth2Service` / `WxCpOAuth2ServiceImpl` → `api/wx_cp_oauth2_service.rs` / `api/impl/wx_cp_oauth2_service_impl.rs`（既有文件；审计脚本复合词「拆分形式→连续拼写」方向缺失导致误报，已修复）
- `WxCpOaOaScheduleServiceImpl` → `api/impl/wx_cp_oa_schedule_service_impl.rs`（文件名去重）
- `WxCpApiPathConsts` → `enums/url_*.rs`（23 文件）
- `WxCpConsts` → `constant/wx_cp_constants.rs`
- `WxCpCryptUtil` → `util/crypto/wx_cp_crypt_utils.rs::WxCpCryptUtils`

### miniapp（5：复合词修复 2 + package-info 2 + 映射 1）
- `WxMaXPayService` / `WxMaXPayServiceImpl` → `api/wx_ma_xpay_service.rs` / `api/impl/wx_ma_xpay_service_impl.rs`（复合词修复）
- `bean/analysis/package-info.java`、`bean/code/package-info.java` → **PLATFORM_NA**（Java 包级文档文件；Rust 以 mod.rs 模块 doc 承载）
- `WxMaApiUrlConstants` → `enums/url_*.rs`（url_core/url_business/g1-g4_urls 等）

### mp（3）
- `WxMpTemplateData` → `bean/template/wx_mp_template_message.rs::WxMpTemplateData`（合并实现，mod.rs 导出）
- `WxMpMapConfigImpl` → `config/impl/wx_mp_default_config_impl.rs`（Java 变体仅将 token 存储换 ConcurrentHashMap；Rust WxMpDefaultConfig 的 Mutex/RwLock 原生并发安全，语义合一）
- `MediaImgUploadHttpRequestExecutor` → **PLATFORM_NA**（Jodd HTTP 变体，同族 Apache/HttpComponents/Okhttp 均已 NA；上传语义由 material service `media_img_upload` + common 执行引擎承载）

### common（4）
- `Required` → `annotation/mod.rs::RequiredField`（合并实现：注解 → `validate_required` 特性）
- `WxOAuth2Service` → `service/wx_oauth2_service.rs`（既有文件，复合词修复）
- `WxOAuth2ServiceDecorator` → `service/wx_oauth2_service.rs`（trait 对象组合；Java 类为 lombok @Delegate 纯委托样板，零自有行为）
- `StringManager` → **PLATFORM_NA**（Tomcat 式 ResourceBundle i18n 管理器；Rust 错误文案直接内联于错误类型）

## 四、审计脚本修正（`scripts/audit_migration_layout.py`）

1. `normalize_compound_words` 补「拆分形式→连续拼写」候选方向（`o_auth2→oauth2`、`x_pay→xpay`）：修复 5 项既有文件误报（cp 2、common 1、miniapp 2）；
2. `package-info.java` 行归 PLATFORM_NA（新增 `EVIDENCE_NA_PACKAGE_INFO`）；
3. 新增三张逐项显式处置表（键 `(module, java_name)`，仅在文件存在性/模糊匹配未命中时生效）：
   - `EXPLICIT_IMPLEMENTED`（29 项：channel 6、cp 4、open 8、mp 2、common 2、pay 6（含 GlobalTradeTypeEnum 改名）、miniapp 1）；
   - `EXPLICIT_PLATFORM_NA`（13 项：pay 11 + mp 1 + common 1（StringManager），另 miniapp package-info×2 经规则命中，合计 15）；
   - `EXPLICIT_DEPENDENCY_REUSED`（2 项：ChannelWxError、WxPayOrderNotifyResultConverter）。
   依据沿各模块台账脚本（audit_channel/cp/open/miniapp/pay_ledger.py）既有 SPECIAL/INLINED 归类，并逐项核对 WxJava 源码（`/Users/wandl/workspaces/workspace-github/WxJava`）。

## 五、测试证据

- 新增离线单测（`crates/wx-rust-pay/src/**` inline `#[cfg(test)]`，共 21 项）：SHA256withRSA 签名/验签往返（随机密钥）、Authorization token 五元组格式、`signUriStripPrefix` 规范化与签名串剥离、`build_message` 布局、validator 非 JSON 放行/缺头 false/篡改 false、公钥验证器兜底路由、X509PublicCertificate 序列号前缀剥离、AES-GCM 加解密往返、RSA-OAEP 往返、验证器构建三模式、payBaseUrl 前缀提取、config holder 线程隔离、http_proxy is_effective、gzip 文件/字节往返、WxPayException 文案拼装与覆盖分支、WxSignTestException 向上转型。
- 既有基线无回归：workspace 1968 全绿（基线 1942，+26）。
- clippy `--workspace --all-targets -D warnings` 干净；`cargo fmt --all` 无 diff。

## 六、ADAPTED 汇总（与 Java 的刻意差异）

| 差异点 | 语义保持说明 |
|---|---|
| v3 接口参数（Apache 请求/响应类型 → 值对象） | `CredentialsRequest`/`ValidationResponse` 承载同一信息（method/path/query/body、五验签头+body） |
| `AutoUpdateCertificatesVerifier` 构造不联网 | Java 构造器同步下载（失败仅告警）；Rust 下载延迟到 `check_and_auto_update`（异步 reqwest），未下载前 verify=false 与 Java `verifier==null` 分支一致 |
| `X509PublicCertificate.getSerialNumber` 返回形态 | Java BigInteger（hex 解析）；Rust 十六进制串（与 `WxPayCertificate::serial_no` 同命名空间），doc 注明 |
| `SpecEncrypt`/`RsaCryptoUtil.encryptFields` 反射加密 | Rust 无运行时反射；调用侧显式 `encrypt_oaep`（标记 trait 约定可选） |
| `WxPayException` 受检异常 | 结构体 + `From → WxErrorException::Runtime`，文案/Builder 逐字段一致 |
| `VerifierBuilder` 不接收 `wxPayHttpProxy` | 代理由 reqwest 承载（Java 参数仅为 Apache 注入用） |
| `WxMpMapConfigImpl` 并发变体 | Rust WxMpDefaultConfig 原生并发安全，无按存储结构分型需求 |
