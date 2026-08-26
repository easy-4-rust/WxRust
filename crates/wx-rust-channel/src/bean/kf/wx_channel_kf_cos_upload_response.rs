//! 对应 Java `me.chanjar.weixin.channel.bean.kf.WxChannelKfCosUploadResponse`。

use crate::bean::base::WxChannelBaseResponse;

/// 客服素材上传响应（对应 Java `WxChannelKfCosUploadResponse`）。
///
/// 继承 `WxChannelBaseResponse`，额外包含 COS 地址。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxChannelKfCosUploadResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 素材在 COS 上的地址
    #[serde(rename = "cos_url", default)]
    pub cos_url: String,
}

impl WxChannelKfCosUploadResponse {
    /// 获取 COS 地址。
    pub fn cos_url(&self) -> &str {
        &self.cos_url
    }
}

impl From<WxChannelKfCosUploadResponse> for WxChannelBaseResponse {
    fn from(resp: WxChannelKfCosUploadResponse) -> Self {
        WxChannelBaseResponse {
            err_code: resp.err_code,
            err_msg: resp.err_msg,
        }
    }
}
