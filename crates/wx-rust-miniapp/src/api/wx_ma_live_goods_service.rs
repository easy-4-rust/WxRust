//! 直播商品管理服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaLiveGoodsService`
//! （`impl.WxMaLiveGoodsServiceImpl`）。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::live::{WxMaLiveGoodInfo, WxMaLiveResult};

/// 直播商品管理服务。
#[async_trait]
pub trait WxMaLiveGoodsService: Send + Sync {
    /// 商品添加并提审（对应 Java `addGoods`）。
    async fn add_goods(&self, goods: &WxMaLiveGoodInfo)
    -> Result<WxMaLiveResult, WxErrorException>;

    /// 撤回审核（对应 Java `resetAudit`）。
    async fn reset_audit(&self, audit_id: i32, goods_id: i32) -> Result<bool, WxErrorException>;

    /// 重新提交审核（对应 Java `auditGoods`，返回审核单 ID）。
    async fn audit_goods(&self, goods_id: i32) -> Result<String, WxErrorException>;

    /// 删除商品（对应 Java `deleteGoods`）。
    async fn delete_goods(&self, goods_id: i32) -> Result<bool, WxErrorException>;

    /// 更新商品（对应 Java `updateGoods`）。
    async fn update_goods(&self, goods: &WxMaLiveGoodInfo) -> Result<bool, WxErrorException>;

    /// 获取商品状态（对应 Java `getGoodsWareHouse`）。
    async fn get_goods_ware_house(
        &self,
        goods_ids: &[i32],
    ) -> Result<WxMaLiveResult, WxErrorException>;

    /// 获取已审核商品列表（对应 Java `getApprovedGoods`）。
    async fn get_approved_goods(
        &self,
        offset: i32,
        limit: i32,
        status: i32,
    ) -> Result<WxMaLiveResult, WxErrorException>;

    /// 直播挂件设置全局 key（对应 Java `setKey`）。
    async fn set_key(&self, goods_key: &[String]) -> Result<bool, WxErrorException>;

    /// 查看当前设定的全局 key（对应 Java `getKey`；响应无 `vendorGoodsKey`
    /// 字段时返回 `None`，Java 返回 null）。
    async fn get_key(&self) -> Result<Option<Vec<String>>, WxErrorException>;
}
