//! 微信支付 builder 辅助。
//!
//! Java 侧（weixin-java-pay）当前**没有**独立 builder 包：`WxPayOrderQueryRequest`
//! 等请求对象在 Java 中直接以 Lombok `@Accessors(chain = true)` 链式 setter 构造
//! （无 `*Builder` 类），分账/退款等 v2 请求亦同。Rust 侧以「字段 `pub` + `Default`
//! 派生」提供等价构造能力（`WxPayRefundRequest::default()` 后逐字段赋值），
//! 无需 builder 类型；如需链式风格，可在调用侧用 struct update 语法。
//!
//! 若后续 Java 引入 builder 类（如 `com.github.binarywang.wxpay.bean.request.*Builder`），
//! 在本模块补生成即可。

// 本模块当前为空（Java 无 builder 类可镜像）。
