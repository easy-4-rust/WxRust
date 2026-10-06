//! 微信小店会员功能服务实现。
//!
//! 对应 Java `com.binarywang.wxjava.store.api.impl.WxStoreVipServiceImpl`。

use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::WxStoreService;
use crate::api::wx_store_vip_service::WxStoreVipService;
use crate::bean::base::WxStoreBaseResponse;
use crate::bean::vip::{
    VipGradeParam, VipInfoParam, VipInfoResponse, VipListParam, VipListResponse, VipOpenIdParam,
    VipScoreParam, VipScoreResponse,
};
use crate::enums::url_vip::{
    GRADE_UPDATE_URL, SCORE_DECREASE_URL, SCORE_INCREASE_URL, VIP_SCORE_URL, VIP_USER_INFO_URL,
    VIP_USER_LIST_URL,
};

/// 微信小店会员功能服务实现（对应 Java `WxStoreVipServiceImpl`）。
pub struct WxStoreVipServiceImpl {
    /// 微信商店服务（弱引用，对应 Java 构造器注入的 `BaseWxStoreServiceImpl`）。
    service: Weak<dyn WxStoreService>,
}

impl WxStoreVipServiceImpl {
    /// 构建服务（对应 Java `new WxStoreVipServiceImpl(shopService)`）。
    pub fn new(service: Weak<dyn WxStoreService>) -> Self {
        Self { service }
    }

    /// 发送 POST 请求并解析响应（对应 Java `shopService.post` +
    /// `ResponseUtils.decode`；errcode 校验由执行引擎完成，同 Java 语义）。
    async fn post_as<T>(
        svc: &dyn WxStoreService,
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
impl WxStoreVipService for WxStoreVipServiceImpl {
    /// 获取用户详情（对应 Java `getVipInfo(String, Boolean)`，内部构造
    /// `VipInfoParam`，请求体 `{"openid":"..","need_phone_number":..}`）。
    async fn get_vip_info(
        &self,
        open_id: String,
        need_phone_number: Option<bool>,
    ) -> Result<VipInfoResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = VipInfoParam {
            open_id: Some(open_id),
            need_phone_number: Some(need_phone_number.unwrap_or(false)),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), VIP_USER_INFO_URL, &req_json).await
    }

    /// 获取用户列表（对应 Java `getVipList(Boolean, Integer, Integer)`，内部构造
    /// `VipListParam`）。
    async fn get_vip_list(
        &self,
        need_phone_number: Option<bool>,
        page_num: Option<i32>,
        page_size: Option<i32>,
    ) -> Result<VipListResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = VipListParam {
            need_phone_number: Some(need_phone_number.unwrap_or(false)),
            page_num: Some(page_num.unwrap_or(0)),
            page_size: Some(page_size.unwrap_or(0)),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), VIP_USER_LIST_URL, &req_json).await
    }

    /// 获取用户积分（对应 Java `getVipScore(String)`，内部构造 `VipOpenIdParam`，
    /// 请求体 `{"openid":".."}`）。
    async fn get_vip_score(&self, open_id: String) -> Result<VipScoreResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = VipOpenIdParam {
            open_id: Some(open_id),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), VIP_SCORE_URL, &req_json).await
    }

    /// 增加用户积分（对应 Java `increaseVipScore(String, String, String, String)`，
    /// 内部构造 `VipScoreParam`）。
    async fn increase_vip_score(
        &self,
        open_id: String,
        score: String,
        remark: String,
        request_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = VipScoreParam {
            open_id: Some(open_id),
            score: Some(score),
            remark: Some(remark),
            request_id: Some(request_id),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), SCORE_INCREASE_URL, &req_json).await
    }

    /// 减少用户积分（对应 Java `decreaseVipScore`，内部构造 `VipScoreParam`）。
    async fn decrease_vip_score(
        &self,
        open_id: String,
        score: String,
        remark: String,
        request_id: String,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = VipScoreParam {
            open_id: Some(open_id),
            score: Some(score),
            remark: Some(remark),
            request_id: Some(request_id),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), SCORE_DECREASE_URL, &req_json).await
    }

    /// 更新用户等级（对应 Java `updateVipGrade(String, Integer)`，内部构造
    /// `VipGradeParam`，请求体 `{"openid":"..","grade":N}`——Java 参数名 score
    /// 映射到 bean 字段 grade，此处照搬）。
    async fn update_vip_grade(
        &self,
        open_id: String,
        score: Option<i32>,
    ) -> Result<WxStoreBaseResponse, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "微信商店服务已释放"))?;
        let param = VipGradeParam {
            open_id: Some(open_id),
            grade: Some(score.unwrap_or(0)),
        };
        let req_json = serde_json::to_string(&param).map_err(WxErrorException::from)?;
        Self::post_as(svc.as_ref(), GRADE_UPDATE_URL, &req_json).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::r#impl::h2b_impls::test_support;

    /// 获取用户详情：请求体 `{"openid":"..","need_phone_number":..}` 与响应解析
    /// （对应 Java `getVipInfo` + `VipInfoParam`）。
    #[tokio::test]
    async fn test_get_vip_info() {
        let (svc, weak) = test_support::build_service(
            r#"{"errcode":0,"errmsg":"ok","info":{"openid":"o_1","user_info":{"phone_number":"13800000000"}}}"#,
        );
        let sub = WxStoreVipServiceImpl::new(weak);
        let resp = sub
            .get_vip_info("o_1".to_string(), Some(true))
            .await
            .unwrap();
        assert_eq!(
            resp.vip_info.clone().unwrap().open_id.clone().unwrap(),
            "o_1"
        );
        assert_eq!(
            resp.vip_info
                .clone()
                .unwrap()
                .user_info
                .clone()
                .unwrap()
                .phone_number
                .clone()
                .unwrap(),
            "13800000000"
        );
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, VIP_USER_INFO_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["openid"], "o_1");
        assert_eq!(json["need_phone_number"], true);
    }

    /// 增加用户积分：`VipScoreParam` 请求体与响应解析（对应 Java
    /// `increaseVipScore`）。
    #[tokio::test]
    async fn test_increase_vip_score() {
        let (svc, weak) = test_support::build_service(r#"{"errcode":0,"errmsg":"ok"}"#);
        let sub = WxStoreVipServiceImpl::new(weak);
        let resp = sub
            .increase_vip_score(
                "o_1".to_string(),
                "100".to_string(),
                "活动赠送".to_string(),
                "req_1".to_string(),
            )
            .await
            .unwrap();
        assert_eq!(resp.err_code, 0);
        let (url, body) = test_support::last_request(&svc);
        assert_eq!(url, SCORE_INCREASE_URL);
        let json: serde_json::Value = serde_json::from_str(&body).unwrap();
        assert_eq!(json["openid"], "o_1");
        assert_eq!(json["score"], "100");
        assert_eq!(json["remark"], "活动赠送");
        assert_eq!(json["request_id"], "req_1");
    }
}
