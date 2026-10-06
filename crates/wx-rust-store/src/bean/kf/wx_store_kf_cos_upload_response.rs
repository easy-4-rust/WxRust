//! 对应 Java `com.binarywang.wxjava.store.bean.kf.WxStoreKfCosUploadResponse`。

use crate::bean::base::WxStoreBaseResponse;

/// 客服素材上传响应（对应 Java `WxStoreKfCosUploadResponse`）。
///
/// 继承 `WxStoreBaseResponse`，额外包含 COS 地址。
#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxStoreKfCosUploadResponse {
    /// 错误码
    #[serde(rename = "errcode", default)]
    pub err_code: i32,
    /// 错误信息
    #[serde(rename = "errmsg", default)]
    pub err_msg: String,
    /// 素材在 COS 上的地址
    #[serde(rename = "cos_url", default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cos_url: Option<String>,
}

impl WxStoreKfCosUploadResponse {
    /// 获取 COS 地址。
    pub fn cos_url(&self) -> Option<&str> {
        self.cos_url.as_deref()
    }
}

impl From<WxStoreKfCosUploadResponse> for WxStoreBaseResponse {
    fn from(resp: WxStoreKfCosUploadResponse) -> Self {
        WxStoreBaseResponse {
            err_code: resp.err_code,
            err_msg: resp.err_msg,
        }
    }
}
