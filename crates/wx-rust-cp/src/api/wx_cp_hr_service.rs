//! 人事助手服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpHrService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{FieldItem, WxCpHrEmployeeFieldDataResp, WxCpHrEmployeeFieldInfoResp};

/// 人事助手服务。
#[async_trait]
pub trait WxCpHrService: Send + Sync {
    /// 获取员工档案字段信息（对应 Java
    /// `WxCpHrService.getFieldInfo(List<String>)`；`fields` 不填则返回
    /// 全部字段）。
    async fn get_field_info(
        &self,
        fields: Option<&[&str]>,
    ) -> Result<WxCpHrEmployeeFieldInfoResp, WxErrorException>;

    /// 获取员工档案数据（对应 Java
    /// `WxCpHrService.getEmployeeFieldInfo(String, List<String>)`）。
    async fn get_employee_field_info(
        &self,
        userid: &str,
        fields: Option<&[&str]>,
    ) -> Result<WxCpHrEmployeeFieldDataResp, WxErrorException>;

    /// 获取员工档案数据（对应 Java
    /// `WxCpHrService.getEmployeeFieldInfo(String, boolean, List<String>)`；
    /// `getAll` 为 true 时获取全部字段）。
    async fn get_employee_field_info_with_get_all(
        &self,
        userid: &str,
        get_all: bool,
        fields: Option<&[&str]>,
    ) -> Result<WxCpHrEmployeeFieldDataResp, WxErrorException>;

    /// 更新员工档案数据（对应 Java
    /// `WxCpHrService.updateEmployeeFieldInfo(String, List<FieldItem>)`）。
    async fn update_employee_field_info(
        &self,
        userid: &str,
        field_list: &[FieldItem],
    ) -> Result<(), WxErrorException>;
}
