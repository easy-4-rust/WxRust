#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""第二批服务 trait + impl 生成（Java 语义严格镜像）。"""
import os

BASE = "/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust"
API = os.path.join(BASE, "crates", "wx-rust-mp", "src", "api")


def snake(name: str) -> str:
    import re
    s = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', name)
    return s.lower()


def gen_trait(name, methods, imports):
    trait_name = "WxMp" + "".join(p.capitalize() for p in name.split("_")) + "Service"
    lines = [
        f"//! {trait_name}。",
        "//!",
        f"//! 对应 Java `me.chanjar.weixin.mp.api.{trait_name}`。",
        "",
        "use async_trait::async_trait;",
        "",
        "use wx_rust_common::error::WxErrorException;",
        "",
    ]
    lines += imports
    lines += ["", f"/// {trait_name.replace('WxMp', '公众号')}。", "#[async_trait]", f"pub trait {trait_name}: Send + Sync {{"]
    for sig, _ in methods:
        lines.append(f"    {sig};")
        lines.append("")
    lines.append("}")
    path = os.path.join(API, f"wx_mp_{name}_service.rs")
    open(path, "w", encoding="utf-8").write("\n".join(lines) + "\n")
    return trait_name


def gen_impl(name, trait_name, methods, imports, helpers=""):
    java = trait_name.replace("WxMp", "WxMp")
    lines = [
        f"//! {trait_name} 实现。",
        "//!",
        f"//! 对应 Java `me.chanjar.weixin.mp.api.impl.{trait_name}Impl`。",
        "",
        "use async_trait::async_trait;",
        "use std::sync::Weak;",
        "",
        "use wx_rust_common::error::WxErrorException;",
        "",
        f"use crate::api::{{WxMpService, {trait_name}}};",
    ]
    lines += imports
    lines += ["", f"/// {trait_name.replace('WxMp', '公众号')}实现。", f"pub struct {trait_name}Impl {{",
              "    service: Weak<dyn WxMpService>,", "}", "",
              f"impl {trait_name}Impl {{", f"    /// 构建 {trait_name.replace('WxMp', '公众号')}。",
              "    pub fn new(service: Weak<dyn WxMpService>) -> Self {", "        Self { service }", "    }"]
    if helpers:
        lines.append(helpers)
    lines += ["}", "", "#[async_trait]", f"impl {trait_name} for {trait_name}Impl {{"]
    for sig, body in methods:
        lines.append(f"    {sig} {{")
        lines.append("        let svc = self")
        lines.append("            .service")
        lines.append("            .upgrade()")
        lines.append('            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;')
        lines.append("        let config = svc.wx_mp_config_storage();")
        lines.append(body)
        lines.append("    }")
        lines.append("")
    lines.append("}")
    path = os.path.join(API, "impl", f"wx_mp_{name}_service_impl.rs")
    open(path, "w", encoding="utf-8").write("\n".join(lines) + "\n")

    # 注册模块
    p = os.path.join(API, "mod.rs")
    src = open(p, encoding="utf-8").read()
    if f"pub mod wx_mp_{name}_service;" not in src:
        src = src.replace("pub mod wx_mp_service;", f"pub mod wx_mp_{name}_service;\npub mod wx_mp_service;", 1)
        src = src.rstrip() + f"\npub use wx_mp_{name}_service::{trait_name};\n"
        open(p, "w", encoding="utf-8").write(src)
    p = os.path.join(API, "impl", "mod.rs")
    src = open(p, encoding="utf-8").read()
    if f"wx_mp_{name}_service_impl" not in src:
        src = src.replace("pub mod wx_mp_service_impl;", f"pub mod wx_mp_{name}_service_impl;\npub mod wx_mp_service_impl;", 1)
        src = src.rstrip() + f"\npub use wx_mp_{name}_service_impl::{trait_name}Impl;\n"
        open(p, "w", encoding="utf-8").write(src)
    print("impl:", trait_name)


# ================ material ================
material_imports = [
    "use crate::bean::material::{",
    "    WxMediaImgUploadResult, WxMpMaterial, WxMpMaterialCountResult, WxMpMaterialFileBatchGetResult,",
    "    WxMpMaterialNews, WxMpMaterialNewsBatchGetResult, WxMpMaterialUploadResult, WxMpMaterialVideoInfoResult,",
    "};",
    "use crate::enums::wx_mp_api_url::material as material_url;",
    "use wx_rust_common::bean::result::WxMediaUploadResult;",
]
material_methods = [
    ("async fn media_upload(&self, media_type: &str, file_path: &str) -> Result<WxMediaUploadResult, WxErrorException>",
     '''        let token = svc.get_access_token().await?;
        let url = material_url::media_upload(config.as_ref(), media_type);
        Self::upload_file(svc, &format!("{url}?access_token={token}"), "media", file_path).await'''),
    ("async fn media_download(&self, media_id: &str) -> Result<Vec<u8>, WxErrorException>",
     '''        let token = svc.get_access_token().await?;
        let url = material_url::media_get(config.as_ref());
        let url = format!("{url}?access_token={token}&media_id={media_id}");
        let bytes = svc.http_client().get(&url).send().await
            .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?
            .bytes().await
            .map_err(|e| WxErrorException::from_code(-99, format!("素材下载失败: {e}")))?;
        Ok(bytes.to_vec())'''),
    ("async fn media_img_upload(&self, file_path: &str) -> Result<WxMediaImgUploadResult, WxErrorException>",
     '''        let token = svc.get_access_token().await?;
        let url = material_url::media_img_upload(config.as_ref());
        let response = Self::upload_file(svc, &format!("{url}?access_token={token}"), "img", file_path).await?;
        WxMediaImgUploadResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn material_file_upload(&self, media_type: &str, material: &WxMpMaterial) -> Result<WxMpMaterialUploadResult, WxErrorException>",
     '''        let token = svc.get_access_token().await?;
        let url = material_url::material_add(config.as_ref(), media_type);
        let url = format!("{url}?access_token={token}");
        let file_path = material.file.as_deref().ok_or_else(|| WxErrorException::from_code(-99, "文件路径为空"))?;
        let mut form = reqwest::multipart::Form::new()
            .part("media", reqwest::multipart::Part::bytes(std::fs::read(file_path)
                .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?)
                .file_name("media"));
        if media_type == "video" {
            let desc = serde_json::json!({"title": material.video_title, "introduction": material.video_introduction});
            form = form.text("description", desc.to_string());
        }
        let text = Self::send_form(svc, &url, form).await?;
        WxMpMaterialUploadResult::from_json(&text).map_err(WxErrorException::Serde)'''),
    ("async fn material_video_info(&self, media_id: &str) -> Result<WxMpMaterialVideoInfoResult, WxErrorException>",
     '''        let body = serde_json::json!({"media_id": media_id});
        let response = svc.post(&material_url::material_get(config.as_ref()), &body.to_string()).await?;
        WxMpMaterialVideoInfoResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn material_news_info(&self, media_id: &str) -> Result<WxMpMaterialNews, WxErrorException>",
     '''        let body = serde_json::json!({"media_id": media_id});
        let response = svc.post(&material_url::material_get(config.as_ref()), &body.to_string()).await?;
        WxMpMaterialNews::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn material_delete(&self, media_id: &str) -> Result<bool, WxErrorException>",
     '''        let body = serde_json::json!({"media_id": media_id});
        let response = svc.post(&material_url::material_del(config.as_ref()), &body.to_string()).await?;
        Self::err_code_is_zero(&response)'''),
    ("async fn material_count(&self) -> Result<WxMpMaterialCountResult, WxErrorException>",
     '''        let response = svc.get(&material_url::material_count(config.as_ref()), "").await?;
        WxMpMaterialCountResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn material_news_batch_get(&self, offset: i32, count: i32) -> Result<WxMpMaterialNewsBatchGetResult, WxErrorException>",
     '''        let body = serde_json::json!({"type": "news", "offset": offset, "count": count});
        let response = svc.post(&material_url::material_batch_get(config.as_ref()), &body.to_string()).await?;
        WxMpMaterialNewsBatchGetResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn material_file_batch_get(&self, r#type: &str, offset: i32, count: i32) -> Result<WxMpMaterialFileBatchGetResult, WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "offset": offset, "count": count});
        let response = svc.post(&material_url::material_batch_get(config.as_ref()), &body.to_string()).await?;
        WxMpMaterialFileBatchGetResult::from_json(&response).map_err(WxErrorException::Serde)'''),
]
material_helpers = '''
    /// 校验响应 errcode 是否为 0。
    fn err_code_is_zero(json: &str) -> Result<bool, WxErrorException> {
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(value.get("errcode").map(|v| v.to_string() == "0").unwrap_or(false))
    }

    /// multipart 上传单个文件字段并返回响应文本。
    async fn upload_file(
        svc: &dyn WxMpService,
        url: &str,
        field: &str,
        file_path: &str,
    ) -> Result<String, WxErrorException> {
        let part = reqwest::multipart::Part::bytes(std::fs::read(file_path)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?)
            .file_name("file");
        let form = reqwest::multipart::Form::new().part(field.to_string(), part);
        Self::send_form(svc, url, form).await
    }

    /// 发送 multipart 表单并校验 errcode。
    async fn send_form(
        svc: &dyn WxMpService,
        url: &str,
        form: reqwest::multipart::Form,
    ) -> Result<String, WxErrorException> {
        let text = svc.http_client().post(url).multipart(form).send().await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?
            .text().await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?;
        let error = wx_rust_common::error::WxError::from_json_with_type(&text, Some(wx_rust_common::enums::WxType::Mp));
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(error.error_code, error.error_msg.unwrap_or_default()));
        }
        Ok(text)
    }
'''

# ================ shake ================
shake_imports = [
    "use crate::bean::{WxMpShakeAroundDeviceBindPageQuery, WxMpShakeAroundPageAddQuery, WxMpShakeAroundPageAddResult, WxMpShakeAroundRelationSearchQuery, WxMpShakeAroundRelationSearchResult, WxMpShakeInfoResult, WxMpShakeQuery};",
    "use crate::enums::wx_mp_api_url::shake;",
]
shake_methods = [
    ("async fn get_shake_info(&self, query: &WxMpShakeQuery) -> Result<WxMpShakeInfoResult, WxErrorException>",
     '''        let body = serde_json::to_string(query).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&shake::get_shake_info(config.as_ref()), &body).await?;
        WxMpShakeInfoResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn page_add(&self, query: &WxMpShakeAroundPageAddQuery) -> Result<WxMpShakeAroundPageAddResult, WxErrorException>",
     '''        let body = serde_json::to_string(query).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&shake::page_add(config.as_ref()), &body).await?;
        WxMpShakeAroundPageAddResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn device_bind_page_query(&self, query: &WxMpShakeAroundDeviceBindPageQuery) -> Result<bool, WxErrorException>",
     '''        let body = serde_json::to_string(query).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&shake::device_bind_page(config.as_ref()), &body).await?;
        Ok(true)'''),
    ("async fn relation_search(&self, query: &WxMpShakeAroundRelationSearchQuery) -> Result<WxMpShakeAroundRelationSearchResult, WxErrorException>",
     '''        let body = serde_json::to_string(query).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&shake::relation_search(config.as_ref()), &body).await?;
        WxMpShakeAroundRelationSearchResult::from_json(&response).map_err(WxErrorException::Serde)'''),
]

# ================ card ================
card_imports = [
    "use crate::bean::card::{WxMpCardCreateRequest, WxMpCardCreateResult, WxMpCardLandingPageCreateRequest, WxMpCardLandingPageCreateResult, WxMpCardQrcodeCreateResult, WxMpCardResult};",
    "use crate::enums::wx_mp_api_url::card as card_url;",
    "use wx_rust_common::bean::WxCardApiSignature;",
    "use wx_rust_common::util::crypto::{Sha1, WxCryptUtil};",
]
card_methods = [
    ("async fn get_card_api_ticket(&self, force_refresh: bool) -> Result<String, WxErrorException>",
     '''        svc.get_ticket(wx_rust_common::config::TicketType::WxCard, force_refresh).await'''),
    ("async fn create_card_api_signature(&self, optional_sign_param: &[&str]) -> Result<WxCardApiSignature, WxErrorException>",
     '''        let ticket = self.get_card_api_ticket(false).await?;
        let nonce_str = format!("{}", chrono::Utc::now().timestamp_millis());
        let timestamp = format!("{}", chrono::Utc::now().timestamp());
        let mut params = vec![ticket.as_str(), nonce_str.as_str(), timestamp.as_str()];
        params.extend_from_slice(optional_sign_param);
        let signature = Sha1::digest(&params)?;
        Ok(WxCardApiSignature::new(
            signature,
            timestamp.parse::<i64>().unwrap_or(0),
            nonce_str,
            ticket,
        ))'''),
    ("async fn decrypt_card_code(&self, encrypt_code: &str) -> Result<String, WxErrorException>",
     '''        let body = serde_json::json!({"encrypt_code": encrypt_code});
        let response = svc.post(&card_url::card_code_decrypt(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("code").and_then(|v| v.as_str()).map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "code 缺失"))'''),
    ("async fn query_card_code(&self, card_id: &str, code: &str, check_consume: bool) -> Result<WxMpCardResult, WxErrorException>",
     '''        let body = serde_json::json!({"card_id": card_id, "code": code, "check_consume": check_consume});
        let response = svc.post(&card_url::card_code_get(config.as_ref()), &body.to_string()).await?;
        WxMpCardResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn consume_card_code(&self, code: &str) -> Result<String, WxErrorException>",
     '''        let body = serde_json::json!({"code": code});
        let response = svc.post(&card_url::card_code_consume(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("card").and_then(|v| v.get("card_id")).and_then(|v| v.as_str()).map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "card_id 缺失"))'''),
    ("async fn mark_card_code(&self, code: &str, card_id: &str, open_id: &str, is_mark: bool) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"code": code, "card_id": card_id, "openid": open_id, "is_mark": if is_mark { 1 } else { 0 }});
        svc.post(&card_url::card_code_mark(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_card_detail(&self, card_id: &str) -> Result<String, WxErrorException>",
     '''        let body = serde_json::json!({"card_id": card_id});
        let response = svc.post(&card_url::card_get(config.as_ref()), &body.to_string()).await?;
        Ok(response)'''),
    ("async fn add_test_white_list(&self, openid: &str) -> Result<String, WxErrorException>",
     '''        let body = serde_json::json!({"openid": [openid]});
        let response = svc.post(&card_url::card_test_whitelist(config.as_ref()), &body.to_string()).await?;
        Ok(response)'''),
    ("async fn create_card(&self, request: &WxMpCardCreateRequest) -> Result<WxMpCardCreateResult, WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&card_url::card_create(config.as_ref()), &body).await?;
        WxMpCardCreateResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn delete_card(&self, card_id: &str) -> Result<WxMpCardDeleteResult, WxErrorException>",
     '''        let body = serde_json::json!({"card_id": card_id});
        let response = svc.post(&card_url::card_delete(config.as_ref()), &body.to_string()).await?;
        WxMpCardDeleteResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn create_landing_page(&self, request: &WxMpCardLandingPageCreateRequest) -> Result<WxMpCardLandingPageCreateResult, WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&card_url::card_landing_page_create(config.as_ref()), &body).await?;
        WxMpCardLandingPageCreateResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn create_qrcode(&self, request: &WxMpCardQrcodeCreateRequest) -> Result<WxMpCardQrcodeCreateResult, WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&card_url::card_qrcode_create(config.as_ref()), &body).await?;
        WxMpCardQrcodeCreateResult::from_json(&response).map_err(WxErrorException::Serde)'''),
]

# ================ member card ================
member_card_imports = [
    "use crate::bean::card::membercard::{MemberCardActivateUserFormRequest, MemberCardActivateUserFormResult, MemberCardUpdateRequest, WxMpMemberCardActivateTempInfoResult, WxMpMemberCardActivatedMessage, WxMpMemberCardCreateMessage, WxMpMemberCardUpdateMessage, WxMpMemberCardUpdateResult, WxMpMemberCardUserInfoResult};",
    "use crate::bean::card::CardUpdateResult;",
    "use crate::enums::wx_mp_api_url::member_card;",
]
member_card_methods = [
    ("async fn create_member_card(&self, message: &WxMpMemberCardCreateMessage) -> Result<String, WxErrorException>",
     '''        let body = serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&member_card::member_card_create(config.as_ref()), &body).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("card_id").and_then(|v| v.as_str()).map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "card_id 缺失"))'''),
    ("async fn activate_member_card(&self, message: &WxMpMemberCardActivatedMessage) -> Result<String, WxErrorException>",
     '''        let body = serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&member_card::member_card_activate(config.as_ref()), &body).await?;
        Ok(response)'''),
    ("async fn get_user_info(&self, card_id: &str, code: &str) -> Result<WxMpMemberCardUserInfoResult, WxErrorException>",
     '''        let body = serde_json::json!({"card_id": card_id, "code": code});
        let response = svc.post(&member_card::member_card_user_info_get(config.as_ref()), &body.to_string()).await?;
        WxMpMemberCardUserInfoResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn update_user_member_card(&self, message: &WxMpMemberCardUpdateMessage) -> Result<WxMpMemberCardUpdateResult, WxErrorException>",
     '''        let body = serde_json::to_string(message).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&member_card::member_card_update_user(config.as_ref()), &body).await?;
        WxMpMemberCardUpdateResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn set_activate_user_form(&self, request: &MemberCardActivateUserFormRequest) -> Result<MemberCardActivateUserFormResult, WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&member_card::member_card_activate_user_form(config.as_ref()), &body).await?;
        MemberCardActivateUserFormResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn update_card_info(&self, request: &MemberCardUpdateRequest) -> Result<CardUpdateResult, WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&member_card::member_card_update(config.as_ref()), &body).await?;
        CardUpdateResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_activate_temp_info(&self, activate_ticket: &str) -> Result<WxMpMemberCardActivateTempInfoResult, WxErrorException>",
     '''        let body = serde_json::json!({"activate_ticket": activate_ticket});
        let response = svc.post(&member_card::member_card_activate_temp_info(config.as_ref()), &body.to_string()).await?;
        WxMpMemberCardActivateTempInfoResult::from_json(&response).map_err(WxErrorException::Serde)'''),
]

# ================ guide ================
guide_imports = [
    "use crate::bean::guide::{WxMpAddGuideAutoReply, WxMpGuideAcctConfig, WxMpGuideConfig, WxMpGuideGroupInfoList, WxMpGuideInfo, WxMpGuideList, WxMpGuideMsgList};",
    "use crate::enums::wx_mp_api_url::guide;",
]
guide_methods = [
    ("async fn add_guide(&self, guide_info: &WxMpGuideInfo) -> Result<(), WxErrorException>",
     '''        let body = serde_json::to_string(guide_info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&guide::add_guide(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn update_guide(&self, guide_info: &WxMpGuideInfo) -> Result<(), WxErrorException>",
     '''        let body = serde_json::to_string(guide_info).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&guide::update_guide(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn get_guide(&self, account: &str, openid: &str) -> Result<WxMpGuideInfo, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid});
        let response = svc.post(&guide::get_guide(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let guide = value.get("guide_info").ok_or_else(|| WxErrorException::from_code(-99, "guide_info 缺失"))?;
        serde_json::from_value(guide.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn del_guide(&self, account: &str, openid: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid});
        svc.post(&guide::del_guide(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn list_guide(&self, page: i32, num: i32) -> Result<WxMpGuideList, WxErrorException>",
     '''        let body = serde_json::json!({"page": page, "num": num});
        let response = svc.post(&guide::list_guide(config.as_ref()), &body.to_string()).await?;
        WxMpGuideList::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn create_guide_qr_code(&self, account: &str, openid: &str, qrcode_info: &str) -> Result<String, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "qrcode_info": qrcode_info});
        let response = svc.post(&guide::create_qr_code(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("qrcode_url").and_then(|v| v.as_str()).map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "qrcode_url 缺失"))'''),
    ("async fn get_guide_chat_record(&self, account: &str, openid: &str, client_openid: &str, begin_time: i64, end_time: i64, page: i32, num: i32) -> Result<WxMpGuideMsgList, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "openid": client_openid, "begin_time": begin_time, "end_time": end_time, "page": page, "num": num});
        let response = svc.post(&guide::get_guide_chat_record(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("guide_msg_list").ok_or_else(|| WxErrorException::from_code(-99, "guide_msg_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn set_guide_config(&self, account: &str, openid: &str, is_delete: bool, guide_fast_reply_list: &[String], guide_auto_reply: &WxMpAddGuideAutoReply, guide_auto_reply_plus: &WxMpAddGuideAutoReply) -> Result<(), WxErrorException>",
     '''        let mut body = serde_json::Map::new();
        body.insert("guide_account".into(), serde_json::json!(account));
        body.insert("guide_openid".into(), serde_json::json!(openid));
        body.insert("is_delete".into(), serde_json::json!(if is_delete { 1 } else { 0 }));
        body.insert("guide_fast_reply_list".into(), serde_json::json!(guide_fast_reply_list));
        body.insert("guide_auto_reply".into(), serde_json::to_value(guide_auto_reply).unwrap_or_default());
        body.insert("guide_auto_reply_plus".into(), serde_json::to_value(guide_auto_reply_plus).unwrap_or_default());
        svc.post(&guide::set_guide_config(config.as_ref()), &serde_json::Value::Object(body).to_string()).await?;
        Ok(())'''),
    ("async fn get_guide_config(&self, account: &str, openid: &str) -> Result<WxMpGuideConfig, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid});
        let response = svc.post(&guide::get_guide_config(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let config = value.get("guide_config").ok_or_else(|| WxErrorException::from_code(-99, "guide_config 缺失"))?;
        serde_json::from_value(config.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn set_guide_acct_config(&self, is_delete: bool, black_keyword: &[String], guide_auto_reply: &str) -> Result<(), WxErrorException>",
     '''        let mut body = serde_json::Map::new();
        body.insert("is_delete".into(), serde_json::json!(if is_delete { 1 } else { 0 }));
        body.insert("black_keyword".into(), serde_json::json!(black_keyword));
        body.insert("guide_auto_reply".into(), serde_json::json!(guide_auto_reply));
        svc.post(&guide::set_guide_acct_config(config.as_ref()), &serde_json::Value::Object(body).to_string()).await?;
        Ok(())'''),
    ("async fn get_guide_acct_config(&self) -> Result<WxMpGuideAcctConfig, WxErrorException>",
     '''        let response = svc.post(&guide::get_guide_acct_config(config.as_ref()), "{}").await?;
        WxMpGuideAcctConfig::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn new_guide_group(&self, name: &str) -> Result<i64, WxErrorException>",
     '''        let body = serde_json::json!({"group_name": name});
        let response = svc.post(&guide::new_guide_group(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("group_id").and_then(|v| v.as_i64())
            .ok_or_else(|| WxErrorException::from_code(-99, "group_id 缺失"))'''),
    ("async fn get_guide_group_list(&self, page: i32, num: i32) -> Result<WxMpGuideGroupInfoList, WxErrorException>",
     '''        let body = serde_json::json!({"page": page, "num": num});
        let response = svc.post(&guide::get_guide_group_list(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("group_list").ok_or_else(|| WxErrorException::from_code(-99, "group_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
]

# ================ marketing ================
marketing_imports = [
    "use crate::bean::marketing::{WxMpAdLeadFilter, WxMpAdLeadResult, WxMpUserAction, WxMpUserActionSet};",
    "use crate::enums::wx_mp_api_url::marketing;",
]
marketing_methods = [
    ("async fn add_user_action_sets(&self, r#type: &str, name: &str, description: &str) -> Result<i64, WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "name": name, "description": description});
        let response = svc.post(&marketing::add_user_action_sets(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("user_action_set_id").and_then(|v| v.as_i64())
            .ok_or_else(|| WxErrorException::from_code(-99, "user_action_set_id 缺失"))'''),
    ("async fn get_user_action_sets(&self, user_action_set_id: i64) -> Result<Vec<WxMpUserActionSet>, WxErrorException>",
     '''        let body = serde_json::json!({"user_action_set_id": user_action_set_id});
        let response = svc.post(&marketing::get_user_action_sets(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("user_action_set").ok_or_else(|| WxErrorException::from_code(-99, "user_action_set 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn add_user_action(&self, actions: &[WxMpUserAction]) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"user_actions": actions});
        svc.post(&marketing::add_user_action(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_ad_leads(&self, begin_date: &str, end_date: &str, filtering: &[WxMpAdLeadFilter], page: i32, page_size: i32) -> Result<WxMpAdLeadResult, WxErrorException>",
     '''        let mut body = serde_json::Map::new();
        body.insert("start_date".into(), serde_json::json!(begin_date));
        body.insert("end_date".into(), serde_json::json!(end_date));
        body.insert("filtering".into(), serde_json::json!(filtering));
        body.insert("page".into(), serde_json::json!(page));
        body.insert("page_size".into(), serde_json::json!(page_size));
        let response = svc.post(&marketing::get_ad_leads(config.as_ref()), &serde_json::Value::Object(body).to_string()).await?;
        WxMpAdLeadResult::from_json(&response).map_err(WxErrorException::Serde)'''),
]

# ================ subscribe_msg ================
subscribe_msg_imports = [
    "use crate::bean::subscribe::WxMpSubscribeMessage;",
    "use wx_rust_common::bean::subscribemsg::{CategoryData, PubTemplateKeyword, PubTemplateTitleListResult, TemplateInfo};",
    "use crate::enums::wx_mp_api_url::subscribe_msg;",
]
subscribe_msg_methods = [
    ("async fn send_once(&self, message: &WxMpSubscribeMessage) -> Result<bool, WxErrorException>",
     '''        let body = message.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&subscribe_msg::send_once(config.as_ref()), &body).await?;
        Self::err_code_is_zero(&response)'''),
    ("async fn send(&self, message: &WxMpSubscribeMessage) -> Result<String, WxErrorException>",
     '''        let body = message.to_json().map_err(WxErrorException::Serde)?;
        let response = svc.post(&subscribe_msg::send(config.as_ref()), &body).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("msgid").map(|v| v.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "msgid 缺失"))'''),
    ("async fn get_pub_template_title_list(&self, ids: &[&str], start: i32, limit: i32) -> Result<PubTemplateTitleListResult, WxErrorException>",
     '''        let mut query = format!("start={start}&limit={limit}");
        for id in ids {
            query.push_str(&format!("&ids={id}"));
        }
        let response = svc.post(&subscribe_msg::get_pub_template_title_list(config.as_ref()), &query).await?;
        PubTemplateTitleListResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_pub_template_key_words_by_id(&self, id: &str) -> Result<Vec<PubTemplateKeyword>, WxErrorException>",
     '''        let body = serde_json::json!({"tid": id});
        let response = svc.post(&subscribe_msg::get_pub_template_key_words_by_id(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("data").ok_or_else(|| WxErrorException::from_code(-99, "data 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn add_template(&self, id: &str, keyword_id_list: &[i32], scene_desc: &str) -> Result<String, WxErrorException>",
     '''        let body = serde_json::json!({"tid": id, "kidList": keyword_id_list, "sceneDesc": scene_desc});
        let response = svc.post(&subscribe_msg::template_add(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("priTmplId").and_then(|v| v.as_str()).map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "priTmplId 缺失"))'''),
    ("async fn get_template_list(&self) -> Result<Vec<TemplateInfo>, WxErrorException>",
     '''        let response = svc.post(&subscribe_msg::template_list(config.as_ref()), "{}").await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("data").ok_or_else(|| WxErrorException::from_code(-99, "data 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn del_template(&self, template_id: &str) -> Result<bool, WxErrorException>",
     '''        let body = serde_json::json!({"priTmplId": template_id});
        let response = svc.post(&subscribe_msg::template_del(config.as_ref()), &body.to_string()).await?;
        Self::err_code_is_zero(&response)'''),
    ("async fn get_category(&self) -> Result<Vec<CategoryData>, WxErrorException>",
     '''        let response = svc.post(&subscribe_msg::get_category(config.as_ref()), "{}").await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("data").ok_or_else(|| WxErrorException::from_code(-99, "data 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
]
subscribe_msg_helpers = '''
    /// 校验响应 errcode 是否为 0。
    fn err_code_is_zero(json: &str) -> Result<bool, WxErrorException> {
        let value: serde_json::Value = serde_json::from_str(json)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        Ok(value.get("errcode").map(|v| v.to_string() == "0").unwrap_or(false))
    }
'''

# ================ ai_open ================
ai_open_imports = [
    "use crate::enums::wx_mp_api_url::ai_open;",
]
ai_open_methods = [
    ("async fn upload_voice(&self, voice_id: &str, lang: &str, file_path: &str) -> Result<(), WxErrorException>",
     '''        let token = svc.get_access_token().await?;
        let url = ai_open::voice_upload(config.as_ref(), &format!("{voice_id}"));  // format 参数占位
        let _ = url;
        // Java：POST voice 文件（multipart），URL 带 format/voice_id/lang
        let url = format!("{}", ai_open::voice_upload(config.as_ref(), ""));
        let url = url.replace("&voice_id=&lang=", "");
        let url = format!("{}&voice_id={}&lang={}", url, voice_id, lang);
        let url = url.replace("format=%s", "format=amr");
        let url = format!("{url}?access_token={token}");
        let part = reqwest::multipart::Part::bytes(std::fs::read(file_path)
            .map_err(|e| WxErrorException::from_code(-99, format!("读取文件失败: {e}")))?)
            .file_name("voice");
        let form = reqwest::multipart::Form::new().part("media", part);
        let text = svc.http_client().post(&url).multipart(form).send().await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?
            .text().await
            .map_err(|e| WxErrorException::from_code(-99, format!("上传失败: {e}")))?;
        let error = wx_rust_common::error::WxError::from_json_with_type(&text, Some(wx_rust_common::enums::WxType::Mp));
        if error.error_code != 0 {
            return Err(WxErrorException::from_code(error.error_code, error.error_msg.unwrap_or_default()));
        }
        Ok(())'''),
    ("async fn query_recognition_result(&self, voice_id: &str, lang: &str) -> Result<String, WxErrorException>",
     '''        let body = serde_json::json!({"voice_id": voice_id, "lang": lang});
        let response = svc.post(&ai_open::voice_query_result(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("result").and_then(|v| v.as_str()).map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "result 缺失"))'''),
    ("async fn translate(&self, lang_from: &str, lang_to: &str, content: &str) -> Result<String, WxErrorException>",
     '''        let body = serde_json::json!({"content": content});
        let url = ai_open::translate(config.as_ref(), lang_from);
        let url = url.replace("lto=%s", &format!("lto={lang_to}"));
        let response = svc.post(&url, &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("to_content").and_then(|v| v.as_str()).map(String::from)
            .ok_or_else(|| WxErrorException::from_code(-99, "to_content 缺失"))'''),
]

# ================ ocr ================
ocr_imports = [
    "use crate::enums::wx_mp_api_url::ocr as ocr_url;",
    "use wx_rust_common::bean::ocr::{WxOcrBankCardResult, WxOcrBizLicenseResult, WxOcrCommResult, WxOcrDrivingLicenseResult, WxOcrDrivingResult, WxOcrIdCardResult};",
]
ocr_methods = [
    ("async fn id_card(&self, img_url: &str) -> Result<WxOcrIdCardResult, WxErrorException>",
     '''        let response = Self::post_img(svc, &ocr_url::id_card(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
    ("async fn bank_card(&self, img_url: &str) -> Result<WxOcrBankCardResult, WxErrorException>",
     '''        let response = Self::post_img(svc, &ocr_url::bank_card(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
    ("async fn driving(&self, img_url: &str) -> Result<WxOcrDrivingResult, WxErrorException>",
     '''        let response = Self::post_img(svc, &ocr_url::driving(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
    ("async fn driving_license(&self, img_url: &str) -> Result<WxOcrDrivingLicenseResult, WxErrorException>",
     '''        let response = Self::post_img(svc, &ocr_url::driving_license(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
    ("async fn biz_license(&self, img_url: &str) -> Result<WxOcrBizLicenseResult, WxErrorException>",
     '''        let response = Self::post_img(svc, &ocr_url::biz_license(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
    ("async fn comm(&self, img_url: &str) -> Result<WxOcrCommResult, WxErrorException>",
     '''        let response = Self::post_img(svc, &ocr_url::comm(config.as_ref()), img_url).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
]
ocr_helpers = '''
    /// 图片 URL OCR 查询（对应 Java `img_url` 表单参数）。
    async fn post_img(svc: &dyn WxMpService, url: &str, img_url: &str) -> Result<String, WxErrorException> {
        let body = serde_json::json!({"img_url": img_url});
        svc.post(url, &body.to_string()).await
    }
'''

# ================ img_proc ================
img_proc_imports = [
    "use crate::enums::wx_mp_api_url::img_proc;",
    "use wx_rust_common::bean::imgproc::{WxImgProcAiCropResult, WxImgProcQrCodeResult, WxImgProcSuperResolutionResult};",
]
img_proc_methods = [
    ("async fn qr_code(&self, img_url: &str) -> Result<WxImgProcQrCodeResult, WxErrorException>",
     '''        let body = serde_json::json!({"img_url": img_url});
        let response = svc.post(&img_proc::qr_code(config.as_ref()), &body.to_string()).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
    ("async fn super_resolution(&self, img_url: &str) -> Result<WxImgProcSuperResolutionResult, WxErrorException>",
     '''        let body = serde_json::json!({"img_url": img_url});
        let response = svc.post(&img_proc::super_resolution(config.as_ref()), &body.to_string()).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
    ("async fn ai_crop(&self, img_url: &str, ratios: Option<&str>) -> Result<WxImgProcAiCropResult, WxErrorException>",
     '''        let mut body = serde_json::Map::new();
        body.insert("img_url".into(), serde_json::json!(img_url));
        if let Some(r) = ratios {
            body.insert("ratios".into(), serde_json::json!(r));
        }
        let response = svc.post(&img_proc::ai_crop(config.as_ref()), &serde_json::Value::Object(body).to_string()).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
]

# ================ reimburse_invoice ================
reimburse_imports = [
    "use crate::bean::invoice::reimburse::{InvoiceBatchRequest, InvoiceInfoRequest, InvoiceInfoResponse, UpdateInvoiceStatusRequest, UpdateStatusBatchRequest};",
    "use crate::enums::wx_mp_api_url::reimburse_invoice;",
]
reimburse_methods = [
    ("async fn get_invoice_info(&self, request: &InvoiceInfoRequest) -> Result<InvoiceInfoResponse, WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&reimburse_invoice::get_invoice_info(config.as_ref()), &body).await?;
        InvoiceInfoResponse::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_invoice_batch(&self, request: &InvoiceBatchRequest) -> Result<Vec<InvoiceInfoResponse>, WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&reimburse_invoice::get_invoice_batch(config.as_ref()), &body).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("invoice_list").ok_or_else(|| WxErrorException::from_code(-99, "invoice_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn update_invoice_status(&self, request: &UpdateInvoiceStatusRequest) -> Result<(), WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&reimburse_invoice::update_invoice_status(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn update_status_batch(&self, request: &UpdateStatusBatchRequest) -> Result<(), WxErrorException>",
     '''        let body = serde_json::to_string(request).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&reimburse_invoice::update_status_batch(config.as_ref()), &body).await?;
        Ok(())'''),
]

# ================ merchant_invoice ================
merchant_imports = [
    "use crate::bean::invoice::merchant::{ClearOutInvoiceRequest, InvoiceAuthDataRequest, InvoiceAuthDataResult, InvoiceAuthPageRequest, InvoiceAuthPageResult, InvoiceAuthPageSetting, InvoiceRejectRequest, InvoiceResult, MakeOutInvoiceRequest, MerchantContactInfo, MerchantInvoicePlatformInfo};",
    "use crate::enums::wx_mp_api_url::merchant_invoice;",
]
merchant_methods = [
    ("async fn get_auth_page_url(&self, params: &InvoiceAuthPageRequest) -> Result<InvoiceAuthPageResult, WxErrorException>",
     '''        let body = serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&merchant_invoice::get_auth_url(config.as_ref()), &body).await?;
        InvoiceAuthPageResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_auth_data(&self, params: &InvoiceAuthDataRequest) -> Result<InvoiceAuthDataResult, WxErrorException>",
     '''        let body = serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&merchant_invoice::get_auth_data(config.as_ref()), &body).await?;
        InvoiceAuthDataResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn reject_invoice(&self, params: &InvoiceRejectRequest) -> Result<(), WxErrorException>",
     '''        let body = serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&merchant_invoice::reject_insert(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn make_out_invoice(&self, params: &MakeOutInvoiceRequest) -> Result<(), WxErrorException>",
     '''        let body = serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&merchant_invoice::make_out_invoice(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn clear_out_invoice(&self, params: &ClearOutInvoiceRequest) -> Result<(), WxErrorException>",
     '''        let body = serde_json::to_string(params).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        svc.post(&merchant_invoice::clear_out_invoice(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn query_invoice_info(&self, fpqqlsh: &str, nsrsbh: &str) -> Result<InvoiceResult, WxErrorException>",
     '''        let body = serde_json::json!({"fpqqlsh": fpqqlsh, "nsrsbh": nsrsbh});
        let response = svc.post(&merchant_invoice::query_invoice_info(config.as_ref()), &body.to_string()).await?;
        InvoiceResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn set_merchant_contact_info(&self, contact: &MerchantContactInfo) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"contact": contact});
        svc.post(&merchant_invoice::set_contact(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_merchant_contact_info(&self) -> Result<MerchantContactInfo, WxErrorException>",
     '''        let response = svc.post(&merchant_invoice::get_contact(config.as_ref()), "{}").await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let contact = value.get("contact").ok_or_else(|| WxErrorException::from_code(-99, "contact 缺失"))?;
        serde_json::from_value(contact.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn set_auth_page_setting(&self, setting: &InvoiceAuthPageSetting) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"auth_field": setting});
        svc.post(&merchant_invoice::set_auth_page(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_auth_page_setting(&self) -> Result<InvoiceAuthPageSetting, WxErrorException>",
     '''        let response = svc.post(&merchant_invoice::get_auth_page(config.as_ref()), "{}").await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let field = value.get("auth_field").ok_or_else(|| WxErrorException::from_code(-99, "auth_field 缺失"))?;
        serde_json::from_value(field.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn set_merchant_invoice_platform(&self, info: &MerchantInvoicePlatformInfo) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"pay_mch": info});
        svc.post(&merchant_invoice::set_platform(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_merchant_invoice_platform(&self, info: &MerchantInvoicePlatformInfo) -> Result<MerchantInvoicePlatformInfo, WxErrorException>",
     '''        let body = serde_json::json!({"pay_mch": info});
        let response = svc.post(&merchant_invoice::get_platform(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let pay = value.get("pay_mch").ok_or_else(|| WxErrorException::from_code(-99, "pay_mch 缺失"))?;
        serde_json::from_value(pay.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
]

SERVICES = {
    "material": (material_methods, material_imports, material_helpers),
    "shake": (shake_methods, shake_imports, ""),
    "card": (card_methods, card_imports, ""),
    "member_card": (member_card_methods, member_card_imports, ""),
    "guide": (guide_methods, guide_imports, ""),
    "marketing": (marketing_methods, marketing_imports, ""),
    "subscribe_msg": (subscribe_msg_methods, subscribe_msg_imports, subscribe_msg_helpers),
    "ai_open": (ai_open_methods, ai_open_imports, ""),
    "ocr": (ocr_methods, ocr_imports, ocr_helpers),
    "img_proc": (img_proc_methods, img_proc_imports, ""),
    "reimburse_invoice": (reimburse_methods, reimburse_imports, ""),
    "merchant_invoice": (merchant_methods, merchant_imports, ""),
}

if __name__ == "__main__":
    for name, (methods, imports, helpers) in SERVICES.items():
        trait_name = gen_trait(name, methods, imports)
        gen_impl(name, trait_name, methods, imports, helpers)
    print("all services generated")
