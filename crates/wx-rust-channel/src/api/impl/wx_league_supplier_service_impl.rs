//! 优选联盟团长数据服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxLeagueSupplierServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_league_supplier_service::WxLeagueSupplierService;
use crate::bean::base::StreamPageParam;
use crate::bean::league::supplier::{
    CommissionOrderListParam, CommissionOrderListResponse, CommissionOrderResponse,
    CoopProductDetailParam, CoopProductListParam, CoopProductListResponse, CoopProductResponse,
    FlowListParam, ShopDetailResponse, ShopListResponse, SupplierBalanceResponse,
    SupplierFlowDetailResponse, SupplierFlowListResponse,
};
use crate::enums::url_league::{
    GET_SUPPLIER_BALANCE_FLOW_DETAIL_URL, GET_SUPPLIER_BALANCE_FLOW_LIST_URL,
    GET_SUPPLIER_BALANCE_URL, GET_SUPPLIER_ITEM_LIST_URL, GET_SUPPLIER_ITEM_URL,
    GET_SUPPLIER_ORDER_LIST_URL, GET_SUPPLIER_ORDER_URL, GET_SUPPLIER_SHOP_LIST_URL,
    GET_SUPPLIER_SHOP_URL,
};

/// 优选联盟团长数据服务实现（对应 Java `WxLeagueSupplierServiceImpl`）。
pub struct WxLeagueSupplierServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxLeagueSupplierServiceImpl {
    /// 构建服务（对应 Java `new WxLeagueSupplierServiceImpl(shopService)`）。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }

    /// 发送 POST 请求并解析响应（对应 Java `shopService.post` +
    /// `ResponseUtils.decode`；errcode 校验由执行引擎完成，同 Java 语义）。
    async fn post_as<T>(
        svc: &dyn WxChannelService,
        url: &str,
        post_data: &str,
    ) -> Result<T, WxErrorException>
    where
        T: for<'de> serde::Deserialize<'de>,
    {
        let response = svc.post(url, post_data).await?;
        serde_json::from_str(&response).map_err(WxErrorException::from)
    }
}

#[async_trait]
impl WxLeagueSupplierService for WxLeagueSupplierServiceImpl {
    /// 获取团长账户余额（对应 Java `getBalanceInfo`，POST 空对象 `{}`）。
    async fn get_balance_info(&self) -> Result<SupplierBalanceResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        Self::post_as(svc.as_ref(), GET_SUPPLIER_BALANCE_URL, "{}").await
    }

    /// 获取资金流水详情（对应 Java `getFlowDetail`，请求体 `{"flow_id":"..."}`）。
    async fn get_flow_detail(
        &self,
        flow_id: String,
    ) -> Result<SupplierFlowDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"flow_id\":\"{flow_id}\"}}");
        Self::post_as(
            svc.as_ref(),
            GET_SUPPLIER_BALANCE_FLOW_DETAIL_URL,
            &req_json,
        )
        .await
    }

    /// 获取团长资金流水列表（对应 Java `getFlowList(FlowListParam)`）。
    async fn get_flow_list(
        &self,
        param: FlowListParam,
    ) -> Result<SupplierFlowListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SUPPLIER_BALANCE_FLOW_LIST_URL, &req_json).await
    }

    /// 获取合作商品详情（对应 Java `getProductDetail`，内部构造
    /// `CoopProductDetailParam`，请求体 `{"product_id":"..","appid":".."}`）。
    async fn get_supplier_product_detail(
        &self,
        product_id: String,
        app_id: String,
    ) -> Result<CoopProductResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CoopProductDetailParam { product_id, app_id };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SUPPLIER_ITEM_URL, &req_json).await
    }

    /// 获取合作商品列表（对应 Java `getProductList(String, Integer, String)`，
    /// 内部构造 `CoopProductListParam`）。
    async fn get_supplier_product_list(
        &self,
        appid: String,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<CoopProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = CoopProductListParam {
            appid,
            page_size: page_size.unwrap_or(0),
            next_key,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SUPPLIER_ITEM_LIST_URL, &req_json).await
    }

    /// 获取佣金单详情（对应 Java `getCommissionOrder`，请求体
    /// `{"order_id":"..","sku_id":".."}`）。
    async fn get_commission_order(
        &self,
        order_id: String,
        sku_id: String,
    ) -> Result<CommissionOrderResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"order_id\":\"{order_id}\",\"sku_id\":\"{sku_id}\"}}");
        Self::post_as(svc.as_ref(), GET_SUPPLIER_ORDER_URL, &req_json).await
    }

    /// 获取佣金单列表（对应 Java `getCommissionOrderList(CommissionOrderListParam)`）。
    async fn get_commission_order_list(
        &self,
        param: CommissionOrderListParam,
    ) -> Result<CommissionOrderListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SUPPLIER_ORDER_LIST_URL, &req_json).await
    }

    /// 获取合作小店详情（对应 Java `getShopDetail`，请求体 `{"appid":"..."}`）。
    async fn get_shop_detail(&self, appid: String) -> Result<ShopDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = format!("{{\"appid\":\"{appid}\"}}");
        Self::post_as(svc.as_ref(), GET_SUPPLIER_SHOP_URL, &req_json).await
    }

    /// 获取合作小店列表（对应 Java `getShopList(Integer, String)`，内部构造
    /// `StreamPageParam`，请求体 `{"page_size":N,"next_key":"..."}`）。
    async fn get_shop_list(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<ShopListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = StreamPageParam {
            page_size: page_size.unwrap_or(0),
            next_key,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_SUPPLIER_SHOP_LIST_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 获取团长账户余额：POST 空对象 `{}` 与响应解析（对应 Java `getBalanceInfo`）。
    #[tokio::test]
    async fn test_get_balance_info() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","available_amount":1000,"pending_amount":200}"#,
        );
        let sub = WxLeagueSupplierServiceImpl::new(weak);
        let resp = sub.get_balance_info().await.unwrap();
        assert_eq!(resp.available_amount, 1000);
        assert_eq!(resp.pending_amount, 200);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_SUPPLIER_BALANCE_URL);
        assert_eq!(body, "{}");
    }

    /// 获取佣金单详情：字面量请求体 `{"order_id":"..","sku_id":".."}` 与响应解析
    /// （对应 Java `getCommissionOrder`）。
    #[tokio::test]
    async fn test_get_commission_order() {
        let (svc, weak) = test_support::build_service(
            // 注意：响应键为 `commssion_order`（Java `@JsonProperty` 原样笔误，
            // 生成 bean 保留该线格式）
            r#"{"errcode":0,"errmsg":"ok","commssion_order":{"order_id":"o1","sku_id":"s1","status":2}}"#,
        );
        let sub = WxLeagueSupplierServiceImpl::new(weak);
        let resp = sub
            .get_commission_order("o1".to_string(), "s1".to_string())
            .await
            .unwrap();
        assert_eq!(resp.commission_order.order_id, "o1");
        assert_eq!(resp.commission_order.status, 2);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_SUPPLIER_ORDER_URL);
        assert_eq!(body, r#"{"order_id":"o1","sku_id":"s1"}"#);
    }

    /// 获取合作小店列表：`StreamPageParam` 请求体与响应解析（对应 Java
    /// `getShopList` + `StreamPageParam`）。
    #[tokio::test]
    async fn test_get_shop_list() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","shop_list":[{"status":1}],"next_key":"nk","has_more":true}"#,
        );
        let sub = WxLeagueSupplierServiceImpl::new(weak);
        let resp = sub.get_shop_list(Some(10), String::new()).await.unwrap();
        assert_eq!(resp.shop_list.len(), 1);
        assert_eq!(resp.shop_list[0].status, 1);
        assert!(resp.has_more);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, GET_SUPPLIER_SHOP_LIST_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["page_size"], 10);
    }
}
