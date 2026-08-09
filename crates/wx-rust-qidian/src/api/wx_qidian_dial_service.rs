//! 基础话务服务接口。
//!
//! 对应 Java `me.chanjar.weixin.qidian.api.WxQidianDialService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::dial::{IVRDialRequest, IVRDialResponse, IVRListResponse};

/// 基础话务相关操作接口。
#[async_trait]
pub trait WxQidianDialService: Send + Sync {
    /// IVR 外呼（对应 Java `ivrDial(IVRDialRequest)`）。
    async fn ivr_dial(
        &self,
        ivr_dial: &IVRDialRequest,
    ) -> Result<IVRDialResponse, WxErrorException>;

    /// 拉取 IVR 列表（对应 Java `getIVRList()`）。
    async fn get_ivr_list(&self) -> Result<IVRListResponse, WxErrorException>;
}
