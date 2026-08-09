//! 企业微信家校应用复学码服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpSchoolServiceImpl`。
//! https://developer.work.weixin.qq.com/document/path/93744

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpSchoolService, WxCpService};
use crate::bean::{
    LivingIdResult, WxCpCustomizeHealthInfo, WxCpLivingResult, WxCpPaymentResult, WxCpResultList,
    WxCpSchoolLivingInfo, WxCpSchoolUnwatchStat, WxCpSchoolWatchStat, WxCpTrade,
};
use crate::enums::url_school;

/// 企业微信家校应用复学码服务实现。
pub struct WxCpSchoolServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpSchoolServiceImpl {
    /// 构建复学码服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造获取师生健康信息请求体（对应 Java
    /// `getTeacherCustomizeHealthInfo`/`getStudentCustomizeHealthInfo` 内的
    /// `JsonObject`：`limit` 缺省 100，`next_key` 非空才放入）。
    fn build_customize_health_info_body(date: &str, next_key: &str, limit: Option<i32>) -> String {
        let mut body = serde_json::json!({
            "date": date,
            "limit": limit.unwrap_or(100),
        });
        if !next_key.is_empty() {
            body["next_key"] = serde_json::json!(next_key);
        }
        body.to_string()
    }

    /// 构造获取师生健康码请求体（对应 Java `getHealthQrCode` 内的
    /// `JsonObject`：`userids` 以 Java `List.toString()` 语义表达
    /// （`[a, b]`，严格镜像 Java 原样拼接），`type` 空时序列化为 null）。
    fn build_health_qr_code_body(user_ids: &[&str], r#type: Option<i32>) -> String {
        // Java `jsonObject.addProperty("userids", userIds.toString())`：
        // `List<String>.toString()` 为 `[a, b]` 形式
        let user_ids_str = format!("[{}]", user_ids.join(", "));
        serde_json::json!({
            "type": r#type,
            "userids": user_ids_str,
        })
        .to_string()
    }

    /// 构造获取付款结果请求体（对应 Java `getPaymentResult` 内的
    /// `JsonObject`：`{"payment_id": ...}`）。
    fn build_payment_result_body(payment_id: &str) -> String {
        serde_json::json!({ "payment_id": payment_id }).to_string()
    }

    /// 构造获取订单详情请求体（对应 Java `getTrade` 内的
    /// `JsonObject`：`{"payment_id": ..., "trade_no": ...}`）。
    fn build_trade_body(payment_id: &str, trade_no: &str) -> String {
        serde_json::json!({
            "payment_id": payment_id,
            "trade_no": trade_no,
        })
        .to_string()
    }

    /// 构造获取观看统计请求体（对应 Java `getWatchStat`/`getUnwatchStat`
    /// 内的 `JsonObject`：`next_key` 非空白才放入，`livingid` 必有）。
    fn build_watch_stat_body(living_id: &str, next_key: &str) -> String {
        let mut body = serde_json::json!({ "livingid": living_id });
        if !next_key.trim().is_empty() {
            body["next_key"] = serde_json::json!(next_key);
        }
        body.to_string()
    }
}

#[async_trait]
impl WxCpSchoolService for WxCpSchoolServiceImpl {
    async fn get_teacher_customize_health_info(
        &self,
        date: &str,
        next_key: &str,
        limit: Option<i32>,
    ) -> Result<WxCpCustomizeHealthInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getTeacherCustomizeHealthInfo`：
        // `POST GET_TEACHER_CUSTOMIZE_HEALTH_INFO`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_TEACHER_CUSTOMIZE_HEALTH_INFO);
        let response = svc
            .post(
                &api_url,
                &Self::build_customize_health_info_body(date, next_key, limit),
            )
            .await?;
        WxCpCustomizeHealthInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_student_customize_health_info(
        &self,
        date: &str,
        next_key: &str,
        limit: Option<i32>,
    ) -> Result<WxCpCustomizeHealthInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getStudentCustomizeHealthInfo`：
        // `POST GET_STUDENT_CUSTOMIZE_HEALTH_INFO`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_STUDENT_CUSTOMIZE_HEALTH_INFO);
        let response = svc
            .post(
                &api_url,
                &Self::build_customize_health_info_body(date, next_key, limit),
            )
            .await?;
        WxCpCustomizeHealthInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_health_qr_code(
        &self,
        user_ids: &[&str],
        r#type: Option<i32>,
    ) -> Result<WxCpResultList, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getHealthQrCode`：`POST GET_HEALTH_QRCODE`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_HEALTH_QRCODE);
        let response = svc
            .post(&api_url, &Self::build_health_qr_code_body(user_ids, r#type))
            .await?;
        WxCpResultList::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_payment_result(
        &self,
        payment_id: &str,
    ) -> Result<WxCpPaymentResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getPaymentResult`：`POST GET_PAYMENT_RESULT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_PAYMENT_RESULT);
        let response = svc
            .post(&api_url, &Self::build_payment_result_body(payment_id))
            .await?;
        WxCpPaymentResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_trade(
        &self,
        payment_id: &str,
        trade_no: &str,
    ) -> Result<WxCpTrade, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getTrade`：`POST GET_TRADE`
        let api_url = svc.wx_cp_config_storage().api_url(url_school::GET_TRADE);
        let response = svc
            .post(&api_url, &Self::build_trade_body(payment_id, trade_no))
            .await?;
        WxCpTrade::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_living_info(
        &self,
        living_id: &str,
    ) -> Result<WxCpSchoolLivingInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getLivingInfo`：`GET GET_LIVING_INFO + livingId`（GET
        // 请求，query 为空传 ""，对应 Java null）
        let api_url = format!(
            "{}{living_id}",
            svc.wx_cp_config_storage()
                .api_url(url_school::GET_LIVING_INFO)
        );
        let response = svc.get(&api_url, "").await?;
        WxCpSchoolLivingInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_user_all_living_id(
        &self,
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> Result<LivingIdResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUserAllLivingId`：委托 `cpService.getLivingService()
        // .getUserAllLivingId(userId, cursor, limit)`；子服务未装配时
        // Java NPE → Rust 错误码 -99（ADAPTED）
        let living_service = svc
            .living_service()
            .ok_or_else(|| WxErrorException::from_code(-99, "直播服务未装配"))?;
        living_service
            .get_user_all_living_id(user_id, cursor, limit)
            .await
    }

    async fn get_watch_stat(
        &self,
        living_id: &str,
        next_key: &str,
    ) -> Result<WxCpSchoolWatchStat, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getWatchStat`：`POST GET_WATCH_STAT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_WATCH_STAT);
        let response = svc
            .post(&api_url, &Self::build_watch_stat_body(living_id, next_key))
            .await?;
        WxCpSchoolWatchStat::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn get_unwatch_stat(
        &self,
        living_id: &str,
        next_key: &str,
    ) -> Result<WxCpSchoolUnwatchStat, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getUnwatchStat`：`POST GET_UNWATCH_STAT`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_school::GET_UNWATCH_STAT);
        let response = svc
            .post(&api_url, &Self::build_watch_stat_body(living_id, next_key))
            .await?;
        WxCpSchoolUnwatchStat::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete_replay_data(
        &self,
        living_id: &str,
    ) -> Result<WxCpLivingResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `deleteReplayData`：委托 `cpService.getLivingService()
        // .deleteReplayData(livingId)`；子服务未装配时 Java NPE → Rust
        // 错误码 -99（ADAPTED）
        let living_service = svc
            .living_service()
            .ok_or_else(|| WxErrorException::from_code(-99, "直播服务未装配"))?;
        living_service.delete_replay_data(living_id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `getTeacherCustomizeHealthInfo`：`limit` 缺省 100，
    /// `next_key` 为空时不放入请求体。
    #[test]
    fn test_build_customize_health_info_body() {
        let body = WxCpSchoolServiceImpl::build_customize_health_info_body("2022-06-01", "", None);
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["date"], "2022-06-01");
        assert_eq!(json["limit"], 100);
        assert!(json.get("next_key").is_none());

        let body =
            WxCpSchoolServiceImpl::build_customize_health_info_body("2022-06-01", "key1", Some(50));
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["limit"], 50);
        assert_eq!(json["next_key"], "key1");
    }

    /// Java `getHealthQrCode`：`userids` 以 Java `List.toString()` 语义
    /// （`[a, b]`）拼接；`type` 空时序列化为 null。
    #[test]
    fn test_build_health_qr_code_body() {
        let body = WxCpSchoolServiceImpl::build_health_qr_code_body(&["zhangsan", "lisi"], None);
        assert_eq!(body, r#"{"type":null,"userids":"[zhangsan, lisi]"}"#);

        let body = WxCpSchoolServiceImpl::build_health_qr_code_body(&["zhangsan"], Some(1));
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["type"], 1);
        assert_eq!(json["userids"], "[zhangsan]");
    }

    /// Java `getWatchStat`：`next_key` 空白时不放入请求体。
    #[test]
    fn test_build_watch_stat_body() {
        let body = WxCpSchoolServiceImpl::build_watch_stat_body("living1", "");
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["livingid"], "living1");
        assert!(json.get("next_key").is_none());

        let body = WxCpSchoolServiceImpl::build_watch_stat_body("living1", "key1");
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["next_key"], "key1");
    }
}
