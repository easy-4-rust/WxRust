//! 小程序交易组件-标准版商品服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaProductService`。

use async_trait::async_trait;
use wx_rust_common::bean::result::WxMinishopImageUploadResult;
use wx_rust_common::error::WxErrorException;

use crate::bean::product::{
    WxMinishopGetBrandResponse, WxMinishopGetCategoryResponse, WxMinishopGetFrightTemplateResponse,
    WxMinishopResult, WxMinishopSku, WxMinishopSkuListResponse, WxMinishopSpu,
    WxMinishopSpuGetResponse, WxMinishopSpuListResponse,
};
use crate::bean::shop::request::WxMaShopSpuPageRequest;
use crate::bean::shop::response::WxMaShopBaseResponse;

/// 小程序交易组件-标准版商品服务。
#[async_trait]
pub trait WxMaProductService: Send + Sync {
    /// 上传图片（对应 Java `uploadImg(File, Integer, Integer, Integer)`，
    /// resp_type/width/height 拼入 URL 查询参数，multipart 上传 media 文件）。
    async fn upload_img(
        &self,
        file_path: &str,
        resp_type: i32,
        width: i32,
        height: i32,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException>;

    /// 上传图片链接（对应 Java `uploadImg(String, Integer)`，
    /// `{"img_url": ...}` POST 至 `IMG_UPLOAD?upload_type=1&resp_type=...`）。
    async fn upload_img_from_url(
        &self,
        img_url: &str,
        resp_type: i32,
    ) -> Result<WxMinishopImageUploadResult, WxErrorException>;

    /// 获取商品类目（对应 Java `getCategory(Integer)`，f_cat_id 可为空）。
    async fn get_category(
        &self,
        f_cat_id: Option<i32>,
    ) -> Result<WxMinishopGetCategoryResponse, WxErrorException>;

    /// 获取品牌列表（对应 Java `getBrand()`）。
    async fn get_brand(&self) -> Result<WxMinishopGetBrandResponse, WxErrorException>;

    /// 获取运费模板（对应 Java `getFreightTemplate()`）。
    async fn get_freight_template(
        &self,
    ) -> Result<WxMinishopGetFrightTemplateResponse, WxErrorException>;

    /// 添加商品（对应 Java `addSpu(WxMinishopSpu)`）。
    async fn add_spu(&self, spu: &WxMinishopSpu) -> Result<WxMinishopResult, WxErrorException>;

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
    ) -> Result<WxMinishopSpuGetResponse, WxErrorException>;

    /// 获取商品列表（对应 Java `getSpuList(WxMaShopSpuPageRequest)`）。
    async fn get_spu_list(
        &self,
        request: &WxMaShopSpuPageRequest,
    ) -> Result<WxMinishopSpuListResponse, WxErrorException>;

    /// 更新商品（对应 Java `updateSpu(WxMinishopSpu)`）。
    async fn update_spu(&self, spu: &WxMinishopSpu) -> Result<WxMinishopResult, WxErrorException>;

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

    /// 获取 SKU 列表（对应 Java `getSkuList(Long, Integer, Integer)`）。
    async fn get_sku_list(
        &self,
        product_id: i64,
        need_real_stock: Option<i32>,
        need_edit_sku: Option<i32>,
    ) -> Result<WxMinishopSkuListResponse, WxErrorException>;

    /// 小商店新增 SKU 信息（对应 Java `minishiopGoodsAddSku(WxMinishopSku)`）。
    async fn minishop_goods_add_sku(
        &self,
        sku: &WxMinishopSku,
    ) -> Result<WxMinishopResult, WxErrorException>;

    /// 小商店批量新增 SKU 信息（对应 Java `minishopGoodsBatchAddSku(List<WxMinishopSku>)`）。
    async fn minishop_goods_batch_add_sku(
        &self,
        sku_list: &[WxMinishopSku],
    ) -> Result<WxMinishopResult, WxErrorException>;

    /// 小商店删除 SKU 消息（对应 Java `minishopGoodsDelSku(Long, Long, String, Long)`）。
    async fn minishop_goods_del_sku(
        &self,
        product_id: i64,
        out_product_id: Option<i64>,
        out_sku_id: Option<&str>,
        sku_id: Option<i64>,
    ) -> Result<WxMaShopBaseResponse, WxErrorException>;

    /// 小商店更新 SKU（对应 Java `minishopGoodsUpdateSku(WxMinishopSku)`）。
    async fn minishop_goods_update_sku(
        &self,
        sku: &WxMinishopSku,
    ) -> Result<WxMinishopResult, WxErrorException>;

    /// 小商店更新 SKU 价格（对应 Java `minishopGoodsUpdateSkuPrice(...)`）。
    async fn minishop_goods_update_sku_price(
        &self,
        product_id: i64,
        out_product_id: Option<&str>,
        out_sku_id: Option<&str>,
        sku_id: Option<i64>,
        sale_price: Option<i64>,
        market_price: Option<i64>,
    ) -> Result<WxMinishopResult, WxErrorException>;

    /// 小商店更新 SKU 库存（对应 Java `minishopGoodsUpdateSkuStock(...)`）。
    async fn minishop_goods_update_sku_stock(
        &self,
        product_id: i64,
        out_product_id: Option<&str>,
        out_sku_id: Option<&str>,
        sku_id: Option<i64>,
        r#type: Option<i32>,
        stock_num: Option<i32>,
    ) -> Result<WxMinishopResult, WxErrorException>;
}
