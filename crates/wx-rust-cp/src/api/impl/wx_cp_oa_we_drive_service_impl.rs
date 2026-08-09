//! 企业微信微盘服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpOaWeDriveServiceImpl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpOaWeDriveService, WxCpService};
use crate::bean::{
    WxCpBaseResp, WxCpFileAclAddRequest, WxCpFileAclDelRequest, WxCpFileCreate, WxCpFileDownload,
    WxCpFileInfo, WxCpFileList, WxCpFileListRequest, WxCpFileMove, WxCpFileMoveRequest,
    WxCpFileRename, WxCpFileShare, WxCpFileUpload, WxCpFileUploadRequest, WxCpSpaceAclAddRequest,
    WxCpSpaceAclDelRequest, WxCpSpaceCreateData, WxCpSpaceCreateRequest, WxCpSpaceInfo,
    WxCpSpaceRenameRequest, WxCpSpaceSettingRequest, WxCpSpaceShare,
};
use crate::enums::url_oa;

/// 企业微信微盘服务实现。
pub struct WxCpOaWeDriveServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpOaWeDriveServiceImpl {
    /// 构建微盘服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造仅含 `spaceid` 的请求体（对应 Java `spaceDismiss`/`spaceInfo`/
    /// `spaceShare` 内的 `JsonObject`：`{"spaceid": spaceId}`）。
    fn build_space_id_body(space_id: &str) -> String {
        serde_json::json!({ "spaceid": space_id }).to_string()
    }

    /// 构造仅含 `fileid` 的请求体（对应 Java `fileInfo`/`fileShare` 内的
    /// `JsonObject`：`{"fileid": fileId}`）。
    fn build_file_id_body(file_id: &str) -> String {
        serde_json::json!({ "fileid": file_id }).to_string()
    }

    /// 构造下载文件请求体（对应 Java `fileDownload(String, String)` 的
    /// `JsonObject` 语义：`fileId`/`selectedTicket` 二选一，空参序列化为
    /// `null`）。
    fn build_file_download_body(file_id: Option<&str>, selected_ticket: Option<&str>) -> String {
        serde_json::json!({
            "fileid": file_id,
            "selected_ticket": selected_ticket,
        })
        .to_string()
    }

    /// 构造重命名文件请求体（对应 Java `fileRename` 内的
    /// `JsonObject`：`{"fileid": ..., "new_name": ...}`）。
    fn build_file_rename_body(file_id: &str, new_name: &str) -> String {
        serde_json::json!({
            "fileid": file_id,
            "new_name": new_name,
        })
        .to_string()
    }

    /// 构造新建文件请求体（对应 Java `fileCreate` 内的
    /// `JsonObject`：`{"spaceid", "fatherid", "file_type", "file_name"}`）。
    fn build_file_create_body(
        space_id: &str,
        father_id: &str,
        file_type: i32,
        file_name: &str,
    ) -> String {
        serde_json::json!({
            "spaceid": space_id,
            "fatherid": father_id,
            "file_type": file_type,
            "file_name": file_name,
        })
        .to_string()
    }

    /// 构造删除文件请求体（对应 Java `fileDelete` 内
    /// `new WxCpFileDeleteRequest(fileIds).toJson()`：
    /// `{"fileid": [...]}`）。
    fn build_file_delete_body(file_ids: &[&str]) -> String {
        serde_json::json!({ "fileid": file_ids }).to_string()
    }

    /// 构造设置文件信息请求体（对应 Java `fileSetting` 内的
    /// `JsonObject`：`fileid`/`auth_scope` 必有，`auth` 非空才放入）。
    fn build_file_setting_body(file_id: &str, auth_scope: i32, auth: Option<i32>) -> String {
        let mut body = serde_json::json!({
            "fileid": file_id,
            "auth_scope": auth_scope,
        });
        if let Some(auth) = auth {
            body["auth"] = serde_json::json!(auth);
        }
        body.to_string()
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
}

/// 结果 bean 反序列化 trait（内部统一 `Xxx.from_json` 语义）。
trait FromJson {
    fn from_json(json: &str) -> Result<Self, String>
    where
        Self: Sized;
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

impl_from_json!(
    WxCpSpaceCreateData,
    WxCpBaseResp,
    WxCpSpaceInfo,
    WxCpSpaceShare,
    WxCpFileList,
    WxCpFileUpload,
    WxCpFileDownload,
    WxCpFileRename,
    WxCpFileCreate,
    WxCpFileMove,
    WxCpFileInfo,
    WxCpFileShare,
);

#[async_trait]
impl WxCpOaWeDriveService for WxCpOaWeDriveServiceImpl {
    async fn space_create(
        &self,
        request: &WxCpSpaceCreateRequest,
    ) -> Result<WxCpSpaceCreateData, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `spaceCreate`：`POST SPACE_CREATE`，返回空间 id
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SPACE_CREATE);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn space_rename(
        &self,
        request: &WxCpSpaceRenameRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `spaceRename`：`POST SPACE_RENAME`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SPACE_RENAME);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn space_dismiss(&self, space_id: &str) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `spaceDismiss`：`POST SPACE_DISMISS`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SPACE_DISMISS);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_space_id_body(space_id)).await
    }

    async fn space_info(&self, space_id: &str) -> Result<WxCpSpaceInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `spaceInfo`：`POST SPACE_INFO`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SPACE_INFO);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_space_id_body(space_id)).await
    }

    async fn space_acl_add(
        &self,
        request: &WxCpSpaceAclAddRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `spaceAclAdd`：`POST SPACE_ACL_ADD`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SPACE_ACL_ADD);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn space_acl_del(
        &self,
        request: &WxCpSpaceAclDelRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `spaceAclDel`：`POST SPACE_ACL_DEL`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SPACE_ACL_DEL);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn space_setting(
        &self,
        request: &WxCpSpaceSettingRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `spaceSetting`：`POST SPACE_SETTING`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SPACE_SETTING);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn space_share(&self, space_id: &str) -> Result<WxCpSpaceShare, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `spaceShare`：`POST SPACE_SHARE`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::SPACE_SHARE);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_space_id_body(space_id)).await
    }

    async fn file_list(
        &self,
        request: &WxCpFileListRequest,
    ) -> Result<WxCpFileList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileList`：`POST FILE_LIST`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_LIST);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn file_upload(
        &self,
        request: &WxCpFileUploadRequest,
    ) -> Result<WxCpFileUpload, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileUpload`：`POST FILE_UPLOAD`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_UPLOAD);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn file_download(
        &self,
        file_id: Option<&str>,
        selected_ticket: Option<&str>,
    ) -> Result<WxCpFileDownload, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileDownload(String, String)`：`POST FILE_DOWNLOAD`，
        // `fileid`/`selected_ticket` 二选一
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_DOWNLOAD);
        Self::post_and_parse(
            svc.as_ref(),
            &api_url,
            &Self::build_file_download_body(file_id, selected_ticket),
        )
        .await
    }

    async fn file_rename(
        &self,
        file_id: &str,
        new_name: &str,
    ) -> Result<WxCpFileRename, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileRename`：`POST FILE_RENAME`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_RENAME);
        Self::post_and_parse(
            svc.as_ref(),
            &api_url,
            &Self::build_file_rename_body(file_id, new_name),
        )
        .await
    }

    async fn file_create(
        &self,
        space_id: &str,
        father_id: &str,
        file_type: i32,
        file_name: &str,
    ) -> Result<WxCpFileCreate, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileCreate`：`POST FILE_CREATE`（`fileType`：1-文件夹，
        // 3-文档，4-表格）
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_CREATE);
        Self::post_and_parse(
            svc.as_ref(),
            &api_url,
            &Self::build_file_create_body(space_id, father_id, file_type, file_name),
        )
        .await
    }

    async fn file_move(
        &self,
        request: &WxCpFileMoveRequest,
    ) -> Result<WxCpFileMove, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileMove`：`POST FILE_MOVE`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_MOVE);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn file_delete(&self, file_ids: &[&str]) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileDelete(List)`：`POST FILE_DELETE`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_DELETE);
        Self::post_and_parse(
            svc.as_ref(),
            &api_url,
            &Self::build_file_delete_body(file_ids),
        )
        .await
    }

    async fn file_info(&self, file_id: &str) -> Result<WxCpFileInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileInfo`：`POST FILE_INFO`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_INFO);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_file_id_body(file_id)).await
    }

    async fn file_acl_add(
        &self,
        request: &WxCpFileAclAddRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileAclAdd`：`POST FILE_ACL_ADD`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_ACL_ADD);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn file_acl_del(
        &self,
        request: &WxCpFileAclDelRequest,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileAclDel`：`POST FILE_ACL_DEL`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_ACL_DEL);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        Self::post_and_parse(svc.as_ref(), &api_url, &body).await
    }

    async fn file_setting(
        &self,
        file_id: &str,
        auth_scope: i32,
        auth: Option<i32>,
    ) -> Result<WxCpBaseResp, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileSetting`：`POST FILE_SETTING`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_SETTING);
        Self::post_and_parse(
            svc.as_ref(),
            &api_url,
            &Self::build_file_setting_body(file_id, auth_scope, auth),
        )
        .await
    }

    async fn file_share(&self, file_id: &str) -> Result<WxCpFileShare, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `fileShare`：`POST FILE_SHARE`
        let api_url = svc.wx_cp_config_storage().api_url(url_oa::FILE_SHARE);
        Self::post_and_parse(svc.as_ref(), &api_url, &Self::build_file_id_body(file_id)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `fileDownload`：`fileId`/`selectedTicket` 二选一，空参
    /// 序列化为 `null`。
    #[test]
    fn test_build_file_download_body() {
        assert_eq!(
            WxCpOaWeDriveServiceImpl::build_file_download_body(Some("f1"), None),
            r#"{"fileid":"f1","selected_ticket":null}"#
        );
        assert_eq!(
            WxCpOaWeDriveServiceImpl::build_file_download_body(None, Some("t1")),
            r#"{"fileid":null,"selected_ticket":"t1"}"#
        );
    }

    /// Java `fileCreate`：请求体
    /// `{"spaceid":"s1","fatherid":"f1","file_type":3,"file_name":"文档"}`。
    #[test]
    fn test_build_file_create_body() {
        assert_eq!(
            WxCpOaWeDriveServiceImpl::build_file_create_body("s1", "f1", 3, "文档"),
            r#"{"spaceid":"s1","fatherid":"f1","file_type":3,"file_name":"文档"}"#
        );
    }

    /// Java `fileDelete`：请求体 `{"fileid":["f1","f2"]}`。
    #[test]
    fn test_build_file_delete_body() {
        assert_eq!(
            WxCpOaWeDriveServiceImpl::build_file_delete_body(&["f1", "f2"]),
            r#"{"fileid":["f1","f2"]}"#
        );
    }

    /// Java `fileSetting`：`auth` 为空时不放入请求体。
    #[test]
    fn test_build_file_setting_body() {
        let body = WxCpOaWeDriveServiceImpl::build_file_setting_body("f1", 2, None);
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["fileid"], "f1");
        assert_eq!(json["auth_scope"], 2);
        assert!(json.get("auth").is_none());

        let body = WxCpOaWeDriveServiceImpl::build_file_setting_body("f1", 2, Some(1));
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["auth"], 1);
    }

    /// Java `spaceDismiss`：请求体 `{"spaceid":"s1"}`。
    #[test]
    fn test_build_space_id_body() {
        assert_eq!(
            WxCpOaWeDriveServiceImpl::build_space_id_body("s1"),
            r#"{"spaceid":"s1"}"#
        );
    }
}
