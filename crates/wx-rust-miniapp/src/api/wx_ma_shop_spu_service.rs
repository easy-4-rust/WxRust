//! 小程序交易组件-商品服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaShopSpuService`。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::shop::WxMaShopSpuInfo;
use crate::bean::shop::WxMaShopSpuWithoutAuditInfo;
use crate::bean::shop::request::WxMaShopSpuPageRequest;
use crate::bean::shop::response::{
    WxMaShopAddSpuResponse, WxMaShopBaseResponse, WxMaShopGetSpuListResponse,
    WxMaShopGetSpuResponse,
};

/// 小程序交易组件-商品服务。
#[async_trait]
pub trait WxMaShopSpuService: Send + Sync {
    /// 添加商品（对应 Java `addSpu(WxMaShopSpuInfo)`）。
    async fn add_spu(
        &self,
        spu_info: &WxMaShopSpuInfo,
    ) -> Result<WxMaShopAddSpuResponse, WxErrorException>;

    /// 删除商品（对应 Java `deleteSpu(Integer, String)`）。
    async fn delete_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 获取商品（对应 Java `getSpu(Integer, String, Integer)`）。
    async fn get_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
        need_edit_spu: Option<i32>,
    ) -> Result<WxMaShopGetSpuResponse, WxErrorException>;

    /// 获取商品列表（对应 Java `getSpuList(WxMaShopSpuPageRequest)`）。
    async fn get_spu_list(
        &self,
        request: &WxMaShopSpuPageRequest,
    ) -> Result<WxMaShopGetSpuListResponse, WxErrorException>;

    /// 更新商品（对应 Java `updateSpu(WxMaShopSpuInfo)`）。
    async fn update_spu(
        &self,
        spu_info: &WxMaShopSpuInfo,
    ) -> Result<WxMaShopAddSpuResponse, WxErrorException>;

    /// 免审核更新商品（对应 Java `updateSpuWithoutAudit(WxMaShopSpuWithoutAuditInfo)`）。
    async fn update_spu_without_audit(
        &self,
        spu_info: &WxMaShopSpuWithoutAuditInfo,
    ) -> Result<WxMaShopAddSpuResponse, WxErrorException>;

    /// 商品上架（对应 Java `listingSpu(Integer, String)`）。
    async fn listing_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 商品下架（对应 Java `delistingSpu(Integer, String)`）。
    async fn delisting_spu(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 撤回审核（对应 Java `deleteAudit(Integer, String)`）。
    async fn delete_audit(
        &self,
        product_id: i32,
        out_product_id: Option<&str>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;
}
