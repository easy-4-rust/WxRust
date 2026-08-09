//! 小程序设备订阅消息服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaDeviceSubscribeService`
//! （`impl.WxMaDeviceSubscribeServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::device::{
    WxMaCreateIotGroupIdRequest, WxMaDeviceSubscribeMessageRequest, WxMaDeviceTicketRequest,
    WxMaGetIotGroupInfoRequest, WxMaIotGroupDeviceInfoResponse, WxMaIotGroupDeviceRequest,
};

/// 小程序设备订阅消息服务。
///
/// 对应 Java `WxMaDeviceSubscribeService`：设备票据、设备订阅消息发送、
/// 设备组（创建/查询/添加设备/删除设备）。
#[async_trait]
pub trait WxMaDeviceSubscribeService: Send + Sync {
    /// 获取设备票据（对应 Java `getSnTicket`）。
    async fn get_sn_ticket(
        &self,
        request: &WxMaDeviceTicketRequest,
    ) -> Result<String, WxErrorException>;

    /// 发送设备订阅消息（对应 Java `sendDeviceSubscribeMsg`）。
    async fn send_device_subscribe_msg(
        &self,
        request: &WxMaDeviceSubscribeMessageRequest,
    ) -> Result<(), WxErrorException>;

    /// 创建设备组（对应 Java `createIotGroupId`，返回设备组唯一标识）。
    async fn create_iot_group_id(
        &self,
        request: &WxMaCreateIotGroupIdRequest,
    ) -> Result<String, WxErrorException>;

    /// 查询设备组信息（对应 Java `getIotGroupInfo`）。
    async fn get_iot_group_info(
        &self,
        request: &WxMaGetIotGroupInfoRequest,
    ) -> Result<WxMaIotGroupDeviceInfoResponse, WxErrorException>;

    /// 设备组添加设备（对应 Java `addIotGroupDevice`，返回成功添加的设备信息）。
    async fn add_iot_group_device(
        &self,
        request: &WxMaIotGroupDeviceRequest,
    ) -> Result<Vec<WxMaDeviceTicketRequest>, WxErrorException>;

    /// 设备组删除设备（对应 Java `removeIotGroupDevice`，返回成功删除的设备信息）。
    async fn remove_iot_group_device(
        &self,
        request: &WxMaIotGroupDeviceRequest,
    ) -> Result<Vec<WxMaDeviceTicketRequest>, WxErrorException>;
}
