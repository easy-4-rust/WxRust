//! 购物订单服务实现。
//!
//! 对应 Java `me.chanjar.weixin.open.api.impl.WxOpenMaShoppingOrdersServiceImpl`
//! （`@AllArgsConstructor` 持有 `WxMaService`——代 ma 桥接服务）。
//!
//! Java 将入参实体经 `WxOpenGsonBuilder.toJson` 序列化后 POST；Rust 以
//! bean serde（rename 与 Java `@SerializedName` 对齐）序列化，线格式一致。

use std::sync::{Arc, Weak};

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
use wx_rust_miniapp::api::WxMaService;

use crate::api::WxOpenMaShoppingOrdersService;
use crate::api::WxOpenService;
use crate::api::r#impl::WxOpenMaService;
use crate::api::r#impl::base_wx_open_service_impl;
use crate::bean::CombinedShippingInfo;
use crate::bean::CombinedShoppingInfo;
use crate::bean::ShippingInfo;
use crate::bean::ShoppingInfo;
use crate::bean::ShoppingInfoVerifyUpload;
use crate::bean::WxOpenResult;
use crate::bean::WxOpenShoppingInfoVerifyUploadResult;
use crate::bean::WxOpenShoppingOrdersConfirmResult;
use crate::enums::url_ma_domain::{
    ma_orders_confirm_permission_url, ma_orders_open_permission_url,
    ma_orders_upload_combined_shipping_info_url, ma_orders_upload_combined_shopping_info_url,
    ma_orders_upload_shipping_info_url, ma_orders_upload_shopping_info_url,
    ma_orders_verify_upload_url,
};

/// 购物订单服务实现（对应 Java `WxOpenMaShoppingOrdersServiceImpl`）。
pub struct WxOpenMaShoppingOrdersServiceImpl {
    wx_open_service: Weak<dyn WxOpenService>,
    app_id: String,
}

impl WxOpenMaShoppingOrdersServiceImpl {
    /// 构建服务（对应 Java
    /// `new WxOpenMaShoppingOrdersServiceImpl(WxMaService)`）。
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
impl WxOpenMaShoppingOrdersService for WxOpenMaShoppingOrdersServiceImpl {
    /// 上传购物详情（对应 Java `upload(ShoppingInfo info)`：POST 实体
    /// 序列化 → `WxOpenResult`）。
    async fn upload_shopping_info(
        &self,
        info: &ShoppingInfo,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(&ma_orders_upload_shopping_info_url(config.as_ref()), &body)
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 上传物流信息（对应 Java `upload(ShippingInfo info)`）。
    async fn upload_shipping_info(
        &self,
        info: &ShippingInfo,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(&ma_orders_upload_shipping_info_url(config.as_ref()), &body)
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 上传合单购物详情（对应 Java `upload(CombinedShoppingInfo info)`）。
    async fn upload_combined_shopping_info(
        &self,
        info: &CombinedShoppingInfo,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(
                &ma_orders_upload_combined_shopping_info_url(config.as_ref()),
                &body,
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 上传合单物流信息（对应 Java `upload(CombinedShippingInfo info)`）。
    async fn upload_combined_shipping_info(
        &self,
        info: &CombinedShippingInfo,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(
                &ma_orders_upload_combined_shipping_info_url(config.as_ref()),
                &body,
            )
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 开通购物订单产品权限（对应 Java
    /// `openShoppingOrderProductPermission()`：POST 空数据包）。
    async fn open_shopping_order_product_permission(
        &self,
    ) -> Result<WxOpenResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .post(&ma_orders_open_permission_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 提交购物订单接入审核（对应 Java `confirmProductPermission()`：
    /// POST 空数据包 → `WxOpenShoppingOrdersConfirmResult`）。
    async fn confirm_product_permission(
        &self,
    ) -> Result<WxOpenShoppingOrdersConfirmResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let response = ma
            .post(&ma_orders_confirm_permission_url(config.as_ref()), "")
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    /// 验证购物订单上传结果（对应 Java
    /// `verifyUploadResult(ShoppingInfoVerifyUpload info)`：POST 实体
    /// 序列化 → `WxOpenShoppingInfoVerifyUploadResult`）。
    async fn verify_upload_result(
        &self,
        info: &ShoppingInfoVerifyUpload,
    ) -> Result<WxOpenShoppingInfoVerifyUploadResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_open_config_storage();
        let ma = self.ma_service()?;
        let body =
            serde_json::to_string(info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = ma
            .post(&ma_orders_verify_upload_url(config.as_ref()), &body)
            .await?;
        // Java Gson 宽松类型转换：数字 errcode 归一化为字符串（bean 的
        // errcode 字段为 String，镜像 Java 字段类型）
        let response = base_wx_open_service_impl::normalize_errcode(&response)?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}
