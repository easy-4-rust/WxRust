//! 基础话务服务实现。
//!
//! 对应 Java `me.chanjar.weixin.qidian.api.impl.WxQidianDialServiceImpl`：
//! 通过门面 `get`/`post` 调用企点话务接口（
//! `https://api.qidian.qq.com/cgi-bin/call/dial/...`，经执行引擎自动注入
//! access_token）。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxQidianDialService, WxQidianService};
use crate::bean::dial::{IVRDialRequest, IVRDialResponse, IVRListResponse};
use crate::enums::wx_qidian_api_url::dial::{GET_IVR_LIST, IVR_DIAL};

/// 基础话务服务实现。
pub struct WxQidianDialServiceImpl {
    /// 门面服务弱引用（对应 Java `WxQidianService wxQidianService` 字段）
    service: Weak<dyn WxQidianService>,
}

impl WxQidianDialServiceImpl {
    /// 构建实现。
    ///
    /// # 参数
    /// - `service`：门面服务弱引用（打破循环引用）
    pub fn new(service: Weak<dyn WxQidianService>) -> Self {
        Self { service }
    }

    /// 门面服务引用（子服务生命周期内必然存在，对应 Java 强引用字段）。
    fn service(&self) -> Result<std::sync::Arc<dyn WxQidianService>, WxErrorException> {
        self.service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已释放"))
    }
}

#[async_trait]
impl WxQidianDialService for WxQidianDialServiceImpl {
    async fn ivr_dial(
        &self,
        ivr_dial: &IVRDialRequest,
    ) -> Result<IVRDialResponse, WxErrorException> {
        // 对应 Java：`ivrDial.toJson()` 后 POST IVR_DIAL
        let json = ivr_dial.to_json();
        let service = self.service()?;
        let result = service.post(&IVR_DIAL, &json).await?;
        IVRDialResponse::from_json(&result).map_err(WxErrorException::Serde)
    }

    async fn get_ivr_list(&self) -> Result<IVRListResponse, WxErrorException> {
        let service = self.service()?;
        let result = service.get(&GET_IVR_LIST, "").await?;
        IVRListResponse::from_json(&result).map_err(WxErrorException::Serde)
    }
}
