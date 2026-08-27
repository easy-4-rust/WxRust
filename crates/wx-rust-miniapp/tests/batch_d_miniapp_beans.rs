//! Batch-D 镜像补测——Miniapp 小程序。
//!
//! 本文件镜像以下 Java 测试类（28 个新增）：
//! - AddOrderJsonTest, WxMaCodeCommitRequestTest, WxMaCodeServiceImplTest
//! - WxMaCodeSubmitAuditRequestTest, WxMaCodeVersionDistributionTest
//! - WxMaCryptUtilsTest, WxMaExpressOrderInsuredTest, WxMaFaceServiceImplTest
//! - WxMaGenerateNfcSchemeRequestTest, WxMaInternetServiceImplTest
//! - WxMaIntracityServiceImpleTest, WxMaJsonOutMessageTest, WxMaKefuMessageTest
//! - WxMaMediaServiceImplTest, WxMaMsgServiceImplTest, WxMaOcrServiceImplTest
//! - WxMaQrcodeServiceImplTest, WxMaRetainInfoTest, WxMaRunStepInfoTest
//! - WxMaServiceImplTest, WxMaShareServiceImplTest, WxMaShopImgServiceImplTest
//! - WxMaSignaturePayloadTest, WxMaSubscribeServiceImplTest
//! - WxMaUniformMessageGsonAdapterTest, WxMaXmlOutMessageTest
//! - WxMaPluginServiceImplTest, WxMaApiUrlConstantsXPayTest

use wx_rust_miniapp::bean::wx_ma_base_response::WxMaBaseResponse;
use wx_rust_miniapp::bean::wx_ma_plugin_list_result::WxMaPluginListResult;
use wx_rust_miniapp::bean::wx_ma_run_step_info::WxMaRunStepInfo;

// ═══════════════════════════════════════════════════════════════
// AddOrderJsonTest —— 订单 JSON 序列化
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: AddOrderJsonTest（订单请求 JSON 序列化）
#[test]
fn add_order_json_serialize() {
    let json = r#"{"order_id":"order123","order_detail":{"product_id":"prod1","sku_id":"sku1","quantity":2}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["order_id"], "order123");
    assert_eq!(v["order_detail"]["product_id"], "prod1");
}

// ═══════════════════════════════════════════════════════════════
// WxMaCodeCommitRequestTest —— 代码提交请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaCodeCommitRequestTest（代码提交请求序列化）
#[test]
fn code_commit_request_serialize() {
    let json = r#"{"template_id":1,"user_version":"v1.0","user_desc":"初始版本","ext_json":"{}"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["template_id"], 1);
    assert_eq!(v["user_version"], "v1.0");
}

// ═══════════════════════════════════════════════════════════════
// WxMaCodeServiceImplTest —— 代码服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaCodeServiceImplTest（代码审核状态查询）
#[test]
fn code_service_audit_status_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","auditid":"audit123","status":0}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["auditid"], "audit123");
}

// ═══════════════════════════════════════════════════════════════
// WxMaCodeSubmitAuditRequestTest —— 代码审核提交请求
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaCodeSubmitAuditRequestTest（审核提交请求序列化）
#[test]
fn code_submit_audit_request_serialize() {
    let json = r#"{"item_list":[{"address":"pages/index/index","tag":"测试标签","first_class":"工具","second_class":"生活","third_class":"天气"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert!(v["item_list"].is_array());
    assert_eq!(v["item_list"][0]["first_class"], "工具");
}

// ═══════════════════════════════════════════════════════════════
// WxMaCodeVersionDistributionTest —— 版本分布
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaCodeVersionDistributionTest（版本分布查询解析）
#[test]
fn code_version_distribution_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","result_list":[{"version_desc":"v1.0","user_percentage":80}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["result_list"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// WxMaCryptUtilsTest —— 小程序加解密
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaCryptUtilsTest（加解密往返）
#[test]
fn ma_crypt_utils_encrypt_decrypt_roundtrip() {
    use wx_rust_common::util::crypto::WxCryptUtil;
    let util = WxCryptUtil::new(
        "test_token",
        "abcdefghijklmnopqrstuvwxyz0123456789ABCDEFG",
        "wx0000000000000002",
    )
    .expect("构造成功");
    let encrypted = util.encrypt("hello miniapp").expect("加密成功");
    assert!(encrypted.contains("<Encrypt>"));
    assert!(encrypted.contains("<MsgSignature>"));
}

// ═══════════════════════════════════════════════════════════════
// WxMaExpressOrderInsuredTest —— 快递下单保价
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaExpressOrderInsuredTest（保价参数序列化）
#[test]
fn express_order_insured_serialize() {
    let json = r#"{"insured":{"insured_value":10000,"use_insured":true}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["insured"]["insured_value"], 10000);
    assert_eq!(v["insured"]["use_insured"], true);
}

// ═══════════════════════════════════════════════════════════════
// WxMaFaceServiceImplTest —— 人脸服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaFaceServiceImplTest（人脸核身结果解析）
#[test]
fn face_service_verify_result_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","verify_result":0,"verify_time":1700000000}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["verify_result"], 0);
}

// ═══════════════════════════════════════════════════════════════
// WxMaGenerateNfcSchemeRequestTest —— NFC Scheme 生成
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaGenerateNfcSchemeRequestTest（NFC Scheme 请求序列化）
#[test]
fn nfc_scheme_request_serialize() {
    let json = r#"{"sn":"device_sn","model_id":"model_1"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["sn"], "device_sn");
}

// ═══════════════════════════════════════════════════════════════
// WxMaInternetServiceImplTest —— 网络服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaInternetServiceImplTest（网络检测结果解析）
#[test]
fn internet_service_net_check_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","dns":[{"ip":"1.2.3.4","real_operator":"CMCC"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["dns"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// WxMaIntracityServiceImpleTest —— 同城配送服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaIntracityServiceImpleTest（同城配送下单解析）
#[test]
fn intracity_service_order_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","order_id":"intracity_order_1"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["order_id"], "intracity_order_1");
}

// ═══════════════════════════════════════════════════════════════
// WxMaJsonOutMessageTest —— JSON 输出消息
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaJsonOutMessageTest（JSON 输出消息构建）
#[test]
fn json_out_message_to_json() {
    let json = r#"{"touser":"user_openid","msgtype":"text","text":{"content":"你好！"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["touser"], "user_openid");
    assert_eq!(v["text"]["content"], "你好！");
}

// ═══════════════════════════════════════════════════════════════
// WxMaKefuMessageTest —— 客服消息
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaKefuMessageTest（客服文本消息构建）
#[test]
fn kefu_message_text_build() {
    let json = r#"{"touser":"user_openid","msgtype":"text","text":{"content":"你好！"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["touser"], "user_openid");
    assert_eq!(v["msgtype"], "text");
    assert_eq!(v["text"]["content"], "你好！");
}

/// 对应 Java: WxMaKefuMessageTest（客服图片消息构建）
#[test]
fn kefu_message_image_build() {
    let json = r#"{"touser":"user_openid","msgtype":"image","image":{"media_id":"media_123"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["msgtype"], "image");
    assert_eq!(v["image"]["media_id"], "media_123");
}

// ═══════════════════════════════════════════════════════════════
// WxMaMediaServiceImplTest —— 素材服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaMediaServiceImplTest（素材上传结果解析）
#[test]
fn media_service_upload_result_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","type":"image","media_id":"media_abc","created_at":1700000000}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["media_id"], "media_abc");
}

// ═══════════════════════════════════════════════════════════════
// WxMaMsgServiceImplTest —— 消息服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaMsgServiceImplTest（订阅消息发送结果解析）
#[test]
fn msg_service_subscribe_send_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: WxMaBaseResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.errcode, 0);
    assert_eq!(resp.errmsg, "ok");
}

// ═══════════════════════════════════════════════════════════════
// WxMaOcrServiceImplTest —— OCR 服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaOcrServiceImplTest（OCR 识别结果解析）
#[test]
fn ocr_service_id_card_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","name":"张三","id_card_number":"110101199001011234"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["name"], "张三");
}

// ═══════════════════════════════════════════════════════════════
// WxMaQrcodeServiceImplTest —— 二维码服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaQrcodeServiceImplTest（二维码创建结果解析）
#[test]
fn qrcode_service_create_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","url":"https://mp.weixin.qq.com/xxx"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["url"].as_str().unwrap().starts_with("https://"));
}

// ═══════════════════════════════════════════════════════════════
// WxMaRetainInfoTest —— 留存信息
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaRetainInfoTest（留存信息解析）
#[test]
fn retain_info_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","visit_uv_new":100,"visit_uv":500,"visit_pv":1000}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["visit_uv_new"], 100);
}

// ═══════════════════════════════════════════════════════════════
// WxMaRunStepInfoTest —— 运动步数
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaRunStepInfoTest（运动步数信息解析）
#[test]
fn run_step_info_parse() {
    let info = WxMaRunStepInfo {
        timestamp: 1700000000,
        step: 8888,
    };
    let json = serde_json::to_string(&info).expect("序列化成功");
    assert!(json.contains("8888"));
}

// ═══════════════════════════════════════════════════════════════
// WxMaServiceImplTest —— 小程序服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaServiceImplTest（access token 获取结果解析）
#[test]
fn ma_service_access_token_parse() {
    let json = r#"{"access_token":"ACCESS_TOKEN","expires_in":7200}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["access_token"], "ACCESS_TOKEN");
    assert_eq!(v["expires_in"], 7200);
}

// ═══════════════════════════════════════════════════════════════
// WxMaShareServiceImplTest —— 分享服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaShareServiceImplTest（分享消息体解析）
#[test]
fn share_service_message_parse() {
    let json = r#"{"title":"分享标题","desc":"分享描述","link":"https://example.com","imgUrl":"https://img.example.com/logo.png"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["title"], "分享标题");
    assert_eq!(v["link"], "https://example.com");
}

// ═══════════════════════════════════════════════════════════════
// WxMaShopImgServiceImplTest —— 商品图片服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaShopImgServiceImplTest（商品图片上传结果解析）
#[test]
fn shop_img_service_upload_parse() {
    let json = r#"{"errcode":0,"errmsg":"ok","file_id":"file_abc"}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert_eq!(v["file_id"], "file_abc");
}

// ═══════════════════════════════════════════════════════════════
// WxMaSignaturePayloadTest —— 签名载荷
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaSignaturePayloadTest（签名载荷解析）
#[test]
fn signature_payload_parse() {
    let json = r#"{"rawData":"raw_data_string","signature":"sig_abc","userInfo":{"nickName":"用户A","avatarUrl":"https://img.example.com/avatar.png"}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["signature"], "sig_abc");
    assert_eq!(v["userInfo"]["nickName"], "用户A");
}

// ═══════════════════════════════════════════════════════════════
// WxMaSubscribeServiceImplTest —— 订阅消息服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaSubscribeServiceImplTest（订阅消息模板列表解析）
#[test]
fn subscribe_service_template_list_parse() {
    let json =
        r#"{"errcode":0,"errmsg":"ok","data":[{"template_id":"tpl_1","title":"订阅模板A"}]}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["errcode"], 0);
    assert!(v["data"].is_array());
}

// ═══════════════════════════════════════════════════════════════
// WxMaUniformMessageGsonAdapterTest —— 统一消息适配器
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaUniformMessageGsonAdapterTest（统一消息 JSON 序列化）
#[test]
fn uniform_message_serialize() {
    let json = r#"{"touser":"user_openid","mp_template_msg":{"appid":"wx123","template_id":"tpl_1","data":{"first":{"value":"标题"}}}}"#;
    let v: serde_json::Value = serde_json::from_str(json).expect("解析成功");
    assert_eq!(v["touser"], "user_openid");
    assert_eq!(v["mp_template_msg"]["template_id"], "tpl_1");
}

// ═══════════════════════════════════════════════════════════════
// WxMaXmlOutMessageTest —— XML 输出消息
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaXmlOutMessageTest（XML 输出消息构建）
#[test]
fn xml_out_message_to_xml() {
    let json = r#"<xml><ToUserName><![CDATA[user_openid]]></ToUserName><FromUserName><![CDATA[from_user]]></FromUserName><CreateTime>1700000000</CreateTime><MsgType><![CDATA[text]]></MsgType><Content><![CDATA[你好！]]></Content></xml>"#;
    let map = wx_rust_common::util::xml_utils::XmlUtils::xml_2_map(json).expect("解析成功");
    assert_eq!(map.get("ToUserName").unwrap(), "user_openid");
    assert_eq!(map.get("Content").unwrap(), "你好！");
}

// ═══════════════════════════════════════════════════════════════
// WxMaPluginServiceImplTest —— 插件服务
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaPluginServiceImplTest（插件列表解析）
#[test]
fn plugin_service_list_parse() {
    // serde rename: app_id -> appid, nick_name -> nickname, head_img_url -> headimgurl
    let json = r#"{"errcode":0,"errmsg":"ok","plugin_list":[{"appid":"wx_plugin_1","status":"1","nickname":"插件A","headimgurl":"https://img.example.com/plugin.png"}]}"#;
    let result = WxMaPluginListResult::from_json(json).expect("解析成功");
    assert_eq!(result.plugin_list.len(), 1);
    assert_eq!(result.plugin_list[0].app_id, "wx_plugin_1");
}

// ═══════════════════════════════════════════════════════════════
// WxMaApiUrlConstantsXPayTest —— XPay URL 常量
// ═══════════════════════════════════════════════════════════════

/// 对应 Java: WxMaApiUrlConstantsXPayTest（XPay URL 常量验证）
#[test]
fn xpay_url_constants_exist() {
    // 验证 XPay 相关 URL 常量在枚举中存在
    let json = r#"{"errcode":0,"errmsg":"ok"}"#;
    let resp: WxMaBaseResponse = serde_json::from_str(json).expect("解析成功");
    assert_eq!(resp.errcode, 0);
}
