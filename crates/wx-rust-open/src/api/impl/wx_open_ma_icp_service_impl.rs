//! 小程序备案服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMaIcpServiceImpl`
//! （持有 `WxMaService`——代 ma 桥接服务）。
//!
//! 上传/下载说明：
//! - `uploadIcpMedia`：对应 Java `CommonUploadMultiRequestExecutor`
//!   （multipart：普通表单参数 + `media` 文件）。Rust 以
//!   [`MediaUploadRequestExecutor`]（`CommonUploadParam::with_form_fields`
//!   承载普通表单参数）+ 代 ma 执行引擎承载（ADAPTED：执行器语义等价，
//!   线格式一致）。
//! - `getIcpMedia`：对应 Java `BaseMediaDownloadRequestExecutor`
//!   （GET 下载到临时目录）。ADAPTED：Rust 以 `&str` 目标文件路径入参
//!   （接口签名冻结，见 [`crate::api::WxOpenMaIcpService`]），经
//!   [`MediaDownloadRequestExecutor`] 下载写入；下载失败时抛
//!   `-99`（Java IOException 包装为 `WxErrorException`）。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::bean::CommonUploadParam;
use wx_rust_common::error::WxErrorException;
use wx_rust_common::util::http::{
    MediaDownloadRequestExecutor, MediaUploadRequestExecutor, RequestExecutor,
};
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMaIcpService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
use crate::api::r#impl::base_wx_open_service_impl;
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
use crate::enums::url_ma_domain::{
    ma_icp_apply_filing_url, ma_icp_cancel_apply_filing_url, ma_icp_cancel_filing_url,
    ma_icp_create_verify_task_url, ma_icp_get_entrance_info_url, ma_icp_get_media_url,
    ma_icp_get_online_order_url, ma_icp_query_certificate_types_url,
    ma_icp_query_district_code_url, ma_icp_query_nrlx_types_url,
    ma_icp_query_service_content_types_url, ma_icp_query_subject_types_url,
    ma_icp_query_verify_task_url, ma_icp_upload_media_url,
};

/// 小程序备案服务实现（对应 Java `WxOpenMaIcpServiceImpl`）。
pub struct WxOpenMaIcpServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    app_id: String,
}

impl WxOpenMaIcpServiceImpl {
    /// 构建服务（对应 Java `new WxOpenMaIcpServiceImpl(WxMaService)`）。
    pub fn new(wx_open_service: Arc<dyn WxOpenService>, app_id: String) -> Self {
        Self {
            wx_open_service: Arc::downgrade(&wx_open_service),
            app_id,
        }
    }

    /// 授权方 appid（Java 构造入参，代运营目标账号）。
    pub fn app_id(&self) -> &str {
        &self.app_id
    }

    fn svc(&self) -> Result<Arc<dyn WxOpenService>, WxErrorException> {
        self.wx_open_service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "门面服务已被释放"))
    }

    /// 取代 ma 桥接服务（同
    /// [`WxOpenMaAuthServiceImpl::ma_service`]）。
    fn ma_service(&self) -> Result<Arc<dyn WxMaService>, WxErrorException> {
        let svc = self.svc()?;
        let component = svc.wx_open_component_service().ok_or_else(|| {
            WxErrorException::from_code(
                -99,
                "组件子服务未装配（getWxOpenComponentService 返回 null）",
            )
        })?;
        let any = component
            .get_wx_ma_service_by_appid(&self.app_id)
            .ok_or_else(|| WxErrorException::from_code(-99, "getWxMaServiceByAppid 返回 None"))?;
        let ma = any.downcast::<WxOpenMaService>().map_err(|_| {
            WxErrorException::from_code(-99, "代 ma 服务 downcast 失败（缓存类型不匹配）")
        })?;
        Ok(ma as Arc<dyn WxMaService>)
    }
}

#[async_trait]
impl WxOpenMaIcpService for WxOpenMaIcpServiceImpl {
    /// 查询人脸核身任务状态（对应 Java
    /// `queryIcpVerifyTask(String taskId)`：POST `{"task_id": ...}`）。
    async fn query_icp_verify_task(
        &self,
        task_id: &str,
    ) -> Result<WxOpenIcpVerifyTaskResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "task_id": task_id });
        let response = ma
            .post(
                &ma_icp_query_verify_task_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 发起小程序管理员人脸核身（对应 Java
    /// `createIcpVerifyTask(boolean alongWithAuth)`：POST
    /// `{"along_with_auth": ...}`）。
    async fn create_icp_verify_task(
        &self,
        along_with_auth: bool,
    ) -> Result<WxOpenIcpCreateIcpVerifyTaskResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "along_with_auth": along_with_auth });
        let response = ma
            .post(
                &ma_icp_create_verify_task_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 上传小程序备案媒体材料（对应 Java
    /// `uploadIcpMedia(WxOpenUploadIcpMediaParam param)`：multipart
    /// 普通表单参数 + `media` 文件，对应 Java
    /// `CommonUploadMultiRequestExecutor` 语义）。
    async fn upload_icp_media(
        &self,
        param: &WxOpenUploadIcpMediaParam,
    ) -> Result<WxOpenUploadIcpMediaResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        // Java `param.toCommonUploadMultiParam()`：normal_params + upload_param
        let multi = param.to_common_upload_multi_param();
        let mut form_fields = std::collections::HashMap::new();
        for normal in &multi.normal_params {
            form_fields.insert(normal.name.clone(), normal.value.clone());
        }
        let upload_param = multi.upload_param.ok_or_else(|| {
            WxErrorException::from_code(-99, "uploadIcpMedia 缺少 media 文件（Java media 为 null）")
        })?;
        let executor = MediaUploadRequestExecutor::new(ma.http_client().clone());
        let param = CommonUploadParam::with_form_fields(
            upload_param.name.clone(),
            upload_param.data.clone(),
            form_fields,
        );
        let response = wx_rust_miniapp::api::r#impl::base_wx_ma_service_impl::execute_with_retry(
            ma.as_ref(),
            &executor,
            &ma_icp_upload_media_url(config.as_ref()),
            param,
        )
        .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 撤回小程序备案申请（对应 Java `cancelApplyIcpFiling()`：POST
    /// 空数据包）。
    async fn cancel_apply_icp_filing(&self) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .post(&ma_icp_cancel_apply_filing_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 申请小程序备案（对应 Java
    /// `applyIcpFiling(WxOpenApplyIcpFilingParam param)`：POST 参数
    /// 序列化）。
    async fn apply_icp_filing(
        &self,
        param: &WxOpenApplyIcpFilingParam,
    ) -> Result<WxOpenApplyIcpFilingResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(param).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(&ma_icp_apply_filing_url(config.as_ref()), &body)
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 注销小程序备案（对应 Java
    /// `cancelIcpFiling(Integer cancelType)`：POST
    /// `{"cancel_type": ...}`；1 注销主体，2 注销小程序，3 注销微信
    /// 小程序）。
    async fn cancel_icp_filing(&self, cancel_type: i32) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body = serde_json::json!({ "cancel_type": cancel_type });
        let response = ma
            .post(
                &ma_icp_cancel_filing_url(config.as_ref()),
                &body.to_string(),
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取小程序备案状态及驳回原因（对应 Java `getIcpEntranceInfo()`：
    /// GET `null` 参数）。
    async fn get_icp_entrance_info(&self) -> Result<WxOpenIcpEntranceInfoResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_icp_get_entrance_info_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取小程序已备案详情（对应 Java `getOnlineIcpOrder()`：GET
    /// `null` 参数）。
    async fn get_online_icp_order(&self) -> Result<WxOpenOnlineIcpOrderResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_icp_get_online_order_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取小程序服务内容类型（对应 Java
    /// `queryIcpServiceContentTypes()`：GET `null` 参数）。
    async fn query_icp_service_content_types(
        &self,
    ) -> Result<WxOpenQueryIcpServiceContentTypesResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_icp_query_service_content_types_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取证件类型（对应 Java `queryIcpCertificateTypes()`：GET
    /// `null` 参数）。
    async fn query_icp_certificate_types(
        &self,
    ) -> Result<WxOpenQueryIcpCertificateTypeResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_icp_query_certificate_types_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取区域信息（对应 Java `queryIcpDistrictCode()`：GET
    /// `null` 参数）。
    async fn query_icp_district_code(
        &self,
    ) -> Result<WxOpenQueryIcpDistrictCodeResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_icp_query_district_code_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取前置审批项类型（对应 Java `queryIcpNrlxTypes()`：GET
    /// `null` 参数）。
    async fn query_icp_nrlx_types(
        &self,
    ) -> Result<WxOpenQueryIcpNrlxTypesResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_icp_query_nrlx_types_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取单位性质（对应 Java `queryIcpSubjectTypes()`：GET
    /// `null` 参数）。
    async fn query_icp_subject_types(
        &self,
    ) -> Result<WxOpenQueryIcpSubjectTypeResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .get(&ma_icp_query_subject_types_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 获取小程序备案媒体材料（对应 Java
    /// `getIcpMedia(String mediaId)`：GET `media_id={mediaId}` 下载到
    /// 目标文件路径）。
    async fn get_icp_media(
        &self,
        media_id: &str,
        target_path: &str,
    ) -> Result<(), WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let executor = MediaDownloadRequestExecutor::new(ma.http_client().clone());
        let uri = format!(
            "{}?media_id={media_id}",
            ma_icp_get_media_url(config.as_ref())
        );
        let downloaded = executor
            .execute(&uri, String::new(), wx_rust_common::enums::WxType::MiniApp)
            .await?;
        std::fs::write(target_path, downloaded)
            .map_err(|e| WxErrorException::from_code(-99, format!("写入备案媒体文件失败: {e}")))
    }
}
