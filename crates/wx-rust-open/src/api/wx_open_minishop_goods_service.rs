//! 微信小商城 商品服务接口。
//!
//! 对应 Java `me.chanjar.weixin.open.api.WxOpenMinishopGoodsService`。
//!
//! URL 常量见 [`crate::enums::url_ma_domain`]（`minishop_goods_*_url`，
//! api_host 前缀模式）。
//!
//! 注意：Java 接口声明了完整的商品/SPU/SKU URL 常量与大量方法占位，
//! 但接口方法仅实现 2 个（`getMinishopGoodsCat`/`addMinishopGoodsSPU`，
//! 且 Java 实现为「执行请求后恒 `return null`」的上游桩代码），Rust
//! 严格镜像接口方法面（方法数一致），实现侧以 `Ok(None)` 镜像
//! `return null`（见 impl 注释）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::AddMinishopGoodsSPU;
use crate::bean::GoodsCatList;
use crate::bean::ParentCatId;
use crate::bean::WxOpenResult;

/// 微信小商城 商品服务（对应 Java `WxOpenMinishopGoodsService`）。
#[async_trait]
pub trait WxOpenMinishopGoodsService: Send + Sync {
    /// 获取商品类目（对应 Java
    /// `getMinishopGoodsCat(ParentCatId fCatId)`，接入商品前必须接口）。
    ///
    /// Java 实现为上游桩（执行请求后恒 `return null`）→ `Ok(None)` 镜像。
    async fn get_minishop_goods_cat(
        &self,
        f_cat_id: &ParentCatId,
    ) -> Result<Option<GoodsCatList>, WxErrorException>;

    /// 新增商品 SPU（对应 Java
    /// `addMinishopGoodsSPU(AddMinishopGoodsSPU dto)`）。
    ///
    /// Java 实现为上游桩（执行请求后恒 `return null`）→ `Ok(None)` 镜像。
    async fn add_minishop_goods_spu(
        &self,
        dto: &AddMinishopGoodsSPU,
    ) -> Result<Option<WxOpenResult>, WxErrorException>;
}
