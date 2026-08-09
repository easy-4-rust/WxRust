//! jsapi 相关服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaJsapiServiceImpl`：
//! ticket 双检锁缓存 + 过期刷新（参照 mp 门面 `get_ticket` 模式，经
//! `WxConfigStorage` 的 TicketType 能力表达 Java `getJsapiTicketLock` /
//! `isJsapiTicketExpired` / `updateJsapiTicket` 语义）+ jsapi 签名。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::bean::WxJsapiSignature;
use wx_rust_common::enums::TicketType;
use wx_rust_common::error::{WxErrorException, WxRuntimeError};
use wx_rust_common::util::RandomUtils;
use wx_rust_common::util::crypto::Sha1;

use crate::api::WxMaService;
use crate::api::g2_services::WxMaJsapiService;
use crate::enums::g2_urls::url_g2_content::jsapi as jsapi_url;

/// jsapi 相关服务实现。
pub struct WxMaJsapiServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaJsapiServiceImpl {
    /// 构建 jsapi 服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl WxMaJsapiService for WxMaJsapiServiceImpl {
    /// 对应 Java `WxMaJsapiServiceImpl.getCardApiTicket()`（不强制刷新）。
    async fn get_card_api_ticket(&self) -> Result<String, WxErrorException> {
        self.get_card_api_ticket_with_force(false).await
    }

    /// 对应 Java `WxMaJsapiServiceImpl.getCardApiTicket(boolean)`。
    ///
    /// GET `/cgi-bin/ticket/getticket?type=wx_card`；force_refresh 时先强制
    /// 过期（Java `expireCardApiTicket`），再按双检锁刷新缓存。
    async fn get_card_api_ticket_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        self.get_ticket_inner(TicketType::WxCard, force_refresh)
            .await
    }

    /// 对应 Java `WxMaJsapiServiceImpl.getJsapiTicket()`（不强制刷新）。
    async fn get_jsapi_ticket(&self) -> Result<String, WxErrorException> {
        self.get_jsapi_ticket_with_force(false).await
    }

    /// 对应 Java `WxMaJsapiServiceImpl.getJsapiTicket(boolean)`。
    ///
    /// GET `/cgi-bin/ticket/getticket?type=jsapi`；force_refresh 时先强制
    /// 过期（Java `expireJsapiTicket`），再按双检锁刷新缓存。
    async fn get_jsapi_ticket_with_force(
        &self,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        self.get_ticket_inner(TicketType::Jsapi, force_refresh)
            .await
    }

    /// 对应 Java `WxMaJsapiServiceImpl.createJsapiSignature(String)`。
    ///
    /// 时间戳为当前 Unix 秒；随机串 `RandomUtils.getRandomStr()`；签名按
    /// `jsapi_ticket=`/`noncestr=`/`timestamp=`/`url=` 顺序拼接后 SHA1
    /// （Java `SHA1.genWithAmple`）。
    async fn create_jsapi_signature(
        &self,
        url: &str,
    ) -> Result<WxJsapiSignature, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let random_str = RandomUtils::get_random_str();
        let jsapi_ticket = self.get_jsapi_ticket_with_force(false).await?;
        let signature = Sha1::digest_with_amp(&[
            &format!("jsapi_ticket={jsapi_ticket}"),
            &format!("noncestr={random_str}"),
            &format!("timestamp={timestamp}"),
            &format!("url={url}"),
        ])
        .map_err(|e| WxErrorException::Runtime(WxRuntimeError::new(e)))?;

        Ok(WxJsapiSignature {
            app_id: svc.wx_ma_config().app_id().to_string(),
            nonce_str: random_str,
            timestamp,
            url: url.to_string(),
            signature,
        })
    }
}

impl WxMaJsapiServiceImpl {
    /// 获取指定类型 ticket（双检锁 + 缓存过期刷新）。
    ///
    /// 对应 Java `WxMaJsapiServiceImpl.getJsapiTicket(boolean)` /
    /// `getCardApiTicket(boolean)` 的锁内双检刷新语义；ticket 缓存经
    /// `WxConfigStorage` 的 `TicketType` 能力存取（Java
    /// `getJsapiTicketLock`/`getCardApiTicketLock` 对应
    /// `ticket_lock(TicketType)`）。
    async fn get_ticket_inner(
        &self,
        ticket_type: TicketType,
        force_refresh: bool,
    ) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();

        if force_refresh {
            config.expire_ticket(ticket_type);
        }

        if config.is_ticket_expired(ticket_type) {
            let lock = config.ticket_lock(ticket_type);
            let _guard = lock.lock().await;
            if config.is_ticket_expired(ticket_type) {
                let url = format!(
                    "{}?type={}",
                    jsapi_url::get_jsapi_ticket_url(config.as_ref()),
                    ticket_type.value()
                );
                let response = svc.get(&url, "").await?;
                let json: serde_json::Value = serde_json::from_str(&response)
                    .map_err(|e| WxErrorException::Serde(e.to_string()))?;
                let ticket = json
                    .get("ticket")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| WxErrorException::from_code(-99, "ticket 字段缺失"))?
                    .to_string();
                let expires_in =
                    json.get("expires_in").and_then(|v| v.as_i64()).unwrap_or(0) as i32;
                config.update_ticket(ticket_type, &ticket, expires_in);
            }
        }

        config
            .ticket(ticket_type)
            .ok_or_else(|| WxErrorException::from_code(-99, "ticket 为空"))
    }
}
