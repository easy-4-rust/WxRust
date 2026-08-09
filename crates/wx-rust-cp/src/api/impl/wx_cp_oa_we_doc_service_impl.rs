//! 企业微信文档服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOaWeDocServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::MediaUploadRequestExecutor;

use crate::api::{WxCpOaWeDocService, WxCpService};
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
use crate::enums::url_oa;

/// 企业微信文档服务实现。
pub struct WxCpOaWeDocServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOaWeDocServiceImpl {
    /// 构建文档服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造仅含 `docid` 的请求体（对应 Java `docInfo`/`docGetAuth`/
    /// `getSheetProperties`/`docGetAdminList` 内的
    /// `JsonObject`：`{"docid": docId}`）。
    fn build_doc_id_body(doc_id: &str) -> String {
        serde_json::json!({ "docid": doc_id }).to_string()
    }

    /// 构造仅含 `formid` 的请求体（对应 Java `formInfo` 内的
    /// `JsonObject`：`{"formid": formId}`）。
    fn build_form_id_body(form_id: &str) -> String {
        serde_json::json!({ "formid": form_id }).to_string()
    }

    /// 构造删除文档请求体（对应 Java `docDelete` 内的 `JsonObject`：
    /// `docid`/`formid` 只能填其中一个；Gson `addProperty` 对 null 值写入
    /// `JsonNull`，故空参序列化为 `null`）。
    fn build_doc_delete_body(doc_id: Option<&str>, form_id: Option<&str>) -> String {
        serde_json::json!({
            "docid": doc_id,
            "formid": form_id,
        })
        .to_string()
    }

    /// 构造分享文档请求体（对应 Java `docShare(String)` 内
    /// `WxCpDocShareRequest.builder().docId(docId).build()` 的 toJson：
    /// Gson 默认省略 null 字段，故仅含 `docid`）。
    fn build_doc_share_body(doc_id: &str) -> String {
        serde_json::json!({ "docid": doc_id }).to_string()
    }

    /// 请求体为请求 bean 的 toJson（对应 Java `request.toJson()`）。
    fn request_to_json<T: ToJson>(request: &T) -> Result<String, WxErrorException> {
        request.to_json().map_err(WxErrorException::Serde)
    }

    /// POST 请求并解析整包响应为结果 bean（对应 Java `Xxx.fromJson`）。
    async fn post_and_parse<T: FromJson>(
        svc: &dyn WxCpService,
        url: &str,
        body: &str,
    ) -> Result<T, WxErrorException> {
        let response = svc.post(url, body).await?;
        T::from_json(&response).map_err(WxErrorException::Serde)
    }

    /// 转换旧接口安全设置请求为新接口请求（对应 Java
    /// `docModifySaftySetting` 默认方法内
    /// `WxCpDocModifySafetySettingRequest.builder()...build()`）。
    fn convert_safty_to_safety(
        request: &WxCpDocModifySaftySettingRequest,
    ) -> WxCpDocModifySafetySettingRequest {
        WxCpDocModifySafetySettingRequest {
            doc_id: request.doc_id.clone(),
            enable_readonly_copy: request.enable_readonly_copy,
            watermark: request.watermark.clone(),
        }
    }
}

/// 请求 bean 序列化 trait（内部统一 `request.to_json()` 语义）。
trait ToJson {
    fn to_json(&self) -> Result<String, String>;
}

/// 结果 bean 反序列化 trait（内部统一 `Xxx.from_json` 语义）。
trait FromJson {
    fn from_json(json: &str) -> Result<Self, String>
    where
        Self: Sized;
}

macro_rules! impl_to_json {
    ($($t:ty),* $(,)?) => {
        $(
            impl ToJson for $t {
                fn to_json(&self) -> Result<String, String> {
                    <$t>::to_json(self)
                }
            }
        )*
    };
}

macro_rules! impl_from_json {
    ($($t:ty),* $(,)?) => {
        $(
            impl FromJson for $t {
                fn from_json(json: &str) -> Result<Self, String> {
                    <$t>::from_json(json)
                }
            }
        )*
    };
}

impl_to_json!(
    WxCpDocCreateRequest,
    WxCpDocRenameRequest,
    WxCpDocShareRequest,
    WxCpDocModifyJoinRuleRequest,
    WxCpDocModifyMemberRequest,
    WxCpDocModifySafetySettingRequest,
    WxCpDocSheetBatchUpdateRequest,
    WxCpDocSheetGetDataRequest,
    WxCpDocGetDataRequest,
    WxCpDocModifyRequest,
    WxCpDocAdminRequest,
    WxCpDocSmartSheetAuthRequest,
    WxCpDocSmartSheetModifyAuthRequest,
    WxCpDocSmartSheetRequest,
    WxCpFormCreateRequest,
    WxCpFormModifyRequest,
    WxCpFormAnswerRequest,
);

impl_from_json!(
    WxCpDocCreateData,
    WxCpBaseResp,
    WxCpDocInfo,
    WxCpDocShare,
    WxCpDocAuthInfo,
    WxCpDocSheetBatchUpdateResponse,
    WxCpDocSheetProperties,
    WxCpDocSheetData,
    WxCpDocData,
    WxCpDocImageUploadResult,
    WxCpDocAdminListResult,
    WxCpDocSmartSheetAuth,
    WxCpDocSmartSheetResult,
    WxCpFormCreateResult,
    WxCpFormInfoResult,
    WxCpFormStatisticResult,
    WxCpFormAnswer,
);

#[async_trait]
impl WxCpOaWeDocService for WxCpOaWeDocServiceImpl {
    async fn doc_create(
        &self,
        request: &WxCpDocCreateRequest,
    ) -> Result<WxCpDocCreateData, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docCreate`：`POST WEDOC_CREATE_DOC`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::WEDOC_CREATE_DOC);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_rename(
        &self,
        request: &WxCpDocRenameRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docRename`：`POST WEDOC_RENAME_DOC`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::WEDOC_RENAME_DOC);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_delete(
        &self,
        doc_id: Option<&str>,
        form_id: Option<&str>,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docDelete`：`POST WEDOC_DEL_DOC`，`docid`/`formid` 二选一
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::WEDOC_DEL_DOC);
        Self::post_and_parse(
            svc.as_ref(),
            &api_url,
            &Self::build_doc_delete_body(doc_id, form_id),
        )
        .await
    }

    async fn doc_info(&self, doc_id: &str) -> Result<WxCpDocInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docInfo`：`POST WEDOC_GET_DOC_BASE_INFO`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_GET_DOC_BASE_INFO);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_doc_id_body(doc_id)).await
    }

    async fn doc_share(&self, doc_id: &str) -> Result<WxCpDocShare, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docShare(String)`：构造仅含 `docId` 的
        // `WxCpDocShareRequest` 后走 `docShare(WxCpDocShareRequest)`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::WEDOC_DOC_SHARE);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_doc_share_body(doc_id)).await
    }

    async fn doc_share_with_request(
        &self,
        request: &WxCpDocShareRequest,
    ) -> Result<WxCpDocShare, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docShare(WxCpDocShareRequest)`：`POST WEDOC_DOC_SHARE`，
        // `docid`/`formid` 二选一
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::WEDOC_DOC_SHARE);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_get_auth(&self, doc_id: &str) -> Result<WxCpDocAuthInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docGetAuth`：`POST WEDOC_DOC_GET_AUTH`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_DOC_GET_AUTH);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_doc_id_body(doc_id)).await
    }

    async fn doc_modify_join_rule(
        &self,
        request: &WxCpDocModifyJoinRuleRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docModifyJoinRule`：`POST WEDOC_MOD_DOC_JOIN_RULE`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_MOD_DOC_JOIN_RULE);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_modify_member(
        &self,
        request: &WxCpDocModifyMemberRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docModifyMember`：`POST WEDOC_MOD_DOC_MEMBER`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_MOD_DOC_MEMBER);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_modify_safety_setting(
        &self,
        request: &WxCpDocModifySafetySettingRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docModifySafetySetting`：`POST WEDOC_MOD_DOC_SAFETY_SETTING`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_MOD_DOC_SAFETY_SETTING);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_modify_safty_setting(
        &self,
        request: &WxCpDocModifySaftySettingRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        // Java `docModifySaftySetting`（`@Deprecated` 默认方法）：转换为
        // `WxCpDocModifySafetySettingRequest` 后委托
        // `docModifySafetySetting`
        self.doc_modify_safety_setting(&Self::convert_safty_to_safety(request))
            .await
    }

    async fn doc_batch_update(
        &self,
        request: &WxCpDocSheetBatchUpdateRequest,
    ) -> Result<WxCpDocSheetBatchUpdateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docBatchUpdate`：`POST WEDOC_SPREADSHEET_BATCH_UPDATE`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SPREADSHEET_BATCH_UPDATE);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn get_sheet_properties(
        &self,
        doc_id: &str,
    ) -> Result<WxCpDocSheetProperties, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getSheetProperties`：
        // `POST WEDOC_SPREADSHEET_GET_SHEET_PROPERTIES`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SPREADSHEET_GET_SHEET_PROPERTIES);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_doc_id_body(doc_id)).await
    }

    async fn get_sheet_range_data(
        &self,
        request: &WxCpDocSheetGetDataRequest,
    ) -> Result<WxCpDocSheetData, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getSheetRangeData`：
        // `POST WEDOC_SPREADSHEET_GET_SHEET_RANGE_DATA`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SPREADSHEET_GET_SHEET_RANGE_DATA);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_get_data(
        &self,
        request: &WxCpDocGetDataRequest,
    ) -> Result<WxCpDocData, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docGetData`：`POST WEDOC_GET_DOC_DATA`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_GET_DOC_DATA);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_modify(
        &self,
        request: &WxCpDocModifyRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docModify`：`POST WEDOC_MOD_DOC`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::WEDOC_MOD_DOC);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_upload_image(
        &self,
        file_path: &str,
    ) -> Result<WxCpDocImageUploadResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docUploadImage(File)`：`cpService.upload(WEDOC_UPLOAD_DOC_IMAGE,
        // CommonUploadParam.fromFile("media", file))`，以 multipart 字段
        // `media` 上传
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_UPLOAD_DOC_IMAGE);
        let content = std::fs::read(file_path).map_err(|e| WxErrorException::Io(e.to_string()))?;
        let file_name = std::path::Path::new(file_path)
            .file_name()
            .map(|s| s.to_string_lossy().to_string());
        let executor = MediaUploadRequestExecutor::new(svc.http_client().clone());
        let param = CommonUploadParam::new("media", CommonUploadData::new(file_name, content));
        let response = crate::api::r#impl::base_wx_cp_service_impl::execute_with_retry(
            svc.as_ref(),
            &executor,
            &api_url,
            param,
        )
        .await?;
        WxCpDocImageUploadResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn doc_add_admin(
        &self,
        request: &WxCpDocAdminRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docAddAdmin`：`POST WEDOC_ADD_ADMIN`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::WEDOC_ADD_ADMIN);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_delete_admin(
        &self,
        request: &WxCpDocAdminRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docDeleteAdmin`：`POST WEDOC_DEL_ADMIN`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::WEDOC_DEL_ADMIN);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn doc_get_admin_list(
        &self,
        doc_id: &str,
    ) -> Result<WxCpDocAdminListResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `docGetAdminList`：`POST WEDOC_GET_ADMIN_LIST`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_GET_ADMIN_LIST);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_doc_id_body(doc_id)).await
    }

    async fn smart_sheet_get_auth(
        &self,
        request: &WxCpDocSmartSheetAuthRequest,
    ) -> Result<WxCpDocSmartSheetAuth, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetGetAuth`：`POST WEDOC_SMARTSHEET_GET_SHEET_AUTH`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_GET_SHEET_AUTH);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_modify_auth(
        &self,
        request: &WxCpDocSmartSheetModifyAuthRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetModifyAuth`：`POST WEDOC_SMARTSHEET_MOD_SHEET_AUTH`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_MOD_SHEET_AUTH);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_get_sheet(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetGetSheet`：`POST WEDOC_SMARTSHEET_GET_SHEET`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_GET_SHEET);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_add_sheet(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetAddSheet`：`POST WEDOC_SMARTSHEET_ADD_SHEET`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_ADD_SHEET);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_delete_sheet(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetDeleteSheet`：`POST WEDOC_SMARTSHEET_DELETE_SHEET`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_DELETE_SHEET);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_update_sheet(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetUpdateSheet`：`POST WEDOC_SMARTSHEET_UPDATE_SHEET`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_UPDATE_SHEET);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_get_views(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetGetViews`：`POST WEDOC_SMARTSHEET_GET_VIEWS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_GET_VIEWS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_add_view(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetAddView`：`POST WEDOC_SMARTSHEET_ADD_VIEW`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_ADD_VIEW);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_delete_views(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetDeleteViews`：`POST WEDOC_SMARTSHEET_DELETE_VIEWS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_DELETE_VIEWS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_update_view(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetUpdateView`：`POST WEDOC_SMARTSHEET_UPDATE_VIEW`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_UPDATE_VIEW);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_get_fields(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetGetFields`：`POST WEDOC_SMARTSHEET_GET_FIELDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_GET_FIELDS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_add_fields(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetAddFields`：`POST WEDOC_SMARTSHEET_ADD_FIELDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_ADD_FIELDS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_delete_fields(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetDeleteFields`：`POST WEDOC_SMARTSHEET_DELETE_FIELDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_DELETE_FIELDS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_update_fields(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetUpdateFields`：`POST WEDOC_SMARTSHEET_UPDATE_FIELDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_UPDATE_FIELDS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_get_records(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetGetRecords`：`POST WEDOC_SMARTSHEET_GET_RECORDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_GET_RECORDS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_add_records(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpDocSmartSheetResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetAddRecords`：`POST WEDOC_SMARTSHEET_ADD_RECORDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_ADD_RECORDS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_delete_records(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetDeleteRecords`：`POST WEDOC_SMARTSHEET_DELETE_RECORDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_DELETE_RECORDS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn smart_sheet_update_records(
        &self,
        request: &WxCpDocSmartSheetRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `smartSheetUpdateRecords`：`POST WEDOC_SMARTSHEET_UPDATE_RECORDS`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_SMARTSHEET_UPDATE_RECORDS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn form_create(
        &self,
        request: &WxCpFormCreateRequest,
    ) -> Result<WxCpFormCreateResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `formCreate`：`POST WEDOC_CREATE_FORM`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_CREATE_FORM);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn form_modify(
        &self,
        request: &WxCpFormModifyRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `formModify`：`POST WEDOC_MODIFY_FORM`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_MODIFY_FORM);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }

    async fn form_info(&self, form_id: &str) -> Result<WxCpFormInfoResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `formInfo`：`POST WEDOC_GET_FORM_INFO`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_GET_FORM_INFO);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_form_id_body(form_id)).await
    }

    async fn form_statistic(
        &self,
        requests: &[WxCpFormStatisticRequest],
    ) -> Result<WxCpFormStatisticResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `formStatistic(List)`：`POST WEDOC_GET_FORM_STATISTIC`，请求体
        // 为 `WxCpFormStatisticRequest.toJson(requests)`（普通 JSON 数组）
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_GET_FORM_STATISTIC);
        let body =
            serde_json::to_string(requests).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn form_statistic_single(
        &self,
        request: &WxCpFormStatisticRequest,
    ) -> Result<WxCpFormStatistic, WxErrorException> {
        // Java `formStatistic(WxCpFormStatisticRequest)`（default 方法）：
        // 以单元素数组请求后取 `statistic_list` 首项；列表为空时 Java
        // 返回 null → Rust 错误码 -99（ADAPTED：无 null 类型）
        let result = self.form_statistic(std::slice::from_ref(request)).await?;
        result
            .statistic_list
            .into_iter()
            .next()
            .ok_or_else(|| WxErrorException::from_code(-99, "收集表统计列表为空"))
    }

    async fn form_answer(
        &self,
        request: &WxCpFormAnswerRequest,
    ) -> Result<WxCpFormAnswer, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `formAnswer`：`POST WEDOC_GET_FORM_ANSWER`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_oa::WEDOC_GET_FORM_ANSWER);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::request_to_json(request)?).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `docDelete`：`docid`/`formid` 二选一，空参序列化为 `null`
    /// （Gson `JsonObject.addProperty` 语义）。
    #[test]
    fn test_build_doc_delete_body() {
        assert_eq!(
            WxCpOaWeDocServiceImpl::build_doc_delete_body(Some("doc1"), None),
            r#"{"docid":"doc1","formid":null}"#
        );
        assert_eq!(
            WxCpOaWeDocServiceImpl::build_doc_delete_body(None, Some("form1")),
            r#"{"docid":null,"formid":"form1"}"#
        );
    }

    /// Java `docShare(String)`：Gson 省略 null 字段，请求体仅含 `docid`。
    #[test]
    fn test_build_doc_share_body() {
        assert_eq!(
            WxCpOaWeDocServiceImpl::build_doc_share_body("doc1"),
            r#"{"docid":"doc1"}"#
        );
    }

    /// Java `formStatistic(List)`：请求体为普通 JSON 数组。
    #[test]
    fn test_form_statistic_body_is_json_array() {
        let requests = vec![WxCpFormStatisticRequest {
            repeated_id: "rep1".to_string(),
            req_type: 1,
            start_time: 1000,
            end_time: 2000,
            limit: 10,
            cursor: 0,
        }];
        let body = serde_json::to_string(&requests).expect("序列化失败");
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert!(json.is_array());
        assert_eq!(json[0]["repeated_id"], "rep1");
        assert_eq!(json[0]["req_type"], 1);
    }

    /// Java `docModifySaftySetting`（@Deprecated）：字段原样转换后委托
    /// 新接口请求体。
    #[test]
    fn test_convert_safty_to_safety() {
        let safty = WxCpDocModifySaftySettingRequest {
            doc_id: "doc1".to_string(),
            enable_readonly_copy: true,
            watermark: Default::default(),
        };
        let safety = WxCpOaWeDocServiceImpl::convert_safty_to_safety(&safty);
        assert_eq!(safety.doc_id, "doc1");
        assert!(safety.enable_readonly_copy);
        assert_eq!(
            safety.to_json().expect("序列化失败"),
            safty.to_json().expect("序列化失败")
        );
    }
}
