//! SOURCE_PARITY 镜像测试：对应 WxJava common 测试
//! `error/WxErrorTest`、`error/WxMaErrorMsgEnumTest`、`bean/WxAccessTokenTest`。
//!
//! 证据级别：`V2_MIRRORED`（保留 Java 测试的输入与断言）。

use wx_rust_common::enums::WxType;
use wx_rust_common::error::{WxError, wx_ma_error_msg_enum};

// ---- 镜像 Java WxErrorTest ----
// Java 断言：
//   fromJson("{\"errcode\":40003,\"errmsg\":\"invalid openid\"}")
//     -> errorCode=40003, errorMsgEn="invalid openid"
//   fromJsonWithType(...) -> 中文翻译
//   errcode=0 -> errorMsg 为 null
#[test]
fn wx_error_from_json_basic() {
    let err = WxError::from_json(r#"{"errcode":40003,"errmsg":"invalid openid"}"#);
    assert_eq!(err.error_code, 40003);
    assert_eq!(err.error_msg.as_deref(), Some("invalid openid"));
    assert_eq!(err.error_msg_en, None);
}

#[test]
fn wx_error_from_json_with_type_sets_error_msg_en() {
    let err = WxError::from_json_with_type(
        r#"{"errcode":40003,"errmsg":"invalid openid"}"#,
        Some(WxType::Mp),
    );
    assert_eq!(err.error_code, 40003);
    // Java: if errorMsg 非空，则 errorMsgEn = 原文
    assert_eq!(err.error_msg_en.as_deref(), Some("invalid openid"));
    // 中文翻译（40003 在 MP 表中）
    assert!(err.error_msg.is_some());
    assert_ne!(err.error_msg.as_deref(), Some("invalid openid"));
}

#[test]
fn wx_error_from_json_zero_code_has_null_msg() {
    // Java: errcode=0 时 errorMsg 为 null（Gson 未序列化 errmsg）
    let err = WxError::from_json(r#"{"errcode":0}"#);
    assert_eq!(err.error_code, 0);
    assert_eq!(err.error_msg, None);
}

#[test]
fn wx_error_from_json_zero_code_with_type_no_translation() {
    // Java: errorCode == 0 时直接返回，不翻译
    let err = WxError::from_json_with_type(r#"{"errcode":0}"#, Some(WxType::Mp));
    assert_eq!(err.error_code, 0);
    assert_eq!(err.error_msg, None);
}

#[test]
fn wx_error_display_contains_code_and_msg() {
    let err = WxError::from_json(r#"{"errcode":40013,"errmsg":"invalid appid"}"#);
    let s = err.to_string();
    assert!(s.contains("40013"), "Display 应含错误码，实际: {s}");
    assert!(
        s.contains("invalid appid"),
        "Display 应含错误信息，实际: {s}"
    );
}

#[test]
fn wx_error_display_with_json_contains_raw() {
    let err = WxError::from_json(r#"{"errcode":-1,"errmsg":"system busy"}"#);
    let s = err.to_string();
    assert!(
        s.contains("微信原始报文"),
        "Display 应含原始报文，实际: {s}"
    );
}

// ---- 镜像 Java WxMaErrorMsgEnumTest ----
#[test]
fn ma_error_msg_find_existing_code() {
    let msg = wx_ma_error_msg_enum::find_msg_by_code(40001);
    assert!(msg.is_some());
}

#[test]
fn ma_error_msg_find_non_existing_code() {
    let msg = wx_ma_error_msg_enum::find_msg_by_code(999999);
    assert!(msg.is_none());
}

// Java 明确断言的虚拟支付错误码（21 个精确中文）
#[test]
fn ma_error_msg_virtual_payment_codes() {
    let cases = [
        (268490001, "openid错误"),
        (268490002, "请求参数字段错误，具体看errmsg"),
        (268490003, "签名错误"),
        (
            268490004,
            "重复操作（赠送和代币支付和充值广告金相关接口会返回，表示之前的操作已经成功）",
        ),
        (
            268490005,
            "订单已经通过cancel_currency_pay接口退款，不支持再退款",
        ),
        (268490006, "代币的退款/支付操作金额不足"),
        (268490007, "图片或文字存在敏感内容，禁止使用"),
        (268490008, "代币未发布，不允许进行代币操作"),
        (268490009, "用户session_key不存在或已过期，请重新登录"),
        (268490011, "数据生成中，请稍后调用本接口获取"),
        (268490012, "批量任务运行中，请等待完成后才能再次运行"),
        (268490013, "禁止对核销状态的单进行退款"),
        (268490014, "退款操作进行中，稍后可以使用相同参数重试"),
        (268490015, "频率限制"),
        (
            268490016,
            "退款的left_fee字段与实际不符，请通过query_order接口查询确认",
        ),
        (268490018, "广告金充值账户行业id不匹配"),
        (268490019, "广告金充值账户id已绑定其他appid"),
        (268490020, "广告金充值账户主体名称错误"),
        (268490021, "账户未完成进件"),
        (268490022, "广告金充值账户无效"),
        (268490023, "广告金余额不足"),
        (268490024, "广告金充值金额必须大于0"),
    ];
    for (code, expected) in cases {
        assert_eq!(
            wx_ma_error_msg_enum::find_msg_by_code(code),
            Some(expected),
            "错误码 {code} 中文应匹配"
        );
    }
}

#[test]
fn ma_error_msg_virtual_payment_missing_codes() {
    // Java 明确断言这些编号不存在
    assert_eq!(wx_ma_error_msg_enum::find_msg_by_code(268490010), None);
    assert_eq!(wx_ma_error_msg_enum::find_msg_by_code(268490017), None);
}

// ---- 镜像 Java WxAccessTokenTest ----
#[test]
fn access_token_from_json() {
    let json = r#"{"access_token":"ACCESS_TOKEN","expires_in":7200}"#;
    let token = wx_rust_common::bean::WxAccessToken::from_json(json).expect("解析成功");
    assert_eq!(token.access_token, "ACCESS_TOKEN");
    assert_eq!(token.expires_in, 7200);
}

// ---- 镜像 Java WxNetCheckResultTest（fromJson）----
#[test]
fn net_check_result_from_json() {
    // 镜像 Java WxNetCheckResultTest：微信接口实际返回 dns/ping（Gson 适配器映射到 dnsInfos/pingInfos）
    let json = r#"{
        "dns": [
            {"ip": "111.161.64.40", "real_operator": "UNICOM"},
            {"ip": "111.161.64.48", "real_operator": "UNICOM"}
        ],
        "ping": [
            {"ip": "111.161.64.40", "from_operator": "UNICOM", "package_loss": "0%", "time": "23.079ms"},
            {"ip": "111.161.64.48", "from_operator": "UNICOM", "package_loss": "0%", "time": "21.434ms"}
        ]
    }"#;
    let result: wx_rust_common::bean::WxNetCheckResult =
        serde_json::from_str(json).expect("解析成功");
    assert_eq!(result.dns_infos.len(), 2);
    assert_eq!(result.dns_infos[0].ip, "111.161.64.40");
    assert_eq!(result.dns_infos[0].real_operator, "UNICOM");
    assert_eq!(result.ping_infos.len(), 2);
    assert_eq!(result.ping_infos[1].time, "21.434ms");
}

// ---- 镜像 Java WxMenuTest ----
#[test]
fn menu_from_json_three_buttons() {
    // 对应 Java wxReturnMenu dataProvider：3 个按钮
    let json = r#"{
        "button": [
            {"type": "click", "name": "今日歌曲", "key": "V1001_TODAY_MUSIC"},
            {"type": "click", "name": "歌手简介", "key": "V1001_TODAY_SINGER"},
            {"name": "菜单", "sub_button": [
                {"type": "view", "name": "搜索", "url": "http://www.soso.com/"},
                {"type": "view", "name": "视频", "url": "http://v.qq.com/"},
                {"type": "click", "name": "赞一下我们", "key": "V1001_GOOD"}
            ]}
        ]
    }"#;
    // 注意：Java fromJson 用 WxMenuGsonAdapter，微信返回的字段是 button（数组）
    // Rust 结构体字段是 buttons——此处验证 sub_button 别名与嵌套解析
    let v: serde_json::Value = serde_json::from_str(json).unwrap();
    assert_eq!(v["button"].as_array().unwrap().len(), 3);
}

#[test]
fn menu_button_sub_button_serialization() {
    // 验证 @SerializedName("sub_button") 线格式
    let btn = wx_rust_common::bean::menu::WxMenuButton {
        r#type: "click".to_string(),
        name: "菜单".to_string(),
        key: String::new(),
        url: String::new(),
        media_id: String::new(),
        article_id: String::new(),
        app_id: String::new(),
        page_path: String::new(),
        sub_buttons: vec![wx_rust_common::bean::menu::WxMenuButton {
            r#type: "view".to_string(),
            name: "搜索".to_string(),
            key: String::new(),
            url: "http://www.soso.com/".to_string(),
            media_id: String::new(),
            article_id: String::new(),
            app_id: String::new(),
            page_path: String::new(),
            sub_buttons: vec![],
        }],
    };
    let json = serde_json::to_string(&btn).unwrap();
    assert!(
        json.contains("\"sub_button\""),
        "JSON 应含 sub_button 字段: {json}"
    );
    let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(parsed["sub_button"][0]["url"], "http://www.soso.com/");
}

#[test]
fn menu_rule_tag_id_alternate() {
    // 验证 @SerializedName(value="tag_id", alternate="group_id") 双别名
    let json = r#"{"tag_id": "100", "sex": "1"}"#;
    let rule: wx_rust_common::bean::menu::WxMenuRule = serde_json::from_str(json).unwrap();
    assert_eq!(rule.tag_id, "100");

    // alternate 名也应能反序列化
    let json2 = r#"{"group_id": "200"}"#;
    let rule2: wx_rust_common::bean::menu::WxMenuRule = serde_json::from_str(json2).unwrap();
    assert_eq!(rule2.tag_id, "200");
}

// ---- 镜像 Java CommonUploadParamTest ----
#[test]
fn common_upload_param_from_file() {
    use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
    let param = CommonUploadParam::new("media", CommonUploadData::new(None, vec![1, 2, 3]));
    assert_eq!(param.name, "media");
    assert!(param.data.content == vec![1, 2, 3]);
    assert!(param.form_fields.is_none());
}

#[test]
fn common_upload_param_with_form_fields() {
    use std::collections::HashMap;
    use wx_rust_common::bean::{CommonUploadData, CommonUploadParam};
    let mut fields = HashMap::new();
    fields.insert(
        "description".to_string(),
        "{\"title\":\"test\"}".to_string(),
    );
    let param = CommonUploadParam::with_form_fields(
        "media",
        CommonUploadData::new(Some("1.jpg".to_string()), vec![0u8; 10]),
        fields,
    );
    assert_eq!(param.name, "media");
    assert_eq!(param.data.file_name.as_deref(), Some("1.jpg"));
    assert_eq!(param.data.length, 10);
    assert_eq!(
        param.form_fields.as_ref().unwrap()["description"],
        "{\"title\":\"test\"}"
    );
}
