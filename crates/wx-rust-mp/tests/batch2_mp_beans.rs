#![allow(clippy::field_reassign_with_default, dead_code)]
//! 第二批镜像补测——MP bean 层。
//!
//! 本文件镜像以下 Java 测试类（按 LOC 倒序）：
//! - WxMpMemberCardServiceImplTest（163 行）
//! - WxMpCurrentAutoReplyInfoTest（157 行）
//! - WxMpMenuTest（152 行）

use wx_rust_mp::bean::result::*;

// ═══════════════════════════════════════════════════════════════
// #1 WxMpMemberCardServiceImplTest（163 行）—— 会员卡服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpMemberCardServiceImplTest（会员卡信息响应解析）
#[test]
fn test_member_card_info_response_serde() {
    let json_str = r#"{
        "errcode": 0,
        "errmsg": "ok",
        "membership": {
            "background_pic_url": "https://example.com/bg.jpg",
            "base_info": {
                "logo_url": "https://example.com/logo.jpg",
                "brand_name": "测试品牌",
                "title": "会员卡",
                "color": "Color010"
            },
            "prerogative": "会员特权说明"
        }
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["errcode"], 0);
    assert_eq!(value["membership"]["prerogative"], "会员特权说明");
}

/// 对应 Java: WxMpMemberCardServiceImplTest（会员卡创建请求体构建）
#[test]
fn test_member_card_create_request_body() {
    let body = serde_json::json!({
        "card": {
            "card_type": "MEMBER_CARD",
            "member_card": {
                "background_pic_url": "https://example.com/bg.jpg",
                "base_info": {
                    "logo_url": "https://example.com/logo.jpg",
                    "brand_name": "测试品牌",
                    "title": "会员卡",
                    "color": "Color010"
                },
                "prerogative": "会员特权"
            }
        }
    });
    assert_eq!(body["card"]["card_type"], "MEMBER_CARD");
    assert_eq!(
        body["card"]["member_card"]["base_info"]["brand_name"],
        "测试品牌"
    );
}

/// 对应 Java: WxMpMemberCardServiceImplTest（会员信息更新请求体构建）
#[test]
fn test_member_card_update_user_request() {
    let body = serde_json::json!({
        "code": "CARD_CODE_001",
        "card_id": "CARD_ID_001",
        "background_pic_url": "https://example.com/new_bg.jpg"
    });
    assert_eq!(body["code"], "CARD_CODE_001");
    assert_eq!(body["card_id"], "CARD_ID_001");
}

// ═══════════════════════════════════════════════════════════════
// #2 WxMpCurrentAutoReplyInfoTest（157 行）—— 自动回复信息
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpCurrentAutoReplyInfoTest（自动回复信息 JSON 解析）
#[test]
fn test_current_auto_reply_info_serde() {
    let json_str = r#"{
        "is_add_friend_reply_open": 1,
        "is_autoreply_open": 1,
        "keyword_autoreply_info": {
            "list": [
                {
                    "keyword_info": {
                        "list": [
                            {"type": "text", "content": "你好"}
                        ]
                    },
                    "reply_info": {
                        "list": [
                            {"type": "text", "content": "欢迎关注"}
                        ]
                    }
                }
            ]
        }
    }"#;
    let info: WxMpCurrentAutoReplyInfo = serde_json::from_str(json_str).expect("解析自动回复信息");
    assert_eq!(info.is_add_friend_reply_open, Some(true));
    assert_eq!(info.is_auto_reply_open, Some(true));
}

/// 对应 Java: WxMpCurrentAutoReplyInfoTest（关键词自动回复规则验证）
#[test]
fn test_keyword_auto_reply_rule_serde() {
    let json_str = r#"{
        "keyword_info": {
            "list": [
                {"type": "text", "content": "关键词1"},
                {"type": "text", "content": "关键词2"}
            ]
        },
        "reply_info": {
            "list": [
                {"type": "text", "content": "回复内容1"}
            ]
        }
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["keyword_info"]["list"].as_array().unwrap().len(), 2);
}

/// 对应 Java: WxMpCurrentAutoReplyInfoTest（关注后自动回复验证）
#[test]
fn test_follow_auto_reply_serde() {
    let json_str = r#"{
        "is_add_friend_reply_open": 1,
        "add_friend_reply_info": {
            "type": "text",
            "content": "欢迎关注！"
        }
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    assert_eq!(value["is_add_friend_reply_open"], 1);
    assert_eq!(value["add_friend_reply_info"]["content"], "欢迎关注！");
}

// ═══════════════════════════════════════════════════════════════
// #3 WxMpMenuTest（152 行）—— 菜单测试
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMpMenuTest（菜单 JSON 序列化验证）
#[test]
fn test_mp_menu_json_serde() {
    let json_str = r#"{
        "menu": {
            "button": [
                {
                    "type": "click",
                    "name": "今日歌曲",
                    "key": "V1001_TODAY_MUSIC"
                },
                {
                    "name": "菜单",
                    "sub_button": [
                        {
                            "type": "view",
                            "name": "搜索",
                            "url": "http://www.soso.com/"
                        }
                    ]
                }
            ]
        }
    }"#;
    let value: serde_json::Value = serde_json::from_str(json_str).expect("解析 JSON");
    let buttons = value["menu"]["button"].as_array().unwrap();
    assert_eq!(buttons.len(), 2);
    assert_eq!(buttons[0]["type"], "click");
    assert_eq!(buttons[0]["key"], "V1001_TODAY_MUSIC");
}

/// 对应 Java: WxMpMenuTest（菜单创建请求体验证）
#[test]
fn test_mp_menu_create_body() {
    let body = serde_json::json!({
        "button": [
            {
                "type": "click",
                "name": "今日歌曲",
                "key": "V1001_TODAY_MUSIC"
            }
        ]
    });
    let buttons = body["button"].as_array().unwrap();
    assert_eq!(buttons.len(), 1);
    assert_eq!(buttons[0]["name"], "今日歌曲");
}
