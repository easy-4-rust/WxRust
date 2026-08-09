//! WxStoreHomePageService（对应 Java `me.chanjar.weixin.channel.api.WxStoreHomePageService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::home::background::{BackgroundApplyResponse, BackgroundGetResponse};
use crate::bean::home::banner::{BannerApplyResponse, BannerGetResponse, BannerInfo};
use crate::bean::home::tree::{
    TreeProductEditInfo, TreeProductListInfo, TreeProductListResponse, TreeShowGetResponse,
    TreeShowInfo, TreeShowSetResponse,
};
use crate::bean::home::window::WindowProductSettingResponse;

/// 微信小店 主页管理相关接口（对应 Java `WxStoreHomePageService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_store_home_page_service_impl` 的
/// `WxStoreHomePageServiceImpl`（Java `WxStoreHomePageServiceImpl`）。
#[async_trait]
pub trait WxStoreHomePageService: Send + Sync {
    /// 添加分类关联的商品（对应 Java `WxStoreHomePageService#addTreeProduct`）。
    async fn add_tree_product(
        &self,
        info: TreeProductEditInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 删除分类关联的商品（对应 Java `WxStoreHomePageService#delTreeProduct`）。
    async fn del_tree_product(
        &self,
        info: TreeProductEditInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取分类关联的商品ID列表（对应 Java `WxStoreHomePageService#getTreeProductList`）。
    async fn get_tree_product_list(
        &self,
        info: TreeProductListInfo,
    ) -> Result<TreeProductListResponse, WxErrorException>;

    /// 设置展示在店铺主页的商品分类（对应 Java `WxStoreHomePageService#setShowTree`）。
    async fn set_show_tree(
        &self,
        info: TreeShowInfo,
    ) -> Result<TreeShowSetResponse, WxErrorException>;

    /// 获取展示在店铺主页的商品分类（对应 Java `WxStoreHomePageService#getShowTree`）。
    async fn get_show_tree(&self) -> Result<TreeShowGetResponse, WxErrorException>;

    /// 获取主页展示商品列表（对应 Java `WxStoreHomePageService#listWindowProduct`）。
    async fn list_window_product(
        &self,
        page_size: Option<i32>,
        next_key: String,
    ) -> Result<WindowProductSettingResponse, WxErrorException>;

    /// 删除主页展示商品（对应 Java `WxStoreHomePageService#reorderWindowProduct`；
    /// `index_num` 为商品重新排序后的新序号，最大移动步长为 500）。
    async fn reorder_window_product(
        &self,
        product_id: String,
        index_num: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 隐藏小店主页商品（对应 Java `WxStoreHomePageService#hideWindowProduct`；
    /// `set_hide`：1-隐藏，0-取消隐藏）。
    async fn hide_window_product(
        &self,
        product_id: String,
        set_hide: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 置顶小店主页商品（对应 Java `WxStoreHomePageService#topWindowProduct`；
    /// `set_top`：1-置顶，0-取消置顶）。
    async fn top_window_product(
        &self,
        product_id: String,
        set_top: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 提交背景图申请（对应 Java `WxStoreHomePageService#applyBackground`；
    /// `img_url` 需为接口上传图片返回的 `img_url`）。
    async fn apply_background(
        &self,
        img_url: String,
    ) -> Result<BackgroundApplyResponse, WxErrorException>;

    /// 查询背景图（对应 Java `WxStoreHomePageService#getBackground`）。
    async fn get_background(&self) -> Result<BackgroundGetResponse, WxErrorException>;

    /// 撤销主页背景图申请（对应 Java `WxStoreHomePageService#cancelBackground`）。
    async fn cancel_background(
        &self,
        apply_id: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 清空主页背景图并撤销流程中的申请（对应 Java `WxStoreHomePageService#removeBackground`）。
    async fn remove_background(&self) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 提交精选展示位申请（对应 Java `WxStoreHomePageService#applyBanner`）。
    async fn apply_banner(&self, info: BannerInfo)
    -> Result<BannerApplyResponse, WxErrorException>;

    /// 查询精选展示位（对应 Java `WxStoreHomePageService#getBanner`）。
    async fn get_banner(&self) -> Result<BannerGetResponse, WxErrorException>;

    /// 撤销精选展示位申请（对应 Java `WxStoreHomePageService#cancelBanner`）。
    async fn cancel_banner(
        &self,
        apply_id: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 清空精选展示位并撤销流程中的申请（对应 Java `WxStoreHomePageService#removeBanner`）。
    async fn remove_banner(&self) -> Result<WxChannelBaseResponse, WxErrorException>;
}
