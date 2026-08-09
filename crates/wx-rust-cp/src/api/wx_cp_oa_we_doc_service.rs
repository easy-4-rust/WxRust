//! 企业微信文档服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOaWeDocService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpBaseResp, WxCpDocAdminListResult, WxCpDocAdminRequest, WxCpDocAuthInfo, WxCpDocCreateData,
    WxCpDocCreateRequest, WxCpDocData, WxCpDocGetDataRequest, WxCpDocImageUploadResult,
    WxCpDocInfo, WxCpDocModifyJoinRuleRequest, WxCpDocModifyMemberRequest, WxCpDocModifyRequest,
    WxCpDocModifySafetySettingRequest, WxCpDocModifySaftySettingRequest, WxCpDocRenameRequest,
    WxCpDocShare, WxCpDocShareRequest, WxCpDocSheetBatchUpdateRequest,
    WxCpDocSheetBatchUpdateResponse, WxCpDocSheetData, WxCpDocSheetGetDataRequest,
    WxCpDocSheetProperties, WxCpDocSmartSheetAuth, WxCpDocSmartSheetAuthRequest,
    WxCpDocSmartSheetModifyAuthRequest, WxCpDocSmartSheetRequest, WxCpDocSmartSheetResult,
    WxCpFormAnswer, WxCpFormAnswerRequest, WxCpFormCreateRequest, WxCpFormCreateResult,
    WxCpFormInfoResult, WxCpFormModifyRequest, WxCpFormStatistic, WxCpFormStatisticRequest,
    WxCpFormStatisticResult,
};

/// 企业微信文档服务。
#[async_trait]
pub trait WxCpOaWeDocService: Send + Sync {
    /// 新建文档（对应 Java
    /// `WxCpOaWeDocService.docCreate(WxCpDocCreateRequest)`）。
    async fn doc_create(
        &self,
        request: &WxCpDocCreateRequest,
    ) -> Result<WxCpDocCreateData, WxErrorException>;

    /// 重命名文档/收集表（对应 Java
    /// `WxCpOaWeDocService.docRename(WxCpDocRenameRequest)`）。
    async fn doc_rename(
        &self,
        request: &WxCpDocRenameRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除文档/收集表（对应 Java
    /// `WxCpOaWeDocService.docDelete(String, String)`；
    /// `docId`/`formId` 只能填其中一个）。
    async fn doc_delete(
        &self,
        doc_id: Option<&str>,
        form_id: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取文档基础信息（对应 Java
    /// `WxCpOaWeDocService.docInfo(String)`）。
    async fn doc_info(&self, doc_id: &str) -> Result<WxCpDocInfo, WxErrorException>;

    /// 分享文档（对应 Java `WxCpOaWeDocService.docShare(String)`）。
    async fn doc_share(&self, doc_id: &str) -> Result<WxCpDocShare, WxErrorException>;

    /// 分享文档/收集表（对应 Java
    /// `WxCpOaWeDocService.docShare(WxCpDocShareRequest)`；
    /// `docid`/`formid` 二选一）。
    async fn doc_share_with_request(
        &self,
        request: &WxCpDocShareRequest,
    ) -> Result<WxCpDocShare, WxErrorException>;

    /// 获取文档权限信息（对应 Java
    /// `WxCpOaWeDocService.docGetAuth(String)`）。
    async fn doc_get_auth(&self, doc_id: &str) -> Result<WxCpDocAuthInfo, WxErrorException>;

    /// 修改文档查看规则（对应 Java
    /// `WxCpOaWeDocService.docModifyJoinRule(WxCpDocModifyJoinRuleRequest)`）。
    async fn doc_modify_join_rule(
        &self,
        request: &WxCpDocModifyJoinRuleRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 修改文档通知范围及权限（对应 Java
    /// `WxCpOaWeDocService.docModifyMember(WxCpDocModifyMemberRequest)`）。
    async fn doc_modify_member(
        &self,
        request: &WxCpDocModifyMemberRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 修改文档安全设置（对应 Java
    /// `WxCpOaWeDocService.docModifySafetySetting(WxCpDocModifySafetySettingRequest)`）。
    async fn doc_modify_safety_setting(
        &self,
        request: &WxCpDocModifySafetySettingRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 修改文档安全设置（旧接口名，对应 Java
    /// `WxCpOaWeDocService.docModifySaftySetting(WxCpDocModifySaftySettingRequest)`，
    /// Java 中已 `@Deprecated`，推荐使用 `doc_modify_safety_setting`）。
    async fn doc_modify_safty_setting(
        &self,
        request: &WxCpDocModifySaftySettingRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 编辑表格内容（对应 Java
    /// `WxCpOaWeDocService.docBatchUpdate(WxCpDocSheetBatchUpdateRequest)`）。
    async fn doc_batch_update(
        &self,
        request: &WxCpDocSheetBatchUpdateRequest,
    ) -> Result<WxCpDocSheetBatchUpdateResponse, WxErrorException>;

    /// 获取表格行列信息（对应 Java
    /// `WxCpOaWeDocService.getSheetProperties(String)`）。
    async fn get_sheet_properties(
        &self,
        doc_id: &str,
    ) -> Result<WxCpDocSheetProperties, WxErrorException>;

    /// 获取指定范围内的在线表格信息（对应 Java
    /// `WxCpOaWeDocService.getSheetRangeData(WxCpDocSheetGetDataRequest)`）。
    async fn get_sheet_range_data(
        &self,
        request: &WxCpDocSheetGetDataRequest,
    ) -> Result<WxCpDocSheetData, WxErrorException>;

    /// 获取文档数据（对应 Java
    /// `WxCpOaWeDocService.docGetData(WxCpDocGetDataRequest)`）。
    async fn doc_get_data(
        &self,
        request: &WxCpDocGetDataRequest,
    ) -> Result<WxCpDocData, WxErrorException>;

    /// 编辑文档内容（对应 Java
    /// `WxCpOaWeDocService.docModify(WxCpDocModifyRequest)`）。
    async fn doc_modify(
        &self,
        request: &WxCpDocModifyRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 上传文档图片（对应 Java
    /// `WxCpOaWeDocService.docUploadImage(File)`；Java `File` 以文件路径
    /// `&str` 表达，ADAPTED）。
    async fn doc_upload_image(
        &self,
        file_path: &str,
    ) -> Result<WxCpDocImageUploadResult, WxErrorException>;

    /// 添加文档高级功能账号（对应 Java
    /// `WxCpOaWeDocService.docAddAdmin(WxCpDocAdminRequest)`）。
    async fn doc_add_admin(
        &self,
        request: &WxCpDocAdminRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除文档高级功能账号（对应 Java
    /// `WxCpOaWeDocService.docDeleteAdmin(WxCpDocAdminRequest)`）。
    async fn doc_delete_admin(
        &self,
        request: &WxCpDocAdminRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取文档高级功能账号列表（对应 Java
    /// `WxCpOaWeDocService.docGetAdminList(String)`）。
    async fn doc_get_admin_list(
        &self,
        doc_id: &str,
    ) -> Result<WxCpDocAdminListResult, WxErrorException>;

    /// 获取智能表格内容权限（对应 Java
    /// `WxCpOaWeDocService.smartSheetGetAuth(WxCpDocSmartSheetAuthRequest)`）。
    async fn smart_sheet_get_auth(
        &self,
        request: &WxCpDocSmartSheetAuthRequest,
    ) -> Result<WxCpDocSmartSheetAuth, WxErrorException>;

    /// 修改智能表格内容权限（对应 Java
    /// `WxCpOaWeDocService.smartSheetModifyAuth(WxCpDocSmartSheetModifyAuthRequest)`）。
    async fn smart_sheet_modify_auth(
        &self,
        request: &WxCpDocSmartSheetModifyAuthRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取智能表格工作表信息（对应 Java
    /// `WxCpOaWeDocService.smartSheetGetSheet(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_get_sheet(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException>;

    /// 新增智能表格工作表（对应 Java
    /// `WxCpOaWeDocService.smartSheetAddSheet(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_add_sheet(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException>;

    /// 删除智能表格工作表（对应 Java
    /// `WxCpOaWeDocService.smartSheetDeleteSheet(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_delete_sheet(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 更新智能表格工作表（对应 Java
    /// `WxCpOaWeDocService.smartSheetUpdateSheet(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_update_sheet(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取智能表格视图（对应 Java
    /// `WxCpOaWeDocService.smartSheetGetViews(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_get_views(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException>;

    /// 新增智能表格视图（对应 Java
    /// `WxCpOaWeDocService.smartSheetAddView(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_add_view(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException>;

    /// 删除智能表格视图（对应 Java
    /// `WxCpOaWeDocService.smartSheetDeleteViews(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_delete_views(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 更新智能表格视图（对应 Java
    /// `WxCpOaWeDocService.smartSheetUpdateView(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_update_view(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取智能表格字段（对应 Java
    /// `WxCpOaWeDocService.smartSheetGetFields(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_get_fields(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException>;

    /// 新增智能表格字段（对应 Java
    /// `WxCpOaWeDocService.smartSheetAddFields(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_add_fields(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException>;

    /// 删除智能表格字段（对应 Java
    /// `WxCpOaWeDocService.smartSheetDeleteFields(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_delete_fields(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 更新智能表格字段（对应 Java
    /// `WxCpOaWeDocService.smartSheetUpdateFields(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_update_fields(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取智能表格记录（对应 Java
    /// `WxCpOaWeDocService.smartSheetGetRecords(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_get_records(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException>;

    /// 新增智能表格记录（对应 Java
    /// `WxCpOaWeDocService.smartSheetAddRecords(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_add_records(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException>;

    /// 删除智能表格记录（对应 Java
    /// `WxCpOaWeDocService.smartSheetDeleteRecords(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_delete_records(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 更新智能表格记录（对应 Java
    /// `WxCpOaWeDocService.smartSheetUpdateRecords(WxCpDocSmartSheetRequest)`）。
    async fn smart_sheet_update_records(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 创建收集表（对应 Java
    /// `WxCpOaWeDocService.formCreate(WxCpFormCreateRequest)`）。
    async fn form_create(
        &self,
        request: &WxCpFormCreateRequest,
    ) -> Result<WxCpFormCreateResult, WxErrorException>;

    /// 编辑收集表（对应 Java
    /// `WxCpOaWeDocService.formModify(WxCpFormModifyRequest)`）。
    async fn form_modify(
        &self,
        request: &WxCpFormModifyRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取收集表信息（对应 Java
    /// `WxCpOaWeDocService.formInfo(String)`）。
    async fn form_info(&self, form_id: &str) -> Result<WxCpFormInfoResult, WxErrorException>;

    /// 获取收集表统计信息（对应 Java
    /// `WxCpOaWeDocService.formStatistic(List<WxCpFormStatisticRequest>)`，
    /// 返回包含 `statistic_list` 的结果）。
    async fn form_statistic(
        &self,
        requests: &[WxCpFormStatisticRequest],
    ) -> Result<WxCpFormStatisticResult, WxErrorException>;

    /// 单个收集表统计查询的兼容封装（对应 Java
    /// `WxCpOaWeDocService.formStatistic(WxCpFormStatisticRequest)`，
    /// Java 为 default 方法，底层仍按官方数组请求发送）。
    async fn form_statistic_single(
        &self,
        request: &WxCpFormStatisticRequest,
    ) -> Result<WxCpFormStatistic, WxErrorException>;

    /// 获取收集表答案（对应 Java
    /// `WxCpOaWeDocService.formAnswer(WxCpFormAnswerRequest)`）。
    async fn form_answer(
        &self,
        request: &WxCpFormAnswerRequest,
    ) -> Result<WxCpFormAnswer, WxErrorException>;
}
