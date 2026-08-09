#!/usr/bin/env python3
"""mp 子域服务批量生成器：trait + impl + URL 模块（严格镜像 Java 实现语义）。

生成内容：
- api/wx_mp_<name>_service.rs（trait，async_trait）
- api/impl/wx_mp_<name>_service_impl.rs（impl，Weak<dyn WxMpService> 注入）
- enums/wx_mp_api_url.rs 追加子模块
"""
import os

BASE = "/Users/wandl/workspaces/workspace-github-easy-4-rust/WxRust"
API = os.path.join(BASE, "crates", "wx-rust-mp", "src", "api")
ENUM = os.path.join(BASE, "crates", "wx-rust-mp", "src", "enums", "wx_mp_api_url.rs")

# ---------------------------------------------------------------- URL 模块
URLS = {
    "tags": [
        ("create", "/cgi-bin/tags/create"),
        ("get", "/cgi-bin/tags/get"),
        ("update", "/cgi-bin/tags/update"),
        ("delete", "/cgi-bin/tags/delete"),
        ("tag_user_get", "/cgi-bin/user/tag/get"),
        ("batch_tagging", "/cgi-bin/tags/members/batchtagging"),
        ("batch_untagging", "/cgi-bin/tags/members/batchuntagging"),
        ("get_id_list", "/cgi-bin/tags/getidlist"),
    ],
    "blacklist": [
        ("get_blacklist", "/cgi-bin/tags/members/getblacklist"),
        ("batch_blacklist", "/cgi-bin/tags/members/batchblacklist"),
        ("batch_unblacklist", "/cgi-bin/tags/members/batchunblacklist"),
    ],
    "store": [
        ("poi_add", "/cgi-bin/poi/addpoi"),
        ("poi_get", "/cgi-bin/poi/getpoi"),
        ("poi_del", "/cgi-bin/poi/delpoi"),
        ("poi_list", "/cgi-bin/poi/getpoilist"),
        ("poi_update", "/cgi-bin/poi/updatepoi"),
        ("wx_category", "/cgi-bin/poi/getwxcategory"),
    ],
    "comment": [
        ("open", "/cgi-bin/comment/open"),
        ("close", "/cgi-bin/comment/close"),
        ("list", "/cgi-bin/comment/list"),
        ("mark_elect", "/cgi-bin/comment/markelect"),
        ("unmark_elect", "/cgi-bin/comment/unmarkelect"),
        ("delete", "/cgi-bin/comment/delete"),
        ("reply_add", "/cgi-bin/comment/reply/add"),
        ("reply_delete", "/cgi-bin/comment/reply/delete"),
    ],
    "datacube": [
        ("get_user_summary", "/datacube/getusersummary"),
        ("get_user_cumulate", "/datacube/getusercumulate"),
        ("get_article_summary", "/datacube/getarticlesummary"),
        ("get_article_total", "/datacube/getarticletotal"),
    ],
    "wifi": [
        ("shop_list", "/bizwifi/shop/list"),
        ("shop_get", "/bizwifi/shop/get"),
        ("shop_update", "/bizwifi/shop/update"),
    ],
    "mass_message": [
        ("upload_news", "/cgi-bin/media/uploadnews"),
        ("upload_video", "/cgi-bin/media/uploadvideo"),
        ("send_all", "/cgi-bin/message/mass/sendall"),
        ("send", "/cgi-bin/message/mass/send"),
        ("preview", "/cgi-bin/message/mass/preview"),
        ("delete", "/cgi-bin/message/mass/delete"),
        ("speed_get", "/cgi-bin/message/mass/speed/get"),
        ("speed_set", "/cgi-bin/message/mass/speed/set"),
        ("get", "/cgi-bin/message/mass/get"),
    ],
    "draft": [
        ("add", "/cgi-bin/draft/add"),
        ("update", "/cgi-bin/draft/update"),
        ("get", "/cgi-bin/draft/get"),
        ("delete", "/cgi-bin/draft/delete"),
        ("list", "/cgi-bin/draft/batchget"),
        ("count", "/cgi-bin/draft/count"),
    ],
    "free_publish": [
        ("submit", "/cgi-bin/freepublish/submit"),
        ("get_article", "/cgi-bin/freepublish/getarticle"),
        ("get_push_status", "/cgi-bin/freepublish/get"),
        ("del_push", "/cgi-bin/freepublish/delete"),
        ("batch_get", "/cgi-bin/freepublish/batchget"),
    ],
    "device": [
        ("trans_msg", "/device/transmsg"),
        ("get_qrcode", "/device/getqrcode"),
        ("authorize", "/device/authorize_device"),
        ("bind", "/device/bind"),
        ("compel_bind", "/device/compel_bind"),
        ("unbind", "/device/unbind"),
        ("compel_unbind", "/device/compel_unbind"),
        ("get_openid", "/device/get_openid"),
        ("get_bind_device", "/device/get_bind_device"),
    ],
}


def gen_url_modules():
    lines = []
    for mod, items in URLS.items():
        lines.append(f"\n/// {mod} 接口地址（对应 Java `WxMpApiUrl.{mod}`）。")
        lines.append(f"pub mod {mod} {{")
        lines.append("    use super::*;")
        lines.append("")
        for name, path in items:
            lines.append(f"    /// {path}")
            lines.append(f'    pub fn {name}(config: &dyn WxMpConfigStorage) -> String {{')
            lines.append("        let h = config.host_config();")
            lines.append(f'        url(config, &h.api_host, "{path}")')
            lines.append("    }")
            lines.append("")
        lines.append("}")
    return "\n".join(lines)


# ---------------------------------------------------------------- 服务定义
# name: (trait 方法列表, impl 代码模板)
# 每个方法: (rust 签名行, 方法体)
SERVICES = {}


def svc(name, doc_java, methods):
    SERVICES[name] = (doc_java, methods)


# ---- userTag
svc("user_tag", "WxMpUserTagService", [
    ("async fn tag_create(&self, name: &str) -> Result<WxUserTag, WxErrorException>", '''        let body = serde_json::json!({"tag": {"name": name}});
        let response = svc.post(&tags::create(config.as_ref()), &body.to_string()).await?;
        WxUserTag::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn tag_get(&self) -> Result<Vec<WxUserTag>, WxErrorException>", '''        let response = svc.get(&tags::get(config.as_ref()), "").await?;
        WxUserTag::list_from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn tag_update(&self, tag_id: i64, name: &str) -> Result<bool, WxErrorException>", '''        let body = serde_json::json!({"tag": {"id": tag_id, "name": name}});
        svc.post(&tags::update(config.as_ref()), &body.to_string()).await?;
        // Java 语义：响应 errcode==0 返回 true（执行器已校验 errcode）
        Ok(true)'''),
    ("async fn tag_delete(&self, tag_id: i64) -> Result<bool, WxErrorException>", '''        let body = serde_json::json!({"tag": {"id": tag_id}});
        svc.post(&tags::delete(config.as_ref()), &body.to_string()).await?;
        Ok(true)'''),
    ("async fn tag_list_user(&self, tag_id: i64, next_openid: &str) -> Result<WxTagListUser, WxErrorException>", '''        let body = serde_json::json!({"tagid": tag_id, "next_openid": next_openid.trim()});
        let response = svc.post(&tags::tag_user_get(config.as_ref()), &body.to_string()).await?;
        WxTagListUser::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn user_tag_list(&self, openid: &str) -> Result<Vec<i64>, WxErrorException>", '''        let body = serde_json::json!({"openid": openid});
        let response = svc.post(&tags::get_id_list(config.as_ref()), &body.to_string()).await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("tagid_list").and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_i64()).collect())
            .ok_or_else(|| WxErrorException::from_code(-99, "tagid_list 字段缺失"))'''),
    ("async fn batch_tagging(&self, tag_id: i64, openids: &[&str]) -> Result<bool, WxErrorException>", '''        let body = serde_json::json!({"tagid": tag_id, "openid_list": openids});
        svc.post(&tags::batch_tagging(config.as_ref()), &body.to_string()).await?;
        Ok(true)'''),
    ("async fn batch_untagging(&self, tag_id: i64, openids: &[&str]) -> Result<bool, WxErrorException>", '''        let body = serde_json::json!({"tagid": tag_id, "openid_list": openids});
        svc.post(&tags::batch_untagging(config.as_ref()), &body.to_string()).await?;
        Ok(true)'''),
])

# ---- userBlacklist
svc("user_blacklist", "WxMpUserBlacklistService", [
    ("async fn get_blacklist(&self, next_openid: &str) -> Result<WxMpUserBlacklistGetResult, WxErrorException>", '''        let body = serde_json::json!({"begin_openid": next_openid});
        let response = svc.post(&blacklist::get_blacklist(config.as_ref()), &body.to_string()).await?;
        WxMpUserBlacklistGetResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn push_to_blacklist(&self, openid_list: &[String]) -> Result<(), WxErrorException>", '''        let body = serde_json::json!({"openid_list": openid_list});
        svc.post(&blacklist::batch_blacklist(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn pull_from_blacklist(&self, openid_list: &[String]) -> Result<(), WxErrorException>", '''        let body = serde_json::json!({"openid_list": openid_list});
        svc.post(&blacklist::batch_unblacklist(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
])

# ---- store
svc("store", "WxMpStoreService", [
    ("async fn add(&self, request: &WxMpStoreBaseInfo) -> Result<(), WxErrorException>", '''        let body = request.to_json();
        svc.post(&store::poi_add(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn get(&self, poi_id: &str) -> Result<WxMpStoreBaseInfo, WxErrorException>", '''        let body = serde_json::json!({"poi_id": poi_id});
        let response = svc.post(&store::poi_get(config.as_ref()), &body.to_string()).await?;
        // Java 语义：取 business.base_info 子对象解析
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let base = value.get("business").and_then(|b| b.get("base_info"))
            .ok_or_else(|| WxErrorException::from_code(-99, "business.base_info 缺失"))?;
        serde_json::from_value(base.clone()).map_err(|e| WxErrorException::Serde(e.to_string()))'''),
    ("async fn delete(&self, poi_id: &str) -> Result<(), WxErrorException>", '''        let body = serde_json::json!({"poi_id": poi_id});
        svc.post(&store::poi_del(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn list(&self, begin: i32, limit: i32) -> Result<WxMpStoreListResult, WxErrorException>", '''        let body = serde_json::json!({"begin": begin, "limit": limit});
        let response = svc.post(&store::poi_list(config.as_ref()), &body.to_string()).await?;
        WxMpStoreListResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn update(&self, request: &WxMpStoreBaseInfo) -> Result<(), WxErrorException>", '''        let body = request.to_json();
        svc.post(&store::poi_update(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn list_categories(&self) -> Result<Vec<String>, WxErrorException>", '''        let response = svc.get(&store::wx_category(config.as_ref()), "").await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("category_list").and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
            .ok_or_else(|| WxErrorException::from_code(-99, "category_list 缺失"))'''),
])

# ---- comment
svc("comment", "WxMpCommentService", [
    ("async fn open(&self, msg_data_id: &str, index: Option<i32>) -> Result<(), WxErrorException>", '''        let mut body = serde_json::Map::new();
        body.insert("msg_data_id".into(), serde_json::json!(msg_data_id));
        if let Some(i) = index { body.insert("index".into(), serde_json::json!(i)); }
        svc.post(&comment::open(config.as_ref()), &serde_json::Value::Object(body).to_string()).await?;
        Ok(())'''),
    ("async fn close(&self, msg_data_id: &str, index: Option<i32>) -> Result<(), WxErrorException>", '''        let mut body = serde_json::Map::new();
        body.insert("msg_data_id".into(), serde_json::json!(msg_data_id));
        if let Some(i) = index { body.insert("index".into(), serde_json::json!(i)); }
        svc.post(&comment::close(config.as_ref()), &serde_json::Value::Object(body).to_string()).await?;
        Ok(())'''),
    ("async fn list(&self, msg_data_id: &str, index: Option<i32>, begin: i32, count: i32, r#type: i32) -> Result<WxMpCommentListVo, WxErrorException>", '''        let mut body = serde_json::Map::new();
        body.insert("msg_data_id".into(), serde_json::json!(msg_data_id));
        body.insert("begin".into(), serde_json::json!(begin));
        body.insert("count".into(), serde_json::json!(count));
        body.insert("type".into(), serde_json::json!(r#type));
        if let Some(i) = index { body.insert("index".into(), serde_json::json!(i)); }
        let response = svc.post(&comment::list(config.as_ref()), &serde_json::Value::Object(body).to_string()).await?;
        WxMpCommentListVo::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn mark_elect(&self, msg_data_id: &str, index: Option<i32>, user_comment_id: i64) -> Result<(), WxErrorException>", '''        let body = Self::build_json(msg_data_id, index, user_comment_id);
        svc.post(&comment::mark_elect(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn unmark_elect(&self, msg_data_id: &str, index: Option<i32>, user_comment_id: i64) -> Result<(), WxErrorException>", '''        let body = Self::build_json(msg_data_id, index, user_comment_id);
        svc.post(&comment::unmark_elect(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn delete(&self, msg_data_id: &str, index: Option<i32>, user_comment_id: i64) -> Result<(), WxErrorException>", '''        let body = Self::build_json(msg_data_id, index, user_comment_id);
        svc.post(&comment::delete(config.as_ref()), &body).await?;
        Ok(())'''),
    ("async fn reply_add(&self, msg_data_id: &str, index: Option<i32>, user_comment_id: i64, content: &str) -> Result<(), WxErrorException>", '''        let mut body = Self::build_json(msg_data_id, index, user_comment_id);
        let mut v: serde_json::Value = serde_json::from_str(&body)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        v.as_object_mut().unwrap().insert("content".into(), serde_json::json!(content));
        svc.post(&comment::reply_add(config.as_ref()), &v.to_string()).await?;
        Ok(())'''),
    ("async fn reply_delete(&self, msg_data_id: &str, index: Option<i32>, user_comment_id: i64) -> Result<(), WxErrorException>", '''        let body = Self::build_json(msg_data_id, index, user_comment_id);
        svc.post(&comment::reply_delete(config.as_ref()), &body).await?;
        Ok(())'''),
])

# ---- datacube
svc("data_cube", "WxMpDataCubeService", [
    ("async fn get_user_summary(&self, begin_date: &str, end_date: &str) -> Result<Vec<WxDataCubeUserSummary>, WxErrorException>", '''        let response = Self::post_dates(svc, &datacube::get_user_summary(config.as_ref()), begin_date, end_date).await?;
        WxDataCubeUserSummary::from_json_list(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_user_cumulate(&self, begin_date: &str, end_date: &str) -> Result<Vec<WxDataCubeUserCumulate>, WxErrorException>", '''        let response = Self::post_dates(svc, &datacube::get_user_cumulate(config.as_ref()), begin_date, end_date).await?;
        WxDataCubeUserCumulate::from_json_list(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_article_summary(&self, begin_date: &str, end_date: &str) -> Result<Vec<WxDataCubeArticleResult>, WxErrorException>", '''        let response = Self::post_dates(svc, &datacube::get_article_summary(config.as_ref()), begin_date, end_date).await?;
        WxDataCubeArticleResult::from_json_list(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_article_total(&self, begin_date: &str, end_date: &str) -> Result<Vec<WxDataCubeArticleTotal>, WxErrorException>", '''        let response = Self::post_dates(svc, &datacube::get_article_total(config.as_ref()), begin_date, end_date).await?;
        WxDataCubeArticleTotal::from_json_list(&response).map_err(WxErrorException::Serde)'''),
])

# ---- wifi
svc("wifi", "WxMpWifiService", [
    ("async fn list_shop(&self, page_index: i32, page_size: i32) -> Result<WxMpWifiShopListResult, WxErrorException>", '''        let body = serde_json::json!({"pageindex": page_index, "pagesize": page_size});
        let response = svc.post(&wifi::shop_list(config.as_ref()), &body.to_string()).await?;
        WxMpWifiShopListResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_shop_wifi_info(&self, shop_id: i32) -> Result<WxMpWifiShopDataResult, WxErrorException>", '''        let body = serde_json::json!({"shop_id": shop_id});
        let response = svc.post(&wifi::shop_get(config.as_ref()), &body.to_string()).await?;
        WxMpWifiShopDataResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn update_shop_wifi_info(&self, shop_id: i32, old_ssid: &str, ssid: &str, password: Option<&str>) -> Result<bool, WxErrorException>", '''        let mut body = serde_json::Map::new();
        body.insert("shop_id".into(), serde_json::json!(shop_id));
        body.insert("old_ssid".into(), serde_json::json!(old_ssid));
        body.insert("ssid".into(), serde_json::json!(ssid));
        if let Some(p) = password { body.insert("password".into(), serde_json::json!(p)); }
        svc.post(&wifi::shop_update(config.as_ref()), &serde_json::Value::Object(body).to_string()).await?;
        Ok(true)'''),
])

# ---- draft
svc("draft", "WxMpDraftService", [
    ("async fn add_draft(&self, add_draft: &WxMpAddDraft) -> Result<String, WxErrorException>", '''        let body = serde_json::to_string(add_draft).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&draft::add(config.as_ref()), &body).await?;
        Self::extract_str(&response, "media_id")'''),
    ("async fn update_draft(&self, update_draft: &WxMpUpdateDraft) -> Result<bool, WxErrorException>", '''        let body = serde_json::to_string(update_draft).map_err(|e| WxErrorException::Serde(e.to_string()))?;
        let response = svc.post(&draft::update(config.as_ref()), &body).await?;
        Self::err_code_is_zero(&response)'''),
    ("async fn get_draft(&self, media_id: &str) -> Result<WxMpDraftInfo, WxErrorException>", '''        let body = serde_json::json!({"media_id": media_id});
        let response = svc.post(&draft::get(config.as_ref()), &body.to_string()).await?;
        WxMpDraftInfo::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn del_draft(&self, media_id: &str) -> Result<bool, WxErrorException>", '''        let body = serde_json::json!({"media_id": media_id});
        let response = svc.post(&draft::delete(config.as_ref()), &body.to_string()).await?;
        Self::err_code_is_zero(&response)'''),
    ("async fn list_draft(&self, offset: i32, count: i32) -> Result<WxMpDraftList, WxErrorException>", '''        let body = serde_json::json!({"offset": offset, "count": count, "no_content": 0});
        let response = svc.post(&draft::list(config.as_ref()), &body.to_string()).await?;
        WxMpDraftList::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn count_draft(&self) -> Result<i64, WxErrorException>", '''        let response = svc.get(&draft::count(config.as_ref()), "").await?;
        let value: serde_json::Value = serde_json::from_str(&response)
            .map_err(|e| WxErrorException::Serde(e.to_string()))?;
        value.get("total_count").and_then(|v| v.as_i64())
            .ok_or_else(|| WxErrorException::from_code(-99, "total_count 缺失"))'''),
])

# ---- free publish
svc("free_publish", "WxMpFreePublishService", [
    ("async fn submit(&self, media_id: &str) -> Result<String, WxErrorException>", '''        let body = serde_json::json!({"media_id": media_id});
        let response = svc.post(&free_publish::submit(config.as_ref()), &body.to_string()).await?;
        Self::extract_str(&response, "publish_id")'''),
    ("async fn get_push_status(&self, publish_id: &str) -> Result<WxMpFreePublishStatus, WxErrorException>", '''        let body = serde_json::json!({"publish_id": publish_id});
        let response = svc.post(&free_publish::get_push_status(config.as_ref()), &body.to_string()).await?;
        WxMpFreePublishStatus::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn delete_push(&self, article_id: &str, index: i32) -> Result<bool, WxErrorException>", '''        let body = serde_json::json!({"article_id": article_id, "index": index});
        let response = svc.post(&free_publish::del_push(config.as_ref()), &body.to_string()).await?;
        Self::err_code_is_zero(&response)'''),
    ("async fn get_article_from_id(&self, article_id: &str) -> Result<WxMpFreePublishInfo, WxErrorException>", '''        let body = serde_json::json!({"article_id": article_id});
        let response = svc.post(&free_publish::get_article(config.as_ref()), &body.to_string()).await?;
        WxMpFreePublishInfo::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_publication_records(&self, offset: i32, count: i32) -> Result<WxMpFreePublishList, WxErrorException>", '''        let body = serde_json::json!({"offset": offset, "count": count, "no_content": 0});
        let response = svc.post(&free_publish::batch_get(config.as_ref()), &body.to_string()).await?;
        WxMpFreePublishList::from_json(&response).map_err(WxErrorException::Serde)'''),
])

# ---- device
svc("device", "WxMpDeviceService", [
    ("async fn trans_msg(&self, msg: &WxDeviceMsg) -> Result<TransMsgResp, WxErrorException>", '''        let body = msg.to_json();
        let response = svc.post(&device::trans_msg(config.as_ref()), &body).await?;
        TransMsgResp::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_qr_code(&self, product_id: &str) -> Result<WxDeviceQrCodeResult, WxErrorException>", '''        let query = format!("product_id={product_id}");
        let response = svc.get(&device::get_qrcode(config.as_ref()), &query).await?;
        WxDeviceQrCodeResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn authorize(&self, authorize: &WxDeviceAuthorize) -> Result<WxDeviceAuthorizeResult, WxErrorException>", '''        let body = authorize.to_json();
        let response = svc.post(&device::authorize(config.as_ref()), &body).await?;
        WxDeviceAuthorizeResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn bind(&self, bind: &WxDeviceBind) -> Result<WxDeviceBindResult, WxErrorException>", '''        let body = bind.to_json();
        let response = svc.post(&device::bind(config.as_ref()), &body).await?;
        WxDeviceBindResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn compel_bind(&self, bind: &WxDeviceBind) -> Result<WxDeviceBindResult, WxErrorException>", '''        let body = bind.to_json();
        let response = svc.post(&device::compel_bind(config.as_ref()), &body).await?;
        WxDeviceBindResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn unbind(&self, bind: &WxDeviceBind) -> Result<WxDeviceBindResult, WxErrorException>", '''        let body = bind.to_json();
        let response = svc.post(&device::unbind(config.as_ref()), &body).await?;
        WxDeviceBindResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn compel_unbind(&self, bind: &WxDeviceBind) -> Result<WxDeviceBindResult, WxErrorException>", '''        let body = bind.to_json();
        let response = svc.post(&device::compel_unbind(config.as_ref()), &body).await?;
        WxDeviceBindResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_open_id(&self, device_type: &str, device_id: &str) -> Result<WxDeviceOpenIdResult, WxErrorException>", '''        let query = format!("device_type={device_type}&device_id={device_id}");
        let response = svc.get(&device::get_openid(config.as_ref()), &query).await?;
        WxDeviceOpenIdResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn get_bind_device(&self, open_id: &str) -> Result<WxDeviceBindDeviceResult, WxErrorException>", '''        let query = format!("openid={open_id}");
        let response = svc.get(&device::get_bind_device(config.as_ref()), &query).await?;
        WxDeviceBindDeviceResult::from_json(&response).map_err(WxErrorException::Serde)'''),
])

# ---- mass message（含 adapter 线格式内联构建）
svc("mass_message", "WxMpMassMessageService", [
    ("async fn mass_news_upload(&self, news: &WxMpMassNews) -> Result<WxMpMassUploadResult, WxErrorException>", '''        let body = Self::news_json(news);
        let response = svc.post(&mass_message::upload_news(config.as_ref()), &body).await?;
        WxMpMassUploadResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn mass_video_upload(&self, video: &WxMpMassVideo) -> Result<WxMpMassUploadResult, WxErrorException>", '''        let body = serde_json::json!({"media_id": video.media_id, "description": video.description, "title": video.title});
        let response = svc.post(&mass_message::upload_video(config.as_ref()), &body.to_string()).await?;
        WxMpMassUploadResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn mass_group_message_send(&self, message: &WxMpMassTagMessage) -> Result<WxMpMassSendResult, WxErrorException>", '''        let body = Self::tag_message_json(message);
        let response = svc.post(&mass_message::send_all(config.as_ref()), &body).await?;
        WxMpMassSendResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn mass_open_ids_message_send(&self, message: &WxMpMassOpenIdsMessage) -> Result<WxMpMassSendResult, WxErrorException>", '''        let body = Self::open_ids_message_json(message);
        let response = svc.post(&mass_message::send(config.as_ref()), &body).await?;
        WxMpMassSendResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn mass_message_preview(&self, preview: &WxMpMassPreviewMessage) -> Result<WxMpMassSendResult, WxErrorException>", '''        let body = Self::preview_message_json(preview);
        let response = svc.post(&mass_message::preview(config.as_ref()), &body).await?;
        WxMpMassSendResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn delete(&self, msg_id: i64, article_index: i32) -> Result<(), WxErrorException>", '''        let body = serde_json::json!({"msg_id": msg_id, "article_idx": article_index});
        svc.post(&mass_message::delete(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn message_mass_speed_get(&self) -> Result<WxMpMassSpeedGetResult, WxErrorException>", '''        let response = svc.post(&mass_message::speed_get(config.as_ref()), "{}").await?;
        WxMpMassSpeedGetResult::from_json(&response).map_err(WxErrorException::Serde)'''),
    ("async fn message_mass_speed_set(&self, speed: i32) -> Result<(), WxErrorException>", '''        let body = serde_json::json!({"speed": speed});
        svc.post(&mass_message::speed_set(config.as_ref()), &body.to_string()).await?;
        Ok(())'''),
    ("async fn message_mass_get(&self, msg_id: i64) -> Result<WxMpMassGetResult, WxErrorException>", '''        let body = serde_json::json!({"msg_id": msg_id});
        let response = svc.post(&mass_message::get(config.as_ref()), &body.to_string()).await?;
        WxMpMassGetResult::from_json(&response).map_err(WxErrorException::Serde)'''),
])


# ---------------------------------------------------------------- 生成
HEADER = '''//! {doc}
//!
//! 对应 Java `me.chanjar.weixin.mp.api.{java}`。

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMpService;
'''


def gen_trait(name, java, methods):
    lines = [HEADER.format(doc=java.replace("Service", "服务"), java=java)]
    trait_name = f"WxMp{name.capitalize()}Service".replace("WxMpData_cube", "WxMpDataCube")
    # 规范化：user_tag -> UserTag, data_cube -> DataCube, mass_message -> MassMessage
    parts = name.split("_")
    trait_name = "WxMp" + "".join(p.capitalize() for p in parts) + "Service"
    lines.append(f"/// {java.replace('Service', '服务')}。")
    lines.append("#[async_trait]")
    lines.append(f"pub trait {trait_name}: Send + Sync {{")
    for sig, _ in methods:
        lines.append(f"    {sig};")
        lines.append("")
    lines.append("}")
    return trait_name, "\n".join(lines)


IMPL_HEADER = '''//! {java} 实现。
//!
//! 对应 Java `me.chanjar.weixin.mp.api.impl.{java}Impl`。

use async_trait::async_trait;
use std::sync::Weak;

use wx_rust_common::error::WxErrorException;

use crate::api::{{WxMpService, {trait_name}}};
'''


def gen_impl(name, java, trait_name, methods, imports):
    impl = [IMPL_HEADER.format(java=java, trait_name=trait_name)]
    for line in imports:
        impl.append(line)
    impl.append("")
    impl.append(f"/// {java.replace('Service', '服务')}实现。")
    impl.append(f"pub struct {java}Impl {{")
    impl.append("    service: Weak<dyn WxMpService>,")
    impl.append("}")
    impl.append("")
    impl.append(f"impl {java}Impl {{")
    impl.append(f"    /// 构建 {java.replace('Service', '服务')}。")
    impl.append("    pub fn new(service: Weak<dyn WxMpService>) -> Self {")
    impl.append("        Self { service }")
    impl.append("    }")
    impl.append("}")
    impl.append("")
    impl.append("#[async_trait]")
    impl.append(f"impl {trait_name} for {java}Impl {{")
    for sig, body in methods:
        impl.append(f"    {sig} {{")
        impl.append("        let svc = self")
        impl.append("            .service")
        impl.append('            .upgrade()')
        impl.append('            .ok_or_else(|| WxErrorException::from_code(-99, "公众号服务已释放"))?;')
        impl.append("        let config = svc.wx_mp_config_storage();")
        impl.append(body)
        impl.append("    }")
        impl.append("")
    impl.append("}")
    return "\n".join(impl)


def gen_all():
    url_mods = gen_url_modules()
    with open(ENUM, "a", encoding="utf-8") as f:
        f.write(url_mods)

    for name, (java, methods) in SERVICES.items():
        trait_name, trait_src = gen_trait(name, java, methods)
        with open(os.path.join(API, f"wx_mp_{name}_service.rs"), "w", encoding="utf-8") as f:
            f.write(trait_src)
        print(f"trait {trait_name}")

    # 模块注册
    api_mod = os.path.join(API, "mod.rs")
    src = open(api_mod).read()
    for name, _ in SERVICES.items():
        if f"pub mod wx_mp_{name}_service;" not in src:
            src = src.replace("pub mod wx_mp_service;",
                              f"pub mod wx_mp_{name}_service;\npub mod wx_mp_service;", 1)
    open(api_mod, "w").write(src)
    print("api/mod.rs updated")


if __name__ == "__main__":
    gen_all()
