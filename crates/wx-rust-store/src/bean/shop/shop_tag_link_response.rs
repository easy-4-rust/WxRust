//! 对应 Java `com.binarywang.wxjava.store.bean.shop.ShopTagLinkResponse.java`。

/// 店铺口令响应。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ShopTagLinkResponse {
    /// 错误码（继承自 WxStoreBaseResponse）。
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息（继承自 WxStoreBaseResponse）。
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 店铺微信口令。
    #[serde(rename = "shop_taglink", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shop_taglink: Option<String>,
}
