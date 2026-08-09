//! 对应 Java `com.github.binarywang.wxpay.service.Apply4SubjectConfirmService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// Apply4SubjectConfirmService（对应 Java `Apply4SubjectConfirmService`）。
#[async_trait]
pub trait Apply4SubjectConfirmService: Send + Sync {
    /// 商户开户意愿确认 产品文档：商户开户意愿确认流程
    async fn applyment(
        &self,
        request: &ApplySubjectConfirmCreateRequest,
    ) -> Result<ApplySubjectConfirmCreateResult, WxErrorException>;

    /// 查询申请单审核结果 详情请见: 查询申请单审核结果
    async fn query_apply_status_by_business_code(
        &self,
        business_code: &str,
    ) -> Result<ApplySubjectConfirmStateQueryResult, WxErrorException>;

    /// 查询申请单审核结果 详情请见: 查询申请单审核结果
    async fn query_apply_status_by_applyment_id(
        &self,
        applyment_id: &str,
    ) -> Result<ApplySubjectConfirmStateQueryResult, WxErrorException>;

    /// 获取商户开户意愿确认状态 详情请见: 获取商户开户意愿确认状态API
    async fn query_merchant_apply_status_by_mch_id(
        &self,
        sub_mch_id: &str,
    ) -> Result<ApplySubjectConfirmMerchantStateQueryResult, WxErrorException>;

    /// 撤销申请单 详情请见: 撤销申请单API
    async fn cancel_apply_by_business_code(
        &self,
        business_code: &str,
    ) -> Result<(), WxErrorException>;

    /// 撤销申请单 详情请见: 撤销申请单API
    async fn cancel_apply_by_applyment_id(
        &self,
        applyment_id: &str,
    ) -> Result<(), WxErrorException>;
}
