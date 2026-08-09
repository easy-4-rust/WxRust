//! 小程序认证（年审）服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMaAuthService`。
//! 微信第三方平台 小程序认证接口（年审），文档：
//! <https://developers.weixin.qq.com/doc/oplatform/Third-party_Platforms/2.0/product/weapp_wxverify.html>
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]（`ma_auth_*_url`，
//! api_host 前缀模式）。

use async_trait::async_trait;

use wx_rust_common::bean::CommonUploadData;
use wx_rust_common::error::WxErrorException;

use crate::bean::MaAuthQueryIdentityTreeResult;
use crate::bean::MaAuthQueryResult;
use crate::bean::MaAuthResubmitParam;
use crate::bean::MaAuthSubmitParam;
use crate::bean::MaAuthSubmitResult;
use crate::bean::MaAuthUploadResult;

/// 微信第三方平台 小程序认证服务（对应 Java `WxOpenMaAuthService`）。
#[async_trait]
pub trait WxOpenMaAuthService: Send + Sync {
    /// 小程序认证（提审）（对应 Java `submit(MaAuthSubmitParam param)`）。
    ///
    /// 返回提交结果，须保存任务 ID 和授权链接。
    async fn submit(
        &self,
        param: &MaAuthSubmitParam,
    ) -> Result<MaAuthSubmitResult, WxErrorException>;

    /// 进度查询（对应 Java `query(String taskId)`）。
    ///
    /// `task_id`：提交任务时返回的任务 ID。
    async fn query(&self, task_id: &str) -> Result<MaAuthQueryResult, WxErrorException>;

    /// 上传补充材料（对应 Java `upload(CommonUploadData data)`，
    /// multipart 字段名 `media`）。
    ///
    /// 仅支持 png/jpeg/jpg/gif 格式，文件后缀名如果填写不对会导致上传
    /// 失败，建议写死 1.jpg。
    async fn upload(&self, data: &CommonUploadData)
    -> Result<MaAuthUploadResult, WxErrorException>;

    /// 重新提审（对应 Java `resubmit(MaAuthResubmitParam param)`）。
    async fn resubmit(
        &self,
        param: &MaAuthResubmitParam,
    ) -> Result<MaAuthSubmitResult, WxErrorException>;

    /// 查询个人认证身份选项列表（对应 Java `queryIdentityTree()`，
    /// GET 请求）。
    async fn query_identity_tree(&self) -> Result<MaAuthQueryIdentityTreeResult, WxErrorException>;
}
