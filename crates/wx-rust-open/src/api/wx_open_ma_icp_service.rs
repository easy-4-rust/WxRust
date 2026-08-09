//! 小程序备案服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMaIcpService`，文档：
//! <https://developers.weixin.qq.com/doc/oplatform/openApi/OpenApiDoc/miniprogram-management/record/queryIcpVerifyTask.html>
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]（`ma_icp_*_url`，
//! api_host 前缀模式）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::WxOpenApplyIcpFilingParam;
use crate::bean::WxOpenApplyIcpFilingResult;
use crate::bean::WxOpenIcpCreateIcpVerifyTaskResult;
use crate::bean::WxOpenIcpEntranceInfoResult;
use crate::bean::WxOpenIcpVerifyTaskResult;
use crate::bean::WxOpenOnlineIcpOrderResult;
use crate::bean::WxOpenQueryIcpCertificateTypeResult;
use crate::bean::WxOpenQueryIcpDistrictCodeResult;
use crate::bean::WxOpenQueryIcpNrlxTypesResult;
use crate::bean::WxOpenQueryIcpServiceContentTypesResult;
use crate::bean::WxOpenQueryIcpSubjectTypeResult;
use crate::bean::WxOpenResult;
use crate::bean::WxOpenUploadIcpMediaParam;
use crate::bean::WxOpenUploadIcpMediaResult;

/// 微信第三方平台 小程序备案服务（对应 Java `WxOpenMaIcpService`）。
#[async_trait]
pub trait WxOpenMaIcpService: Send + Sync {
    /// 查询人脸核身任务状态（对应 Java
    /// `queryIcpVerifyTask(String taskId)`）。
    async fn query_icp_verify_task(
        &self,
        task_id: &str,
    ) -> Result<WxOpenIcpVerifyTaskResult, WxErrorException>;

    /// 发起小程序管理员人脸核身（对应 Java
    /// `createIcpVerifyTask(boolean alongWithAuth)`）。
    ///
    /// `along_with_auth`：小程序认证及备案二合一场景填 true，否则为
    /// 小程序备案场景，默认值 false。
    async fn create_icp_verify_task(
        &self,
        along_with_auth: bool,
    ) -> Result<WxOpenIcpCreateIcpVerifyTaskResult, WxErrorException>;

    /// 上传小程序备案媒体材料（对应 Java
    /// `uploadIcpMedia(WxOpenUploadIcpMediaParam param)`，multipart：
    /// 普通表单参数 + `media` 文件，对应 Java
    /// `CommonUploadMultiRequestExecutor` 语义）。
    async fn upload_icp_media(
        &self,
        param: &WxOpenUploadIcpMediaParam,
    ) -> Result<WxOpenUploadIcpMediaResult, WxErrorException>;

    /// 撤回小程序备案申请（对应 Java `cancelApplyIcpFiling()`）。
    async fn cancel_apply_icp_filing(&self) -> Result<WxOpenResult, WxErrorException>;

    /// 申请小程序备案（对应 Java
    /// `applyIcpFiling(WxOpenApplyIcpFilingParam param)`）。
    async fn apply_icp_filing(
        &self,
        param: &WxOpenApplyIcpFilingParam,
    ) -> Result<WxOpenApplyIcpFilingResult, WxErrorException>;

    /// 注销小程序备案（对应 Java
    /// `cancelIcpFiling(Integer cancelType)`；`cancel_type`：1 注销主体，
    /// 2 注销小程序，3 注销微信小程序）。
    async fn cancel_icp_filing(&self, cancel_type: i32) -> Result<WxOpenResult, WxErrorException>;

    /// 获取小程序备案状态及驳回原因（对应 Java `getIcpEntranceInfo()`，
    /// GET 请求）。
    async fn get_icp_entrance_info(&self) -> Result<WxOpenIcpEntranceInfoResult, WxErrorException>;

    /// 获取小程序已备案详情（对应 Java `getOnlineIcpOrder()`，GET 请求）。
    async fn get_online_icp_order(&self) -> Result<WxOpenOnlineIcpOrderResult, WxErrorException>;

    /// 获取小程序服务内容类型（对应 Java
    /// `queryIcpServiceContentTypes()`，GET 请求）。
    async fn query_icp_service_content_types(
        &self,
    ) -> Result<WxOpenQueryIcpServiceContentTypesResult, WxErrorException>;

    /// 获取证件类型（对应 Java `queryIcpCertificateTypes()`，GET 请求）。
    async fn query_icp_certificate_types(
        &self,
    ) -> Result<WxOpenQueryIcpCertificateTypeResult, WxErrorException>;

    /// 获取区域信息（对应 Java `queryIcpDistrictCode()`，GET 请求）。
    async fn query_icp_district_code(
        &self,
    ) -> Result<WxOpenQueryIcpDistrictCodeResult, WxErrorException>;

    /// 获取前置审批项类型（对应 Java `queryIcpNrlxTypes()`，GET 请求）。
    async fn query_icp_nrlx_types(&self)
    -> Result<WxOpenQueryIcpNrlxTypesResult, WxErrorException>;

    /// 获取单位性质（对应 Java `queryIcpSubjectTypes()`，GET 请求）。
    async fn query_icp_subject_types(
        &self,
    ) -> Result<WxOpenQueryIcpSubjectTypeResult, WxErrorException>;

    /// 获取小程序备案媒体材料（对应 Java
    /// `getIcpMedia(String mediaId)`，GET 下载）。
    ///
    /// ADAPTED：Java 返回 `File`（BaseMediaDownloadRequestExecutor 下载
    /// 到临时目录），Rust 以 `&str` 目标文件路径入参、下载写入该路径，
    /// 返回 `()`。
    async fn get_icp_media(
        &self,
        media_id: &str,
        target_path: &str,
    ) -> Result<(), WxErrorException>;
}
