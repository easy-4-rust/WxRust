//! 企业微信微盘服务。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.WxCpOaWeDriveService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxCpBaseResp, WxCpFileAclAddRequest, WxCpFileAclDelRequest, WxCpFileCreate, WxCpFileDownload,
    WxCpFileInfo, WxCpFileList, WxCpFileListRequest, WxCpFileMove, WxCpFileMoveRequest,
    WxCpFileRename, WxCpFileShare, WxCpFileUpload, WxCpFileUploadRequest, WxCpSpaceAclAddRequest,
    WxCpSpaceAclDelRequest, WxCpSpaceCreateData, WxCpSpaceCreateRequest, WxCpSpaceInfo,
    WxCpSpaceRenameRequest, WxCpSpaceSettingRequest, WxCpSpaceShare,
};

/// 企业微信微盘服务。
#[async_trait]
pub trait WxCpOaWeDriveService: Send + Sync {
    /// 新建空间（对应 Java
    /// `WxCpOaWeDriveService.spaceCreate(WxCpSpaceCreateRequest)`，
    /// 返回空间 id）。
    async fn space_create(
        &self,
        request: &WxCpSpaceCreateRequest,
    ) -> Result<WxCpSpaceCreateData, WxErrorException>;

    /// 重命名空间（对应 Java
    /// `WxCpOaWeDriveService.spaceRename(WxCpSpaceRenameRequest)`）。
    async fn space_rename(
        &self,
        request: &WxCpSpaceRenameRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 解散空间（对应 Java `WxCpOaWeDriveService.spaceDismiss(String)`）。
    async fn space_dismiss(&self, space_id: &str) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取空间信息（对应 Java `WxCpOaWeDriveService.spaceInfo(String)`）。
    async fn space_info(&self, space_id: &str) -> Result<WxCpSpaceInfo, WxErrorException>;

    /// 添加成员/部门（对应 Java
    /// `WxCpOaWeDriveService.spaceAclAdd(WxCpSpaceAclAddRequest)`）。
    async fn space_acl_add(
        &self,
        request: &WxCpSpaceAclAddRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 移除成员/部门（对应 Java
    /// `WxCpOaWeDriveService.spaceAclDel(WxCpSpaceAclDelRequest)`）。
    async fn space_acl_del(
        &self,
        request: &WxCpSpaceAclDelRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 权限管理（对应 Java
    /// `WxCpOaWeDriveService.spaceSetting(WxCpSpaceSettingRequest)`）。
    async fn space_setting(
        &self,
        request: &WxCpSpaceSettingRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取邀请链接（对应 Java `WxCpOaWeDriveService.spaceShare(String)`）。
    async fn space_share(&self, space_id: &str) -> Result<WxCpSpaceShare, WxErrorException>;

    /// 获取文件列表（对应 Java
    /// `WxCpOaWeDriveService.fileList(WxCpFileListRequest)`）。
    async fn file_list(
        &self,
        request: &WxCpFileListRequest,
    ) -> Result<WxCpFileList, WxErrorException>;

    /// 上传文件（对应 Java
    /// `WxCpOaWeDriveService.fileUpload(WxCpFileUploadRequest)`）。
    async fn file_upload(
        &self,
        request: &WxCpFileUploadRequest,
    ) -> Result<WxCpFileUpload, WxErrorException>;

    /// 下载文件（对应 Java
    /// `WxCpOaWeDriveService.fileDownload(String, String)`；
    /// `fileId`/`selectedTicket` 二选一）。
    async fn file_download(
        &self,
        file_id: Option<&str>,
        selected_ticket: Option<&str>,
    ) -> Result<WxCpFileDownload, WxErrorException>;

    /// 重命名文件（对应 Java
    /// `WxCpOaWeDriveService.fileRename(String, String)`）。
    async fn file_rename(
        &self,
        file_id: &str,
        new_name: &str,
    ) -> Result<WxCpFileRename, WxErrorException>;

    /// 新建文件夹/文档（对应 Java
    /// `WxCpOaWeDriveService.fileCreate(String, String, Integer, String)`；
    /// `fileType`：1-文件夹，3-文档，4-表格）。
    async fn file_create(
        &self,
        space_id: &str,
        father_id: &str,
        file_type: i32,
        file_name: &str,
    ) -> Result<WxCpFileCreate, WxErrorException>;

    /// 移动文件（对应 Java
    /// `WxCpOaWeDriveService.fileMove(WxCpFileMoveRequest)`）。
    async fn file_move(
        &self,
        request: &WxCpFileMoveRequest,
    ) -> Result<WxCpFileMove, WxErrorException>;

    /// 删除文件（对应 Java
    /// `WxCpOaWeDriveService.fileDelete(List<String>)`）。
    async fn file_delete(&self, file_ids: &[&str]) -> Result<WxCpBaseResp, WxErrorException>;

    /// 文件信息（对应 Java `WxCpOaWeDriveService.fileInfo(String)`）。
    async fn file_info(&self, file_id: &str) -> Result<WxCpFileInfo, WxErrorException>;

    /// 新增指定人（对应 Java
    /// `WxCpOaWeDriveService.fileAclAdd(WxCpFileAclAddRequest)`）。
    async fn file_acl_add(
        &self,
        request: &WxCpFileAclAddRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 删除指定人（对应 Java
    /// `WxCpOaWeDriveService.fileAclDel(WxCpFileAclDelRequest)`）。
    async fn file_acl_del(
        &self,
        request: &WxCpFileAclDelRequest,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 分享设置（对应 Java
    /// `WxCpOaWeDriveService.fileSetting(String, Integer, Integer)`）。
    async fn file_setting(
        &self,
        file_id: &str,
        auth_scope: i32,
        auth: Option<i32>,
    ) -> Result<WxCpBaseResp, WxErrorException>;

    /// 获取分享链接（对应 Java `WxCpOaWeDriveService.fileShare(String)`）。
    async fn file_share(&self, file_id: &str) -> Result<WxCpFileShare, WxErrorException>;
}
