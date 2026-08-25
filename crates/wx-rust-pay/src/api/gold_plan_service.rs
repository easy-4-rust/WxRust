//! 点金计划服务。
//!
//! 对应 Java `com.github.binarywang.wxpay.service.GoldPlanService`。
//!
//! 产品介绍: <https://pay.weixin.qq.com/doc/v3/partner/4012072130>

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::goldplan::gold_plan_result::GoldPlanResult;

/// 点金计划服务（对应 Java `GoldPlanService`）。
#[async_trait]
pub trait GoldPlanService: Send + Sync {
    /// 为特约商户开通点金计划。
    ///
    /// 对应 Java: `GoldPlanService#openGoldPlan`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4012473796>
    ///
    /// # 参数
    /// - `sub_mch_id`: 特约商户号
    /// - `operation_pay_scene`: 支付场景，可选值为 `JSAPI_AND_MINIPROGRAM`、`JSAPI`、`MINIPROGRAM`；不传时默认为 `JSAPI`
    async fn open_gold_plan(
        &self,
        sub_mch_id: &str,
        operation_pay_scene: Option<&str>,
    ) -> Result<GoldPlanResult, WxErrorException>;

    /// 为特约商户关闭点金计划。
    ///
    /// 对应 Java: `GoldPlanService#closeGoldPlan`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4012473796>
    ///
    /// # 参数
    /// - `sub_mch_id`: 特约商户号
    /// - `operation_pay_scene`: 支付场景，可选值为 `JSAPI_AND_MINIPROGRAM`、`JSAPI`、`MINIPROGRAM`；不传时默认为 `JSAPI`
    async fn close_gold_plan(
        &self,
        sub_mch_id: &str,
        operation_pay_scene: Option<&str>,
    ) -> Result<GoldPlanResult, WxErrorException>;

    /// 为特约商户开通商家小票。
    ///
    /// 对应 Java: `GoldPlanService#openCustomPage`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4012473788>
    ///
    /// # 参数
    /// - `sub_mch_id`: 特约商户号
    async fn open_custom_page(
        &self,
        sub_mch_id: &str,
    ) -> Result<GoldPlanResult, WxErrorException>;

    /// 为特约商户关闭商家小票。
    ///
    /// 对应 Java: `GoldPlanService#closeCustomPage`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4012473788>
    ///
    /// # 参数
    /// - `sub_mch_id`: 特约商户号
    async fn close_custom_page(
        &self,
        sub_mch_id: &str,
    ) -> Result<GoldPlanResult, WxErrorException>;

    /// 设置特约商户的点金计划同业过滤标签。
    ///
    /// 对应 Java: `GoldPlanService#setAdvertisingIndustryFilter`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4012473784>
    ///
    /// # 参数
    /// - `sub_mch_id`: 特约商户号
    /// - `advertising_industry_filters`: 同业过滤标签，最少一个，最多三个
    async fn set_advertising_industry_filter(
        &self,
        sub_mch_id: &str,
        advertising_industry_filters: &[String],
    ) -> Result<(), WxErrorException>;

    /// 为特约商户的点金计划页面开通广告展示。
    ///
    /// 对应 Java: `GoldPlanService#openAdvertisingShow`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4012473794>
    ///
    /// # 参数
    /// - `sub_mch_id`: 特约商户号
    /// - `advertising_industry_filters`: 同业过滤标签，可选，最多三个
    async fn open_advertising_show(
        &self,
        sub_mch_id: &str,
        advertising_industry_filters: Option<&[String]>,
    ) -> Result<(), WxErrorException>;

    /// 为特约商户的点金计划页面关闭广告展示。
    ///
    /// 对应 Java: `GoldPlanService#closeAdvertisingShow`
    ///
    /// 接口文档: <https://pay.weixin.qq.com/doc/v3/partner/4012473781>
    ///
    /// # 参数
    /// - `sub_mch_id`: 特约商户号
    async fn close_advertising_show(
        &self,
        sub_mch_id: &str,
    ) -> Result<(), WxErrorException>;
}
