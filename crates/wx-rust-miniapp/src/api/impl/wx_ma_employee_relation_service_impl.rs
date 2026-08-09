//! 小程序用工关系相关操作服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaEmployeeRelationServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g3_services::WxMaEmployeeRelationService;
use crate::bean::employee::{WxMaSendEmployeeMsgRequest, WxMaUnbindEmployeeRequest};
use crate::enums::g3_urls::url_g3_shop::employee as employee_url;

/// 小程序用工关系服务实现。
pub struct WxMaEmployeeRelationServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaEmployeeRelationServiceImpl {
    /// 构建用工关系服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaEmployeeRelationService for WxMaEmployeeRelationServiceImpl {
    /// 对应 Java `WxMaEmployeeRelationServiceImpl.unbindEmployee`：
    /// POST `UNBIND_EMPLOYEE_URL`（`request.toJson()`）；Java 无返回值。
    async fn unbind_employee(
        &self,
        request: &WxMaUnbindEmployeeRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = request
            .to_json()
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&employee_url::unbind_employee_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }

    /// 对应 Java `WxMaEmployeeRelationServiceImpl.sendEmployeeMsg`：
    /// POST `SEND_EMPLOYEE_MSG_URL`（`request.toJson()`）；Java 无返回值。
    async fn send_employee_msg(
        &self,
        request: &WxMaSendEmployeeMsgRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let body = request
            .to_json()
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&employee_url::send_employee_msg_url(config.as_ref()), &body)
            .await?;
        Ok(())
    }
}
