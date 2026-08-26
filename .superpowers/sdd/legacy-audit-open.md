# wx-rust-open 存量语义审计报告

## 审计范围

`crates/wx-rust-open/src/api/impl/` 全部 service 实现，对照 Java
`weixin-java-open/src/main/java/me/chanjar/weixin/open/api/impl/`。

## 审计服务清单

| # | Rust 文件 | Java 对应 | 方法数 | 状态 |
|---|-----------|-----------|--------|------|
| 1 | `wx_open_component_service_impl.rs` | `WxOpenComponentServiceImpl.java` | ~55 | 1 处缺陷已修复 |
| 2 | `wx_open_service_impl.rs` | `WxOpenServiceImpl.java` | 5 | 无差异 |
| 3 | `base_wx_open_service_impl.rs` | `WxOpenServiceAbstractImpl.java` | 3 (引擎) | 无差异 |
| 4 | `wx_open_ma_service.rs` (桥接) | `WxOpenMaServiceImpl.java` | ~8 | 无差异 |
| 5 | `wx_open_mp_service.rs` (桥接) | `WxOpenMpServiceImpl.java` | ~5 | 无差异 |
| 6 | `wx_open_ma_privacy_service_impl.rs` | `WxOpenMaPrivacyServiceImpl.java` | 4 | 无差异 |
| 7 | `wx_open_ma_basic_service_impl.rs` | `WxOpenMaBasicServiceImpl.java` | 14 | 无差异 |
| 8 | `wx_open_ma_auth_service_impl.rs` | `WxOpenMaAuthServiceImpl.java` | 5 | 无差异 |
| 9 | `wx_open_ma_icp_service_impl.rs` | `WxOpenMaIcpServiceImpl.java` | 12 | 无差异 |
| 10 | `wx_open_ma_embedded_service_impl.rs` | `WxOpenMaEmbeddedServiceImpl.java` | 6 | 无差异 |
| 11 | `wx_open_ma_auth_and_icp_service_impl.rs` | `WxOpenMaAuthAndIcpServiceImpl.java` | 2 | 无差异 |
| 12 | `wx_open_ma_shopping_orders_service_impl.rs` | `WxOpenMaShoppingOrdersServiceImpl.java` | 7 | 无差异 |
| 13 | `wx_open_mp_o_auth2_service_impl.rs` | `WxOpenMpOAuth2ServiceImpl.java` | 4 | 无差异 |
| 14 | `wx_open_o_auth2_service_impl.rs` | `WxOpenOAuth2ServiceImpl.java` | 5 | 无差异 |
| 15 | `wx_open_minishop_service_impl.rs` | `WxOpenMinishopServiceImpl.java` | (minishop 子服务) | 无差异 |
| 16 | `wx_open_minishop_goods_service_impl.rs` | `WxOpenMinishopGoodsServiceImpl.java` | (minishop 子服务) | 无差异 |

## 三向核对结果

### 1. URL 逐字符对照

全部 URL 常量（`WxOpenComponentService` 接口 50+ 常量、`WxOpenMaService` 接口
30+ 常量、`WxOpenMaPrivacyService`/`WxOpenMaBasicService`/`WxOpenMaShoppingOrdersService`
等子接口常量）已逐字符对照，路径与域名均一致。Rust 以 `api(config, path)` 函数
风格统一前缀拼接（支持自定义 apiHostUrl 替换），语义与 Java 常量值完全对齐。

### 2. 参数字段名对照

全部 `@SerializedName` 已与 Rust bean serde rename 对齐。minishop 入参
bean 的 Java `toJsonObject()` 手工拼装的 snake_case 键名已在 Rust
`minishop_json` 模块中逐键镜像。

### 3. 响应反序列化对照

全部响应 bean 的字段映射已对照。数字 errcode 归一化为字符串（Java Gson
宽松转换语义）已在 `normalize_errcode` 中统一处理。

### 4. 特殊逻辑对照

- preauthcode 换取流程：POST `api_create_preauthcode` 取 `pre_auth_code`
  后按 `COMPONENT_LOGIN_PAGE_URL`/`COMPONENT_MOBILE_LOGIN_PAGE_URL` 格式化，
  auth_type/biz_appid 占位符替换逻辑已对齐（**本次修复**）。
- 授权组件回调解析：`route()` 方法的 info_type 分发逻辑已对齐。
- 快速注册流程：`fast_register_weapp`/`fast_register_personal_weapp`/
  `fast_register_beta_weapp` 已对齐。
- open 帐号管理：`openAccountServicePost` 的 mp/mini 分发已对齐。

## 修复清单

### Fix 1: `create_pre_auth_url` 占位符清理缺失

**文件**: `wx_open_component_service_impl.rs`
**方法**: `create_pre_auth_url`
**问题**: Java 在 `authType`/`bizAppid` 为空或 null 时，将 URL 中的
`&auth_type=xxx`/`&biz_appid=xxx` 占位符替换为空字符串（移除）。Rust
仅在值非空时替换，空值时占位符 `xxx` 残留在 URL 中，导致生成的预授权链接
包含无效参数 `auth_type=xxx`/`biz_appid=xxx`。
**修复**: 添加 else 分支，空值/None 时替换为空字符串，与 Java 行为一致。

```rust
// 修复前：
if let Some(auth_type) = auth_type {
    if !auth_type.is_empty() {
        pre_auth_url_str = pre_auth_url_str.replace("&auth_type=xxx", ...);
    }
    // 空值时不做任何处理，xxx 残留
}

// 修复后：
if let Some(auth_type) = auth_type {
    if !auth_type.is_empty() {
        pre_auth_url_str = pre_auth_url_str.replace("&auth_type=xxx", ...);
    } else {
        pre_auth_url_str = pre_auth_url_str.replace("&auth_type=xxx", "");
    }
} else {
    pre_auth_url_str = pre_auth_url_str.replace("&auth_type=xxx", "");
}
```

## 已知上游 bug（严格镜像，不修复）

### `minishopGoodsUpdateSkuPrice` sale_price/market_price 赋值错误

Java 代码（`WxOpenComponentServiceImpl.java:1171-1172`）将 `sale_price` 和
`market_price` 均赋值为 `outSkuId`，忽略了实际的 `salePrice`/`marketPrice`
参数。Rust 严格镜像此行为（`minishop_goods_update_sku_price` 方法），并以注释
标注为上游 bug。修复此 bug 需要同步修改 Java 上游或明确偏离镜像约定。

## 缺失方法（trait 层面，非 impl 语义差异）

以下 Java 接口方法在 Rust `WxOpenComponentService` trait 中未声明（trait
冻结时未收录），不属于 impl 语义差异：

- `fastRegisterEnterpriseWeapp` / `fastRegisterEnterpriseWeappQuery`
  （企业快速注册，Java 用 `access_token` 非 `component_access_token`）
- `checkSignature(String appid, ...)` （带 appid 的签名校验，Java 恒返回 false）
- `oauth2buildAuthorizationUrl` （oauth2 授权 URL 构建，Java 在组件层）

## 测试结果

```
cargo test -p wx-rust-open: 190 passed, 0 failed
cargo clippy -p wx-rust-open -- -D warnings: clean
cargo fmt -p wx-rust-open -- --check: clean
```

## 审计结论

审计 16 个 service 实现、约 150+ 方法，发现并修复 1 处真实语义缺陷
（`create_pre_auth_url` 占位符清理缺失）。其余方法的 URL、参数、响应映射、
特殊逻辑均已对齐，无额外差异。open 总测试数 190，全部通过。
