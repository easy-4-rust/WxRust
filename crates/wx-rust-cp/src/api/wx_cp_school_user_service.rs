//! 企业微信家校沟通服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpSchoolUserService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpAllowScope, WxCpBaseResp, WxCpBatchCreateParentRequest, WxCpBatchCreateStudentRequest,
    WxCpBatchDeleteStudentRequest, WxCpBatchResultList, WxCpBatchUpdateParentRequest,
    WxCpBatchUpdateStudentRequest, WxCpCreateDepartment, WxCpCreateDepartmentRequest,
    WxCpCreateParentRequest, WxCpDepartmentList, WxCpExternalContact, WxCpListParentResult,
    WxCpOauth2UserInfo, WxCpSetUpgradeInfo, WxCpSubscribeQrCode, WxCpUpdateDepartmentRequest,
    WxCpUpdateParentRequest, WxCpUserListResult, WxCpUserResult,
};

/// 企业微信家校沟通服务。
#[async_trait]
pub trait WxCpSchoolUserService: Send + Sync {
    /// 获取访问用户身份（对应 Java
    /// `WxCpSchoolUserService.getUserInfo(String)`）。
    async fn get_user_info(&self, code: &str) -> Result<WxCpOauth2UserInfo, WxErrorException>;

    /// 获取家校访问用户身份（对应 Java
    /// `WxCpSchoolUserService.getSchoolUserInfo(String)`）。
    async fn get_school_user_info(
        &self,
        code: &str,
    ) -> Result<WxCpOauth2UserInfo, WxErrorException>;

    /// 创建学生（对应 Java
    /// `WxCpSchoolUserService.createStudent(String, String, List<Integer>)`）。
    async fn create_student(
        &self,
        student_user_id: &str,
        name: &str,
        departments: &[i32],
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 批量创建学生（对应 Java
    /// `WxCpSchoolUserService.batchCreateStudent(WxCpBatchCreateStudentRequest)`）。
    async fn batch_create_student(
        &self,
        request: &WxCpBatchCreateStudentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException>;

    /// 批量删除学生（对应 Java
    /// `WxCpSchoolUserService.batchDeleteStudent(WxCpBatchDeleteStudentRequest)`）。
    async fn batch_delete_student(
        &self,
        request: &WxCpBatchDeleteStudentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException>;

    /// 批量更新学生（对应 Java
    /// `WxCpSchoolUserService.batchUpdateStudent(WxCpBatchUpdateStudentRequest)`）。
    async fn batch_update_student(
        &self,
        request: &WxCpBatchUpdateStudentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException>;

    /// 删除学生（对应 Java
    /// `WxCpSchoolUserService.deleteStudent(String)`）。
    async fn delete_student(&self, student_user_id: &str)
    -> Result<WxCpBaseResp, WxErrorException>;

    /// 更新学生（对应 Java
    /// `WxCpSchoolUserService.updateStudent(String, String, String,
    /// List<Integer>)`）。
    async fn update_student(
        &self,
        student_user_id: &str,
        new_student_user_id: Option<&str>,
        name: Option<&str>,
        departments: Option<&[i32]>,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 创建家长（对应 Java
    /// `WxCpSchoolUserService.createParent(WxCpCreateParentRequest)`）。
    async fn create_parent(
        &self,
        request: &WxCpCreateParentRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 批量创建家长（对应 Java
    /// `WxCpSchoolUserService.batchCreateParent(WxCpBatchCreateParentRequest)`）。
    async fn batch_create_parent(
        &self,
        request: &WxCpBatchCreateParentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException>;

    /// 批量删除家长（对应 Java
    /// `WxCpSchoolUserService.batchDeleteParent(String... userIdList)`）。
    async fn batch_delete_parent(
        &self,
        user_id_list: &[&str],
    ) -> Result<WxCpBatchResultList, WxErrorException>;

    /// 批量更新家长（对应 Java
    /// `WxCpSchoolUserService.batchUpdateParent(WxCpBatchUpdateParentRequest)`）。
    async fn batch_update_parent(
        &self,
        request: &WxCpBatchUpdateParentRequest,
    ) -> Result<WxCpBatchResultList, WxErrorException>;

    /// 读取学生或家长（对应 Java
    /// `WxCpSchoolUserService.getUser(String)`）。
    async fn get_user(&self, user_id: &str) -> Result<WxCpUserResult, WxErrorException>;

    /// 获取部门成员详情（对应 Java
    /// `WxCpSchoolUserService.getUserList(Integer, Integer)`）。
    async fn get_user_list(
        &self,
        department_id: i32,
        fetch_child: Option<i32>,
    ) -> Result<WxCpUserListResult, WxErrorException>;

    /// 获取部门家长详情（对应 Java
    /// `WxCpSchoolUserService.getUserListParent(Integer)`）。
    async fn get_user_list_parent(
        &self,
        department_id: i32,
    ) -> Result<WxCpListParentResult, WxErrorException>;

    /// 更新家长（对应 Java
    /// `WxCpSchoolUserService.updateParent(WxCpUpdateParentRequest)`）。
    async fn update_parent(
        &self,
        request: &WxCpUpdateParentRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除家长（对应 Java
    /// `WxCpSchoolUserService.deleteParent(String)`）。
    async fn delete_parent(&self, user_id: &str) -> Result<WxCpBaseResp, WxErrorException>;

    /// 设置家校通讯录自动同步模式（对应 Java
    /// `WxCpSchoolUserService.setArchSyncMode(Integer)`；
    /// `archSyncMode`：1/2/3）。
    async fn set_arch_sync_mode(
        &self,
        arch_sync_mode: i32,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 创建部门（对应 Java
    /// `WxCpSchoolUserService.createDepartment(WxCpCreateDepartmentRequest)`）。
    async fn create_department(
        &self,
        request: &WxCpCreateDepartmentRequest,
    ) -> Result<WxCpCreateDepartment, WxErrorException>;

    /// 更新部门（对应 Java
    /// `WxCpSchoolUserService.updateDepartment(WxCpUpdateDepartmentRequest)`）。
    async fn update_department(
        &self,
        request: &WxCpUpdateDepartmentRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除部门（对应 Java
    /// `WxCpSchoolUserService.deleteDepartment(Integer)`）。
    async fn delete_department(&self, id: i32) -> Result<WxCpBaseResp, WxErrorException>;

    /// 设置关注「学校通知」的模式（对应 Java
    /// `WxCpSchoolUserService.setSubscribeMode(Integer)`；
    /// `subscribeMode`：1-可扫码填写资料加入，2-禁止扫码填写资料加入）。
    async fn set_subscribe_mode(
        &self,
        subscribe_mode: i32,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取关注「学校通知」的模式（对应 Java
    /// `WxCpSchoolUserService.getSubscribeMode()`）。
    async fn get_subscribe_mode(&self) -> Result<i32, WxErrorException>;

    /// 获取外部联系人详情（对应 Java
    /// `WxCpSchoolUserService.getExternalContact(String)`）。
    async fn get_external_contact(
        &self,
        external_user_id: &str,
    ) -> Result<WxCpExternalContact, WxErrorException>;

    /// 获取可使用的家长范围（对应 Java
    /// `WxCpSchoolUserService.getAllowScope(Integer)`）。
    async fn get_allow_scope(&self, agent_id: i32) -> Result<WxCpAllowScope, WxErrorException>;

    /// 外部联系人 openid 转换（对应 Java
    /// `WxCpSchoolUserService.convertToOpenId(String)`）。
    async fn convert_to_open_id(&self, external_user_id: &str) -> Result<String, WxErrorException>;

    /// 获取部门列表（对应 Java
    /// `WxCpSchoolUserService.listDepartment(Integer)`；`id` 不填默认获取
    /// 全量组织架构）。
    async fn list_department(
        &self,
        id: Option<i32>,
    ) -> Result<WxCpDepartmentList, WxErrorException>;

    /// 获取「学校通知」二维码（对应 Java
    /// `WxCpSchoolUserService.getSubscribeQrCode()`）。
    async fn get_subscribe_qr_code(&self) -> Result<WxCpSubscribeQrCode, WxErrorException>;

    /// 修改自动升年级的配置（对应 Java
    /// `WxCpSchoolUserService.setUpgradeInfo(Long, Integer)`）。
    async fn set_upgrade_info(
        &self,
        upgrade_time: Option<i64>,
        upgrade_switch: Option<i32>,
    ) -> Result<WxCpSetUpgradeInfo, WxErrorException>;
}
