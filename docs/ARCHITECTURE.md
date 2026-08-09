# WxRust 架构设计（B1 权威决策）

> 本文档是 WxRust 迁移的**架构宪法**：B2 整批实现必须遵循的全部决策。
> 来源：WxJava 源码分析（`WXJAVA_ANALYSIS.md` 7 大设计模式）+ rust-java-migration 技能设计原则
> + 9 模块四文档中已冻结的组件替换决策。
>
> - 锁定日期：2026-08-01（B1）
> - Java 基线：`a49d6e1461752c06b752d2afd8aeeb7e6e78cefe`（4.8.4.B）
> - 对象分母：3287 main + 379 test；方法分母：36010 javap 公共方法（29461 源声明）
> - 文档状态：`LOCKED`（B1 冻结，变更需评审）

---

## 1. 总体架构

```
┌──────────────────────────────────────────────────────────┐
│  集成层（后续）  wx-rust-vernal（对标 Spring Boot starter） │
├──────────────────────────────────────────────────────────┤
│  业务层  wx-rust-mp / miniapp / pay / cp / open / channel  │
│          / aispeech / qidian                              │
│          门面 Service(trait) + 子域 Service + bean         │
├──────────────────────────────────────────────────────────┤
│  基础层  wx-rust-common                                   │
│          error / config / http / bean / session / util    │
├──────────────────────────────────────────────────────────┤
│  facade  wx-rust（feature 门控重导出）                     │
└──────────────────────────────────────────────────────────┘
```

**分层原则**：
1. `wx-rust-common` 不依赖任何业务 crate；业务 crate 只依赖 common。
2. 业务 crate 之间不互相依赖（open 复用 mp/ma 能力通过组合/trait，不建依赖）。
3. facade 只做重导出，不含实现。

---

## 2. 已锁定的组件替换（B1 决策）

| Java 职责 | Rust 组件 | 版本基线 | 状态 | 证据 |
|---|---|---|---|---|
| HTTP（Apache/OkHttp/Jodd 三后端） | `reqwest`（rustls，`default-features=false`，features=`json,rustls,stream`） | 0.13.x | `LOCKED` | 语义表·组件替换；三后端 `PLATFORM_NA` |
| JSON（Gson） | `serde` + `serde_json`（`preserve_order`） | 1.0.x | `LOCKED` | 所有 TypeAdapter `PLATFORM_NA` |
| XML（支付/消息） | `quick-xml` + `serde` | 0.41.x | `LOCKED` | 语义表·序列化 |
| 加解密/签名 | RustCrypto：`aes` / `rsa` / `sha1` / `sha2` / `hmac` / `base64` / `hex` | 锁定版本 | `LOCKED` | 已知向量测试 |
| 日志（SLF4J） | `tracing` + `tracing-subscriber` | 0.1.x | `LOCKED` | 语义表·组件替换 |
| 时间（Joda-time/java.util） | `chrono` + std | 0.4.x | `LOCKED` | 时间边界测试 |
| 集合（Guava） | std（`HashMap`/`Vec`）+ `DashMap`（并发） | — | `LOCKED` | 语义表·集合 |
| Redis（Jedis/Redisson） | `redis` crate（feature 门控 `redis`） | 0.29.x | `CANDIDATE`→待 POC | 集成测试 |
| 错误 | `thiserror`（`WxErrorException` 为 typed enum） | 2.0.x | `LOCKED` | 错误体系 |
| 异步 | `tokio`（sync/macros/rt/time）+ `async-trait` | 1.52.x / 0.1.x | `LOCKED` | 并发模型 |
| URL 处理（URIUtil） | `url` + `percent-encoding` | 2.5.x / 2.3.x | `LOCKED` | 语义表 |

> `CANDIDATE` 项（redis）必须在 B2 前完成 POC 并升级为 `LOCKED` 或替换。

---

## 3. Java → Rust 机制映射（权威）

### 3.1 继承链消解（模式 B：ServiceImpl → HttpComponentsImpl → BaseImpl）

Rust 无继承。映射规则：

| Java | Rust |
|---|---|
| `WxMpService`（接口） | `pub trait WxMpService: Send + Sync`（`#[async_trait]`） |
| `BaseWxMpServiceImpl<H,P>`（抽象基类，含逻辑） | `pub struct WxMpServiceCore`（内部实现 + `Arc<dyn WxHttpClient>`），提供默认方法 |
| `WxMpServiceHttpComponentsImpl`（绑定 HTTP 客户端） | 无独立类型：core 直接持有 `reqwest::Client`（reqwest 统一后端） |
| `WxMpServiceImpl`（空壳） | 无独立类型：`WxMpService` 的默认实现结构体 |
| 子域 Service（Kefu/Menu/...） | 每子域一个 trait + 实现，构造时注入 `Arc<dyn WxMpService>` 或共享 core |

> **设计要点**：Java 的"默认实现壳 + 多 HTTP 后端"在 Rust 中坍缩为**一个实现结构体**，
> 因为 reqwest 统一了 HTTP 层。继承语义（子域 Service 复用主 Service 的 execute/token）
> 通过**持有 `Arc<主Service>`** 实现——这正是 Java 中 `new WxMpKefuServiceImpl(this)` 的等价物。

### 3.2 Token 双重检查锁（模式 D）→ async

```rust
pub async fn get_access_token(&self, force_refresh: bool) -> Result<String, WxErrorException> {
    // ① 快速路径
    if !force_refresh && !self.config.is_access_token_expired() {
        return Ok(self.config.access_token());
    }
    // ② 取锁 + 超时抢锁（3s，100ms 轮询），对应 Java tryLock(100ms) 循环
    let lock = self.config.access_token_lock().clone();
    let guard = tokio::time::timeout(Duration::from_secs(3), async {
        loop {
            match lock.try_lock() {
                Ok(g) => break g,
                Err(_) => tokio::time::sleep(Duration::from_millis(100)).await,
            }
        }
    }).await.map_err(|_| WxRuntimeException::new("获取accessToken超时"))?;
    // ③ 拿到锁后再查一次（双检）
    if !force_refresh && !self.config.is_access_token_expired() {
        return Ok(self.config.access_token());
    }
    // ④ 模板方法：trait 提供 do_get_access_token_request（stable 变体同理）
    let resp = if self.config.is_stable_access_token() {
        self.do_get_stable_access_token_request(force_refresh).await?
    } else {
        self.do_get_access_token_request().await?
    };
    // ⑤ 解析并更新缓存
    self.extract_access_token(&resp)
    // guard 在函数结束 drop（等价 finally unlock）
}
```

### 3.3 请求执行引擎（模式 E）→ async loop（不用递归）

```rust
pub async fn execute<T, E>(&self, executor: &dyn RequestExecutor<T, E>, url: &str, data: E)
    -> Result<T, WxErrorException>
where E: Send + Sync {
    let mut retry = 0;
    loop {
        match self.execute_internal(executor, url, &data, false).await {
            Ok(v) => return Ok(v),
            Err(e) if e.error_code() == -1 => {
                if retry + 1 > self.max_retry_times() {   // 默认 5
                    return Err(WxRuntimeException::new("微信服务端异常，超出重试次数").into());
                }
                let sleep = Duration::from_millis(self.retry_sleep_millis() * (1 << retry));
                tokio::time::sleep(sleep).await;          // 指数退避 1s,2s,4s,8s,16s
                retry += 1;
            }
            Err(e) => return Err(e),
        }
    }
}

async fn execute_internal<T, E>(&self, executor: &dyn RequestExecutor<T, E>, url: &str, data: &E, no_auto_refresh: bool)
    -> Result<T, WxErrorException> {
    let token = self.get_access_token(false).await?;
    let url_with_token = format!("{}{}access_token={}", url,
        if url.contains('?') { "&" } else { "?" }, token);
    match executor.execute(&url_with_token, data, WxType::Mp).await {
        Ok(v) => Ok(v),
        Err(e) if ACCESS_TOKEN_ERROR_CODES.contains(&e.error_code()) => {
            // token 过期：失效缓存（带锁，且校验 token 未变）
            let lock = self.config.access_token_lock().clone();
            let _g = lock.lock().await;
            if self.config.access_token() == token {
                self.config.expire_access_token();
            }
            drop(_g);
            if self.config.auto_refresh_token() && !no_auto_refresh {
                // 单次重试（no_auto_refresh=true 防无限循环/栈溢出）
                return self.execute_internal(executor, url, data, true).await;
            }
            Err(e)
        }
        Err(e) if e.error_code() != 0 => Err(e),
        Err(_) => Ok(unsafe_placeholder()),  // Java 语义：errorCode==0 返回 null
    }
}
```

> **注**：`errorCode==0` 返回 null 的 Java 语义 → Rust `Ok(None)` 或文档说明（B2 时按具体泛型 T 决定，通常 T 为 `Option`）。

### 3.4 HTTP 执行器（RequestExecutor 策略）

```rust
#[async_trait]
pub trait RequestExecutor<T, E>: Send + Sync {
    async fn execute(&self, uri: &str, data: E, wx_type: WxType) -> Result<T, WxErrorException>;
}
// 实现：SimpleGetRequestExecutor / SimplePostRequestExecutor / MediaUploadRequestExecutor / ...
// 每个 Java executor 类对应一个 Rust 文件（util/http/ 下），持有 Arc<reqwest::Client> + 代理配置
```

### 3.5 错误体系

```rust
#[derive(Error, Debug)]
pub enum WxErrorException {
    #[error("微信接口错误: {0}")]
    Wx(#[from] WxError),                       // 对应 Java WxErrorException(e)
    #[error("运行时错误: {0}")]
    Runtime(#[from] WxRuntimeException),
    #[error("IO 错误: {0}")]
    Io(#[from] std::io::Error),
    #[error("HTTP 错误: {0}")]
    Http(#[from] reqwest::Error),
    #[error("序列化错误: {0}")]
    Serde(#[from] serde_json::Error),
    // ...
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WxError {
    pub error_code: i32,
    pub error_msg: Option<String>,
    pub error_msg_en: Option<String>,
    #[serde(skip)]
    pub json: Option<String>,
}
// WxError::from_json / from_json_with_type：错误码 → 中文翻译（分 WxType 表）
```

### 3.6 消息路由（模式 G）→ builder 模式

```rust
pub struct WxMpMessageRouter {
    rules: Vec<WxMpMessageRouterRule>,
    service: Arc<dyn WxMpService>,
    duplicate_checker: Arc<dyn WxMessageDuplicateChecker>,
    session_manager: Arc<dyn WxSessionManager>,
    exception_handler: Arc<dyn WxErrorExceptionHandler>,
}

pub struct WxMpMessageRouterRule {
    matchers: Vec<...>,
    interceptors: Vec<...>,
    handlers: Vec<...>,
    // builder: .msg_type("text").matcher(m).interceptor(i).handler(h).end()
}

impl WxMpMessageRouter {
    pub fn rule(&mut self) -> &mut WxMpMessageRouterRule { ... }
    pub async fn route(&self, msg: &WxMpXmlMessage, ctx: &mut Map<String, Value>)
        -> Option<WxMpXmlOutMessage> {
        // 1. 重复检查 2. 匹配规则 3. interceptor+handler（同步/异步执行）
    }
}
```

### 3.7 ConfigStorage 多租户

```rust
#[async_trait]
pub trait WxConfigStorage: Send + Sync {
    fn app_id(&self) -> &str;
    fn secret(&self) -> &str;
    fn access_token(&self) -> String;                 // getter
    fn is_access_token_expired(&self) -> bool;
    fn expire_access_token(&self);
    fn update_access_token(&self, token: &str, expires_in: i32);
    fn access_token_lock(&self) -> Arc<tokio::sync::Mutex<()>>;
    fn is_stable_access_token(&self) -> bool;
    // + ticket 相关 + host_config + 代理配置
}

// 内存实现（对应 WxMpDefaultConfigImpl）：
pub struct WxMpDefaultConfig {
    pub app_id: String,
    pub secret: String,
    pub token: Option<String>,
    pub aes_key: Option<String>,
    pub access_token: Arc<RwLock<Option<TokenEntry>>>,   // TokenEntry { value, expires_at }
    lock: Arc<tokio::sync::Mutex<()>>,
}
// 多租户（对应 multi starter / WxMpConfigStorageHolder）：
// DashMap<String, Arc<dyn WxConfigStorage>> + switchover 方法
```

---

## 4. 命名规则（对象名称一致性基准）

| Java | Rust 规则 | 示例 |
|---|---|---|
| 类/接口/枚举 | PascalCase 类型 + snake_case 文件 | `WxMpService` → `wx_mp_service.rs` |
| 包 → 目录 | 保留末 2 层，snake_case | `me.chanjar.weixin.mp.api.impl` → `api/impl/` |
| getter | `name()`（去 get_ 前缀） | `getAccessToken()` → `access_token()` |
| setter | `set_name(v)`（受控变更） | `setAppId(v)` → `set_app_id(v)` |
| 布尔 | 语义谓词 | `isAccessTokenExpired()` → `is_access_token_expired()` |
| 转换 | `as_`/`to_`/`into_` 语义 | `toJson()` → `to_json()` |
| 重载 | canonical + `_with_*` 后缀 | `fromJson(String)` / `fromJson(String,WxType)` → `from_json()` / `from_json_with_type()` |
| 构造 | `new()` / `from_*` | `fromJson` → `from_json` |
| 常量 | `SCREAMING_SNAKE_CASE` | `ACCESS_TOKEN_ERROR_CODES` → `ACCESS_TOKEN_ERROR_CODES` |
| 禁止 | `get_*` 机械镜像 JavaBean；`Deref` 模拟继承；wildcard import | — |

**目录/文件映射（retain_segments=2）**：
- `me.chanjar.weixin.common.error.WxError` → `crates/wx-rust-common/src/error/wx_error.rs`
- `me.chanjar.weixin.mp.api.impl.BaseWxMpServiceImpl` → `crates/wx-rust-mp/src/api/impl/base_wx_mp_service_impl.rs`
- `cn.binarywang.wx.miniapp.bean.WxMaUserInfo` → `crates/wx-rust-miniapp/src/bean/wx_ma_user_info.rs`

---

## 5. 注释迁移规范（语义 100%）

- 每个 Java 对象 → 中文 `//!` crate/模块级文档 + 类型级 `///` 文档
- 每个方法 → `///` 中文文档，保留 `# 参数`（含泛型参数）、`# 返回`、`# 错误`（对应 `@throws`）、`# 版本`（`@since`）、`# 弃用`（`@deprecated`）、相关 `# 参见`（`@see`）
- 关键行内/块注释迁移到对应分支或不变量旁
- 禁止在 Rust 注释中写"对应 Java"字样（对应关系只在四文档中）
- 方法体逻辑逐行对齐，不删减分支、副作用、错误路径

---

## 6. 并发与生命周期契约

| Java | Rust | 注意 |
|---|---|---|
| `ReentrantLock`（token 锁） | `tokio::sync::Mutex<()>` | 超时抢锁语义（tryLock 100ms × 30） |
| `synchronized` | `tokio::sync::Mutex` / `RwLock` | 锁范围最小化，无跨 await 持锁 |
| `ExecutorService`（Router） | `tokio::task::spawn` | JoinHandle 管理，取消语义 |
| `Thread.sleep`（退避） | `tokio::time::sleep` | 可取消 |
| `ThreadLocal`（ConfigStorageHolder） | 显式参数传递 / `tokio::task_local!` | Java ThreadLocal 在 async 中不可用 |
| 单例（InMemoryDuplicateCheckerSingleton） | `once_cell::Lazy` / `OnceLock` | 等价 |
| 资源释放 | RAII（Drop） | `reqwest::Client`、文件句柄自动释放 |

**无 `unsafe`**：全 workspace `#![forbid(unsafe_code)]`。所有能力必须纯 Rust 实现或经 `DEPENDENCY_REUSED` 的审计 crate。

---

## 7. 依赖管理（B1 基线）

- workspace 统一版本管理（`[workspace.dependencies]`），版本对齐 hutool-rust 基线
- `deny.toml`：license 白名单（Apache-2.0/MIT/BSD/ISC/Unlicense），禁止 copyleft
- 依赖准入：MSRV ≤ 1.85、无 `unsafe` 优先、有维护、有测试
- `redis` 为可选 feature（`wx-rust-common/redis`），默认关闭
- 禁止运行时动态加载/反射替代品

---

## 8. B2 实现批次定义

**批次 = 完整模块**（技能要求：冻结整批 → 实现 → 冻结 → 统一审计，禁止逐对象验收）。

| 批次 | 模块 | 依赖 | 对象数 | 方法数 |
|---|---|---|---|---|
| 1 | `wx-rust-common` | 无 | 174 | 958 |
| 2 | `wx-rust-mp` | common | 428 | 3748 |
| 3 | `wx-rust-miniapp` | common | 611 | 4942 |
| 4 | `wx-rust-pay` | common | 570 | 6788 |
| 5 | `wx-rust-cp` | common | 594 | 6099 |
| 6 | `wx-rust-open` | common | 240 | 2077 |
| 7 | `wx-rust-channel` | common | 618 | 4308 |
| 8 | `wx-rust-aispeech` | common | 25 | 256 |
| 9 | `wx-rust-qidian` | common | 27 | 285 |

每个批次完成后：V0 统一静态审计（`audit_migration_layout.py`）→ V1 工程验证 → V2 行为验证（Java 测试镜像 + golden 差分）→ 更新四文档。

---

## 9. 架构变更管理

- 本文档 `LOCKED` 状态；任何变更（新增依赖、改 trait 签名、改命名规则）需：
  1. 记录变更原因与影响范围
  2. 同步更新 9 模块四文档 + 本文档
  3. 变更后重跑受影响批次的验证
- 架构决策与代码在同一变更中提交，不允许"先实现后补文档"
