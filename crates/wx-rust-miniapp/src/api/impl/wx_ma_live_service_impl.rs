//! 直播房间管理服务实现。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.impl.WxMaLiveServiceImpl`：
//! URL/请求体字段/响应解析逐方法对齐；GET 请求的 query 串与 Java
//! `Joiner.on("&").withKeyValueSeparator("=").join(map)` 语义一致
//! （Java `HashMap` 迭代序，Rust 侧以 `serde_json::Map` 保持插入序）。

use std::collections::HashMap;
use std::sync::Weak;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::api::WxMaService;
use crate::api::g4_services::WxMaLiveService;
use crate::bean::live::{
    Assistant, RoomInfo, WxMaAssistantResult, WxMaCreateRoomResult, WxMaLiveAssistantInfo,
    WxMaLiveResult, WxMaLiveRoomInfo, WxMaLiveSharedCode,
};
use crate::enums::g4_urls::url_g4_ability::live as live_url;

/// 直播房间管理服务实现。
pub struct WxMaLiveServiceImpl {
    service: Weak<dyn WxMaService>,
}

impl WxMaLiveServiceImpl {
    /// 构建直播房间管理服务。
    pub fn new(service: Weak<dyn WxMaService>) -> Self {
        Self { service }
    }

    /// 将键值对拼接为 query 串（对应 Java
    /// `Joiner.on("&").withKeyValueSeparator("=").join(map)`）。
    fn join_query(params: &HashMap<String, String>) -> String {
        let mut parts: Vec<String> = params.iter().map(|(k, v)| format!("{k}={v}")).collect();
        parts.sort();
        parts.join("&")
    }
}

#[async_trait]
impl WxMaLiveService for WxMaLiveServiceImpl {
    /// 创建直播间（对应 Java `WxMaLiveServiceImpl.createRoom`）。
    ///
    /// errorCode=300036 时按 Java 语义解析错误报文中的房间数据返回
    /// （`WxErrorException(WxError.fromJson(...))` 的 `WxError.json` 承载
    /// 原始报文，Rust 侧经 `wx_error()` 取回）。
    async fn create_room(
        &self,
        room_info: &WxMaLiveRoomInfo,
    ) -> Result<WxMaCreateRoomResult, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::to_string(room_info).map_err(WxErrorException::from)?;
        let response_content = match svc
            .post(
                &live_url::room::create_room_url(config.as_ref()),
                &post_body,
            )
            .await
        {
            Ok(content) => content,
            Err(e) => {
                if e.error_code() == Some(300036) {
                    // Java: 错误 300036 时从错误报文解析房间数据返回
                    let json = e
                        .wx_error()
                        .and_then(|w| w.json.as_deref())
                        .unwrap_or_default();
                    return serde_json::from_str(json).map_err(WxErrorException::from);
                }
                return Err(e);
            }
        };
        serde_json::from_str(&response_content).map_err(WxErrorException::from)
    }

    /// 删除直播间（对应 Java `WxMaLiveServiceImpl.deleteRoom`）。
    async fn delete_room(&self, room_id: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "id": room_id }).to_string();
        svc.post(
            &live_url::room::delete_room_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 编辑直播间（对应 Java `WxMaLiveServiceImpl.editRoom`）。
    async fn edit_room(&self, room_info: &WxMaLiveRoomInfo) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::to_string(room_info).map_err(WxErrorException::from)?;
        svc.post(&live_url::room::edit_room_url(config.as_ref()), &post_body)
            .await?;
        Ok(true)
    }

    /// 获取直播间推流地址（对应 Java `WxMaLiveServiceImpl.getPushUrl`）。
    async fn get_push_url(&self, room_id: i32) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let query = Self::join_query(&HashMap::from([(
            "roomId".to_string(),
            room_id.to_string(),
        )]));
        let response_content = svc
            .get(&live_url::room::get_push_url_url(config.as_ref()), &query)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        json.get("pushAddr")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "pushAddr 字段缺失"))
    }

    /// 获取直播间分享二维码（对应 Java `WxMaLiveServiceImpl.getSharedCode`）。
    async fn get_shared_code(
        &self,
        room_id: i32,
        params: Option<&str>,
    ) -> Result<WxMaLiveSharedCode, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let mut map = HashMap::from([("roomId".to_string(), room_id.to_string())]);
        if let Some(params) = params {
            map.insert("params".to_string(), params.to_string());
        }
        let query = Self::join_query(&map);
        let response_content = svc
            .get(
                &live_url::room::get_shared_code_url(config.as_ref()),
                &query,
            )
            .await?;
        serde_json::from_str(&response_content).map_err(WxErrorException::from)
    }

    /// 获取直播房间列表（分页，对应 Java
    /// `WxMaLiveServiceImpl.getLiveInfo(Integer, Integer)`）。
    async fn get_live_info(
        &self,
        start: i32,
        limit: i32,
    ) -> Result<WxMaLiveResult, WxErrorException> {
        let json_object = self.get_live_info_inner(start, limit, None).await?;
        WxMaLiveResult::from_json(&json_object.to_string()).map_err(WxErrorException::Serde)
    }

    /// 获取所有直播间信息（对应 Java `WxMaLiveServiceImpl.getLiveInfos`）。
    ///
    /// 循环拉取直至 `results.size() > total`，每轮间隔 100ms
    /// （Java `Thread.sleep(100)`）。
    async fn get_live_infos(&self) -> Result<Vec<RoomInfo>, WxErrorException> {
        let mut results: Vec<RoomInfo> = Vec::new();
        let mut start: i32 = 0;
        let limit: i32 = 80;
        let mut total: i32 = 0;
        loop {
            if total != 0 && total <= start {
                break;
            }
            let live_info = self.get_live_info(start, limit).await?;
            results.extend(live_info.room_infos);
            total = live_info.total;
            start = results.len() as i32;
            // Java `do..while (results.size() <= total)`：total=0 且无房间时
            // 会无限循环，Rust 侧显式退出该边缘路径（ADAPTED）
            if total == 0 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            if results.len() as i32 > total {
                break;
            }
        }
        Ok(results)
    }

    /// 获取直播房间回放数据信息（对应 Java
    /// `WxMaLiveServiceImpl.getLiveReplay(String, Integer, Integer, Integer)`）。
    async fn get_live_replay(
        &self,
        action: &str,
        room_id: i32,
        start: i32,
        limit: i32,
    ) -> Result<WxMaLiveResult, WxErrorException> {
        let mut map = HashMap::new();
        map.insert("action".to_string(), action.to_string());
        map.insert("room_id".to_string(), room_id.to_string());
        let json_object = self.get_live_info_inner(start, limit, Some(&map)).await?;
        WxMaLiveResult::from_json(&json_object.to_string()).map_err(WxErrorException::Serde)
    }

    /// 获取直播房间回放数据信息（默认 `get_replay`，对应 Java
    /// `WxMaLiveServiceImpl.getLiveReplay(Integer, Integer, Integer)`）。
    async fn get_live_replay_default(
        &self,
        room_id: i32,
        start: i32,
        limit: i32,
    ) -> Result<WxMaLiveResult, WxErrorException> {
        self.get_live_replay("get_replay", room_id, start, limit)
            .await
    }

    /// 直播间导入商品（对应 Java `WxMaLiveServiceImpl.addGoodsToRoom`）。
    async fn add_goods_to_room(
        &self,
        room_id: i32,
        goods_ids: &[i32],
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "ids": goods_ids }).to_string();
        svc.post(&live_url::room::add_goods_url(config.as_ref()), &post_body)
            .await?;
        Ok(true)
    }

    /// 添加管理直播间小助手（对应 Java `WxMaLiveServiceImpl.addAssistant`）。
    async fn add_assistant(
        &self,
        room_id: i32,
        users: &[WxMaLiveAssistantInfo],
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "users": users }).to_string();
        svc.post(
            &live_url::room::add_assistant_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 修改直播间小助手昵称（对应 Java `WxMaLiveServiceImpl.modifyAssistant`）。
    async fn modify_assistant(
        &self,
        room_id: i32,
        username: &str,
        nickname: &str,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({
            "roomId": room_id,
            "username": username,
            "nickname": nickname,
        })
        .to_string();
        svc.post(
            &live_url::room::modify_assistant_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 删除直播间小助手（对应 Java `WxMaLiveServiceImpl.removeAssistant`）。
    async fn remove_assistant(
        &self,
        room_id: i32,
        username: &str,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "username": username }).to_string();
        svc.post(
            &live_url::room::remove_assistant_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 查询直播间小助手（对应 Java `WxMaLiveServiceImpl.getAssistantList`）。
    async fn get_assistant_list(&self, room_id: i32) -> Result<Vec<Assistant>, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let query = Self::join_query(&HashMap::from([(
            "roomId".to_string(),
            room_id.to_string(),
        )]));
        let response_content = svc
            .get(
                &live_url::room::get_assistant_list_url(config.as_ref()),
                &query,
            )
            .await?;
        let result =
            WxMaAssistantResult::from_json(&response_content).map_err(WxErrorException::Serde)?;
        Ok(result.list)
    }

    /// 添加主播副号（对应 Java `WxMaLiveServiceImpl.addSubanchor`）。
    async fn add_subanchor(&self, room_id: i32, username: &str) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "username": username }).to_string();
        svc.post(
            &live_url::room::add_subanchor_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 修改主播副号（对应 Java `WxMaLiveServiceImpl.modifySubanchor`）。
    async fn modify_subanchor(
        &self,
        room_id: i32,
        username: &str,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "username": username }).to_string();
        svc.post(
            &live_url::room::modify_subanchor_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 删除主播副号（对应 Java `WxMaLiveServiceImpl.deleteSubanchor`）。
    async fn delete_subanchor(&self, room_id: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id }).to_string();
        svc.post(
            &live_url::room::delete_subanchor_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 获取主播副号（对应 Java `WxMaLiveServiceImpl.getSubanchor`）。
    async fn get_subanchor(&self, room_id: i32) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let query = Self::join_query(&HashMap::from([(
            "roomId".to_string(),
            room_id.to_string(),
        )]));
        let response_content = svc
            .get(&live_url::room::get_subanchor_url(config.as_ref()), &query)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        json.get("username")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "username 字段缺失"))
    }

    /// 开启/关闭直播间官方收录（对应 Java `WxMaLiveServiceImpl.updatefeedpublic`）。
    async fn updatefeedpublic(
        &self,
        room_id: i32,
        is_feeds_public: i32,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body =
            serde_json::json!({ "roomId": room_id, "isFeedsPublic": is_feeds_public }).to_string();
        svc.post(
            &live_url::room::update_feed_public_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 开启/关闭回放功能（对应 Java `WxMaLiveServiceImpl.updatereplay`）。
    async fn updatereplay(
        &self,
        room_id: i32,
        close_replay: i32,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body =
            serde_json::json!({ "roomId": room_id, "closeReplay": close_replay }).to_string();
        svc.post(
            &live_url::room::update_replay_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 开启/关闭客服功能（对应 Java `WxMaLiveServiceImpl.updatekf`）。
    async fn updatekf(&self, room_id: i32, close_kf: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "closeKf": close_kf }).to_string();
        svc.post(&live_url::room::update_kf_url(config.as_ref()), &post_body)
            .await?;
        Ok(true)
    }

    /// 开启/关闭直播间全局禁言（对应 Java `WxMaLiveServiceImpl.updatecomment`）。
    async fn updatecomment(
        &self,
        room_id: i32,
        ban_comment: i32,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body =
            serde_json::json!({ "roomId": room_id, "banComment": ban_comment }).to_string();
        svc.post(
            &live_url::room::update_comment_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 上下架商品（对应 Java `WxMaLiveServiceImpl.onsale`）。
    async fn onsale(
        &self,
        room_id: i32,
        goods_id: i32,
        on_sale: i32,
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({
            "roomId": room_id,
            "goodsId": goods_id,
            "onSale": on_sale,
        })
        .to_string();
        svc.post(&live_url::room::onsale_url(config.as_ref()), &post_body)
            .await?;
        Ok(true)
    }

    /// 删除直播间商品（对应 Java `WxMaLiveServiceImpl.deleteInRoom`）。
    async fn delete_in_room(&self, room_id: i32, goods_id: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "goodsId": goods_id }).to_string();
        svc.post(
            &live_url::room::delete_in_room_url(config.as_ref()),
            &post_body,
        )
        .await?;
        Ok(true)
    }

    /// 推送商品（对应 Java `WxMaLiveServiceImpl.push`）。
    async fn push(&self, room_id: i32, goods_id: i32) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "goodsId": goods_id }).to_string();
        svc.post(&live_url::room::push_url(config.as_ref()), &post_body)
            .await?;
        Ok(true)
    }

    /// 直播间商品排序（对应 Java `WxMaLiveServiceImpl.sort`，goods 形如
    /// `[{"goodsId":"123"}, {"goodsId":"234"}]`）。
    async fn sort(
        &self,
        room_id: i32,
        goods: &[HashMap<String, String>],
    ) -> Result<bool, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "goods": goods }).to_string();
        svc.post(&live_url::room::sort_url(config.as_ref()), &post_body)
            .await?;
        Ok(true)
    }

    /// 下载商品讲解视频（对应 Java `WxMaLiveServiceImpl.getVideo`）。
    async fn get_video(&self, room_id: i32, goods_id: i32) -> Result<String, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let post_body = serde_json::json!({ "roomId": room_id, "goodsId": goods_id }).to_string();
        let response_content = svc
            .post(&live_url::room::get_video_url(config.as_ref()), &post_body)
            .await?;
        let json: serde_json::Value =
            serde_json::from_str(&response_content).map_err(WxErrorException::from)?;
        json.get("url")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| WxErrorException::from_code(-99, "url 字段缺失"))
    }
}

impl WxMaLiveServiceImpl {
    /// 私有实现：POST 获取直播信息（对应 Java
    /// `WxMaLiveServiceImpl.getLiveInfo(Integer, Integer, Map)`）。
    ///
    /// 请求体固定含 `start`/`limit`；map 非空时携带 `action`/`room_id`
    /// 等回放查询参数。
    async fn get_live_info_inner(
        &self,
        start: i32,
        limit: i32,
        map: Option<&HashMap<String, String>>,
    ) -> Result<serde_json::Value, WxErrorException> {
        let svc = self
            .service
            .upgrade()
            .ok_or_else(|| WxErrorException::from_code(-99, "小程序服务已释放"))?;
        let config = svc.wx_ma_config();
        let mut body = serde_json::Map::new();
        if let Some(map) = map {
            for (k, v) in map {
                body.insert(k.clone(), serde_json::Value::String(v.clone()));
            }
        }
        body.insert("start".to_string(), serde_json::json!(start));
        body.insert("limit".to_string(), serde_json::json!(limit));
        let response_content = svc
            .post(
                &live_url::room::get_live_info_url(config.as_ref()),
                &serde_json::Value::Object(body).to_string(),
            )
            .await?;
        serde_json::from_str(&response_content).map_err(WxErrorException::from)
    }
}
