//! 优选联盟商品操作服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxLeagueProductServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_league_product_service::WxLeagueProductService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::league::product::{
    BatchAddParam, BatchAddResponse, ProductDeleteParam, ProductDetailParam, ProductDetailResponse,
    ProductListParam, ProductListResponse, ProductUpdateParam, ProductUpdateResponse,
};
use crate::enums::url_league::{
    BATCH_ADD_LEAGUE_ITEM_URL, DELETE_LEAGUE_ITEM_URL, GET_LEAGUE_ITEM_LIST_URL,
    GET_LEAGUE_ITEM_URL, UPDATE_LEAGUE_ITEM_URL,
};

/// 优选联盟商品操作服务实现（对应 Java `WxLeagueProductServiceImpl`）。
pub struct WxLeagueProductServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxLeagueProductServiceImpl {
    /// 构建服务（对应 Java `new WxLeagueProductServiceImpl(shopService)`）。
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
impl WxLeagueProductService for WxLeagueProductServiceImpl {
    /// 批量新增联盟商品（对应 Java `batchAddProduct(BatchAddParam)`）。
    async fn batch_add_product(
        &self,
        param: BatchAddParam,
    ) -> Result<BatchAddResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), BATCH_ADD_LEAGUE_ITEM_URL, &req_json).await
    }

    /// 更新联盟商品信息（对应 Java `updateProduct(ProductUpdateParam)`）。
    async fn update_league_product(
        &self,
        param: ProductUpdateParam,
    ) -> Result<ProductUpdateResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), UPDATE_LEAGUE_ITEM_URL, &req_json).await
    }

    /// 删除联盟商品（对应 Java `deleteProduct(Integer, String, String)`，内部构造
    /// `ProductDeleteParam`，请求体 `{"type":N,"product_id":"..","info_id":".."}`；
    /// `type`：1 普通推广 / 2 定向推广 / 3 专属推广）。
    async fn delete_league_product(
        &self,
        r#type: Option<i32>,
        product_id: String,
        info_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = ProductDeleteParam {
            r#type: r#type.unwrap_or(0),
            product_id,
            info_id,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), DELETE_LEAGUE_ITEM_URL, &req_json).await
    }

    /// 拉取联盟商品详情（对应 Java `getProductDetail(ProductDetailParam)`）。
    async fn get_product_detail(
        &self,
        param: ProductDetailParam,
    ) -> Result<ProductDetailResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_LEAGUE_ITEM_URL, &req_json).await
    }

    /// 拉取联盟商品推广列表（对应 Java `listProduct(ProductListParam)`）。
    async fn list_league_product(
        &self,
        param: ProductListParam,
    ) -> Result<ProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GET_LEAGUE_ITEM_LIST_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 删除联盟商品：`ProductDeleteParam` 请求体（含 `type` 关键字字段）与响应解析
    /// （对应 Java `deleteProduct` + `ProductDeleteParam`）。
    #[tokio::test]
    async fn test_delete_league_product() {
        let (svc, weak) = test_support::build_service(r#"{"errcode":0,"errmsg":"ok"}"#);
        let sub = WxLeagueProductServiceImpl::new(weak);
        let resp = sub
            .delete_league_product(Some(1), "pid_1".to_string(), String::new())
            .await
            .unwrap();
        assert_eq!(resp.err_code, 0);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, DELETE_LEAGUE_ITEM_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["type"], 1);
        assert_eq!(json["product_id"], "pid_1");
        assert_eq!(json["info_id"], "");
    }

    /// 批量新增联盟商品：参数透传与响应解析（对应 Java `batchAddProduct`）。
    #[tokio::test]
    async fn test_batch_add_product() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","result_info_list":[{"product_id":"pid_1","err_code":0}]}"#,
        );
        let sub = WxLeagueProductServiceImpl::new(weak);
        let param = BatchAddParam {
            list: vec![crate::bean::league::product::Product {
                product_id: "pid_1".to_string(),
                ..Default::default()
            }],
            ..Default::default()
        };
        let resp = sub.batch_add_product(param).await.unwrap();
        assert_eq!(resp.result_info_list.len(), 1);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, BATCH_ADD_LEAGUE_ITEM_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["list"][0]["product_id"], "pid_1");
    }
}
