//! 对应 Java `com.github.binarywang.wxpay.service.Applyment4SubService`。
//!
//! 由 `scripts/gen_pay_sub_service_traits.py` 从 Java 接口签名生成
//! （Wave 5 P5），方法体在 `api/impl/*_service_impl.rs` 镜像 Java
//! `service.impl.*ServiceImpl`；`File`/`InputStream` 媒体参数 ADAPTED
//! 为 `(文件名, 字节)`，Java 泛型返回值以 `serde_json::Value` 类型擦除。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::*;

/// Applyment4SubService（对应 Java `Applyment4SubService`）。
#[async_trait]
pub trait Applyment4SubService: Send + Sync {
    /// 特约商户进件 产品介绍：https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/tool/applyment4sub/chapter1_1.shtml
    async fn create_apply(
        &self,
        request: &WxPayApplyment4SubCreateRequest,
    ) -> Result<WxPayApplymentCreateResult, WxErrorException>;

    /// 通过业务申请编号查询申请状态 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/tool/applyment4sub/chapter3_2.sh
    async fn query_apply_status_by_business_code(
        &self,
        business_code: &str,
    ) -> Result<ApplymentStateQueryResult, WxErrorException>;

    /// 通过申请单号查询申请状态 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3/wxpay/tool/applyment4sub/chapter3_2.shtm
    async fn query_apply_status_by_applyment_id(
        &self,
        applyment_id: &str,
    ) -> Result<ApplymentStateQueryResult, WxErrorException>;

    /// 根据特约子商户ID查询结算账户 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3_partner/apis/chapter11_1_4.shtml 接口链接
    async fn query_settlement_by_sub_mchid(
        &self,
        sub_mchid: &str,
    ) -> Result<SettlementInfoResult, WxErrorException>;

    /// 修改结算帐号 文档详见: https://pay.weixin.qq.com/wiki/doc/apiv3_partner/apis/chapter11_1_3.shtml 接口链接：https://
    async fn modify_settlement(
        &self,
        sub_mchid: &str,
        request: &ModifySettlementRequest,
    ) -> Result<String, WxErrorException>;

    /// 查询结算账户修改申请状态 接口链接：https://api.mch.weixin.qq.com/v3/apply4sub/sub_merchants/{sub_mchid}/application/{
    async fn query_settlement_modify_status_by_application_no(
        &self,
        sub_mchid: &str,
        application_no: &str,
    ) -> Result<SettlementModifyStateQueryResult, WxErrorException>;
}
