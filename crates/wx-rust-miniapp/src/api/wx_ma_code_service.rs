//! 小程序代码管理相关 API（大部分只能是第三方平台调用）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaCodeService`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::{
    WxMaCodeAuditStatus, WxMaCodeCommitRequest, WxMaCodeSubmitAuditItem,
    WxMaCodeSubmitAuditRequest, WxMaCodeVersionDistribution, WxMaCodeVersionInfo,
};

/// 小程序代码管理相关 API。
#[async_trait]
pub trait WxMaCodeService: Send + Sync {
    /// 为授权的小程序帐号上传小程序代码（对应 Java `commit(WxMaCodeCommitRequest)`，
    /// 仅仅支持第三方开放平台）。
    async fn commit(&self, commit_request: &WxMaCodeCommitRequest) -> Result<(), WxErrorException>;

    /// 获取体验小程序的体验二维码（对应 Java `getQrCode(String)`）。
    ///
    /// Java 内部将 path 做 `URLEncoder.encode(path, "UTF-8")` 后拼入
    /// `?path=` 查询参数，下载成功后返回 bytes（临时文件即删）；Rust 直接
    /// 返回二维码字节 `Vec<u8>`（ADAPTED）。
    async fn get_qr_code(&self, path: &str) -> Result<Vec<u8>, WxErrorException>;

    /// 获取授权小程序帐号的可选类目（对应 Java `getCategory()`）。
    ///
    /// 响应无 `category_list` 字段时返回 `None`（Java 返回 null）。
    async fn get_category(&self) -> Result<Option<Vec<WxMaCodeSubmitAuditItem>>, WxErrorException>;

    /// 获取小程序的第三方提交代码的页面配置（对应 Java `getPage()`）。
    ///
    /// 响应无 `page_list` 字段时返回 `None`（Java 返回 null）。
    async fn get_page(&self) -> Result<Option<Vec<String>>, WxErrorException>;

    /// 将第三方提交的代码包提交审核（对应 Java `submitAudit(WxMaCodeSubmitAuditRequest)`，
    /// 返回审核编号 `auditid`）。
    async fn submit_audit(
        &self,
        audit_request: &WxMaCodeSubmitAuditRequest,
    ) -> Result<i64, WxErrorException>;

    /// 查询某个指定版本的审核状态（对应 Java `getAuditStatus(long)`）。
    async fn get_audit_status(
        &self,
        audit_id: i64,
    ) -> Result<WxMaCodeAuditStatus, WxErrorException>;

    /// 查询最新一次提交的审核状态（对应 Java `getLatestAuditStatus()`）。
    async fn get_latest_audit_status(&self) -> Result<WxMaCodeAuditStatus, WxErrorException>;

    /// 发布已通过审核的小程序（对应 Java `release()`）。
    async fn release(&self) -> Result<(), WxErrorException>;

    /// 修改小程序线上代码的可见状态（对应 Java `changeVisitStatus(String)`）。
    ///
    /// `action`：close 为不可见，open 为可见。
    async fn change_visit_status(&self, action: &str) -> Result<(), WxErrorException>;

    /// 小程序版本回退（对应 Java `revertCodeRelease()`）。
    async fn revert_code_release(&self) -> Result<(), WxErrorException>;

    /// 查询当前设置的最低基础库版本及各版本用户占比（对应 Java
    /// `getSupportVersion()`）。
    async fn get_support_version(&self) -> Result<WxMaCodeVersionDistribution, WxErrorException>;

    /// 查询小程序版本信息（对应 Java `getVersionInfo()`）。
    async fn get_version_info(&self) -> Result<WxMaCodeVersionInfo, WxErrorException>;

    /// 设置最低基础库版本（对应 Java `setSupportVersion(String)`）。
    async fn set_support_version(&self, version: &str) -> Result<(), WxErrorException>;

    /// 小程序审核撤回（对应 Java `undoCodeAudit()`）。
    async fn undo_code_audit(&self) -> Result<(), WxErrorException>;
}
