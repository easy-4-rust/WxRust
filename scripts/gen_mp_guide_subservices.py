#!/usr/bin/env python3
# -*- coding: utf-8 -*-
"""guide 4 子服务：Buyer/Tag/Material/MassedJob trait + impl。"""
import os

BASE = "/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust"
API = os.path.join(BASE, "crates", "wx-rust-mp", "src", "api")

TRAIT_HDR = '''//! {java}。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.{java}`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;
'''

def gen(name, java, methods, imports, helpers=""):
    trait_name = "WxMp" + "".join(p.capitalize() for p in name.split("_")) + "Service"
    lines = [TRAIT_HDR.format(java=java)]
    lines += imports
    lines += ["", f"/// {java.replace('WxMp', '公众号')}。", "#[async_trait]", f"pub trait {trait_name}: Send + Sync {{"]
    for sig, _ in methods:
        lines.append(f"    {sig};")
        lines.append("")
    lines.append("}")
    open(os.path.join(API, f"wx_mp_{name}_service.rs"), "w", encoding="utf-8").write("\n".join(lines) + "\n")

    impl = [f"//! {java} 实现。", "//!", f"//! 对应 Java `me.chanjar.weixin.mp.api.impl.{java}Impl`。", "",
            "use async_trait::async_trait;", "use std::sync::Weak;", "",
            "use wx_rust_common::error::WxErrorException;", "",
            f"use crate::api::{{WxMpService, {trait_name}}};"]
    impl += imports
    impl += ["", f"pub struct {java}Impl {{", "    service: Weak<dyn WxMpService>,", "}", "",
             f"impl {java}Impl {{", "    pub fn new(service: Weak<dyn WxMpService>) -> Self {", "        Self { service }", "    }"]
    if helpers:
        impl.append(helpers)
    impl += ["}", "", "#[async_trait]", f"impl {trait_name} for {java}Impl {{"]
    for sig, body in methods:
        impl.append(f"    {sig} {{")
        impl.append("        let svc = self")
        impl.append("            .service")
        impl.append("            .upgrade()")
        impl.append('            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;')
        impl.append("        let config = svc.wx_mp_config_storage();")
        impl.append(body)
        impl.append("    }")
        impl.append("")
    impl.append("}")
    open(os.path.join(API, "impl", f"wx_mp_{name}_service_impl.rs"), "w", encoding="utf-8").write("\n".join(impl) + "\n")

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
        src = src.rstrip() + f"\npub use wx_mp_{name}_service_impl::{java}Impl;\n"
        open(p, "w", encoding="utf-8").write(src)
    print("done:", trait_name)

GUIDE_BEANS = "use crate::bean::guide::{WxMpAddGuideBuyerInfo, WxMpGuideBuyerInfoList, WxMpGuideBuyerResp, WxMpGuideCardMaterialInfo, WxMpGuideImgMaterialInfoList, WxMpGuideMassed, WxMpGuideMassedInfo, WxMpGuideMaterialInfo, WxMpGuideTagInfo, WxMpGuideWordMaterialInfoList};\nuse crate::enums::wx_mp_api_url::guide;"

# ---- GuideBuyerService
gen("guide_buyer", "WxMpGuideBuyerService", [
    ("async fn add_guide_buyer_relation(&self, account: &str, openid: &str, infos: &[WxMpAddGuideBuyerInfo]) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "buyer_list": infos});
        let response = svc.post(&guide::add_guide_buyer_relation(config.as_ref()), &body.to_string()).await?;
        Self::parse_buyer_resp_list(&response)'''),
    ("async fn del_guide_buyer_relation(&self, account: &str, openid: &str, buyer_open_ids: &[String]) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "openid_list": buyer_open_ids});
        let response = svc.post(&guide::del_guide_buyer_relation(config.as_ref()), &body.to_string()).await?;
        Self::parse_buyer_resp_list(&response)'''),
    ("async fn get_guide_buyer_relation_list(&self, account: &str, openid: &str, page: i32, num: i32) -> Result<WxMpGuideBuyerInfoList, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "page": page, "num": num});
        let response = svc.post(&guide::get_guide_buyer_relation_list(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("buyer_list").ok_or_else(|| WxErrorException::from_code(-99, "buyer_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn rebind_guide_acct_for_buyer(&self, old_account: &str, old_openid: &str, account: &str, openid: &str, buyer_open_ids: &[String]) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException>",
     '''        let body = serde_json::json!({"old_guide_account": old_account, "old_guide_openid": old_openid, "guide_account": account, "guide_openid": openid, "openid_list": buyer_open_ids});
        let response = svc.post(&guide::rebind_guide_acct_for_buyer(config.as_ref()), &body.to_string()).await?;
        Self::parse_buyer_resp_list(&response)'''),
    ("async fn update_guide_buyer_relation(&self, account: &str, openid: &str, user_openid: &str, nickname: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "openid": user_openid, "nickname": nickname});
        svc.post(&guide::update_guide_buyer_relation(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
], [GUIDE_BEANS], '''
    /// 解析买家操作响应列表（对应 Java `GsonParser.parse(json).get("buyer_resp").getAsJsonArray()`）。
    fn parse_buyer_resp_list(response: &str) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException> {
        let value: serde_json::Value = serde_json::from_str(response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("buyer_resp").ok_or_else(|| WxErrorException::from_code(-99, "buyer_resp 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
''')

# ---- GuideTagService
gen("guide_tag", "WxMpGuideTagService", [
    ("async fn new_guide_tag_option(&self, tag_name: &str, values: &[String]) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"tag_name": tag_name, "tag_values": values});
        svc.post(&guide::new_guide_tag_option(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn del_guide_tag_option(&self, tag_name: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"tag_name": tag_name});
        svc.post(&guide::del_guide_tag_option(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn add_guide_tag_option(&self, tag_name: &str, values: &[String]) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"tag_name": tag_name, "tag_values": values});
        svc.post(&guide::add_guide_tag_option(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_guide_tag_option(&self) -> Result<Vec<WxMpGuideTagInfo>, WxErrorException>",
     '''        let response = svc.post(&guide::get_guide_tag_option(config.as_ref()), "{}").await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("tag_option").ok_or_else(|| WxErrorException::from_code(-99, "tag_option 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn add_guide_buyer_tag(&self, account: &str, openid: &str, value: &str, user_open_ids: &[String]) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "tag_value": value, "openid_list": user_open_ids});
        let response = svc.post(&guide::add_guide_buyer_tag(config.as_ref()), &body.to_string()).await?;
        Self::parse_buyer_resp_list(&response)'''),
    ("async fn get_guide_buyer_tag(&self, account: &str, openid: &str, user_openid: &str, is_exclude: bool) -> Result<Vec<String>, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "openid": user_openid, "is_exclude": is_exclude});
        let response = svc.post(&guide::get_guide_buyer_tag(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("tag_values").and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .ok_or_else(|| WxErrorException::from_code(-99, "tag_values 缺失"))'''),
    ("async fn query_guide_buyer_by_tag(&self, account: &str, openid: &str, push_count: i32, values: &[String]) -> Result<Vec<String>, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "push_count": push_count, "tag_values": values});
        let response = svc.post(&guide::query_guide_buyer_by_tag(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("openid_list").and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .ok_or_else(|| WxErrorException::from_code(-99, "openid_list 缺失"))'''),
], [GUIDE_BEANS], '''
    /// 解析买家操作响应列表（对应 Java `get("buyer_resp")`）。
    fn parse_buyer_resp_list(response: &str) -> Result<Vec<WxMpGuideBuyerResp>, WxErrorException> {
        let value: serde_json::Value = serde_json::from_str(response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("buyer_resp").ok_or_else(|| WxErrorException::from_code(-99, "buyer_resp 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))
    }
''')

# ---- GuideMaterialService
gen("guide_material", "WxMpGuideMaterialService", [
    ("async fn set_guide_card_material(&self, media_id: &str, r#type: i32, title: &str, path: &str, app_id: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "media_id": media_id, "title": title, "path": path, "appid": app_id});
        svc.post(&guide::set_guide_card_material(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_guide_card_material(&self, r#type: i32) -> Result<Vec<WxMpGuideCardMaterialInfo>, WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type});
        let response = svc.post(&guide::get_guide_card_material(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("card_material_list").ok_or_else(|| WxErrorException::from_code(-99, "card_material_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn del_guide_card_material(&self, r#type: i32, title: &str, path: &str, app_id: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "title": title, "path": path, "appid": app_id});
        svc.post(&guide::del_guide_card_material(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn set_guide_image_material(&self, media_id: &str, r#type: i32) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "media_id": media_id});
        svc.post(&guide::set_guide_image_material(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_guide_image_material(&self, r#type: i32, start: i32, num: i32) -> Result<WxMpGuideImgMaterialInfoList, WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "start": start, "num": num});
        let response = svc.post(&guide::get_guide_image_material(config.as_ref()), &body.to_string()).await?;
        WxMpGuideImgMaterialInfoList::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn del_guide_image_material(&self, r#type: i32, pic_url: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "pic_url": pic_url});
        svc.post(&guide::del_guide_image_material(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn set_guide_word_material(&self, r#type: i32, word: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "word": word});
        svc.post(&guide::set_guide_word_material(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn get_guide_word_material(&self, r#type: i32, start: i32, num: i32) -> Result<WxMpGuideWordMaterialInfoList, WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "start": start, "num": num});
        let response = svc.post(&guide::get_guide_word_material(config.as_ref()), &body.to_string()).await?;
        WxMpGuideWordMaterialInfoList::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn del_guide_word_material(&self, r#type: i32, word: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"type": r#type, "word": word});
        svc.post(&guide::del_guide_word_material(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
], [GUIDE_BEANS])

# ---- GuideMassedJobService
gen("guide_massed_job", "WxMpGuideMassedJobService", [
    ("async fn add_guide_massed_job(&self, account: &str, openid: &str, task_name: &str, task_remark: &str, push_time: i64, user_open_ids: &[String], material_infos: &[WxMpGuideMaterialInfo]) -> Result<WxMpGuideMassed, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "task_name": task_name, "task_remark": task_remark, "push_time": push_time, "openid_list": user_open_ids, "material_info_list": material_infos});
        let response = svc.post(&guide::add_guide_massed_job(config.as_ref()), &body.to_string()).await?;
        serde_json::from_str(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_guide_massed_job_list(&self, account: &str, openid: &str, task_status: &[i32], offset: i32, limit: i32) -> Result<Vec<WxMpGuideMassedInfo>, WxErrorException>",
     '''        let body = serde_json::json!({"guide_account": account, "guide_openid": openid, "task_status": task_status, "offset": offset, "limit": limit});
        let response = svc.post(&guide::get_guide_massed_job_list(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let list = value.get("task_list").ok_or_else(|| WxErrorException::from_code(-99, "task_list 缺失"))?;
        serde_json::from_value(list.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn get_guide_massed_job(&self, task_id: &str) -> Result<WxMpGuideMassedInfo, WxErrorException>",
     '''        let body = serde_json::json!({"task_id": task_id});
        let response = svc.post(&guide::get_guide_massed_job(config.as_ref()), &body.to_string()).await?;
        WxMpGuideMassedInfo::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn update_guide_massed_job(&self, task_id: &str, task_name: &str, task_remark: &str, push_time: i64, user_open_ids: &[String], material_infos: &[WxMpGuideMaterialInfo]) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"task_id": task_id, "task_name": task_name, "task_remark": task_remark, "push_time": push_time, "openid_list": user_open_ids, "material_info_list": material_infos});
        svc.post(&guide::update_guide_massed_job(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn cancel_guide_massed_job(&self, task_id: &str) -> Result<(), WxErrorException>",
     '''        let body = serde_json::json!({"task_id": task_id});
        svc.post(&guide::cancel_guide_massed_job(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
], [GUIDE_BEANS])
print("all guide sub-services generated")
