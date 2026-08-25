//! Coverage boost: `qidian_response.rs` (89 missed, 3.3%).
//!
//! Exercises `errmsg_chinese()` for all error codes and serde roundtrip.

use wx_rust_qidian::bean::common::QidianResponse;

#[test]
fn errmsg_chinese_all_codes() {
    let codes_and_msgs = vec![
        (-1, "系统繁忙"),
        (0, "请求成功"),
        (
            40001,
            "获取access_token时AppSecret错误，或者access_token无效",
        ),
        (40002, "不合法的凭证类型"),
        (40003, "不合法的OpenID"),
        (40004, "不合法的媒体文件类型"),
        (40005, "不合法的文件类型"),
        (40006, "不合法的文件大小"),
        (40007, "不合法的媒体文件id"),
        (40008, "不合法的消息类型"),
        (40009, "不合法的图片文件大小"),
        (40010, "不合法的语音文件大小"),
        (40011, "不合法的视频文件大小"),
        (40012, "不合法的缩略图文件大小"),
        (40013, "不合法的APPID"),
        (40014, "不合法的access_token"),
        (40015, "不合法的菜单类型"),
        (40016, "不合法的按钮个数"),
        (40017, "不合法的按钮个数"),
        (40018, "不合法的按钮名字长度"),
        (40019, "不合法的按钮KEY长度"),
        (40020, "不合法的按钮URL长度"),
        (40021, "不合法的菜单版本号"),
        (40022, "不合法的子菜单级数"),
        (40023, "不合法的子菜单按钮个数"),
        (40024, "不合法的子菜单按钮类型"),
        (40025, "不合法的子菜单按钮名字长度"),
        (40026, "不合法的子菜单按钮KEY长度"),
        (40027, "不合法的子菜单按钮URL长度"),
        (40028, "不合法的自定义菜单使用用户"),
        (40029, "不合法的oauth_code"),
        (40030, "不合法的refresh_token"),
        (40031, "不合法的openid列表"),
        (40032, "不合法的openid列表长度"),
        (40033, "不合法的请求字符，不能包含\\uxxxx格式的字符"),
        (40035, "不合法的参数"),
        (40038, "不合法的请求格式"),
        (40039, "不合法的URL长度"),
        (40050, "不合法的分组id"),
        (40051, "分组名字不合法"),
        (41001, "缺少access_token参数"),
        (41002, "缺少appid参数"),
        (41003, "缺少refresh_token参数"),
        (41004, "缺少secret参数"),
        (41005, "缺少多媒体文件数据"),
        (41006, "缺少media_id参数"),
        (41007, "缺少子菜单数据"),
        (41008, "缺少oauth code"),
        (41009, "缺少openid"),
        (42001, "access_token超时"),
        (42002, "refresh_token超时"),
        (42003, "oauth_code超时"),
        (43001, "需要GET请求"),
        (43002, "需要POST请求"),
        (43003, "需要HTTPS请求"),
        (43004, "需要接收者关注"),
        (43005, "需要好友关系"),
        (44001, "多媒体文件为空"),
        (44002, "POST的数据包为空"),
        (44003, "图文消息内容为空"),
        (44004, "文本消息内容为空"),
        (45001, "多媒体文件大小超过限制"),
        (45002, "消息内容超过限制"),
        (45003, "标题字段超过限制"),
        (45004, "描述字段超过限制"),
        (45005, "链接字段超过限制"),
        (45006, "图片链接字段超过限制"),
        (45007, "语音播放时间超过限制"),
        (45008, "图文消息超过限制"),
        (45009, "接口调用超过限制"),
        (45010, "创建菜单个数超过限制"),
        (45015, "回复时间超过限制"),
        (45016, "系统分组，不允许修改"),
        (45017, "分组名字过长"),
        (45018, "分组数量超过上限"),
        (46001, "不存在媒体数据"),
        (46002, "不存在的菜单版本"),
        (46003, "不存在的菜单数据"),
        (46004, "不存在的用户"),
        (47001, "解析JSON/XML内容错误"),
        (48001, "api功能未授权"),
        (50001, "用户未授权该api"),
    ];
    for (code, expected_msg) in &codes_and_msgs {
        let resp = QidianResponse {
            errcode: *code,
            ..Default::default()
        };
        assert_eq!(
            resp.errmsg_chinese(),
            Some(*expected_msg),
            "mismatch for code {code}"
        );
    }
}

#[test]
fn errmsg_chinese_unknown_code() {
    let resp = QidianResponse {
        errcode: 99999,
        ..Default::default()
    };
    assert!(resp.errmsg_chinese().is_none());
}

#[test]
fn default_values() {
    let resp = QidianResponse::default();
    assert_eq!(resp.code, 0);
    assert!(resp.msg.is_none());
    assert_eq!(resp.errcode, 0);
    assert!(resp.errmsg_chinese.is_none());
}

#[test]
fn serde_roundtrip() {
    let resp = QidianResponse {
        code: 0,
        msg: Some("success".into()),
        errcode: 0,
        errmsg: "ok".into(),
        errmsg_chinese: None,
    };
    let json = serde_json::to_string(&resp).unwrap();
    let back: QidianResponse = serde_json::from_str(&json).unwrap();
    assert_eq!(resp.code, back.code);
    assert_eq!(resp.msg, back.msg);
    assert_eq!(resp.errcode, back.errcode);
    assert_eq!(resp.errmsg, back.errmsg);
}

#[test]
fn serde_deserialize_with_defaults() {
    let json = r#"{"code":1,"msg":"test"}"#;
    let resp: QidianResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.code, 1);
    assert_eq!(resp.msg, Some("test".into()));
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.errmsg, "ok");
}

#[test]
fn serde_deserialize_errmsg_default() {
    let json = r#"{"errcode":40001}"#;
    let resp: QidianResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.errcode, 40001);
    assert_eq!(resp.errmsg, "ok");
}

#[test]
fn serde_deserialize_full() {
    let json = r#"{"code":0,"msg":"ok","errcode":0,"errmsg":"ok"}"#;
    let resp: QidianResponse = serde_json::from_str(json).unwrap();
    assert_eq!(resp.code, 0);
}

#[test]
fn debug_format() {
    let resp = QidianResponse::default();
    let dbg = format!("{resp:?}");
    assert!(dbg.contains("QidianResponse"));
}

#[test]
fn clone_works() {
    let resp = QidianResponse::default();
    let cloned = resp.clone();
    assert_eq!(resp, cloned);
}

#[test]
fn partial_eq() {
    let a = QidianResponse::default();
    let b = QidianResponse::default();
    assert_eq!(a, b);
}
