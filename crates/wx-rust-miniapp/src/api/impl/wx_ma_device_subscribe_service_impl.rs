//! 小程序设备订阅消息服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaDeviceSubscribeServiceImpl`：
//! URL/请求体字段/响应解析（`sn_ticket`/`group_id`/`device_list`）逐方法对齐。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaDeviceSubscribeService;
use crate::bean::device::{
    WxMaCreateIotGroupIdRequest, WxMaDeviceSubscribeMessageRequest, WxMaDeviceTicketRequest,
    WxMaGetIotGroupInfoRequest, WxMaIotGroupDeviceInfoResponse, WxMaIotGroupDeviceRequest,
};
use crate::enums::g4_urls::url_g4_ability::device_subscribe as device_url;

/// 小程序设备订阅消息服务实现。
pub struct WxMaDeviceSubscribeServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaDeviceSubscribeServiceImpl {
    /// 构建设备订阅消息服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 序列化请求对象为 JSON（对应 Java `request.toJson()`）。
    fn to_json<T: serde::Serialize>(request: &T) -> Result<String, WxErrorException> {
        serde_json::to_string(request).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxMaDeviceSubscribeService for WxMaDeviceSubscribeServiceImpl {
    /// 获取设备票据（对应 Java `WxMaDeviceSubscribeServiceImpl.getSnTicket`，
    /// 解析 `sn_ticket`）。
    async fn get_sn_ticket(
        &self,
        request: &WxMaDeviceTicketRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &device_url::get_sn_ticket_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        json.get("sn_ticket")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "sn_ticket 字段缺失"))
    }

    /// 发送设备订阅消息（对应 Java
    /// `WxMaDeviceSubscribeServiceImpl.sendDeviceSubscribeMsg`）。
    async fn send_device_subscribe_msg(
        &self,
        request: &WxMaDeviceSubscribeMessageRequest,
    ) -> Result<(), WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        svc.post(
            &device_url::send_device_subscribe_msg_url(config.as_ref()),
            &Self::to_json(request)?,
        )
        .await?;
        Ok(())
    }

    /// 创建设备组（对应 Java
    /// `WxMaDeviceSubscribeServiceImpl.createIotGroupId`，解析 `group_id`）。
    async fn create_iot_group_id(
        &self,
        request: &WxMaCreateIotGroupIdRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &device_url::create_iot_group_id_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        json.get("group_id")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "group_id 字段缺失"))
    }

    /// 查询设备组信息（对应 Java
    /// `WxMaDeviceSubscribeServiceImpl.getIotGroupInfo`）。
    async fn get_iot_group_info(
        &self,
        request: &WxMaGetIotGroupInfoRequest,
    ) -> Result<WxMaIotGroupDeviceInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &device_url::get_iot_group_info_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        serde_json::from_str(&response_content).map_err(WxErrorException::from)
    }

    /// 设备组添加设备（对应 Java
    /// `WxMaDeviceSubscribeServiceImpl.addIotGroupDevice`，解析 `device_list`）。
    async fn add_iot_group_device(
        &self,
        request: &WxMaIotGroupDeviceRequest,
    ) -> Result<Vec<WxMaDeviceTicketRequest>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &device_url::add_iot_group_device_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        let device_list = json
            .get("device_list")
            .cloned()
            .unwrap_or_else(|| serde_json::json!([]));
        serde_json::from_value(device_list).map_err(WxErrorException::from)
    }

    /// 设备组删除设备（对应 Java
    /// `WxMaDeviceSubscribeServiceImpl.removeIotGroupDevice`，解析 `device_list`）。
    async fn remove_iot_group_device(
        &self,
        request: &WxMaIotGroupDeviceRequest,
    ) -> Result<Vec<WxMaDeviceTicketRequest>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .post(
                &device_url::remove_iot_group_device_url(config.as_ref()),
                &Self::to_json(request)?,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        let device_list = json
            .get("device_list")
            .cloned()
            .unwrap_or_else(|| serde_json::json!([]));
        serde_json::from_value(device_list).map_err(WxErrorException::from)
    }
}
