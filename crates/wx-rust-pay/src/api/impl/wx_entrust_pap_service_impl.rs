//! 对应 Java `service.impl.WxEntrustPapServiceImpl`。

use std::sync::Arc;
use std::sync::Weak;

use async_trait::async_trait;
use wx_rust_common::error::WxErrorException;

use crate::api::{WxEntrustPapService, WxPayService};
use crate::bean::*;
use crate::util::wx_pay_service_impl_utils as impl_utils;
use crate::util::wx_pay_service_impl_utils::V2Request;

/// WxEntrustPapService 实现（对应 Java `WxEntrustPapServiceImpl`）。
pub struct WxEntrustPapServiceImpl {
    /// 门面弱引用（对应 Java 构造器注入的 `WxPayService payService`）。
    pay_service: Weak<dyn WxPayService>,
}

impl WxEntrustPapServiceImpl {
    /// 构建实现（对应 Java 构造器 `WxEntrustPapServiceImpl(WxPayService)`）。
    pub fn new(pay_service: Weak<dyn WxPayService>) -> Self {
        Self { pay_service }
    }

    /// 升级门面引用（对应 Java `this.payService` 直接使用）。
    fn svc(&self) -> Result<Arc<dyn WxPayService>, WxErrorException> {
        self.pay_service
            .upgrade()
            .ok_or_else(|| impl_utils::runtime("WxPayService 已释放"))
    }
}

#[async_trait]
impl WxEntrustPapService for WxEntrustPapServiceImpl {
    async fn mp_sign(
        &self,
        wx_mp_entrust_request: &WxMpEntrustRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_mp_entrust_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        // 对应 Java：拼接签约链接（URLEncoder.encode 各字段）
        let mut sign_str = format!("{}/papay/entrustweb", svc.get_pay_base_url());
        sign_str.push_str(&format!(
            "?appid={}",
            request.appid.as_deref().unwrap_or_default()
        ));
        sign_str.push_str(&format!(
            "&contract_code={}",
            request.contract_code.as_deref().unwrap_or_default()
        ));
        sign_str.push_str(&format!(
            "&contract_display_account={}",
            urlencoding(
                request
                    .contract_display_account
                    .as_deref()
                    .unwrap_or_default()
            )
        ));
        sign_str.push_str(&format!(
            "&mch_id={}",
            request.mch_id.as_deref().unwrap_or_default()
        ));
        sign_str.push_str(&format!(
            "&notify_url={}",
            urlencoding(request.notify_url.as_deref().unwrap_or_default())
        ));
        sign_str.push_str(&format!(
            "&plan_id={}",
            request.plan_id.as_deref().unwrap_or_default()
        ));
        sign_str.push_str(&format!(
            "&request_serial={}",
            request.request_serial.unwrap_or_default()
        ));
        sign_str.push_str(&format!(
            "&timestamp={}",
            request.timestamp.as_deref().unwrap_or_default()
        ));
        if request.return_web == Some(1) {
            sign_str.push_str("&return_web=1");
        }
        if let Some(outer_id) = request.outer_id.as_deref() {
            if !outer_id.trim().is_empty() {
                sign_str.push_str(&format!("&outerid={}", urlencoding(outer_id)));
            }
        }
        sign_str.push_str(&format!(
            "&version={}",
            request.version.as_deref().unwrap_or_default()
        ));
        sign_str.push_str(&format!(
            "&sign={}",
            request.sign.as_deref().unwrap_or_default()
        ));
        Ok(sign_str)
    }

    async fn ma_sign(
        &self,
        wx_ma_entrust_request: &WxMaEntrustRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_ma_entrust_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        // 对应 Java：notify_url URL 编码后以 toString()（Bean 字段拼接）返回
        request.notify_url = Some(urlencoding(
            request.notify_url.as_deref().unwrap_or_default(),
        ));
        Ok(entrust_to_string(&request))
    }

    async fn h5_sign(
        &self,
        wx_h5_entrust_request: &WxH5EntrustRequest,
    ) -> Result<WxH5EntrustResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_h5_entrust_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        // 对应 Java：sign_type 置空后按 HMAC-SHA256 重签
        request.sign_type = None;
        let mut map = std::collections::HashMap::new();
        map.insert(
            "appid".to_string(),
            request.appid.clone().unwrap_or_default(),
        );
        map.insert(
            "contract_code".to_string(),
            request.contract_code.clone().unwrap_or_default(),
        );
        map.insert(
            "contract_display_account".to_string(),
            request.contract_display_account.clone().unwrap_or_default(),
        );
        map.insert(
            "mch_id".to_string(),
            request.mch_id.clone().unwrap_or_default(),
        );
        map.insert(
            "notify_url".to_string(),
            request.notify_url.clone().unwrap_or_default(),
        );
        map.insert(
            "plan_id".to_string(),
            request.plan_id.clone().unwrap_or_default(),
        );
        map.insert(
            "outer_id".to_string(),
            request.outer_id.clone().unwrap_or_default(),
        );
        map.insert(
            "return_appid".to_string(),
            request.return_appid.clone().unwrap_or_default(),
        );
        map.insert(
            "clientip".to_string(),
            request.client_ip.clone().unwrap_or_default(),
        );
        map.insert(
            "request_serial".to_string(),
            request.request_serial.unwrap_or_default().to_string(),
        );
        map.insert(
            "timestamp".to_string(),
            request.timestamp.clone().unwrap_or_default(),
        );
        map.insert(
            "version".to_string(),
            request.version.clone().unwrap_or_default(),
        );
        let sign = crate::util::sign_utils::SignUtils::create_sign(
            &map,
            Some(crate::constant::wx_pay_constants::sign_type::HMAC_SHA256),
            config.ent_pay_key().unwrap_or_default(),
            &[],
        )?;
        let mut url = format!("{}/papay/h5entrustweb", svc.get_pay_base_url());
        url.push_str(&format!(
            "?appid={}",
            request.appid.as_deref().unwrap_or_default()
        ));
        url.push_str(&format!(
            "&contract_code={}",
            request.contract_code.as_deref().unwrap_or_default()
        ));
        url.push_str(&format!(
            "&contract_display_account={}",
            urlencoding(
                request
                    .contract_display_account
                    .as_deref()
                    .unwrap_or_default()
            )
        ));
        url.push_str(&format!(
            "&mch_id={}",
            request.mch_id.as_deref().unwrap_or_default()
        ));
        url.push_str(&format!(
            "&notify_url={}",
            urlencoding(request.notify_url.as_deref().unwrap_or_default())
        ));
        url.push_str(&format!(
            "&plan_id={}",
            request.plan_id.as_deref().unwrap_or_default()
        ));
        if let Some(outer_id) = request.outer_id.as_deref() {
            if !outer_id.trim().is_empty() {
                url.push_str(&format!("&outerid={}", urlencoding(outer_id)));
            }
        }
        if let Some(return_appid) = request.return_appid.as_deref() {
            if !return_appid.trim().is_empty() {
                url.push_str(&format!("&return_appid={return_appid}"));
            }
        }
        url.push_str(&format!(
            "&clientip={}",
            request.client_ip.as_deref().unwrap_or_default()
        ));
        url.push_str(&format!(
            "&request_serial={}",
            request.request_serial.unwrap_or_default()
        ));
        url.push_str(&format!(
            "&timestamp={}",
            request.timestamp.as_deref().unwrap_or_default()
        ));
        url.push_str(&format!(
            "&version={}",
            request.version.as_deref().unwrap_or_default()
        ));
        url.push_str(&format!("&sign={sign}"));
        // 对应 Java：以 getV3 通道请求（h5entrustweb）
        let response_content = svc.get_v3(&url).await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type.as_deref(),
            true,
            WxH5EntrustResult::from_xml,
        )
    }

    async fn pay_sign(
        &self,
        wx_pay_entrust_request: &WxPayEntrustRequest,
    ) -> Result<WxPayEntrustResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_pay_entrust_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/contractorder", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxPayEntrustResult::from_xml,
        )
    }

    async fn withhold(
        &self,
        wx_withhold_request: &WxWithholdRequest,
    ) -> Result<WxWithholdResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_withhold_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/pappayapply", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxWithholdResult::from_xml,
        )
    }

    async fn withhold_partner(
        &self,
        wx_withhold_request: &WxWithholdRequest,
    ) -> Result<WxPayCommonResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_withhold_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/partner/pappayapply", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxPayCommonResult::from_xml,
        )
    }

    async fn pre_withhold(
        &self,
        wx_pre_withhold_request: &WxPreWithholdRequest,
    ) -> Result<String, WxErrorException> {
        let svc = self.svc()?;
        let body = serde_json::to_string(wx_pre_withhold_request)
            .map_err(|e| impl_utils::runtime(e.to_string()))?;
        let url = format!(
            "{}/v3/papay/contracts/{}/notify",
            svc.get_pay_base_url(),
            wx_pre_withhold_request
                .contract_id
                .as_deref()
                .unwrap_or_default()
        );
        svc.post_v3(&url, &body).await
    }

    async fn query_sign(
        &self,
        wx_sign_query_request: &WxSignQueryRequest,
    ) -> Result<WxSignQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_sign_query_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/papay/querycontract", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxSignQueryResult::from_xml,
        )
    }

    async fn termination_contract(
        &self,
        wx_terminated_contract_request: &WxTerminatedContractRequest,
    ) -> Result<WxTerminationContractResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_terminated_contract_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/papay/deletecontract", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxTerminationContractResult::from_xml,
        )
    }

    async fn pap_order_query(
        &self,
        wx_withhold_order_query_request: &WxWithholdOrderQueryRequest,
    ) -> Result<WxWithholdOrderQueryResult, WxErrorException> {
        let svc = self.svc()?;
        let config = svc.wx_pay_config();
        let mut request = wx_withhold_order_query_request.clone();
        impl_utils::check_and_sign(config.as_ref(), &mut request)?;
        let url = format!("{}/pay/paporderquery", svc.get_pay_base_url());
        let response_content = svc
            .post(&url, &request.to_xml().map_err(impl_utils::runtime)?, false)
            .await?;
        impl_utils::parse_v2_result(
            config.as_ref(),
            &response_content,
            request.sign_type(),
            true,
            WxWithholdOrderQueryResult::from_xml,
        )
    }

    async fn parse_sign_notify_result(
        &self,
        xml_data: &str,
    ) -> Result<WxSignQueryResult, WxErrorException> {
        // 对应 Java `WxSignQueryResult.fromXML(xmlData)`
        WxSignQueryResult::from_xml(xml_data).map_err(impl_utils::runtime)
    }
}

/// URL 编码（对应 Java `URLEncoder.encode(s, "UTF-8")`）。
fn urlencoding(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        match b {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'*' => {
                out.push(b as char)
            }
            b' ' => out.push('+'),
            _ => out.push_str(&format!("%{b:02X}")),
        }
    }
    out
}

/// 对应 Java `WxMaEntrustRequest.toString()`（Bean 字段拼接，XStream 语义）。
fn entrust_to_string(_request: &WxMaEntrustRequest) -> String {
    // ADAPTED：Java 依赖 Lombok @ToString 按字段顺序拼接 key=value 对；
    // 拼接结果仅用于构造签约跳转链接，Rust 侧以序列化 JSON 表达同义信息
    serde_json::to_string(_request).unwrap_or_default()
}
