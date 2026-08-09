//! 直播商品管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaLiveGoodsServiceImpl`：
//! URL/请求体字段/响应解析逐方法对齐；`getApprovedGoods` 的驼峰→下划线
//! 响应改写逻辑（Java `jsonObject.addProperty("goods_id", ...)` 系列）照搬。

use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaLiveGoodsService;
use crate::bean::live::{WxMaLiveGoodInfo, WxMaLiveResult};
use crate::enums::g4_urls::url_g4_ability::live as live_url;

/// 直播商品管理服务实现。
pub struct WxMaLiveGoodsServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaLiveGoodsServiceImpl {
    /// 构建直播商品管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaLiveGoodsService for WxMaLiveGoodsServiceImpl {
    /// 商品添加并提审（对应 Java `WxMaLiveGoodsServiceImpl.addGoods`）。
    async fn add_goods(
        &self,
        goods: &WxMaLiveGoodInfo,
    ) -> Result<WxMaLiveResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "goodsInfo": goods }).to_string();
        let response_content = svc
            .post(&live_url::goods::add_goods_url(config.as_ref()), &post_body)
            .await?;
        WxMaLiveResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    /// 撤回审核（对应 Java `WxMaLiveGoodsServiceImpl.resetAudit`）。
    async fn reset_audit(&self, audit_id: i32, goods_id: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "auditId": audit_id, "goodsId": goods_id }).to_string();
        svc.post(
            &live_url::goods::reset_audit_goods_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 重新提交审核（对应 Java `WxMaLiveGoodsServiceImpl.auditGoods`）。
    async fn audit_goods(&self, goods_id: i32) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "goodsId": goods_id }).to_string();
        let response_content = svc
            .post(
                &live_url::goods::audit_goods_url(config.as_ref()),
                &post_body,
            )
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        json.get("auditId")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "auditId 字段缺失"))
    }

    /// 删除商品（对应 Java `WxMaLiveGoodsServiceImpl.deleteGoods`）。
    async fn delete_goods(&self, goods_id: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "goodsId": goods_id }).to_string();
        svc.post(
            &live_url::goods::delete_goods_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 更新商品（对应 Java `WxMaLiveGoodsServiceImpl.updateGoods`）。
    async fn update_goods(&self, goods: &WxMaLiveGoodInfo) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "goodsInfo": goods }).to_string();
        svc.post(
            &live_url::goods::update_goods_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 获取商品状态（对应 Java `WxMaLiveGoodsServiceImpl.getGoodsWareHouse`）。
    async fn get_goods_ware_house(
        &self,
        goods_ids: &[i32],
    ) -> Result<WxMaLiveResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "goods_ids": goods_ids }).to_string();
        let response_content = svc
            .post(
                &live_url::goods::get_goods_ware_house_url(config.as_ref()),
                &post_body,
            )
            .await?;
        WxMaLiveResult::from_json(&response_content).map_err(WxErrorException::Serde)
    }

    /// 获取已审核商品列表（对应 Java
    /// `WxMaLiveGoodsServiceImpl.getApprovedGoods`）。
    ///
    /// 接口返回的 key 是驼峰（`goodsId`/`coverImgUrl`/`priceType`/
    /// `thirdPartyTag`），Java 将其改写为下划线（`goods_id`/`cover_img_url`/
    /// `price_type`/`third_party_tag`）并补 `audit_status` 后解析。
    async fn get_approved_goods(
        &self,
        offset: i32,
        limit: i32,
        status: i32,
    ) -> Result<WxMaLiveResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let query = format!("status={status}&offset={offset}&limit={limit}");
        let response_content = svc
            .get(
                &live_url::goods::get_approved_goods_url(config.as_ref()),
                &query,
            )
            .await?;
        let mut json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if let Some(goods_arr) = json.get_mut("goods").and_then(|v| v.as_array_mut()) {
            if !goods_arr.is_empty() {
                for goods in goods_arr.iter_mut() {
                    if let Some(obj) = goods.as_object_mut() {
                        // 接口返回 key 是驼峰，改写为下划线（对齐 Java）
                        if let Some(v) = obj.get("goodsId") {
                            obj.insert("goods_id".to_string(), v.clone());
                        }
                        if let Some(v) = obj.get("coverImgUrl") {
                            obj.insert("cover_img_url".to_string(), v.clone());
                        }
                        if let Some(v) = obj.get("priceType") {
                            obj.insert("price_type".to_string(), v.clone());
                        }
                        if let Some(v) = obj.get("thirdPartyTag") {
                            obj.insert("third_party_tag".to_string(), v.clone());
                        }
                        obj.insert("audit_status".to_string(), serde_json::json!(status));
                    }
                }
            }
        }
        WxMaLiveResult::from_json(&json.to_string()).map_err(WxErrorException::Serde)
    }

    /// 直播挂件设置全局 key（对应 Java `WxMaLiveGoodsServiceImpl.setKey`）。
    async fn set_key(&self, goods_key: &[String]) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "goodsKey": goods_key }).to_string();
        svc.post(&live_url::goods::set_key_url(config.as_ref()), &post_body)
            .await?;
        Ok(true)
    }

    /// 查看当前设定的全局 key（对应 Java `WxMaLiveGoodsServiceImpl.getKey`；
    /// 响应无 `vendorGoodsKey` 字段时返回 `None`，Java 返回 null）。
    async fn get_key(&self) -> Result<Option<Vec<String>>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let response_content = svc
            .get(&live_url::goods::get_key_url(config.as_ref()), "")
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        if let Some(arr) = json.get("vendorGoodsKey").and_then(|v| v.as_array()) {
            let list = arr
                .iter()
                .filter_map(|v| v.as_str().map(|s| s.to_string()))
                .collect();
            Ok(Some(list))
        } else {
            Ok(None)
        }
    }
}
