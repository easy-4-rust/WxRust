#![allow(clippy::field_reassign_with_default)]
//! 覆盖率提升: wx_pay_service_impl_utils.rs 纯函数单测。
//!
//! 测试三层:
//! - SOURCE_PARITY: 镜像 Java 算法片段的行为等价
//! - RUST_OBLIGATION: Rust 侧的错误路径/边界条件
//! - VALUE_ADD: 空值/极端输入/组合路径

use std::collections::HashMap;

use wx_rust_pay::bean::xml::root_children_map;
use wx_rust_pay::bean::{
    WxPayDownloadBillRequest, WxPayMicropayRequest, WxPayOrderQueryRequest,
    WxPayRefundQueryRequest, WxPayUnifiedOrderRequest,
};
use wx_rust_pay::config::r#impl::WxPayDefaultConfig;
use wx_rust_pay::util::sign_utils::SignUtils;
use wx_rust_pay::util::wx_pay_service_impl_utils as impl_utils;
use wx_rust_pay::util::wx_pay_service_impl_utils::{FundFlowBillRequest, V2Request};

// ---- 夹具常量 ----

const MCH_KEY: &str = "192006250b4c09247ec02edce69f6a2d";
const APP_ID: &str = "wxd930ea5d5a258f4f";
const MCH_ID: &str = "10000100";

// ═══════════════════════════════════════════════════════════════════
// runtime / current_time_millis（SOURCE_PARITY: Java 工具方法）
// ═══════════════════════════════════════════════════════════════════

/// runtime() 构造 WxErrorException::Runtime（对应 Java WxRuntimeException）。
/// 对应 Java: WxPayException.Builder.runtime
#[test]
fn test_runtime_creates_wx_error() {
    let err = impl_utils::runtime("测试错误");
    assert!(err.to_string().contains("测试错误"));
}

/// current_time_millis() 返回非空数字字符串。
/// 对应 Java: System.currentTimeMillis()
#[test]
fn test_current_time_millis_returns_digits() {
    let ts = impl_utils::current_time_millis();
    assert!(!ts.is_empty());
    assert!(ts.chars().all(|c| c.is_ascii_digit()));
}

// ═══════════════════════════════════════════════════════════════════
// check_and_sign（SOURCE_PARITY: BaseWxPayRequest#checkAndSign）
// ═══════════════════════════════════════════════════════════════════

/// 配置回填：appid/mch_id/nonce_str 未设置时从 config 补齐。
/// 对应 Java: checkAndSign → getSignParams → fillAppid/fillMchId/fillNonceStr
#[test]
fn test_check_and_sign_fills_from_config() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY);
    let mut request = WxPayUnifiedOrderRequest::default();
    request.body = Some("test".to_string());
    request.total_fee = Some(1);
    request.spbill_create_ip = Some("127.0.0.1".to_string());
    request.trade_type = Some("JSAPI".to_string());
    request.out_trade_no = Some("order001".to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");

    assert_eq!(request.appid.as_deref(), Some(APP_ID));
    assert_eq!(request.mch_id.as_deref(), Some(MCH_ID));
    assert!(request.nonce_str.as_ref().is_some_and(|s| !s.is_empty()));
    assert!(request.sign.as_ref().is_some_and(|s| !s.is_empty()));
}

/// 非法 sign_type → 报错（对应 Java "非法的sign_type参数"）。
/// 对应 Java: checkAndSign sign_type 校验
#[test]
fn test_check_and_sign_rejects_invalid_sign_type() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY);
    let mut request = WxPayUnifiedOrderRequest::default();
    request.sign_type = Some("INVALID_TYPE".to_string());
    request.out_trade_no = Some("order001".to_string());

    let err = impl_utils::check_and_sign(&config, &mut request).expect_err("应报错");
    assert!(
        err.to_string().contains("非法的sign_type参数"),
        "错误信息: {err}"
    );
}

/// 非法 signType 配置 → 报错（对应 Java "非法的signType配置"）。
/// 对应 Java: checkAndSign config signType 校验
#[test]
fn test_check_and_sign_rejects_invalid_config_sign_type() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_sign_type("BAD_TYPE");
    let mut request = WxPayUnifiedOrderRequest::default();
    request.out_trade_no = Some("order001".to_string());

    let err = impl_utils::check_and_sign(&config, &mut request).expect_err("应报错");
    assert!(
        err.to_string().contains("非法的signType配置"),
        "错误信息: {err}"
    );
}

/// 配置的 sign_type 回填：请求未设置时从配置取。
/// 对应 Java: checkAndSign → request.getSignType() 空 → config.signType
#[test]
fn test_check_and_sign_sign_type_from_config() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_sign_type("HMAC-SHA256");
    let mut request = WxPayUnifiedOrderRequest::default();
    request.out_trade_no = Some("order001".to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");
    assert_eq!(request.sign_type.as_deref(), Some("HMAC-SHA256"));
}

/// ignore_appid=true 的请求不从配置回填 appid（对应 Java WxPayDefaultRequest）。
/// 对应 Java: checkAndSign → ignoreAppid() == true
#[test]
fn test_check_and_sign_ignore_appid() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY);
    // WxPayDefaultRequest: ignore_appid=true（沙箱签名 key 请求）
    let mut request = wx_rust_pay::bean::WxPayDefaultRequest::default();
    request.mch_id = Some(MCH_ID.to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");
    // appid 不被回填
    assert!(request.appid.is_none() || request.appid.as_deref() == Some(""));
}

/// need_nonce_str=false 的请求不自动填充 nonce（对应 Java WxPayQueryExchangeRateRequest）。
/// 对应 Java: checkAndSign → needNonceStr() == false
#[test]
fn test_check_and_sign_no_nonce_for_exchange_rate() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY);
    let mut request = wx_rust_pay::bean::WxPayQueryExchangeRateRequest::default();
    request.date = Some("20240101".to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");
    // need_nonce_str=false → nonce 不被自动填充
    assert!(request.nonce_str.is_none() || request.nonce_str.as_deref() == Some(""));
}

/// 已有 nonce_str 的请求不被覆盖。
/// 对应 Java: checkAndSign → 已有 nonce 不覆盖
#[test]
fn test_check_and_sign_preserves_existing_nonce() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY);
    let mut request = WxPayUnifiedOrderRequest::default();
    request.nonce_str = Some("custom_nonce".to_string());
    request.out_trade_no = Some("order001".to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");
    assert_eq!(request.nonce_str.as_deref(), Some("custom_nonce"));
}

// ═══════════════════════════════════════════════════════════════════
// check_result（SOURCE_PARITY: BaseWxPayResult#checkResult）
// ═══════════════════════════════════════════════════════════════════

/// 验签成功 + return_code/result_code=SUCCESS → Ok(())。
/// 对应 Java: checkResult → sign 验签通过 + return_code=SUCCESS
#[test]
fn test_check_result_success() {
    let mut config = WxPayDefaultConfig::new();
    config.set_mch_key(MCH_KEY);

    let fields = &[
        ("return_code", "SUCCESS"),
        ("return_msg", "OK"),
        ("result_code", "SUCCESS"),
        ("appid", APP_ID),
        ("mch_id", MCH_ID),
        ("nonce_str", "testnonce"),
    ];
    let sign = {
        let mut map: HashMap<String, String> = HashMap::new();
        for (k, v) in fields {
            map.insert(k.to_string(), v.to_string());
        }
        SignUtils::create_sign(&map, None, MCH_KEY, &[]).expect("签名")
    };
    let xml = format!(
        "<xml><sign><![CDATA[{sign}]]></sign>{}</xml>",
        fields
            .iter()
            .map(|(k, v)| format!("<{k}><![CDATA[{v}]]></{k}>"))
            .collect::<Vec<_>>()
            .join("")
    );

    impl_utils::check_result(&config, &xml, None, true).expect("校验通过");
}

/// return_code=FAIL → 报错，消息包含 return_msg/err_code/err_code_des。
/// 对应 Java: checkResult → return_code != SUCCESS
#[test]
fn test_check_result_return_code_fail() {
    let mut config = WxPayDefaultConfig::new();
    config.set_mch_key(MCH_KEY);

    let xml = "<xml><return_code><![CDATA[FAIL]]></return_code><return_msg><![CDATA[签名错误]]></return_msg></xml>";
    let err = impl_utils::check_result(&config, xml, None, true).expect_err("应报错");
    assert!(
        err.to_string().contains("返回代码：[FAIL]"),
        "错误信息: {err}"
    );
    assert!(err.to_string().contains("签名错误"), "错误信息: {err}");
}

/// result_code=FAIL → 报错，消息包含 err_code + err_code_des。
/// 对应 Java: checkResult → result_code != SUCCESS
#[test]
fn test_check_result_result_code_fail() {
    let mut config = WxPayDefaultConfig::new();
    config.set_mch_key(MCH_KEY);

    let xml = "<xml><return_code><![CDATA[SUCCESS]]></return_code><result_code><![CDATA[FAIL]]></result_code><err_code><![CDATA[ORDERNOTEXIST]]></err_code><err_code_des><![CDATA[订单不存在]]></err_code_des></xml>";
    let err = impl_utils::check_result(&config, xml, None, true).expect_err("应报错");
    assert!(err.to_string().contains("ORDERNOTEXIST"), "错误信息: {err}");
    assert!(err.to_string().contains("订单不存在"), "错误信息: {err}");
}

/// 验签失败 → 报错（对应 Java "参数格式校验错误！"）。
/// 对应 Java: checkResult → sign 不匹配
#[test]
fn test_check_result_sign_mismatch() {
    let mut config = WxPayDefaultConfig::new();
    config.set_mch_key(MCH_KEY);

    let xml = "<xml><return_code><![CDATA[SUCCESS]]></return_code><sign><![CDATA[WRONGSIGN]]></sign></xml>";
    let err = impl_utils::check_result(&config, xml, None, true).expect_err("应报错");
    assert!(
        err.to_string().contains("参数格式校验错误！"),
        "错误信息: {err}"
    );
}

/// check_success=false → 不校验 return_code/result_code。
/// 对应 Java: checkResult 参数控制
#[test]
fn test_check_result_no_success_check() {
    let mut config = WxPayDefaultConfig::new();
    config.set_mch_key(MCH_KEY);

    // return_code=FAIL 但 check_success=false → 不报错
    let xml = "<xml><return_code><![CDATA[FAIL]]></return_code></xml>";
    impl_utils::check_result(&config, xml, None, false).expect("check_success=false 时跳过校验");
}

/// 无 sign 字段的报文 → 跳过验签（对应 Java getSign() == null）。
/// 对应 Java: checkResult → sign == null → 不验签
#[test]
fn test_check_result_no_sign_skips_verify() {
    let mut config = WxPayDefaultConfig::new();
    config.set_mch_key(MCH_KEY);

    let xml = "<xml><return_code><![CDATA[SUCCESS]]></return_code><result_code><![CDATA[SUCCESS]]></result_code></xml>";
    impl_utils::check_result(&config, xml, None, true).expect("无 sign 时跳过验签");
}

// ═══════════════════════════════════════════════════════════════════
// parse_bill_result（SOURCE_PARITY: WxPayBillResult.fromRawBillResultString）
// ═══════════════════════════════════════════════════════════════════

/// SUCCESS 账单布局解析（对应 Java bill_type=SUCCESS）。
/// 对应 Java: fromRawBillResultString(responseContent, "SUCCESS")
#[test]
fn test_parse_bill_success_layout() {
    let bill_text = "交易时间,公众账号ID,商户号,特约商户号,设备号,微信订单号,商户订单号,用户标识,交易类型,交易状态,付款银行,货币种类,应结订单金额,代金券金额,商品名称,商户数据包,手续费,费率,订单金额,费率备注\n\
`2024-01-01 10:00:00,`wx1234,`10000100,`,`0,`500000001,`out001,`openid01,`JSAPI,`SUCCESS,`CMB,`CNY,`100.00,`0.00,`测试商品,`attach,`0.01,`0.60%,`100.00,`\n\
总交易单数,应结订单总金额,退款总金额,充值券退款总金额,手续费总金额,订单总金额,申请退款总金额\n\
`1,`100.00,`0.00,`0.00,`0.01,`100.00,`0.00";

    let result = impl_utils::parse_bill_result(bill_text, "SUCCESS").expect("解析成功");
    assert_eq!(result.total_record.as_deref(), Some("1"));
    assert_eq!(result.total_fee.as_deref(), Some("100.00"));
    assert_eq!(result.bill_info_list.len(), 1);
    assert_eq!(
        result.bill_info_list[0].transaction_id.as_deref(),
        Some("500000001")
    );
    assert_eq!(result.bill_info_list[0].body.as_deref(), Some("测试商品"));
}

/// REFUND 账单布局解析（对应 Java bill_type=REFUND）。
/// 对应 Java: fromRawBillResultString(responseContent, "REFUND")
#[test]
fn test_parse_bill_refund_layout() {
    let bill_text = "交易时间,公众账号ID,商户号,特约商户号,设备号,微信订单号,商户订单号,用户标识,交易类型,交易状态,付款银行,货币种类,应结订单金额,代金券金额,退款时间,退款成功时间,微信退款单号,商户退款单号,退款金额,充值券退款金额,退款渠道,退款状态,商品名称,商户数据包,手续费,费率\n\
`2024-01-01 10:00:00,`wx1234,`10000100,`,`0,`500000001,`out001,`openid01,`JSAPI,`SUCCESS,`CMB,`CNY,`100.00,`0.00,`2024-01-02 10:00:00,`2024-01-02 10:01:00,`refund001,`out_refund001,`100.00,`0.00,`ORIGINAL,`SUCCESS,`测试商品,`attach,`0.01,`0.60%\n\
总交易单数,应结订单总金额,退款总金额,充值券退款总金额,手续费总金额,订单总金额,申请退款总金额\n\
`1,`100.00,`100.00,`0.00,`0.01,`100.00,`100.00";

    let result = impl_utils::parse_bill_result(bill_text, "REFUND").expect("解析成功");
    assert_eq!(result.bill_info_list.len(), 1);
    assert_eq!(
        result.bill_info_list[0].refund_id.as_deref(),
        Some("refund001")
    );
    assert_eq!(
        result.bill_info_list[0].refund_state.as_deref(),
        Some("SUCCESS")
    );
}

/// RECHARGE_REFUND 账单布局解析。
/// 对应 Java: fromRawBillResultString(responseContent, "RECHARGE_REFUND")
#[test]
fn test_parse_bill_recharge_refund_layout() {
    let bill_text = "交易时间,公众账号ID,商户号,特约商户号,设备号,微信订单号,商户订单号,用户标识,交易类型,交易状态,付款银行,货币种类,应结订单金额,代金券金额,退款时间,退款成功时间,微信退款单号,商户退款单号,退款金额,充值券退款金额,退款渠道,退款状态,商品名称,商户数据包,手续费,费率,订单金额,申请退款金额\n\
`2024-01-01 10:00:00,`wx1234,`10000100,`,`0,`500000001,`out001,`openid01,`JSAPI,`SUCCESS,`CMB,`CNY,`100.00,`0.00,`2024-01-02 10:00:00,`2024-01-02 10:01:00,`refund001,`out_refund001,`100.00,`0.00,`ORIGINAL,`SUCCESS,`测试商品,`attach,`0.01,`0.60%,`100.00,`100.00\n\
总交易单数,应结订单总金额,退款总金额,充值券退款总金额,手续费总金额,订单总金额,申请退款总金额\n\
`1,`100.00,`100.00,`0.00,`0.01,`100.00,`100.00";

    let result = impl_utils::parse_bill_result(bill_text, "RECHARGE_REFUND").expect("解析成功");
    assert_eq!(result.bill_info_list.len(), 1);
    assert_eq!(
        result.bill_info_list[0].refund_id.as_deref(),
        Some("refund001")
    );
}

/// 未知账单类型 → None（对应 Java default 分支）。
/// 对应 Java: fromRawBillResultString → 未知 billType → null
#[test]
fn test_parse_bill_unknown_type_returns_none() {
    assert!(impl_utils::parse_bill_result("any", "UNKNOWN").is_none());
}

/// 空账单文本 → 空列表。
/// 对应 Java: fromRawBillResultString → 空内容
#[test]
fn test_parse_bill_empty_text() {
    let result = impl_utils::parse_bill_result("", "ALL").expect("解析成功");
    assert!(result.bill_info_list.is_empty());
}

// ═══════════════════════════════════════════════════════════════════
// parse_fund_flow_result（SOURCE_PARITY: BaseWxPayServiceImpl.handleFundFlow）
// ═══════════════════════════════════════════════════════════════════

/// 资金账单解析：明细段 + 汇总段。
/// 对应 Java: handleFundFlow → 明细段 ` 分割 + 汇总段
#[test]
fn test_parse_fund_flow_with_detail_and_summary() {
    let text = "资金流水明细\n\
账单日期,业务编号,资金流水ID,业务名称,业务类型,资金类型,收支金额,账户结余,资金变更提交方,备注,业务凭证号\n\
`2024-01-01 10:00:00,`BIZ001,`FL001,`测试,`退款,`支出,`100.00,`500.00,`系统,`备注,`V001\n\
资金流水总笔数,收入笔数,收入金额,支出笔数,支出金额\n\
`1,`0,`0.00,`1,`100.00";

    let result = impl_utils::parse_fund_flow_result(text);
    assert_eq!(result.total_record.as_deref(), Some("1"));
    assert_eq!(result.expenditure_record.as_deref(), Some("1"));
    assert_eq!(result.expenditure_amount.as_deref(), Some("100.00"));
    assert_eq!(result.wx_pay_fund_flow_base_result_list.len(), 1);
    assert_eq!(
        result.wx_pay_fund_flow_base_result_list[0]
            .fund_flow_id
            .as_deref(),
        Some("FL001")
    );
}

/// 空资金账单 → 默认结构。
/// 对应 Java: handleFundFlow → 空内容
#[test]
fn test_parse_fund_flow_empty() {
    let result = impl_utils::parse_fund_flow_result("");
    assert!(result.wx_pay_fund_flow_base_result_list.is_empty());
    assert!(result.total_record.is_none());
}

// ═══════════════════════════════════════════════════════════════════
// gunzip_to_text（SOURCE_PARITY: ZipUtils.unGzip）
// ═══════════════════════════════════════════════════════════════════

/// GZIP 解压 roundtrip。
/// 对应 Java: ZipUtils.unGzip → Files.readAllLines → join("\n")
#[test]
fn test_gunzip_roundtrip() {
    let original = "line1\nline2\nline3";
    let compressed = {
        use std::io::Write;
        let mut encoder = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::default());
        encoder.write_all(original.as_bytes()).expect("写入");
        encoder.finish().expect("完成")
    };

    let result = impl_utils::gunzip_to_text(&compressed).expect("解压成功");
    assert_eq!(result, original);
}

/// 无效 GZIP 数据 → 报错。
/// 对应 Java: ZipUtils.unGzip → IOException
#[test]
fn test_gunzip_invalid_data() {
    let err = impl_utils::gunzip_to_text(b"not gzip data").expect_err("应报错");
    assert!(
        err.to_string().contains("解压对账单文件时出错"),
        "错误信息: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════
// guess_file_content_type（SOURCE_PARITY: URLConnection.guessContentTypeFromName）
// ═══════════════════════════════════════════════════════════════════

/// 各扩展名 → 对应 Content-Type。
/// 对应 Java: URLConnection.guessContentTypeFromName
#[test]
fn test_guess_file_content_type_all_extensions() {
    assert_eq!(
        impl_utils::guess_file_content_type("photo.png"),
        "image/png"
    );
    assert_eq!(
        impl_utils::guess_file_content_type("photo.jpg"),
        "image/jpeg"
    );
    assert_eq!(
        impl_utils::guess_file_content_type("photo.jpeg"),
        "image/jpeg"
    );
    assert_eq!(
        impl_utils::guess_file_content_type("photo.gif"),
        "image/gif"
    );
    assert_eq!(
        impl_utils::guess_file_content_type("photo.bmp"),
        "image/bmp"
    );
    assert_eq!(
        impl_utils::guess_file_content_type("photo.webp"),
        "image/webp"
    );
    assert_eq!(
        impl_utils::guess_file_content_type("video.mp4"),
        "video/mp4"
    );
    assert_eq!(
        impl_utils::guess_file_content_type("video.mov"),
        "video/quicktime"
    );
    assert_eq!(
        impl_utils::guess_file_content_type("video.avi"),
        "video/x-msvideo"
    );
    // 未知扩展名
    assert_eq!(
        impl_utils::guess_file_content_type("file.xyz"),
        "application/octet-stream"
    );
    // 无扩展名
    assert_eq!(
        impl_utils::guess_file_content_type("noext"),
        "application/octet-stream"
    );
    // 大小写不敏感
    assert_eq!(
        impl_utils::guess_file_content_type("photo.PNG"),
        "image/png"
    );
}

// ═══════════════════════════════════════════════════════════════════
// build_multipart_meta_file（SOURCE_PARITY: WechatPayUploadHttpPost.Builder）
// ═══════════════════════════════════════════════════════════════════

/// multipart 构造：包含 meta JSON 段和 file 段。
/// 对应 Java: WechatPayUploadHttpPost.Builder → addBinaryBody + addTextBody
#[test]
fn test_build_multipart_meta_file() {
    let meta = r#"{"filename":"test.png"}"#;
    let file_data = b"binary content here";
    let (content_type, body) =
        impl_utils::build_multipart_meta_file("test.png", "image/png", file_data, meta);

    assert!(content_type.starts_with("multipart/form-data; boundary="));
    let body_str = String::from_utf8_lossy(&body);
    assert!(body_str.contains("Content-Disposition: form-data; name=\"meta\""));
    assert!(body_str.contains("Content-Type: application/json"));
    assert!(body_str.contains(meta));
    assert!(
        body_str.contains("Content-Disposition: form-data; name=\"file\"; filename=\"test.png\"")
    );
    assert!(body_str.contains("Content-Type: image/png"));
    // 二进制内容原样嵌入
    assert!(body.windows(file_data.len()).any(|w| w == file_data));
}

// ═══════════════════════════════════════════════════════════════════
// validate_v3_response（SOURCE_PARITY: WxPayValidator.validate）
// ═══════════════════════════════════════════════════════════════════

/// 非 JSON 响应 → 跳过验签（对应 Java Content-Type 非 JSON → true）。
/// 对应 Java: validate → !isJson → return true
#[test]
fn test_validate_v3_non_json_passes() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_public_key_id("PUB_KEY_ID")
        .set_public_key_content(b"dummy".to_vec());
    impl_utils::validate_v3_response(&config, Some("text/plain"), &[], "not json")
        .expect("非 JSON 响应应通过");
}

/// 缺少签名头 → 报错（对应 Java 四头任一缺失 → false）。
/// 对应 Java: validate → headers missing → return false
#[test]
fn test_validate_v3_missing_headers() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_public_key_id("PUB_KEY_ID")
        .set_public_key_content(b"dummy".to_vec());

    let err =
        impl_utils::validate_v3_response(&config, Some("application/json"), &[], r#"{"ok":true}"#)
            .expect_err("应报错");
    assert!(err.to_string().contains("签名验证失败"), "错误信息: {err}");
}

/// Content-Type 含 charset 参数时仍识别为 JSON。
/// 对应 Java: mime = ct.split(';')[0].trim()
#[test]
fn test_validate_v3_json_with_charset() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_public_key_id("PUB_KEY_ID")
        .set_public_key_content(b"dummy".to_vec());

    let err = impl_utils::validate_v3_response(
        &config,
        Some("application/json; charset=utf-8"),
        &[],
        r#"{"ok":true}"#,
    )
    .expect_err("应报错");
    assert!(err.to_string().contains("签名验证失败"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════
// wechatpay_serial_header（SOURCE_PARITY: getWechatPaySerial）
// ═══════════════════════════════════════════════════════════════════

/// publicKeyId 已设置 → 返回该值。
/// 对应 Java: getWechatPaySerial → publicKeyId 优先
#[test]
fn test_wechatpay_serial_header_with_public_key_id() {
    let mut config = WxPayDefaultConfig::new();
    config.set_public_key_id("PUB_KEY_ID_TEST");
    assert_eq!(
        impl_utils::wechatpay_serial_header(&config),
        "PUB_KEY_ID_TEST"
    );
}

/// 未设置 publicKeyId → 空串。
/// 对应 Java: getWechatPaySerial → 无 publicKeyId → 空
#[test]
fn test_wechatpay_serial_header_empty() {
    let config = WxPayDefaultConfig::new();
    assert_eq!(impl_utils::wechatpay_serial_header(&config), "");
}

// ═══════════════════════════════════════════════════════════════════
// load_merchant_private_key（SOURCE_PARITY: initApiV3HttpClient 私钥加载）
// ═══════════════════════════════════════════════════════════════════

/// 未配置任何私钥 → 报错。
/// 对应 Java: initApiV3HttpClient → 无 privateKey → exception
#[test]
fn test_load_merchant_private_key_none() {
    let config = WxPayDefaultConfig::new();
    let err = impl_utils::load_merchant_private_key(&config).expect_err("应报错");
    assert!(
        err.to_string().contains("请确保私钥配置"),
        "错误信息: {err}"
    );
}

/// private_key_string（base64 PEM）→ 加载成功。
/// 对应 Java: initApiV3HttpClient → privateKeyString base64 decode
#[test]
fn test_load_merchant_private_key_from_base64() {
    let pem = "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCaZzehUwzcxdLg\nGn/UNryHLdX0yBvCqD0p92/BdlCIBi0dmzZzrfc+FF0xK70AP9b2+Ry5q+zXUU+d\nPucJmgwABiZ+Lte+4euMxqPCEkdEu9lyiphalpoaOVPbmDNatzq9k5a/P454QRWM\ndkLUJZCeoL9bF2Gn/2+wWEw3sL8zFFcOM8Jr1PdOLmAx+h7pf/87jcmXXCm+SZqw\n5MtILKQi9zHWujYdMA0IcYNeQaNl1h/NUnungdIHKaaU+17wCXqTcZsipAGoqfqr\nHx/sr30ZszOdHYOvFNiB+rhEldBGSLWwSYE6LFkbP9GdQWKIQCip3E5dLj5ZFkDe\nOv4Hekf9AgMBAAECggEAEsVsqnS90hNMzUj7dHHJHsgQRGeVlGc+tFzsHcGEDd1u\nW7SUfKDQN6BjKgiuvBqGyFTFzL7dltnAS5YroWu0fMZCpMGOIhs2N1Go8/2j43PQ\n/k9iMVUw2/JPQxmwWJ2BCy4nvA1+hRkohQCVpFQCzn4tdWYUzcdMrUw2y+h1fkCQ\n5MJn7iw9QHKQSeFeCl1/xq2PvOtiK/r1LsckyKNSSNgFEfxyWYaKbnK9OH+5rFKQ\nQuI+fnAgE6QiLvmW0NqqZUSfqkLKi/FSWI13ns0H6OxjqpLX8VQ6+Cw5qq8fCuv2\ngzkVk8A85ZTCQL/q9qDilt9uAE0bE924WU+n2zkBoQKBgQDQvIufN6fKpm27k4yx\nRNV23fj9nojewaVqGg/3yuyiAu6w/yFcTXkGMVOicTYraX2mliHTIoyP8ywKGqqa\nXS/Kk3tGD1K04KriPiFwWXU54+DmOJEyYoJlmXOm4BoZ1lW0z5HECC9eO/VPSDY+\nzQdRYSCTdSHEgYuOGQSLPPwdrQKBgQC9XTrvkUkIthayc+4IhV6m4kT8uwzuoc3f\nuaJhFFcpLKqzcpQBH71TYCXfqkucnO0no0sGerBB4HJQoVRK+jsdhNZhcw1JKLau\nE+YlCSRLZ62vyzBTzLw1fnFBp82z1VZBujCrMP+DdwXBTsnkRaUtmDEG0s6YsCwd\nfayF0PB9kQKBgAHc/P4R2ByV+brH6WSXsbQa7SMObDhY0CovS18x34Tes9S+okSZ\nqG/mttFnY01l5qo7AthIoaqTSBxa+pTgKhIL2PjaICnfK4dTeKbxFXvLzfEgJiOl\n/3X6ta6Sp4j9gcxYYfu2+v1DWcA4a8uJtvwB+vF2BTQk1+MP1BuOEs4NAoGBAK3y\n+HKdOUPBUPQ4vk4hhaMzcz/d67FB/UYo1lrrPm3aVCxnckHeECKIzgG6A58oIEor\nHH4lMcgyD5C1wiLl3mvtXKlD8M5lkfoy2VToIukJomk783bnOXTCY/N12+X4cTYL\nfS2k4vK24RiD8b25pFRP26ly+MkV/FBS46pBFsmhAoGAOjfl2vGyJo8CRQ/HBLsS\nBw2VQgRvZU7mom1qa2SKA2VlsFz/aiBCcT8XEsTJxkVYkvbdfbyx2Z9kCbI6Y2cz\nZ+M/0rQSRv/eOiBW7anBImZMg6WcYNfRfumkuNq+6fcCKKtZuyrd7ZFc0jTcVasI\nxNJ1TM6J6kRQdn5O6Ot4ERw=\n-----END PRIVATE KEY-----";
    let b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, pem.as_bytes());
    let mut config = WxPayDefaultConfig::new();
    config.set_private_key_string(&b64);
    impl_utils::load_merchant_private_key(&config).expect("base64 PEM 加载成功");
}

// ═══════════════════════════════════════════════════════════════════
// v3_context（SOURCE_PARITY: initApiV3HttpClient 上下文构建）
// ═══════════════════════════════════════════════════════════════════

/// api_v3_key 未设置 → 报错。
/// 对应 Java: initApiV3HttpClient → apiV3Key 为空 → exception
#[test]
fn test_v3_context_no_api_v3_key() {
    let config = WxPayDefaultConfig::new();
    if let Err(err) = impl_utils::v3_context(&config) {
        assert!(
            err.to_string().contains("请确保apiV3Key值已设置"),
            "错误信息: {err}"
        );
    } else {
        panic!("应报错");
    }
}

/// cert_serial_no 未设置且无私钥证书 → 报错。
/// 对应 Java: initApiV3HttpClient → certSerialNo 为空 → exception
#[test]
fn test_v3_context_no_cert_serial() {
    let pem = "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQCaZzehUwzcxdLg\nGn/UNryHLdX0yBvCqD0p92/BdlCIBi0dmzZzrfc+FF0xK70AP9b2+Ry5q+zXUU+d\nPucJmgwABiZ+Lte+4euMxqPCEkdEu9lyiphalpoaOVPbmDNatzq9k5a/P454QRWM\ndkLUJZCeoL9bF2Gn/2+wWEw3sL8zFFcOM8Jr1PdOLmAx+h7pf/87jcmXXCm+SZqw\n5MtILKQi9zHWujYdMA0IcYNeQaNl1h/NUnungdIHKaaU+17wCXqTcZsipAGoqfqr\nHx/sr30ZszOdHYOvFNiB+rhEldBGSLWwSYE6LFkbP9GdQWKIQCip3E5dLj5ZFkDe\nOv4Hekf9AgMBAAECggEAEsVsqnS90hNMzUj7dHHJHsgQRGeVlGc+tFzsHcGEDd1u\nW7SUfKDQN6BjKgiuvBqGyFTFzL7dltnAS5YroWu0fMZCpMGOIhs2N1Go8/2j43PQ\n/k9iMVUw2/JPQxmwWJ2BCy4nvA1+hRkohQCVpFQCzn4tdWYUzcdMrUw2y+h1fkCQ\n5MJn7iw9QHKQSeFeCl1/xq2PvOtiK/r1LsckyKNSSNgFEfxyWYaKbnK9OH+5rFKQ\nQuI+fnAgE6QiLvmW0NqqZUSfqkLKi/FSWI13ns0H6OxjqpLX8VQ6+Cw5qq8fCuv2\ngzkVk8A85ZTCQL/q9qDilt9uAE0bE924WU+n2zkBoQKBgQDQvIufN6fKpm27k4yx\nRNV23fj9nojewaVqGg/3yuyiAu6w/yFcTXkGMVOicTYraX2mliHTIoyP8ywKGqqa\nXS/Kk3tGD1K04KriPiFwWXU54+DmOJEyYoJlmXOm4BoZ1lW0z5HECC9eO/VPSDY+\nzQdRYSCTdSHEgYuOGQSLPPwdrQKBgQC9XTrvkUkIthayc+4IhV6m4kT8uwzuoc3f\nuaJhFFcpLKqzcpQBH71TYCXfqkucnO0no0sGerBB4HJQoVRK+jsdhNZhcw1JKLau\nE+YlCSRLZ62vyzBTzLw1fnFBp82z1VZBujCrMP+DdwXBTsnkRaUtmDEG0s6YsCwd\nfayF0PB9kQKBgAHc/P4R2ByV+brH6WSXsbQa7SMObDhY0CovS18x34Tes9S+okSZ\nqG/mttFnY01l5qo7AthIoaqTSBxa+pTgKhIL2PjaICnfK4dTeKbxFXvLzfEgJiOl\n/3X6ta6Sp4j9gcxYYfu2+v1DWcA4a8uJtvwB+vF2BTQk1+MP1BuOEs4NAoGBAK3y\n+HKdOUPBUPQ4vk4hhaMzcz/d67FB/UYo1lrrPm3aVCxnckHeECKIzgG6A58oIEor\nHH4lMcgyD5C1wiLl3mvtXKlD8M5lkfoy2VToIukJomk783bnOXTCY/N12+X4cTYL\nfS2k4vK24RiD8b25pFRP26ly+MkV/FBS46pBFsmhAoGAOjfl2vGyJo8CRQ/HBLsS\nBw2VQgRvZU7mom1qa2SKA2VlsFz/aiBCcT8XEsTJxkVYkvbdfbyx2Z9kCbI6Y2cz\nZ+M/0rQSRv/eOiBW7anBImZMg6WcYNfRfumkuNq+6fcCKKtZuyrd7ZFc0jTcVasI\nxNJ1TM6J6kRQdn5O6Ot4ERw=\n-----END PRIVATE KEY-----";
    let mut config = WxPayDefaultConfig::new();
    config
        .set_api_v3_key("a7cde1ZJB1kG2e7VfTs3jQzaWizur8Gb")
        .set_mch_id("10000100")
        .set_private_key(pem);
    // 未设置 cert_serial_no → 应报错（因为也无私钥证书）
    if let Err(err) = impl_utils::v3_context(&config) {
        assert!(err.to_string().contains("certSerialNo"), "错误信息: {err}");
    } else {
        panic!("应报错");
    }
}

// ═══════════════════════════════════════════════════════════════════
// platform_public_key（SOURCE_PARITY: VerifierBuilder 公钥加载）
// ═══════════════════════════════════════════════════════════════════

/// 非完全公钥模式 → 报错（证书模式未就绪）。
/// 对应 Java: fullPublicKeyModel=false → AutoUpdateCertificatesVerifier
#[test]
fn test_platform_public_key_non_full_model() {
    let mut config = WxPayDefaultConfig::new();
    config.set_full_public_key_model(false);
    let err = impl_utils::platform_public_key(&config).expect_err("应报错");
    assert!(
        err.to_string().contains("平台证书模式未就绪"),
        "错误信息: {err}"
    );
}

/// 完全公钥模式但未设置 publicKeyId → 报错。
/// 对应 Java: fullPublicKeyModel=true → publicKeyId 为空 → exception
#[test]
fn test_platform_public_key_no_public_key_id() {
    let mut config = WxPayDefaultConfig::new();
    config.set_full_public_key_model(true);
    let err = impl_utils::platform_public_key(&config).expect_err("应报错");
    assert!(
        err.to_string().contains("publicKeyId配套使用"),
        "错误信息: {err}"
    );
}

/// 完全公钥模式但未配置任何公钥内容 → 报错。
/// 对应 Java: fullPublicKeyModel=true → publicKeyId+publicKey 都无 → exception
#[test]
fn test_platform_public_key_no_key_material() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_full_public_key_model(true)
        .set_public_key_id("PUB_KEY_ID");
    let err = impl_utils::platform_public_key(&config).expect_err("应报错");
    assert!(err.to_string().contains("公钥配置"), "错误信息: {err}");
}

// ═══════════════════════════════════════════════════════════════════
// load_p12_bytes（SOURCE_PARITY: loadConfigInputStream 三路优先）
// ═══════════════════════════════════════════════════════════════════

/// key_content 优先级：keyContent 已设置 → 返回。
/// 对应 Java: loadConfigInputStream → keyContent 优先
#[test]
fn test_load_p12_bytes_from_content() {
    let mut config = WxPayDefaultConfig::new();
    config.set_key_content(vec![0x30, 0x82, 0x01]);
    let result = impl_utils::load_p12_bytes(&config).expect("加载成功");
    assert_eq!(result, Some(vec![0x30, 0x82, 0x01]));
}

/// 全部未设置 → None。
/// 对应 Java: loadConfigInputStream → 三路为空 → null
#[test]
fn test_load_p12_bytes_none() {
    let config = WxPayDefaultConfig::new();
    let result = impl_utils::load_p12_bytes(&config).expect("加载成功");
    assert!(result.is_none());
}

/// key_string base64 解码失败 → 报错。
/// 对应 Java: loadConfigInputStream → keyString base64 decode fail
#[test]
fn test_load_p12_bytes_invalid_base64() {
    let mut config = WxPayDefaultConfig::new();
    config.set_key_string("not-valid-base64!!!");
    let err = impl_utils::load_p12_bytes(&config).expect_err("应报错");
    assert!(
        err.to_string().contains("base64 解码失败"),
        "错误信息: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════
// encrypt_spec_fields_json（SOURCE_PARITY: RsaCryptoUtil.encryptFields）
// ═══════════════════════════════════════════════════════════════════

/// RSA-OAEP 加密指定 JSON 路径字段。
/// 对应 Java: RsaCryptoUtil.encryptFields → @SpecEncrypt 字段加密
#[test]
fn test_encrypt_spec_fields_json_basic() {
    // 生成测试 RSA 公钥（2048 位太慢，用 1024 位仅测试路径逻辑）
    let mut rng = rand_core::OsRng;
    let private_key = rsa::RsaPrivateKey::new(&mut rng, 1024).expect("生成私钥");
    let public_key = rsa::RsaPublicKey::from(&private_key);

    let mut value = serde_json::json!({
        "name": "张三",
        "id_card": "110101199001011234",
        "nested": {
            "bank_account": "6222021234567890123"
        }
    });

    impl_utils::encrypt_spec_fields_json(
        &mut value,
        &public_key,
        &["name", "id_card", "nested.bank_account"],
    )
    .expect("加密成功");

    // 加密后的字段应为 Base64 字符串（不再是原值）
    assert_ne!(value["name"].as_str().unwrap(), "张三");
    assert_ne!(value["id_card"].as_str().unwrap(), "110101199001011234");
    assert_ne!(
        value["nested"]["bank_account"].as_str().unwrap(),
        "6222021234567890123"
    );
    // 验证 Base64 格式（能解码）
    let _ = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        value["name"].as_str().unwrap(),
    )
    .expect("Base64 格式");
}

/// 空字段跳过加密（对应 Java oldStr.trim().isEmpty() 跳过）。
/// 对应 Java: RsaCryptoUtil.encryptFields → 空串不加密
#[test]
fn test_encrypt_spec_fields_json_skip_empty() {
    let mut rng = rand_core::OsRng;
    let private_key = rsa::RsaPrivateKey::new(&mut rng, 1024).expect("生成私钥");
    let public_key = rsa::RsaPublicKey::from(&private_key);

    let mut value = serde_json::json!({
        "name": "",
        "id_card": "  ",
    });

    impl_utils::encrypt_spec_fields_json(&mut value, &public_key, &["name", "id_card"])
        .expect("空字段跳过");
    // 空字段保持原样
    assert_eq!(value["name"].as_str().unwrap(), "");
    assert_eq!(value["id_card"].as_str().unwrap(), "  ");
}

/// 不存在的路径字段 → 跳过（不报错）。
/// 对应 Java: encryptJsonPath → get null → skip
#[test]
fn test_encrypt_spec_fields_json_missing_path() {
    let mut rng = rand_core::OsRng;
    let private_key = rsa::RsaPrivateKey::new(&mut rng, 1024).expect("生成私钥");
    let public_key = rsa::RsaPublicKey::from(&private_key);

    let mut value = serde_json::json!({"name": "张三"});

    impl_utils::encrypt_spec_fields_json(&mut value, &public_key, &["nonexistent.path"])
        .expect("缺失路径跳过");
    assert_eq!(value["name"].as_str().unwrap(), "张三");
}

/// 数组通配符路径 `*` 逐元素加密。
/// 对应 Java: RsaCryptoUtil.encryptFields → Collection 递归
#[test]
fn test_encrypt_spec_fields_json_array_wildcard() {
    let mut rng = rand_core::OsRng;
    let private_key = rsa::RsaPrivateKey::new(&mut rng, 1024).expect("生成私钥");
    let public_key = rsa::RsaPublicKey::from(&private_key);

    let mut value = serde_json::json!({
        "items": [
            {"secret": "aaa"},
            {"secret": "bbb"},
        ]
    });

    impl_utils::encrypt_spec_fields_json(&mut value, &public_key, &["items.*.secret"])
        .expect("数组加密成功");
    assert_ne!(value["items"][0]["secret"].as_str().unwrap(), "aaa");
    assert_ne!(value["items"][1]["secret"].as_str().unwrap(), "bbb");
}

// ═══════════════════════════════════════════════════════════════════
// V2Request trait + FundFlowBillRequest（RUST_OBLIGATION: 自定义请求 bean）
// ═══════════════════════════════════════════════════════════════════

/// FundFlowBillRequest V2Request trait 实现完整覆盖。
/// 对应 Java: WxPayDownloadFundFlowRequest（全字段）
#[test]
fn test_fund_flow_bill_request_v2_request() {
    let mut req = FundFlowBillRequest::default();
    req.appid = Some("wx1234".to_string());
    req.mch_id = Some("10000100".to_string());
    req.bill_date = Some("20240101".to_string());
    req.account_type = Some("Basic".to_string());

    // V2Request trait 方法
    assert_eq!(V2Request::appid(&req), Some("wx1234"));
    assert_eq!(V2Request::mch_id(&req), Some("10000100"));

    V2Request::set_appid(&mut req, Some("wx5678".to_string()));
    assert_eq!(V2Request::appid(&req), Some("wx5678"));

    V2Request::set_nonce_str(&mut req, Some("nonce123".to_string()));
    assert_eq!(V2Request::nonce_str(&req), Some("nonce123"));

    V2Request::set_sign(&mut req, Some("sign123".to_string()));

    let xml = V2Request::to_xml(&req).expect("序列化成功");
    assert!(xml.contains("wx5678"));
    assert!(xml.contains("20240101"));
    assert!(xml.contains("Basic"));

    // ignore_appid 默认 false
    assert!(!V2Request::ignore_appid(&req));
    assert!(V2Request::need_nonce_str(&req));
}

/// check_and_sign 生成的 XML 签名自洽（重算一致）。
/// 对应 Java: checkAndSign → sign = createSign → XML 含 sign
#[test]
fn test_check_and_sign_xml_sign_self_consistent() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY);
    let mut request = WxPayRefundQueryRequest::default();
    request.out_refund_no = Some("R001".to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");

    let xml = request.to_xml().expect("XML 序列化");
    let map = root_children_map(&xml).expect("XML 解析");
    let expected_sign = SignUtils::create_sign(&map, None, MCH_KEY, &[]).expect("重算签名");
    assert_eq!(
        map.get("sign").map(String::as_str),
        Some(expected_sign.as_str())
    );
}

/// check_and_sign 生成的 HMAC-SHA256 签名自洽。
/// 对应 Java: checkAndSign + HMAC-SHA256 sign type
#[test]
fn test_check_and_sign_hmac_sha256_self_consistent() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY)
        .set_sign_type("HMAC-SHA256");
    let mut request = WxPayDownloadBillRequest::default();
    request.bill_date = Some("20240101".to_string());
    request.bill_type = Some("ALL".to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");
    assert_eq!(request.sign_type.as_deref(), Some("HMAC-SHA256"));

    let xml = request.to_xml().expect("XML");
    let map = root_children_map(&xml).expect("XML 解析");
    let expected =
        SignUtils::create_sign(&map, Some("HMAC-SHA256"), MCH_KEY, &[]).expect("重算 HMAC 签名");
    assert_eq!(map.get("sign").map(String::as_str), Some(expected.as_str()));
}

/// v2 查询订单请求 check_and_sign + XML roundtrip。
/// 对应 Java: queryOrder → checkAndSign → post
#[test]
fn test_query_order_request_check_and_sign() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY);
    let mut request = WxPayOrderQueryRequest::default();
    request.transaction_id = Some("4001312001201707262674894706".to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");
    let xml = request.to_xml().expect("XML");
    assert!(xml.contains("4001312001201707262674894706"));
    assert!(xml.contains("<sign>"));
}

/// v2 扫码支付请求 check_and_sign + XML roundtrip。
/// 对应 Java: micropay → checkAndSign → post
#[test]
fn test_micropay_request_check_and_sign() {
    let mut config = WxPayDefaultConfig::new();
    config
        .set_app_id(APP_ID)
        .set_mch_id(MCH_ID)
        .set_mch_key(MCH_KEY);
    let mut request = WxPayMicropayRequest::default();
    request.body = Some("测试商品".to_string());
    request.out_trade_no = Some("order001".to_string());
    request.total_fee = Some(100);
    request.spbill_create_ip = Some("127.0.0.1".to_string());
    request.auth_code = Some("134567890123456789".to_string());

    impl_utils::check_and_sign(&config, &mut request).expect("签名成功");
    let xml = request.to_xml().expect("XML");
    assert!(xml.contains("测试商品"), "body: {xml}");
    assert!(xml.contains("100"), "total_fee: {xml}");
    assert!(xml.contains("auth_code"), "auth_code: {xml}");
}

// ═══════════════════════════════════════════════════════════════════
// decrypt_refund_req_info 边界（RUST_OBLIGATION: 错误路径）
// ═══════════════════════════════════════════════════════════════════

/// 无效 Base64 → 报错。
/// 对应 Java: decryptReqInfo → Base64 decode fail
#[test]
fn test_decrypt_refund_req_info_invalid_base64() {
    let err =
        impl_utils::decrypt_refund_req_info(MCH_KEY, "not-valid-base64!!!").expect_err("应报错");
    assert!(
        err.to_string().contains("解密退款通知加密信息时出错"),
        "错误信息: {err}"
    );
}

/// AES 密文长度不足 → 报错。
/// 对应 Java: decryptReqInfo → AES decrypt fail
#[test]
fn test_decrypt_refund_req_info_short_ciphertext() {
    // 有效 Base64 但不够 16 字节 AES 块
    let short = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, b"short");
    let err = impl_utils::decrypt_refund_req_info(MCH_KEY, &short).expect_err("应报错");
    assert!(
        err.to_string().contains("解密退款通知加密信息时出错"),
        "错误信息: {err}"
    );
}

// ═══════════════════════════════════════════════════════════════════
// canonical_url_strip_prefix / normalize_strip_prefix / strip_path_prefix
// （SOURCE_PARITY: WxPayCredentials.buildMessage URL 规范化）
// ═══════════════════════════════════════════════════════════════════

/// 无前缀裁剪 → 返回原始路径。
/// 对应 Java: buildMessage → stripPrefix=null → rawPath
#[test]
fn test_canonical_url_no_strip() {
    let result = impl_utils::canonical_url_strip_prefix(
        "https://api.mch.weixin.qq.com/v3/pay/transactions/jsapi",
        None,
    )
    .expect("规范化成功");
    assert_eq!(result, "/v3/pay/transactions/jsapi");
}

/// 带前缀裁剪。
/// 对应 Java: buildMessage → stripPathPrefix(path, prefix)
#[test]
fn test_canonical_url_with_strip() {
    let result = impl_utils::canonical_url_strip_prefix(
        "https://api.mch.weixin.qq.com/v3/pay/transactions/jsapi",
        Some("/v3"),
    )
    .expect("规范化成功");
    assert_eq!(result, "/pay/transactions/jsapi");
}

/// URL 含 query 参数时保留。
/// 对应 Java: buildMessage → query 保留
#[test]
fn test_canonical_url_with_query() {
    let result = impl_utils::canonical_url_strip_prefix(
        "https://api.mch.weixin.qq.com/v3/pay/transactions/out-trade-no/out123?mchid=10000100",
        None,
    )
    .expect("规范化成功");
    assert_eq!(
        result,
        "/v3/pay/transactions/out-trade-no/out123?mchid=10000100"
    );
}

/// 前缀裁剪后路径为空 → 返回 "/"。
/// 对应 Java: stripPathPrefix → stripped 为空 → "/"
#[test]
fn test_canonical_url_strip_to_root() {
    let result =
        impl_utils::canonical_url_strip_prefix("https://api.mch.weixin.qq.com/v3", Some("/v3"))
            .expect("规范化成功");
    assert_eq!(result, "/");
}
