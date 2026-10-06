//! 对应 Java `com.binarywang.wxjava.store.bean.kf.WxStoreKfSendMsgParam.java`。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
/// 微信小店 WxStoreKfSendMsgParam；对应 Java com.binarywang.wxjava.store.bean.kf.WxStoreKfSendMsgParam.java。
pub struct WxStoreKfSendMsgParam {
    /// 用户 open_id
    #[serde(rename = "open_id", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id: Option<String>,
    /// 消息类型
    #[serde(rename = "msg_type", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg_type: Option<String>,
    /// 文本内容（msg_type 为 text 时使用）
    #[serde(rename = "content", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 图片 URL（msg_type 为 image 时使用）
    #[serde(rename = "image_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    /// 视频 URL（msg_type 为 video 时使用）
    #[serde(rename = "video_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    /// 上游字段 request_id。
    #[serde(
        rename = "request_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub request_id: Option<String>,
    /// 上游字段 text。
    #[serde(rename = "text", skip_serializing_if = "Option::is_none")]
    pub text: Option<WxStoreKfSendMsgParamText>,
    /// 上游字段 image。
    #[serde(rename = "image", skip_serializing_if = "Option::is_none")]
    pub image: Option<WxStoreKfSendMsgParamCosUrlMessage>,
    /// 上游字段 video。
    #[serde(rename = "video", skip_serializing_if = "Option::is_none")]
    pub video: Option<WxStoreKfSendMsgParamCosUrlMessage>,
    /// 上游字段 file。
    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    pub file: Option<WxStoreKfSendMsgParamCosUrlMessage>,
    /// 上游字段 product_share。
    #[serde(
        rename = "product_share",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_share: Option<WxStoreKfSendMsgParamProductShareMessage>,
    /// 上游字段 order_share。
    #[serde(
        rename = "order_share",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub order_share: Option<WxStoreKfSendMsgParamOrderShareMessage>,
}

/// 微信小店嵌套数据。对应 Java: kf.WxStoreKfSendMsgParam#Text
/// 微信小店 WxStoreKfSendMsgParamText 数据类型；对应 Java com.binarywang.wxjava.store.bean.kf.WxStoreKfSendMsgParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxStoreKfSendMsgParamText {
    /// 上游字段 content。
    #[serde(rename = "content", skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: kf.WxStoreKfSendMsgParam#CosUrlMessage
/// 微信小店 WxStoreKfSendMsgParamCosUrlMessage 数据类型；对应 Java com.binarywang.wxjava.store.bean.kf.WxStoreKfSendMsgParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxStoreKfSendMsgParamCosUrlMessage {
    /// 上游字段 cos_url。
    #[serde(rename = "cos_url", skip_serializing_if = "Option::is_none")]
    pub cos_url: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: kf.WxStoreKfSendMsgParam#ProductShareMessage
/// 微信小店 WxStoreKfSendMsgParamProductShareMessage 数据类型；对应 Java com.binarywang.wxjava.store.bean.kf.WxStoreKfSendMsgParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxStoreKfSendMsgParamProductShareMessage {
    /// 上游字段 product_id。
    #[serde(
        rename = "product_id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_id: Option<String>,
}

/// 微信小店嵌套数据。对应 Java: kf.WxStoreKfSendMsgParam#OrderShareMessage
/// 微信小店 WxStoreKfSendMsgParamOrderShareMessage 数据类型；对应 Java com.binarywang.wxjava.store.bean.kf.WxStoreKfSendMsgParam.java。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxStoreKfSendMsgParamOrderShareMessage {
    /// 上游字段 order_id。
    #[serde(rename = "order_id", skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
}
