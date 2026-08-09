//! jsapi 相关测试（镜像 Java `WxMpJsAPITest`）。
//!
//! `WxMpJsAPITest` 直接验证 `SHA1.genWithAmple` 的已知向量
//! （`c6f04b64d6351d197b71bd23fb7dd2d44c0db486`）；本文件除镜像该向量
//! 外，还经 MockServer 端到端验证 `createJsapiSignature`（ticket 接口 +
//! 签名生成）。

mod common;

use common::{MockServer, dispatch, json, service_with_host};

use wx_rust_qidian::api::WxQidianService;

/// 镜像 Java `WxMpJsAPITest.test`：`SHA1.genWithAmple` 已知向量。
#[test]
fn test_gen_with_ample_vector() {
    let timestamp = 1419835025_i64;
    let url = "http://omstest.vmall.com:23568/thirdparty/wechat/vcode/gotoshare?quantity=1&batchName=MATE7";
    let noncestr = "82693e11-b9bc-448e-892f-f5289f46cd0f";
    let jsapi_ticket =
        "bxLdikRXVbTPdHSM05e5u4RbEYQn7pNQMPrfzl8lJNb1foLDa3HIwI3BRMkQmSO_5F64VFa75uURcq6Uz7QHgA";
    let result = wx_rust_common::util::crypto::Sha1::digest_with_amp(&[
        &format!("jsapi_ticket={jsapi_ticket}"),
        &format!("noncestr={noncestr}"),
        &format!("timestamp={timestamp}"),
        &format!("url={url}"),
    ])
    .expect("签名成功");
    assert_eq!(result, "c6f04b64d6351d197b71bd23fb7dd2d44c0db486");
}

/// 端到端：createJsapiSignature（ticket 经 MockServer 获取）。
#[tokio::test]
async fn test_create_jsapi_signature() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/ticket/getticket") {
            json(r#"{"errcode":0,"errmsg":"ok","ticket":"TICKET_1","expires_in":7200}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));

    let signature = service
        .create_jsapi_signature("http://www.baidu.com")
        .await
        .expect("签名成功");
    assert_eq!(signature.app_id, "wxqidian_default");
    assert_eq!(signature.url, "http://www.baidu.com");
    assert_eq!(signature.nonce_str.len(), 16);
    assert!(!signature.signature.is_empty(), "签名非空");
    // 签名格式校验：sha1 为 40 位十六进制
    assert_eq!(signature.signature.len(), 40);

    // ticket 已缓存：再次签名不再请求 ticket 接口
    let _ = service
        .create_jsapi_signature("http://www.baidu.com")
        .await
        .expect("第二次签名成功");
    assert_eq!(
        server.path_hits("/cgi-bin/ticket/getticket"),
        1,
        "ticket 只请求一次"
    );
}
