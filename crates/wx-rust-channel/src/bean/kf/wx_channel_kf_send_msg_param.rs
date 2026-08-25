//! 对应 Java `me.chanjar.weixin.channel.bean.kf.WxChannelKfSendMsgParam.java`。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxChannelKfSendMsgParam {
    /// 用户 open_id
    #[serde(rename = "open_id", default)]
    pub open_id: String,
    /// 消息类型
    #[serde(rename = "msg_type", default)]
    pub msg_type: String,
    /// 文本内容（msg_type 为 text 时使用）
    #[serde(rename = "content", default)]
    pub content: String,
    /// 图片 URL（msg_type 为 image 时使用）
    #[serde(rename = "image_url", default)]
    pub image_url: String,
    /// 视频 URL（msg_type 为 video 时使用）
    #[serde(rename = "video_url", default)]
    pub video_url: String,
}
