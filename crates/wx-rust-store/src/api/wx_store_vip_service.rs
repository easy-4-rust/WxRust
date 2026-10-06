//! WxStoreVipService（对应 Java `com.binarywang.wxjava.store.api.WxStoreVipService`）。

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::bean::base::WxStoreBaseResponse;
use crate::bean::vip::{VipInfoResponse, VipListResponse, VipScoreResponse};

/// 微信小店 会员功能接口（对应 Java `WxStoreVipService`）。
///
/// 真实实现见 `crate::api::r#impl::h2b_impls::wx_store_vip_service_impl` 的
/// `WxStoreVipServiceImpl`（Java `WxStoreVipServiceImpl`）。
#[async_trait]
pub trait WxStoreVipService: Send + Sync {
    /// 获取用户详情（对应 Java `WxStoreVipService#getVipInfo(String, Boolean)`）。
    async fn get_vip_info(
        &self,
        open_id: String,
        need_phone_number: Option<bool>,
    ) -> Result<VipInfoResponse, WxErrorException>;

    /// 获取用户列表（对应 Java `WxStoreVipService#getVipList(Boolean, Integer, Integer)`）。
    async fn get_vip_list(
        &self,
        need_phone_number: Option<bool>,
        page_num: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<VipListResponse, WxErrorException>;

    /// 获取用户积分（对应 Java `WxStoreVipService#getVipScore(String)`）。
    async fn get_vip_score(&self, open_id: String) -> Result<VipScoreResponse, WxErrorException>;

    /// 增加用户积分（对应 Java `WxStoreVipService#increaseVipScore(String, String, String, String)`）。
    async fn increase_vip_score(
        &self,
        open_id: String,
        score: String,
        remark: String,
        request_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 减少用户积分（对应 Java `WxStoreVipService#decreaseVipScore(String, String, String, String)`）。
    async fn decrease_vip_score(
        &self,
        open_id: String,
        score: String,
        remark: String,
        request_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;

    /// 更新用户等级（对应 Java `WxStoreVipService#updateVipGrade(String, Integer)`；
    /// `score`：用户积分，用于计算等级）。
    async fn update_vip_grade(
        &self,
        open_id: String,
        score: Option<i32>,
    ) -> Result<WxStoreBaseResponse, WxErrorException>;
}
