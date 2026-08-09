//! 充值、扣费主体（枚举）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.bean.intractiy.PayMode`。
//! 线格式由 `@SerializedName` 决定：`PAY_MODE_STORE`/`PAY_MODE_APP`/
//! `PAY_MODE_COMPONENT`；不传 pay_mode 默认 `PAY_MODE_STORE`（Java 文档语义）。

/// 充值、扣费主体。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize, Default)]
pub enum PayMode {
    /// 门店（Java `@SerializedName("PAY_MODE_STORE")`，默认值）。
    #[serde(rename = "PAY_MODE_STORE")]
    #[default]
    Store,
    /// 小程序（Java `@SerializedName("PAY_MODE_APP")`）。
    #[serde(rename = "PAY_MODE_APP")]
    App,
    /// 服务商（Java `@SerializedName("PAY_MODE_COMPONENT")`）。
    #[serde(rename = "PAY_MODE_COMPONENT")]
    Component,
}
