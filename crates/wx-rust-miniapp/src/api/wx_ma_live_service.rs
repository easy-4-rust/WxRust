//! 直播房间管理服务。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.api.WxMaLiveService`
//! （`impl.WxMaLiveServiceImpl`）。

use std::collections::HashMap;

use async_trait::async_trait;

use wx_rust_common::error::WxErrorException;

use crate::bean::live::{
    Assistant, RoomInfo, WxMaCreateRoomResult, WxMaLiveAssistantInfo, WxMaLiveResult,
    WxMaLiveRoomInfo, WxMaLiveSharedCode,
};

/// 直播房间管理服务。
#[async_trait]
pub trait WxMaLiveService: Send + Sync {
    /// 创建直播间（对应 Java `createRoom`，errorCode=300036 时按 Java 语义
    /// 解析错误报文中的房间数据返回）。
    async fn create_room(
        &self,
        room_info: &WxMaLiveRoomInfo,
    ) -> Result<WxMaCreateRoomResult, WxErrorException>;

    /// 删除直播间（对应 Java `deleteRoom`）。
    async fn delete_room(&self, room_id: i32) -> Result<bool, WxErrorException>;

    /// 编辑直播间（对应 Java `editRoom`）。
    async fn edit_room(&self, room_info: &WxMaLiveRoomInfo) -> Result<bool, WxErrorException>;

    /// 获取直播间推流地址（对应 Java `getPushUrl`）。
    async fn get_push_url(&self, room_id: i32) -> Result<String, WxErrorException>;

    /// 获取直播间分享二维码（对应 Java `getSharedCode`）。
    async fn get_shared_code(
        &self,
        room_id: i32,
        params: Option<&str>,
    ) -> Result<WxMaLiveSharedCode, WxErrorException>;

    /// 获取直播房间列表（分页，对应 Java `getLiveInfo(Integer, Integer)`）。
    async fn get_live_info(
        &self,
        start: i32,
        limit: i32,
    ) -> Result<WxMaLiveResult, WxErrorException>;

    /// 获取所有直播间信息（没有分页直接获取全部，对应 Java `getLiveInfos`）。
    async fn get_live_infos(&self) -> Result<Vec<RoomInfo>, WxErrorException>;

    /// 获取直播房间回放数据信息（对应 Java `getLiveReplay(String, Integer,
    /// Integer, Integer)`）。
    async fn get_live_replay(
        &self,
        action: &str,
        room_id: i32,
        start: i32,
        limit: i32,
    ) -> Result<WxMaLiveResult, WxErrorException>;

    /// 获取直播房间回放数据信息（默认 `get_replay`，对应 Java
    /// `getLiveReplay(Integer, Integer, Integer)`）。
    async fn get_live_replay_default(
        &self,
        room_id: i32,
        start: i32,
        limit: i32,
    ) -> Result<WxMaLiveResult, WxErrorException>;

    /// 直播间导入商品（对应 Java `addGoodsToRoom`）。
    async fn add_goods_to_room(
        &self,
        room_id: i32,
        goods_ids: &[i32],
    ) -> Result<bool, WxErrorException>;

    /// 添加管理直播间小助手（对应 Java `addAssistant`）。
    async fn add_assistant(
        &self,
        room_id: i32,
        users: &[WxMaLiveAssistantInfo],
    ) -> Result<bool, WxErrorException>;

    /// 修改直播间小助手昵称（对应 Java `modifyAssistant`）。
    async fn modify_assistant(
        &self,
        room_id: i32,
        username: &str,
        nickname: &str,
    ) -> Result<bool, WxErrorException>;

    /// 删除直播间小助手（对应 Java `removeAssistant`）。
    async fn remove_assistant(
        &self,
        room_id: i32,
        username: &str,
    ) -> Result<bool, WxErrorException>;

    /// 查询直播间小助手（对应 Java `getAssistantList`）。
    async fn get_assistant_list(&self, room_id: i32) -> Result<Vec<Assistant>, WxErrorException>;

    /// 添加主播副号（对应 Java `addSubanchor`）。
    async fn add_subanchor(&self, room_id: i32, username: &str) -> Result<bool, WxErrorException>;

    /// 修改主播副号（对应 Java `modifySubanchor`）。
    async fn modify_subanchor(
        &self,
        room_id: i32,
        username: &str,
    ) -> Result<bool, WxErrorException>;

    /// 删除主播副号（对应 Java `deleteSubanchor`）。
    async fn delete_subanchor(&self, room_id: i32) -> Result<bool, WxErrorException>;

    /// 获取主播副号（对应 Java `getSubanchor`）。
    async fn get_subanchor(&self, room_id: i32) -> Result<String, WxErrorException>;

    /// 开启/关闭直播间官方收录（对应 Java `updatefeedpublic`）。
    async fn updatefeedpublic(
        &self,
        room_id: i32,
        is_feeds_public: i32,
    ) -> Result<bool, WxErrorException>;

    /// 开启/关闭回放功能（对应 Java `updatereplay`）。
    async fn updatereplay(&self, room_id: i32, close_replay: i32)
    -> Result<bool, WxErrorException>;

    /// 开启/关闭客服功能（对应 Java `updatekf`）。
    async fn updatekf(&self, room_id: i32, close_kf: i32) -> Result<bool, WxErrorException>;

    /// 开启/关闭直播间全局禁言（对应 Java `updatecomment`）。
    async fn updatecomment(&self, room_id: i32, ban_comment: i32)
    -> Result<bool, WxErrorException>;

    /// 上下架商品（对应 Java `onsale`）。
    async fn onsale(
        &self,
        room_id: i32,
        goods_id: i32,
        on_sale: i32,
    ) -> Result<bool, WxErrorException>;

    /// 删除直播间商品（对应 Java `deleteInRoom`）。
    async fn delete_in_room(&self, room_id: i32, goods_id: i32) -> Result<bool, WxErrorException>;

    /// 推送商品（对应 Java `push`）。
    async fn push(&self, room_id: i32, goods_id: i32) -> Result<bool, WxErrorException>;

    /// 直播间商品排序（对应 Java `sort`，goods 形如
    /// `[{"goodsId":"123"}, {"goodsId":"234"}]`）。
    async fn sort(
        &self,
        room_id: i32,
        goods: &[HashMap<String, String>],
    ) -> Result<bool, WxErrorException>;

    /// 下载商品讲解视频（对应 Java `getVideo`）。
    async fn get_video(&self, room_id: i32, goods_id: i32) -> Result<String, WxErrorException>;
}
