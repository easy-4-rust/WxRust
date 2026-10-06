//! 对应 Java `com.binarywang.wxjava.store.bean.shop.ShopQrCodeResponse.java`。

/// 店铺二维码响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopQrCodeResponse {
    /// 错误码（继承自 WxStoreBaseResponse）。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息（继承自 WxStoreBaseResponse）。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 店铺二维码链接。
    #[serde(rename = "shop_qrcode", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_qrcode: Option<String>,
}
