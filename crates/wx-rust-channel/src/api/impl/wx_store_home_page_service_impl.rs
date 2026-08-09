//! 微信小店主页管理服务实现。
//!
//! 对应 Java `me.chanjar.weixin.channel.api.impl.WxStoreHomePageServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxChannelService;
use crate::api::wx_store_home_page_service::WxStoreHomePageService;
use crate::bean::base::WxChannelBaseResponse;
use crate::bean::home::background::{BackgroundApplyResponse, BackgroundGetResponse};
use crate::bean::home::banner::{
    BannerApplyParam, BannerApplyResponse, BannerGetResponse, BannerInfo,
};
use crate::bean::home::tree::{
    TreeProductEditInfo, TreeProductEditParam, TreeProductListInfo, TreeProductListParam,
    TreeProductListResponse, TreeShowGetResponse, TreeShowInfo, TreeShowParam, TreeShowSetResponse,
};
use crate::bean::home::window::{
    WindowProductIndexParam, WindowProductListParam, WindowProductSetting,
    WindowProductSettingResponse,
};
use crate::enums::url_home_page::{
    ADD_TREE_PRODUCT_URL, APPLY_BACKGROUND_URL, APPLY_BANNER_URL, CANCEL_BACKGROUND_URL,
    CANCEL_BANNER_URL, DEL_TREE_PRODUCT_URL, GET_BACKGROUND_URL, GET_BANNER_URL, GET_SHOW_TREE_URL,
    HIDE_WINDOW_PRODUCT_URL, LIST_TREE_PRODUCT_URL, LIST_WINDOW_PRODUCT_URL, REMOVE_BACKGROUND_URL,
    REMOVE_BANNER_URL, REORDER_WINDOW_PRODUCT_URL, SET_SHOW_TREE_URL, TOP_WINDOW_PRODUCT_URL,
};

/// 微信小店主页管理服务实现（对应 Java `WxStoreHomePageServiceImpl`）。
pub struct WxStoreHomePageServiceImpl {
    /// 微信小店服务（弱引用，对应 Java 构造器注入的 `BaseWxChannelServiceImpl`）。
    service: Weak<dyn WxChannelService>,
}

impl WxStoreHomePageServiceImpl {
    /// 构建主页管理服务（对应 Java `new WxStoreHomePageServiceImpl(storeService)`）。
    pub fn new(service: Weak<dyn WxChannelService>) -> Self {
        Self { service }
    }

    /// 发送 POST 请求并解析响应（对应 Java `storeService.post` +
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
impl WxStoreHomePageService for WxStoreHomePageServiceImpl {
    /// 添加分类关联的商品（对应 Java `addTreeProduct`，内部构造
    /// `TreeProductEditParam`，请求体 `{"req":{...}}`）。
    async fn add_tree_product(
        &self,
        info: TreeProductEditInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = TreeProductEditParam { req: info };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), ADD_TREE_PRODUCT_URL, &req_json).await
    }

    /// 删除分类关联的商品（对应 Java `delTreeProduct`）。
    async fn del_tree_product(
        &self,
        info: TreeProductEditInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = TreeProductEditParam { req: info };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), DEL_TREE_PRODUCT_URL, &req_json).await
    }

    /// 获取分类关联的商品ID列表（对应 Java `getTreeProductList`，请求体 `{"req":{...}}`）。
    async fn get_tree_product_list(
        &self,
        info: TreeProductListInfo,
    ) -> Result<TreeProductListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = TreeProductListParam { req: info };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), LIST_TREE_PRODUCT_URL, &req_json).await
    }

    /// 设置展示在店铺主页的商品分类（对应 Java `setShowTree`，请求体 `{"req":{...}}`）。
    async fn set_show_tree(
        &self,
        info: TreeShowInfo,
    ) -> Result<TreeShowSetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = TreeShowParam { req: info };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), SET_SHOW_TREE_URL, &req_json).await
    }

    /// 获取展示在店铺主页的商品分类（对应 Java `getShowTree`，POST 空串，与 Java 一致）。
    async fn get_show_tree(&self) -> Result<TreeShowGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        Self::post_as(svc.as_ref(), GET_SHOW_TREE_URL, "").await
    }

    /// 获取主页展示商品列表（对应 Java `listWindowProduct(Integer, String)`，
    /// 内部构造 `WindowProductListParam`）。
    async fn list_window_product(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<WindowProductSettingResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = WindowProductListParam {
            page_size: page_size.unwrap_or(0),
            next_key,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), LIST_WINDOW_PRODUCT_URL, &req_json).await
    }

    /// 删除主页展示商品（对应 Java `reorderWindowProduct`，内部构造
    /// `WindowProductIndexParam`，请求体 `{"product_id":..,"index_num":..}`）。
    async fn reorder_window_product(
        &self,
        product_id: String,
        index_num: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = WindowProductIndexParam {
            product_id,
            index_num: index_num.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), REORDER_WINDOW_PRODUCT_URL, &req_json).await
    }

    /// 隐藏小店主页商品（对应 Java `hideWindowProduct`，内部构造
    /// `WindowProductSetting`，请求体 `{"product_id":..,"is_set_hide":..}`）。
    async fn hide_window_product(
        &self,
        product_id: String,
        set_hide: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = WindowProductSetting {
            product_id,
            set_hide: set_hide.unwrap_or(0),
            set_top: 0,
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), HIDE_WINDOW_PRODUCT_URL, &req_json).await
    }

    /// 置顶小店主页商品（对应 Java `topWindowProduct`，内部构造
    /// `WindowProductSetting`，请求体 `{"product_id":..,"is_set_top":..}`）。
    async fn top_window_product(
        &self,
        product_id: String,
        set_top: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = WindowProductSetting {
            product_id,
            set_hide: 0,
            set_top: set_top.unwrap_or(0),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), TOP_WINDOW_PRODUCT_URL, &req_json).await
    }

    /// 提交背景图申请（对应 Java `applyBackground`，请求体 `{"img_url":"..."}`）。
    async fn apply_background(
        &self,
        img_url: String,
    ) -> Result<BackgroundApplyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param_json = format!("{{\"img_url\":\"{img_url}\"}}");
        Self::post_as(svc.as_ref(), APPLY_BACKGROUND_URL, &param_json).await
    }

    /// 查询背景图（对应 Java `getBackground`，POST 空串，与 Java 一致）。
    async fn get_background(&self) -> Result<BackgroundGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        Self::post_as(svc.as_ref(), GET_BACKGROUND_URL, "").await
    }

    /// 撤销主页背景图申请（对应 Java `cancelBackground`，请求体 `{"apply_id":N}`）。
    async fn cancel_background(
        &self,
        apply_id: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param_json = format!("{{\"apply_id\":{}}}", apply_id.unwrap_or(0));
        Self::post_as(svc.as_ref(), CANCEL_BACKGROUND_URL, &param_json).await
    }

    /// 清空主页背景图并撤销流程中的申请（对应 Java `removeBackground`，POST 空串）。
    async fn remove_background(&self) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        Self::post_as(svc.as_ref(), REMOVE_BACKGROUND_URL, "").await
    }

    /// 提交精选展示位申请（对应 Java `applyBanner`，内部构造 `BannerApplyParam`，
    /// 请求体 `{"banner":{...}}`）。
    async fn apply_banner(
        &self,
        info: BannerInfo,
    ) -> Result<BannerApplyResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param = BannerApplyParam { banner: info };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), APPLY_BANNER_URL, &req_json).await
    }

    /// 查询精选展示位（对应 Java `getBanner`，POST 空串，与 Java 一致）。
    async fn get_banner(&self) -> Result<BannerGetResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        Self::post_as(svc.as_ref(), GET_BANNER_URL, "").await
    }

    /// 撤销精选展示位申请（对应 Java `cancelBanner`，请求体 `{"apply_id":N}`）。
    async fn cancel_banner(
        &self,
        apply_id: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        let param_json = format!("{{\"apply_id\":{}}}", apply_id.unwrap_or(0));
        Self::post_as(svc.as_ref(), CANCEL_BANNER_URL, &param_json).await
    }

    /// 清空精选展示位并撤销流程中的申请（对应 Java `removeBanner`，POST 空串）。
    async fn remove_banner(&self) -> Result<WxChannelBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信小店服务已释放"))?;
        Self::post_as(svc.as_ref(), REMOVE_BANNER_URL, "").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;
    use crate::bean::home::tree::LevelTreeInfo;

    /// 提交背景图申请：字面量请求体 `{"img_url":"..."}` 与响应解析
    /// （对应 Java `applyBackground`）。
    #[tokio::test]
    async fn test_apply_background() {
        let (svc, weak) =
            test_support::build_service(r#"{"errcode":0,"errmsg":"ok","apply_id":123}"#);
        let sub = WxStoreHomePageServiceImpl::new(weak);
        let resp = sub
            .apply_background("https://img.example.com/bg.png".to_string())
            .await
            .unwrap();
        assert_eq!(resp.apply_id, 123);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, APPLY_BACKGROUND_URL);
        assert_eq!(body, r#"{"img_url":"https://img.example.com/bg.png"}"#);
    }

    /// 设置展示分类：请求体 `{"req":{...}}` 嵌套结构与响应解析
    /// （对应 Java `setShowTree` + `TreeShowParam`）。
    #[tokio::test]
    async fn test_set_show_tree() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","resp":{"version":2,"audit_results":[]}}"#,
        );
        let sub = WxStoreHomePageServiceImpl::new(weak);
        let info = TreeShowInfo {
            tree: LevelTreeInfo { level1: vec![] },
            version: 1,
            classification_id_deleted: vec![],
        };
        let resp = sub.set_show_tree(info).await.unwrap();
        assert_eq!(resp.resp.version, 2);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, SET_SHOW_TREE_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["req"]["version"], 1);
        assert_eq!(
            json["req"]["classification_id_deleted"],
            serde_json::Value::Array(vec![])
        );
    }

    /// 隐藏主页商品：请求体 `{"product_id":..,"is_set_hide":..}` 字段名对齐
    /// Java `WindowProductSetting` 的 `@JsonProperty`。
    #[tokio::test]
    async fn test_hide_window_product() {
        let (svc, weak) = test_support::build_service(r#"{"errcode":0,"errmsg":"ok"}"#);
        let sub = WxStoreHomePageServiceImpl::new(weak);
        let resp = sub
            .hide_window_product("pid_1".to_string(), Some(1))
            .await
            .unwrap();
        assert_eq!(resp.err_code, 0);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, HIDE_WINDOW_PRODUCT_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["product_id"], "pid_1");
        assert_eq!(json["is_set_hide"], 1);
        assert_eq!(json["is_set_top"], 0);
    }
}
