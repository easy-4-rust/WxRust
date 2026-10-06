//! 对应 Java `com.binarywang.wxjava.store.bean.shop.ShopH5UrlResponse.java`。

/// 店铺 H5 链接响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopH5UrlResponse {
    /// 错误码（继承自 WxStoreBaseResponse）。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息（继承自 WxStoreBaseResponse）。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 店铺 H5 链接。
    #[serde(rename = "shop_h5url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_h5url: Option<String>,
}
