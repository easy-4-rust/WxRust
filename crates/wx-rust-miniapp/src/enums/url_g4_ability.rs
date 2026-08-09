//! 小程序能力类子服务接口地址（Wave 2 G4 能力服务组）。
//!
//! 对应 Java `cn.binarywang.wx.miniapp.constant.WxMaApiUrlConstants` 中
//! 能力类子域地址：Broadcast（Room/Goods/Role）、Cloud、Marketing、
//! Promotion、Intracity、Complaint、DeviceSubscribe、Face、Invoice、
//! QrcodeJump、Vod、XPay。
//!
//! 函数风格与 `url_core`/`url_business` 一致：config 参数 + api_host 前缀
//! 模式。经逐条核对 Java 常量：**全部**地址均使用
//! `https://api.weixin.qq.com`（含 Cloud 域全部 `/tcb/*` 路径与 XPay 域
//! `/xpay/*` 路径），因此统一按 `api_host` 前缀拼接。
//!
//! XPay 域地址中的 `%s` 为 Java `String.format(url, paySig, sig)` 占位符
//! （`pay_sig` 或 `pay_sig+signature`），由 `WxMaXPayServiceImpl` 签名后
//! 以 `replacen("%s", ...)` 按序替换（本模块保持原样输出）。

use crate::config::{DEFAULT_API_HOST_URL, WxMaConfig};

/// 生成完整接口地址：域名前缀 + 路径。
fn url(_config: &dyn WxMaConfig, host: &str, path: &str) -> String {
    format!("{host}{path}")
}

/// 直播域接口地址（对应 Java `WxMaApiUrlConstants.Broadcast`）。
pub mod live {
    use super::*;

    /// 直播间管理接口地址（对应 Java `Broadcast.Room`）。
    pub mod room {
        use super::*;

        /// 创建直播间（对应 Java `Room.CREATE_ROOM`）。
        pub fn create_room_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/create")
        }

        /// 获取直播间列表/回放（对应 Java `Room.GET_LIVE_INFO`）。
        pub fn get_live_info_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxa/business/getliveinfo")
        }

        /// 直播间导入商品（对应 Java `Room.ADD_GOODS`）。
        pub fn add_goods_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/addgoods")
        }

        /// 删除直播间（对应 Java `Room.DELETE_ROOM`）。
        pub fn delete_room_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/deleteroom")
        }

        /// 编辑直播间（对应 Java `Room.EDIT_ROOM`）。
        pub fn edit_room_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/editroom")
        }

        /// 获取直播间推流地址（对应 Java `Room.GET_PUSH_URL`）。
        pub fn get_push_url_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/getpushurl")
        }

        /// 获取直播间分享二维码（对应 Java `Room.GET_SHARED_CODE`）。
        pub fn get_shared_code_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/getsharedcode")
        }

        /// 添加管理直播间小助手（对应 Java `Room.ADD_ASSISTANT`）。
        pub fn add_assistant_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/addassistant")
        }

        /// 修改管理直播间小助手（对应 Java `Room.MODIFY_ASSISTANT`）。
        pub fn modify_assistant_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(
                config,
                &h.api_host,
                "/wxaapi/broadcast/room/modifyassistant",
            )
        }

        /// 删除管理直播间小助手（对应 Java `Room.REMOVE_ASSISTANT`）。
        pub fn remove_assistant_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(
                config,
                &h.api_host,
                "/wxaapi/broadcast/room/removeassistant",
            )
        }

        /// 查询管理直播间小助手（对应 Java `Room.GET_ASSISTANT_LIST`）。
        pub fn get_assistant_list_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(
                config,
                &h.api_host,
                "/wxaapi/broadcast/room/getassistantlist",
            )
        }

        /// 添加主播副号（对应 Java `Room.ADD_SUBANCHOR`）。
        pub fn add_subanchor_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/addsubanchor")
        }

        /// 修改主播副号（对应 Java `Room.MODIFY_SUBANCHOR`）。
        pub fn modify_subanchor_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(
                config,
                &h.api_host,
                "/wxaapi/broadcast/room/modifysubanchor",
            )
        }

        /// 删除主播副号（对应 Java `Room.DELETE_SUBANCHOR`）。
        pub fn delete_subanchor_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(
                config,
                &h.api_host,
                "/wxaapi/broadcast/room/deletesubanchor",
            )
        }

        /// 获取主播副号（对应 Java `Room.GET_SUBANCHOR`）。
        pub fn get_subanchor_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/getsubanchor")
        }

        /// 开启/关闭直播间官方收录（对应 Java `Room.UPDATE_FEED_PUBLIC`）。
        pub fn update_feed_public_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(
                config,
                &h.api_host,
                "/wxaapi/broadcast/room/updatefeedpublic",
            )
        }

        /// 开启/关闭回放功能（对应 Java `Room.UPDATE_REPLAY`）。
        pub fn update_replay_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/updatereplay")
        }

        /// 开启/关闭客服功能（对应 Java `Room.UPDATE_KF`）。
        pub fn update_kf_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/updatekf")
        }

        /// 开启/关闭直播间全局禁言（对应 Java `Room.UPDATE_COMMENT`）。
        pub fn update_comment_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/room/updatecomment")
        }

        /// 上下架商品（对应 Java `Room.ONSALE`）。
        pub fn onsale_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/onsale")
        }

        /// 删除直播间商品（对应 Java `Room.DELETE_IN_ROOM`）。
        pub fn delete_in_room_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/deleteInRoom")
        }

        /// 推送商品（对应 Java `Room.PUSH`）。
        pub fn push_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/push")
        }

        /// 直播间商品排序（对应 Java `Room.SORT`）。
        pub fn sort_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/sort")
        }

        /// 下载商品讲解视频（对应 Java `Room.GET_VIDEO`）。
        pub fn get_video_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/getVideo")
        }
    }

    /// 直播商品管理接口地址（对应 Java `Broadcast.Goods`）。
    pub mod goods {
        use super::*;

        /// 商品添加并提审（对应 Java `Goods.ADD_GOODS`）。
        pub fn add_goods_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/add")
        }

        /// 撤回审核（对应 Java `Goods.RESET_AUDIT_GOODS`）。
        pub fn reset_audit_goods_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/resetaudit")
        }

        /// 重新提交审核（对应 Java `Goods.AUDIT_GOODS`）。
        pub fn audit_goods_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/audit")
        }

        /// 删除商品（对应 Java `Goods.DELETE_GOODS`）。
        pub fn delete_goods_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/delete")
        }

        /// 更新商品（对应 Java `Goods.UPDATE_GOODS`）。
        pub fn update_goods_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/update")
        }

        /// 获取商品状态（对应 Java `Goods.GET_GOODS_WARE_HOUSE`）。
        pub fn get_goods_ware_house_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxa/business/getgoodswarehouse")
        }

        /// 获取已审核商品列表（对应 Java `Goods.GET_APPROVED_GOODS`）。
        pub fn get_approved_goods_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/getapproved")
        }

        /// 直播挂件设置全局 Key（对应 Java `Goods.SET_KEY`）。
        pub fn set_key_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/setkey")
        }

        /// 直播挂件获取全局 Key（对应 Java `Goods.GET_KEY`）。
        pub fn get_key_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/goods/getkey")
        }
    }

    /// 小程序直播成员管理接口地址（对应 Java `Broadcast.Role`）。
    pub mod role {
        use super::*;

        /// 设置成员角色（对应 Java `Role.ADD_ROLE`）。
        pub fn add_role_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/role/addrole")
        }

        /// 解除成员角色（对应 Java `Role.DELETE_ROLE`）。
        pub fn delete_role_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/role/deleterole")
        }

        /// 查询成员列表（对应 Java `Role.LIST_BY_ROLE`）。
        pub fn list_by_role_url(config: &dyn WxMaConfig) -> String {
            let h = config.host_config();
            url(config, &h.api_host, "/wxaapi/broadcast/role/getrolelist")
        }
    }
}

/// 云开发接口地址（对应 Java `WxMaApiUrlConstants.Cloud`）。
///
/// Java 全部使用 `https://api.weixin.qq.com/tcb/*`，未使用独立 cloud 域名，
/// 因此统一走 `api_host` 前缀。
pub mod cloud {
    use super::*;

    /// 触发云函数（对应 Java `Cloud.INVOKE_CLOUD_FUNCTION_URL`，`%s` 依次为
    /// 云环境 ID/云函数名）。
    pub fn invoke_cloud_function_url(config: &dyn WxMaConfig, env: &str, name: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/tcb/invokecloudfunction?env={env}&name={name}"),
        )
    }

    /// 获取集合信息（对应 Java `Cloud.DATABASE_COLLECTION_GET_URL`）。
    pub fn database_collection_get_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasecollectionget")
    }

    /// 删除集合（对应 Java `Cloud.DATABASE_COLLECTION_DELETE_URL`）。
    pub fn database_collection_delete_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasecollectiondelete")
    }

    /// 新增集合（对应 Java `Cloud.DATABASE_COLLECTION_ADD_URL`）。
    pub fn database_collection_add_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasecollectionadd")
    }

    /// 获取腾讯云 API 调用凭证（对应 Java `Cloud.GET_QCLOUD_TOKEN_URL`）。
    pub fn get_qcloud_token_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/getqcloudtoken")
    }

    /// 删除文件（对应 Java `Cloud.BATCH_DELETE_FILE_URL`）。
    pub fn batch_delete_file_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/batchdeletefile")
    }

    /// 获取文件下载链接（对应 Java `Cloud.BATCH_DOWNLOAD_FILE_URL`）。
    pub fn batch_download_file_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/batchdownloadfile")
    }

    /// 获取文件上传链接（对应 Java `Cloud.UPLOAD_FILE_URL`）。
    pub fn upload_file_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/uploadfile")
    }

    /// 数据库迁移状态查询（对应 Java `Cloud.DATABASE_MIGRATE_QUERY_INFO_URL`）。
    pub fn database_migrate_query_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasemigratequeryinfo")
    }

    /// 数据库导出（对应 Java `Cloud.DATABASE_MIGRATE_EXPORT_URL`）。
    pub fn database_migrate_export_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasemigrateexport")
    }

    /// 数据库导入（对应 Java `Cloud.DATABASE_MIGRATE_IMPORT_URL`）。
    pub fn database_migrate_import_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasemigrateimport")
    }

    /// 变更数据库索引（对应 Java `Cloud.UPDATE_INDEX_URL`）。
    pub fn update_index_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/updateindex")
    }

    /// 统计集合记录数（对应 Java `Cloud.DATABASE_COUNT_URL`）。
    pub fn database_count_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasecount")
    }

    /// 数据库聚合（对应 Java `Cloud.DATABASE_AGGREGATE_URL`）。
    pub fn database_aggregate_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databaseaggregate")
    }

    /// 数据库查询（对应 Java `Cloud.DATABASE_QUERY_URL`）。
    pub fn database_query_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasequery")
    }

    /// 数据库更新（对应 Java `Cloud.DATABASE_UPDATE_URL`）。
    pub fn database_update_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databaseupdate")
    }

    /// 数据库删除（对应 Java `Cloud.DATABASE_DELETE_URL`）。
    pub fn database_delete_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databasedelete")
    }

    /// 数据库插入（对应 Java `Cloud.DATABASE_ADD_URL`）。
    pub fn database_add_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/databaseadd")
    }

    /// 发送携带 URL Link 的短信（对应 Java `Cloud.SEND_SMS_V2_URL`）。
    pub fn send_sms_v2_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/tcb/sendsmsv2")
    }
}

/// 微信营销接口地址（对应 Java `WxMaMarketingServiceImpl` 内联常量）。
pub mod marketing {
    use super::*;

    /// 创建数据源（对应 Java `USER_ACTION_SETS_ADD`）。
    pub fn user_action_sets_add_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/marketing/user_action_sets/add?version=v1.0",
        )
    }

    /// 回传数据（对应 Java `USER_ACTIONS_ADD`）。
    pub fn user_actions_add_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/marketing/user_actions/add?version=v1.0",
        )
    }
}

/// 小程序推广员接口地址（对应 Java `WxMaApiUrlConstants.Promotion`）。
pub mod promotion {
    use super::*;

    /// 新增角色（对应 Java `Promotion.PROMOTION_ADD_ROLE`）。
    pub fn add_role_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/addrole")
    }

    /// 查询角色（对应 Java `Promotion.PROMOTION_GET_ROLE`）。
    pub fn get_role_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/getrole")
    }

    /// 修改角色（对应 Java `Promotion.PROMOTION_UPDATE_ROLE`）。
    pub fn update_role_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/updaterole")
    }

    /// 声明推广员身份（对应 Java `Promotion.PROMOTION_ADD_PROMOTER`）。
    pub fn add_promoter_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/addpromoter")
    }

    /// 查询推广员身份（对应 Java `Promotion.PROMOTION_GET_PROMOTER`）。
    pub fn get_promoter_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/getpromoter")
    }

    /// 修改推广员身份（对应 Java `Promotion.PROMOTION_UPDATE_PROMOTER`）。
    pub fn update_promoter_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/updatepromoter")
    }

    /// 获取推广员邀请素材（对应 Java `Promotion.PROMOTION_GET_INVITATION_MATERIAL`）。
    pub fn get_invitation_material_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/getinvitationmaterial")
    }

    /// 群发消息（对应 Java `Promotion.PROMOTION_SEND_MSG`）。
    pub fn send_msg_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/sendmsg")
    }

    /// 单发消息（对应 Java `Promotion.PROMOTION_SINGLE_SEND_MSG`）。
    pub fn single_send_msg_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/singlesendmsg")
    }

    /// 查询送达结果（对应 Java `Promotion.PROMOTION_GET_MSG`）。
    pub fn get_msg_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/getmsg")
    }

    /// 分析点击效果（对应 Java `Promotion.PROMOTION_GET_MSG_CLICK_DATA`）。
    pub fn get_msg_click_data_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/getmsgclickdata")
    }

    /// 生成推广素材（对应 Java `Promotion.PROMOTION_GET_SHARE_MATERIAL`）。
    pub fn get_share_material_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/getsharematerial")
    }

    /// 分析触达效果（对应 Java `Promotion.PROMOTION_GET_RELATION`）。
    pub fn get_relation_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/getrelation")
    }

    /// 查询推广订单（对应 Java `Promotion.PROMOTION_GET_ORDER`）。
    pub fn get_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/promoter/getorder")
    }
}

/// 同城配送接口地址（对应 Java `WxMaApiUrlConstants.Intracity`）。
pub mod intracity {
    use super::*;

    /// 申请开通门店权限（对应 Java `Intracity.APPLY_URL`）。
    pub fn apply_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/intracity/apply")
    }

    /// 创建门店（对应 Java `Intracity.CREATE_STORE_URL`）。
    pub fn create_store_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/intracity/createstore",
        )
    }

    /// 查询门店（对应 Java `Intracity.QUERY_STORE_URL`）。
    pub fn query_store_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/intracity/querystore")
    }

    /// 更新门店（对应 Java `Intracity.UPDATE_STORE_URL`）。
    pub fn update_store_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/intracity/updatestore",
        )
    }

    /// 门店运费充值（对应 Java `Intracity.STORE_CHARGE`）。
    pub fn store_charge_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/intracity/storecharge",
        )
    }

    /// 门店运费退款（对应 Java `Intracity.STORE_REFUND`）。
    pub fn store_refund_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/intracity/storerefund",
        )
    }

    /// 门店运费流水查询（对应 Java `Intracity.QUERY_FLOW`）。
    pub fn query_flow_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/intracity/queryflow")
    }

    /// 查询门店余额（对应 Java `Intracity.BALANCE_QUERY`）。
    pub fn balance_query_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/intracity/balancequery",
        )
    }

    /// 查询扣费主体（对应 Java `Intracity.GET_PAY_MODE`）。
    pub fn get_pay_mode_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/intracity/getpaymode")
    }

    /// 设置扣费主体（对应 Java `Intracity.SET_PAY_MODE`）。
    pub fn set_pay_mode_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/intracity/setpaymode")
    }

    /// 查询运费（对应 Java `Intracity.PRE_ADD_ORDER`）。
    pub fn pre_add_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/intracity/preaddorder",
        )
    }

    /// 创建配送单（对应 Java `Intracity.ADD_ORDER`）。
    pub fn add_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/intracity/addorder")
    }

    /// 查询配送单（对应 Java `Intracity.QUERY_ORDER`）。
    pub fn query_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/intracity/queryorder")
    }

    /// 取消配送单（对应 Java `Intracity.CANCEL_ORDER`）。
    pub fn cancel_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/express/intracity/cancelorder",
        )
    }

    /// 查询支持同城配送的城市（对应 Java `Intracity.GET_CITY`）。
    pub fn get_city_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/express/intracity/getcity")
    }
}

/// 小程序交易投诉接口地址（对应 Java `WxMaApiUrlConstants.Complaint`）。
pub mod complaint {
    use super::*;

    /// 查询投诉单列表（对应 Java `Complaint.QUERY_COMPLAINTS_URL`）。
    pub fn query_complaints_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/miniapp/complaint/list")
    }

    /// 查询投诉单详情（对应 Java `Complaint.GET_COMPLAINT_URL`）。
    pub fn get_complaint_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/miniapp/complaint/detail")
    }

    /// 查询投诉协商历史（对应 Java `Complaint.QUERY_NEGOTIATION_HISTORY_URL`）。
    pub fn query_negotiation_history_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/miniapp/complaint/negotiation/history",
        )
    }

    /// 创建投诉通知回调地址（对应 Java `Complaint.ADD_COMPLAINT_NOTIFY_URL`）。
    pub fn add_complaint_notify_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/miniapp/complaint/notify/add")
    }

    /// 查询投诉通知回调地址（对应 Java `Complaint.GET_COMPLAINT_NOTIFY_URL`）。
    pub fn get_complaint_notify_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/miniapp/complaint/notify/get")
    }

    /// 更新投诉通知回调地址（对应 Java `Complaint.UPDATE_COMPLAINT_NOTIFY_URL`）。
    pub fn update_complaint_notify_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/miniapp/complaint/notify/update",
        )
    }

    /// 删除投诉通知回调地址（对应 Java `Complaint.DELETE_COMPLAINT_NOTIFY_URL`）。
    pub fn delete_complaint_notify_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/miniapp/complaint/notify/delete",
        )
    }

    /// 提交回复（对应 Java `Complaint.SUBMIT_RESPONSE_URL`）。
    pub fn submit_response_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/miniapp/complaint/response")
    }

    /// 反馈处理完成（对应 Java `Complaint.COMPLETE_COMPLAINT_URL`）。
    pub fn complete_complaint_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/miniapp/complaint/complete")
    }

    /// 上传反馈图片（对应 Java `Complaint.UPLOAD_RESPONSE_IMAGE_URL`）。
    pub fn upload_response_image_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/cgi-bin/miniapp/complaint/upload")
    }
}

/// 设备订阅消息接口地址（对应 Java `WxMaApiUrlConstants.DeviceSubscribe`）。
pub mod device_subscribe {
    use super::*;

    /// 获取设备票据（对应 Java `DeviceSubscribe.GET_SN_TICKET_URL`）。
    pub fn get_sn_ticket_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/getsnticket")
    }

    /// 发送设备订阅消息（对应 Java `DeviceSubscribe.SEND_DEVICE_SUBSCRIBE_MSG_URL`）。
    pub fn send_device_subscribe_msg_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cgi-bin/message/device/subscribe/send",
        )
    }

    /// 创建设备组（对应 Java `DeviceSubscribe.CREATE_IOT_GROUP_ID_URL`）。
    pub fn create_iot_group_id_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/business/group/createid")
    }

    /// 设备组添加设备（对应 Java `DeviceSubscribe.ADD_IOT_GROUP_DEVICE_URL`）。
    pub fn add_iot_group_device_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/business/group/adddevice")
    }

    /// 设备组删除设备（对应 Java `DeviceSubscribe.REMOVE_IOT_GROUP_DEVICE_URL`）。
    pub fn remove_iot_group_device_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/business/group/removedevice")
    }

    /// 查询设备组信息（对应 Java `DeviceSubscribe.GET_IOT_GROUP_INFO_URL`）。
    pub fn get_iot_group_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/business/group/getinfo")
    }
}

/// 微信人脸核身接口地址（对应 Java `WxMaApiUrlConstants.Face`）。
pub mod face {
    use super::*;

    /// 获取用户人脸核身会话唯一标识（对应 Java `Face.GET_VERIFY_ID_URL`）。
    pub fn get_verify_id_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cityservice/face/identify/getverifyid",
        )
    }

    /// 查询用户人脸核身真实验证结果（对应 Java `Face.QUERY_VERIFY_INFO_URL`）。
    pub fn query_verify_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/cityservice/face/identify/queryverifyinfo",
        )
    }
}

/// 电子发票报销方接口地址（对应 Java `WxMaApiUrlConstants.Invoice`）。
pub mod invoice {
    use super::*;

    /// 报销方查询报销发票信息（对应 Java `Invoice.GET_INVOICE_INFO`）。
    pub fn get_invoice_info_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/reimburse/getinvoiceinfo",
        )
    }

    /// 报销方批量查询报销发票信息（对应 Java `Invoice.GET_INVOICE_BATCH`）。
    pub fn get_invoice_batch_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/reimburse/getinvoicebatch",
        )
    }

    /// 报销方更新发票状态（对应 Java `Invoice.UPDATE_INVOICE_STATUS`）。
    pub fn update_invoice_status_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/reimburse/updateinvoicestatus",
        )
    }

    /// 报销方批量更新发票状态（对应 Java `Invoice.UPDATE_STATUS_BATCH`）。
    pub fn update_status_batch_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/card/invoice/reimburse/updatestatusbatch",
        )
    }
}

/// URL Link 二维码快速跳转规则管理接口地址（对应 Java
/// `WxMaApiUrlConstants.QrcodeJump`）。
pub mod qrcode_jump {
    use super::*;

    /// 添加二维码快速跳转规则（对应 Java `QrcodeJump.QRCODE_JUMP_ADD`）。
    pub fn add_rule_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/wxaqrcodefast/addcategoryrule")
    }

    /// 获取二维码快速跳转规则（对应 Java `QrcodeJump.QRCODE_JUMP_GET`）。
    pub fn get_rules_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxaapi/wxaqrcodefast/getcategory")
    }

    /// 分页获取二维码快速跳转规则列表（对应 Java `QrcodeJump.QRCODE_JUMP_GET_LIST`）。
    pub fn get_rule_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/wxaapi/wxaqrcodefast/getcategorybypage",
        )
    }

    /// 删除二维码快速跳转规则（对应 Java `QrcodeJump.QRCODE_JUMP_DELETE`）。
    pub fn delete_rule_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/wxaapi/wxaqrcodefast/deletecategoryrule",
        )
    }
}

/// 小程序短剧管理（视频点播）接口地址（对应 Java `WxMaApiUrlConstants.Vod`）。
pub mod vod {
    use super::*;

    /// 获取媒体列表（对应 Java `Vod.LIST_MEDIA_URL`）。
    pub fn list_media_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/listmedia")
    }

    /// 获取媒体详情（对应 Java `Vod.GET_MEDIA_URL`）。
    pub fn get_media_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/getmedia")
    }

    /// 获取媒体播放链接（对应 Java `Vod.GET_MEDIA_LINK_URL`）。
    pub fn get_media_link_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/getmedialink")
    }

    /// 删除媒体文件（对应 Java `Vod.DELETE_MEDIA_URL`）。
    pub fn delete_media_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/deletemedia")
    }

    /// 审核剧集（对应 Java `Vod.AUDIT_DRAMA_URL`）。
    pub fn audit_drama_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/auditdrama")
    }

    /// 获取剧集列表（对应 Java `Vod.LIST_DRAMAS_URL`）。
    pub fn list_dramas_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/listdramas")
    }

    /// 获取剧集详情（对应 Java `Vod.GET_DRAMA_URL`）。
    pub fn get_drama_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/getdrama")
    }

    /// 单文件上传（对应 Java `Vod.SINGLE_FILE_UPLOAD_URL`）。
    pub fn single_file_upload_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/singlefileupload")
    }

    /// 拉取上传（对应 Java `Vod.PULL_UPLOAD_URL`）。
    pub fn pull_upload_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/pullupload")
    }

    /// 获取任务状态（对应 Java `Vod.GET_TASK_URL`）。
    pub fn get_task_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/gettask")
    }

    /// 申请上传（对应 Java `Vod.APPLY_UPLOAD_URL`）。
    pub fn apply_upload_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/applyupload")
    }

    /// 上传分片（对应 Java `Vod.UPLOAD_PART_URL`）。
    pub fn upload_part_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/uploadpart")
    }

    /// 确认上传（对应 Java `Vod.COMMIT_UPLOAD_URL`）。
    pub fn commit_upload_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/commitupload")
    }

    /// 获取 CDN 用量数据（对应 Java `Vod.GET_CDN_USAGE_DATA_URL`）。
    pub fn get_cdn_usage_data_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/getcdnusagedata")
    }

    /// 获取 CDN 日志（对应 Java `Vod.GET_CDN_LOGS_URL`）。
    pub fn get_cdn_logs_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/wxa/sec/vod/getcdnlogs")
    }
}

/// 小程序虚拟支付接口地址（对应 Java `WxMaApiUrlConstants.XPay`）。
///
/// `%s` 为 Java `String.format` 占位符：`pay_sig=%s` 单签名，或
/// `pay_sig=%s&signature=%s` 双签名，由 `WxMaXPayServiceImpl` 签名后按序替换。
pub mod xpay {
    use super::*;

    /// 查询用户虚拟币余额（对应 Java `XPay.QUERY_USER_BALANCE_URL`，双签名）。
    pub fn query_user_balance_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/xpay/query_user_balance?pay_sig=%s&signature=%s",
        )
    }

    /// 虚拟币充值下单（对应 Java `XPay.CURRENCY_PAY_URL`，双签名）。
    pub fn currency_pay_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/xpay/currency_pay?pay_sig=%s&signature=%s",
        )
    }

    /// 查询订单信息（对应 Java `XPay.QUERY_ORDER_URL`）。
    pub fn query_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/query_order?pay_sig=%s")
    }

    /// 取消虚拟币充值订单（对应 Java `XPay.CANCEL_CURRENCY_PAY_URL`，双签名）。
    pub fn cancel_currency_pay_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/xpay/cancel_currency_pay?pay_sig=%s&signature=%s",
        )
    }

    /// 通知发货（对应 Java `XPay.NOTIFY_PROVIDE_GOODS_URL`）。
    pub fn notify_provide_goods_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/notify_provide_goods?pay_sig=%s")
    }

    /// 赠送虚拟币（对应 Java `XPay.PRESENT_CURRENCY_URL`）。
    pub fn present_currency_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/present_currency?pay_sig=%s")
    }

    /// 道具直购（对应 Java `XPay.PRESENT_GOODS_URL`）。
    pub fn present_goods_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/present_goods?pay_sig=%s")
    }

    /// 下载对账单（对应 Java `XPay.DOWNLOAD_BILL_URL`）。
    pub fn download_bill_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/download_bill?pay_sig=%s")
    }

    /// 退款申请（对应 Java `XPay.REFUND_ORDER_URL`）。
    pub fn refund_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/refund_order?pay_sig=%s")
    }

    /// 创建提现订单（对应 Java `XPay.CREATE_WITHDRAW_ORDER_URL`）。
    pub fn create_withdraw_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/xpay/create_withdraw_order?pay_sig=%s",
        )
    }

    /// 查询提现订单（对应 Java `XPay.QUERY_WITHDRAW_ORDER_URL`）。
    pub fn query_withdraw_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/query_withdraw_order?pay_sig=%s")
    }

    /// 启动道具上传（对应 Java `XPay.START_UPLOAD_GOODS_URL`）。
    pub fn start_upload_goods_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/start_upload_goods?pay_sig=%s")
    }

    /// 查询道具上传状态（对应 Java `XPay.QUERY_UPLOAD_GOODS_URL`）。
    pub fn query_upload_goods_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/query_upload_goods?pay_sig=%s")
    }

    /// 启动道具发布（对应 Java `XPay.START_PUBLISH_GOODS_URL`）。
    pub fn start_publish_goods_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/start_publish_goods?pay_sig=%s")
    }

    /// 查询道具发布状态（对应 Java `XPay.QUERY_PUBLISH_GOODS_URL`）。
    pub fn query_publish_goods_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/query_publish_goods?pay_sig=%s")
    }

    /// 查询商家账户可提现余额（对应 Java `XPay.QUERY_BIZ_BALANCE_URL`）。
    pub fn query_biz_balance_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/query_biz_balance?pay_sig=%s")
    }

    /// 查询广告金充值账户（对应 Java `XPay.QUERY_TRANSFER_ACCOUNT_URL`）。
    pub fn query_transfer_account_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/xpay/query_transfer_account?pay_sig=%s",
        )
    }

    /// 查询广告金发放记录（对应 Java `XPay.QUERY_ADVER_FUNDS_URL`）。
    pub fn query_adver_funds_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/query_adver_funds?pay_sig=%s")
    }

    /// 充值广告金（对应 Java `XPay.CREATE_FUNDS_BILL_URL`）。
    pub fn create_funds_bill_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/create_funds_bill?pay_sig=%s")
    }

    /// 绑定广告金充值账户（对应 Java `XPay.BIND_TRANSFER_ACCOUNT_URL`）。
    pub fn bind_transfer_account_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/bind_transfer_accout?pay_sig=%s")
    }

    /// 查询广告金充值记录（对应 Java `XPay.QUERY_FUNDS_BILL_URL`）。
    pub fn query_funds_bill_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/query_funds_bill?pay_sig=%s")
    }

    /// 查询广告金回收记录（对应 Java `XPay.QUERY_RECOVER_BILL_URL`）。
    pub fn query_recover_bill_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/query_recover_bill?pay_sig=%s")
    }

    /// 获取投诉列表（对应 Java `XPay.GET_COMPLAINT_LIST_URL`）。
    pub fn get_complaint_list_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/get_complaint_list?pay_sig=%s")
    }

    /// 获取投诉详情（对应 Java `XPay.GET_COMPLAINT_DETAIL_URL`）。
    pub fn get_complaint_detail_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/get_complaint_detail?pay_sig=%s")
    }

    /// 获取协商历史（对应 Java `XPay.GET_NEGOTIATION_HISTORY_URL`）。
    pub fn get_negotiation_history_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/xpay/get_negotiation_history?pay_sig=%s",
        )
    }

    /// 回复用户（对应 Java `XPay.RESPONSE_COMPLAINT_URL`）。
    pub fn response_complaint_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/response_complaint?pay_sig=%s")
    }

    /// 完成投诉处理（对应 Java `XPay.COMPLETE_COMPLAINT_URL`）。
    pub fn complete_complaint_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/complete_complaint?pay_sig=%s")
    }

    /// 上传媒体文件（对应 Java `XPay.UPLOAD_VP_FILE_URL`）。
    pub fn upload_vp_file_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/upload_vp_file?pay_sig=%s")
    }

    /// 获取微信支付反馈投诉图片的签名头部（对应 Java `XPay.GET_UPLOAD_FILE_SIGN_URL`）。
    pub fn get_upload_file_sign_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(config, &h.api_host, "/xpay/get_upload_file_sign?pay_sig=%s")
    }

    /// 下载广告金对应的商户订单信息（对应 Java `XPay.DOWNLOAD_ADVERFUNDS_ORDER_URL`）。
    pub fn download_adverfunds_order_url(config: &dyn WxMaConfig) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            "/xpay/download_adverfunds_order?pay_sig=%s",
        )
    }
}

/// OCR 识别接口地址（对应 Java `WxMaApiUrlConstants.Ocr`）。
///
/// 六个 `%s` 占位均为 **URLEncoder 编码后的** imgUrl：Java 服务内先
/// `URLEncoder.encode(imgUrl, UTF_8)` 再 `String.format` 填入；Rust 侧由
/// `WxMaOcrServiceImpl` 以 `url::form_urlencoded::byte_serialize`（与 Java
/// `URLEncoder` 同语义：空格转 `+`、`~` 转 `%7E`、其余 `%XX` 大写）编码后传入，
/// 本模块只做格式化（对应 Java 常量 + `String.format`）。
pub mod ocr {
    use super::*;

    /// 身份证识别（对应 Java `Ocr.IDCARD`）。
    pub fn id_card_url(config: &dyn WxMaConfig, img_url: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/ocr/idcard?img_url={img_url}"),
        )
    }

    /// 银行卡识别（对应 Java `Ocr.BANK_CARD`）。
    pub fn bank_card_url(config: &dyn WxMaConfig, img_url: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/ocr/bankcard?img_url={img_url}"),
        )
    }

    /// 行驶证识别（对应 Java `Ocr.DRIVING`）。
    pub fn driving_url(config: &dyn WxMaConfig, img_url: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/ocr/driving?img_url={img_url}"),
        )
    }

    /// 驾驶证识别（对应 Java `Ocr.DRIVING_LICENSE`）。
    pub fn driving_license_url(config: &dyn WxMaConfig, img_url: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/ocr/drivinglicense?img_url={img_url}"),
        )
    }

    /// 营业执照识别（对应 Java `Ocr.BIZ_LICENSE`）。
    pub fn biz_license_url(config: &dyn WxMaConfig, img_url: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/ocr/bizlicense?img_url={img_url}"),
        )
    }

    /// 通用印刷体识别（对应 Java `Ocr.COMM`）。
    pub fn comm_url(config: &dyn WxMaConfig, img_url: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/ocr/comm?img_url={img_url}"),
        )
    }
}

/// 图像处理接口地址（对应 Java `WxMaApiUrlConstants.ImgProc`）。
///
/// `img_url` 为 **URLEncoder 编码后的** 值（同 `ocr` 模块说明）；`ratios`
/// 保持 Java 原样不编码（Java `String.format(AI_CROP, imgUrl, ratios)` 直接
/// 代入，未过 `URLEncoder`）。
pub mod img_proc {
    use super::*;

    /// 二维码/条码识别（对应 Java `ImgProc.QRCODE`）。
    pub fn qrcode_url(config: &dyn WxMaConfig, img_url: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/img/qrcode?img_url={img_url}"),
        )
    }

    /// 图片高清化（对应 Java `ImgProc.SUPER_RESOLUTION`）。
    pub fn super_resolution_url(config: &dyn WxMaConfig, img_url: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/img/superresolution?img_url={img_url}"),
        )
    }

    /// 图片智能裁剪（对应 Java `ImgProc.AI_CROP`，`%s` 依次为编码后的
    /// imgUrl 与未编码的 ratios）。
    pub fn ai_crop_url(config: &dyn WxMaConfig, img_url: &str, ratios: &str) -> String {
        let h = config.host_config();
        url(
            config,
            &h.api_host,
            &format!("/cv/img/aicrop?img_url={img_url}&ratios={ratios}"),
        )
    }
}

/// 默认 API 域名字面量（与 `url_core::API_HOST` 一致，供本模块内部使用）。
#[allow(unused)]
const API_HOST: &str = DEFAULT_API_HOST_URL;
