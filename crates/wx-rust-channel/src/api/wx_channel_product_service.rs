//! WxChannelProductService（对应 Java `me.chanjar.weixin.channel.api.WxChannelProductService`）。

use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxChannelBaseResponse;
use crate::bean::limit::{LimitTaskAddResponse, LimitTaskListResponse, LimitTaskParam};
use crate::bean::product::link::{
    ProductH5UrlResponse, ProductQrCodeResponse, ProductTagLinkResponse,
};
use crate::bean::product::{
    SkuStockBatchResponse, SkuStockResponse, SpuFastInfo, SpuGetResponse, SpuInfo, SpuListResponse,
    SpuUpdateInfo, SpuUpdateResponse,
};

/// 商品服务（对应 Java `WxChannelProductService`）。
///
/// 真实实现见 `crate::api::r#impl::wx_channel_product_service_impl` 的
/// `WxChannelProductServiceImpl`（Java `WxChannelProductServiceImpl`）。
#[async_trait::async_trait]
pub trait WxChannelProductService: Send + Sync {
    /// 添加商品（对应 Java `WxChannelProductService#addProduct(SpuUpdateInfo)`）。
    async fn add_product(&self, info: SpuUpdateInfo)
    -> Result<SpuUpdateResponse, WxErrorException>;

    /// 更新商品（对应 Java `WxChannelProductService#updateProduct(SpuUpdateInfo)`）。
    async fn update_product(
        &self,
        info: SpuUpdateInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException>;

    /// 添加商品（对应 Java `WxChannelProductService#addProduct(SpuInfo)`）。
    async fn add_product_with_spu_info(
        &self,
        info: SpuInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException>;

    /// 更新商品（对应 Java `WxChannelProductService#updateProduct(SpuInfo)`）。
    async fn update_product_with_spu_info(
        &self,
        info: SpuInfo,
    ) -> Result<SpuUpdateResponse, WxErrorException>;

    /// 免审更新商品（对应 Java `WxChannelProductService#updateProductAuditFree(SpuFastInfo)`）。
    async fn update_product_audit_free(
        &self,
        info: SpuFastInfo,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 更新商品库存（仅对 `edit_status != 2` 的商品适用，其他状态的商品无法
    /// 通过该接口修改库存；对应 Java
    /// `WxChannelProductService#updateStock(String, String, Integer, Integer)`）。
    async fn update_stock(
        &self,
        product_id: String,
        sku_id: String,
        diff_type: Option<i32>,
        num: Option<i32>,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 删除商品（对应 Java `WxChannelProductService#deleteProduct(String)`）。
    async fn delete_product(
        &self,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 撤回商品审核（对应 Java `WxChannelProductService#cancelProductAudit(String)`）。
    async fn cancel_product_audit(
        &self,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取商品（对应 Java `WxChannelProductService#getProduct(String, Integer)`）。
    ///
    /// # 参数
    /// - `data_type`：默认取 1。1 获取线上数据，2 获取草稿数据，3 同时获取线上和
    ///   草稿数据（注意：需成功上架后才有线上数据）
    async fn get_product(
        &self,
        product_id: String,
        data_type: Option<i32>,
    ) -> Result<SpuGetResponse, WxErrorException>;

    /// 获取商品列表（对应 Java
    /// `WxChannelProductService#listProduct(Integer, String, Integer)`）。
    async fn list_product(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<SpuListResponse, WxErrorException>;

    /// 上架商品（对应 Java `WxChannelProductService#upProduct(String)`）。
    async fn up_product(
        &self,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 下架商品（对应 Java `WxChannelProductService#downProduct(String)`）。
    async fn down_product(
        &self,
        product_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 获取商品实时库存（对应 Java
    /// `WxChannelProductService#getSkuStock(String, String)`）。
    async fn get_sku_stock(
        &self,
        product_id: String,
        sku_id: String,
    ) -> Result<SkuStockResponse, WxErrorException>;

    /// 批量获取库存信息（单次请求不能超过 50 个商品 ID；对应 Java
    /// `WxChannelProductService#getSkuStockBatch(List<String>)`）。
    async fn get_sku_stock_batch(
        &self,
        product_ids: Vec<String>,
    ) -> Result<SkuStockBatchResponse, WxErrorException>;

    /// 获取商品 H5 链接（对应 Java `WxChannelProductService#getProductH5Url(String)`）。
    async fn get_product_h5_url(
        &self,
        product_id: String,
    ) -> Result<ProductH5UrlResponse, WxErrorException>;

    /// 获取商品二维码（对应 Java `WxChannelProductService#getProductQrCode(String)`）。
    async fn get_product_qr_code(
        &self,
        product_id: String,
    ) -> Result<ProductQrCodeResponse, WxErrorException>;

    /// 获取商品口令（对应 Java `WxChannelProductService#getProductTagLink(String)`）。
    async fn get_product_tag_link(
        &self,
        product_id: String,
    ) -> Result<ProductTagLinkResponse, WxErrorException>;

    /// 添加限时抢购任务（对应 Java `WxChannelProductService#addLimitTask(LimitTaskParam)`）。
    async fn add_limit_task(
        &self,
        param: LimitTaskParam,
    ) -> Result<LimitTaskAddResponse, WxErrorException>;

    /// 拉取限时抢购任务列表（对应 Java
    /// `WxChannelProductService#listLimitTask(Integer, String, Integer)`）。
    async fn list_limit_task(
        &self,
        page_size: Option<i32>,
        next_key: String,
        status: Option<i32>,
    ) -> Result<LimitTaskListResponse, WxErrorException>;

    /// 停止限时抢购任务（对应 Java `WxChannelProductService#stopLimitTask(String)`）。
    async fn stop_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;

    /// 停止限时抢购任务（对应 Java `WxChannelProductService#deleteLimitTask(String)`）。
    async fn delete_limit_task(
        &self,
        task_id: String,
    ) -> Result<WxChannelBaseResponse, WxErrorException>;
}
