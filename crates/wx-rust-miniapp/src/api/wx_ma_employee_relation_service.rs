//! 小程序用工关系相关操作接口。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaEmployeeRelationService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::employee::{WxMaSendEmployeeMsgRequest, WxMaUnbindEmployeeRequest};

/// 小程序用工关系服务。
#[async_trait]
pub trait WxMaEmployeeRelationService: Send + Sync {
    /// 解绑用工关系（对应 Java `unbindEmployee(WxMaUnbindEmployeeRequest)`，
    /// 企业解除和用户的 B2C 用工关系，Java 无返回值）。
    async fn unbind_employee(
        &self,
        request: &WxMaUnbindEmployeeRequest,
    ) -> Result<(), WxErrorException>;

    /// 推送用工消息（对应 Java `sendEmployeeMsg(WxMaSendEmployeeMsgRequest)`，
    /// 企业向用户推送用工相关消息，Java 无返回值）。
    async fn send_employee_msg(
        &self,
        request: &WxMaSendEmployeeMsgRequest,
    ) -> Result<(), WxErrorException>;
}
