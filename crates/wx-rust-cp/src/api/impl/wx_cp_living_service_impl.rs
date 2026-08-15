//! 企业微信直播服务实现。
//!
//! 对应 Java `me.chanjar.weixin.cp.api.impl.WxCpLivingServiceImpl`。
//! https://developer.work.weixin.qq.com/document/path/93633

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{WxCpLivingService, WxCpService};
use crate::bean::{
    LivingIdResult, WxCpLivingCreateRequest, WxCpLivingInfo, WxCpLivingModifyRequest,
    WxCpLivingResult, WxCpLivingShareInfo, WxCpWatchStat,
};
use crate::enums::url_living;

/// 企业微信直播服务实现。
pub struct WxCpLivingServiceImpl {
    service: Weak<dyn WxCpService>,
}

impl WxCpLivingServiceImpl {
    /// 构建直播服务。
    pub fn new(service: Weak<dyn WxCpService>) -> Self {
        Self { service }
    }

    /// 构造获取直播预约码请求体（对应 Java `getLivingCode` 内的
    /// `JsonObject`：`{"openid": ..., "livingid": ...}`）。
    fn build_get_living_code_body(open_id: &str, living_id: &str) -> String {
        serde_json::json!({
            "openid": open_id,
            "livingid": living_id,
        })
        .to_string()
    }

    /// 构造获取直播观看明细请求体（对应 Java `getWatchStat` 内的
    /// `JsonObject`：`next_key` 非空白才放入，`livingid` 必有）。
    fn build_get_watch_stat_body(living_id: &str, next_key: &str) -> String {
        let mut body = serde_json::json!({ "livingid": living_id });
        if !next_key.trim().is_empty() {
            body["next_key"] = serde_json::json!(next_key);
        }
        body.to_string()
    }

    /// 构造获取成员直播 ID 列表请求体（对应 Java `getUserAllLivingId`
    /// 内的 `JsonObject`：`cursor`/`limit` 非空才放入，`userid` 必有）。
    fn build_get_user_all_living_id_body(
        user_id: &str,
        cursor: Option<&str>,
        limit: Option<i32>,
    ) -> String {
        let mut body = serde_json::json!({ "userid": user_id });
        if let Some(cursor) = cursor {
            body["cursor"] = serde_json::json!(cursor);
        }
        if let Some(limit) = limit {
            body["limit"] = serde_json::json!(limit);
        }
        body.to_string()
    }

    /// 构造获取直播观众信息请求体（对应 Java `getLivingShareInfo` 内的
    /// `JsonObject`：`{"ww_share_code": ...}`）。
    fn build_get_living_share_info_body(ww_share_code: &str) -> String {
        serde_json::json!({ "ww_share_code": ww_share_code }).to_string()
    }

    /// 构造仅含 `livingid` 的请求体（对应 Java `livingCancel`/
    /// `deleteReplayData` 内的 `JsonObject`：`{"livingid": livingId}`）。
    fn build_living_id_body(living_id: &str) -> String {
        serde_json::json!({ "livingid": living_id }).to_string()
    }

    /// 从响应中提取字符串字段（对应 Java `GsonHelper.getString`；
    /// 字段缺失时 Java 返回 null → Rust 错误码 -99，ADAPTED）。
    fn get_string_field(response: &str, field: &str) -> Result<String, WxErrorException> {
        let json: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        json.get(field)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, format!("{field} 字段缺失")))
    }

    /// 从响应中解析子对象（对应 Java `GsonParser.parse(response).get(...)`
    /// + `TypeToken` 反序列化；字段缺失时 Java NPE → Rust 错误码 -99）。
    fn parse_sub_json<T: serde::de::DeserializeOwned>(
        response: &str,
        field: &str,
    ) -> Result<T, WxErrorException> {
        let json: serde_json::Value =
            serde_json::from_str(response).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let sub = json
            .get(field)
            .ok_or_else(|| WxErrorException::from_code(-99, format!("{field} 字段缺失")))?;
        serde_json::from_value(sub.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
}

#[async_trait]
impl WxCpLivingService for WxCpLivingServiceImpl {
    async fn get_living_code(
        &self,
        open_id: &str,
        living_id: &str,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getLivingCode`：`POST GET_LIVING_CODE`，提取 `living_code`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_living::GET_LIVING_CODE);
        let response = svc
            .post(
                &api_url,
                &Self::build_get_living_code_body(open_id, living_id),
            )
            .await?;
        Self::get_string_field(&response, "living_code")
    }

    async fn get_living_info(&self, living_id: &str) -> Result<WxCpLivingInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getLivingInfo`：`GET GET_LIVING_INFO + livingId`（GET
        // 请求，query 为空传 ""，对应 Java null），解析 `living_info`
        let api_url = format!(
            "{}{living_id}",
            svc.wx_cp_config_storage()
                .api_url(url_living::GET_LIVING_INFO)
        );
        let response = svc.get(&api_url, "").await?;
        let living_info: WxCpLivingInfo = Self::parse_sub_json(&response, "living_info")?;
        Ok(living_info)
    }

    async fn get_watch_stat(
        &self,
        living_id: &str,
        next_key: &str,
    ) -> Result<WxCpWatchStat, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getWatchStat`：`POST GET_WATCH_STAT`（`nextKey` 初次调用
        // 可以填 `"0"`）
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_living::GET_WATCH_STAT);
        let response = svc
            .post(
                &api_url,
                &Self::build_get_watch_stat_body(living_id, next_key),
            )
            .await?;
        WxCpWatchStat::from_json(&response).map_err(WxErrorException::Serde)
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
        // Java `getUserAllLivingId`：`POST GET_USER_ALL_LIVINGID`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_living::GET_USER_ALL_LIVINGID);
        let response = svc
            .post(
                &api_url,
                &Self::build_get_user_all_living_id_body(user_id, cursor, limit),
            )
            .await?;
        serde_json::from_str(&response).map_err(|e| WxErrorException::Serde(e.to_string()))
    }

    async fn get_living_share_info(
        &self,
        ww_share_code: &str,
    ) -> Result<WxCpLivingShareInfo, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `getLivingShareInfo`：`POST GET_LIVING_SHARE_INFO`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_living::GET_LIVING_SHARE_INFO);
        let response = svc
            .post(
                &api_url,
                &Self::build_get_living_share_info_body(ww_share_code),
            )
            .await?;
        WxCpLivingShareInfo::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn living_create(
        &self,
        request: &WxCpLivingCreateRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `livingCreate`：`POST CREATE`，提取 `livingid`，返回直播 id
        let api_url = svc.wx_cp_config_storage().api_url(url_living::CREATE);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        Self::get_string_field(&response, "livingid")
    }

    async fn living_modify(
        &self,
        request: &WxCpLivingModifyRequest,
    ) -> Result<WxCpLivingResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `livingModify`：`POST MODIFY`
        let api_url = svc.wx_cp_config_storage().api_url(url_living::MODIFY);
        let body = request.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&api_url, &body).await?;
        WxCpLivingResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn living_cancel(&self, living_id: &str) -> Result<WxCpLivingResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `livingCancel`：`POST CANCEL`
        let api_url = svc.wx_cp_config_storage().api_url(url_living::CANCEL);
        let response = svc
            .post(&api_url, &Self::build_living_id_body(living_id))
            .await?;
        WxCpLivingResult::from_json(&response).map_err(WxErrorException::Serde)
    }

    async fn delete_replay_data(
        &self,
        living_id: &str,
    ) -> Result<WxCpLivingResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "企业微信服务已释放"))?;
        // Java `deleteReplayData`：`POST DELETE_REPLAY_DATA`
        let api_url = svc
            .wx_cp_config_storage()
            .api_url(url_living::DELETE_REPLAY_DATA);
        let response = svc
            .post(&api_url, &Self::build_living_id_body(living_id))
            .await?;
        WxCpLivingResult::from_json(&response).map_err(WxErrorException::Serde)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Java `getWatchStat`：`next_key` 空白时不放入请求体。
    #[test]
    fn test_build_get_watch_stat_body() {
        let body = WxCpLivingServiceImpl::build_get_watch_stat_body("living1", "");
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["livingid"], "living1");
        assert!(json.get("next_key").is_none());

        let body = WxCpLivingServiceImpl::build_get_watch_stat_body("living1", "0");
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["next_key"], "0");
    }

    /// Java `getUserAllLivingId`：`cursor`/`limit` 为空时不放入请求体。
    #[test]
    fn test_build_get_user_all_living_id_body() {
        let body = WxCpLivingServiceImpl::build_get_user_all_living_id_body("user1", None, None);
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["userid"], "user1");
        assert!(json.get("cursor").is_none());
        assert!(json.get("limit").is_none());

        let body =
            WxCpLivingServiceImpl::build_get_user_all_living_id_body("user1", Some("c1"), Some(20));
        let json: serde_json::Value = serde_json::from_str(&body).expect("非法 JSON");
        assert_eq!(json["cursor"], "c1");
        assert_eq!(json["limit"], 20);
    }

    /// Java `getLivingCode`：请求体 `{"openid":"o1","livingid":"l1"}`，
    /// 响应 `living_code` 提取。
    #[test]
    fn test_get_living_code_body_and_field() {
        let body = WxCpLivingServiceImpl::build_get_living_code_body("o1", "l1");
        assert_eq!(body, r#"{"openid":"o1","livingid":"l1"}"#);

        let code = WxCpLivingServiceImpl::get_string_field(
            r#"{"errcode":0,"errmsg":"ok","living_code":"CODE123"}"#,
            "living_code",
        )
        .expect("提取失败");
        assert_eq!(code, "CODE123");
    }

    /// Java `getLivingInfo`：响应 `living_info` 子对象解析。
    #[test]
    fn test_parse_living_info_sub_json() {
        let response =
            r#"{"errcode":0,"errmsg":"ok","living_info":{"theme":"直播主题","status":1}}"#;
        let info: WxCpLivingInfo =
            WxCpLivingServiceImpl::parse_sub_json(response, "living_info").expect("解析失败");
        assert_eq!(info.theme, "直播主题");
        assert_eq!(info.status, 1);
    }
}
