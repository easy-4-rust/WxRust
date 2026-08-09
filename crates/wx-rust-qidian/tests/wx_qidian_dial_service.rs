//! 话务与通话数据服务测试（镜像 Java `WxQidianDialServiceImplTest`，
//! 经 MockServer 验证）。
//!
//! Java 测试经 Guice 注入真实配置，按「拉 IVR 列表 → 拉总机号 → 用总机号
//! 外呼」串起三个接口；本文件以 MockServer 镜像同一流程与断言（errcode/
//! code 为 0、node 大小、ivr_id 提取、switchBoards 大小、callid 返回）。

mod common;

use common::{MockServer, dispatch, json, service_with_host};

use wx_rust_qidian::api::WxQidianService;
use wx_rust_qidian::bean::dial::IVRDialRequest;

/// 镜像 `WxQidianDialServiceImplTest.dial` 的三段流程。
#[tokio::test]
async fn test_dial_flow() {
    let server = MockServer::start(dispatch(|path| {
        if path.contains("/cgi-bin/call/dial/getivrlist") {
            json(
                r#"{"errcode":0,"errmsg":"ok","node":[{"ivr_id":"433","ivr_name":"自动接听需求测试"},{"ivr_id":"1","ivr_name":"default"}]}"#,
            )
        } else if path.contains("/cgi-bin/call/callData/getswitchboardlist") {
            json(
                r#"{"errcode":0,"errmsg":"ok","data":{"records":[{"switchboard":"01012345678","create_time":"2020-01-01","callin_status":true,"callout_status":true,"sp_name":"电信","city_name":"北京"},{"switchboard":"01087654321","create_time":"2020-01-02","callin_status":true,"callout_status":false,"sp_name":"联通","city_name":"上海"}]}}"#,
            )
        } else if path.contains("/cgi-bin/call/dial/ivrdial") {
            json(r#"{"code":0,"msg":"ok","callid":"call-001"}"#)
        } else {
            json("{}")
        }
    }))
    .await;
    let service = service_with_host(&server.url(""));

    // 1. 拉 IVR 列表（对应 Java：errcode==0、按名称过滤、提取 ivr_id）
    let ivr_list = service
        .dial_service()
        .expect("话务服务存在")
        .get_ivr_list()
        .await
        .expect("IVR 列表成功");
    assert_eq!(ivr_list.base.errcode, 0);
    assert_eq!(ivr_list.node.as_ref().map(|n| n.len()), Some(2), "ivr 数量");
    let ivr = ivr_list
        .node
        .as_ref()
        .expect("node 非空")
        .iter()
        .find(|o| o.ivr_name.as_deref() == Some("自动接听需求测试"))
        .expect("按名称找到 IVR");
    let ivr_id = ivr.ivr_id.as_deref().expect("ivr_id 非空");

    // 2. 拉总机号列表（对应 Java：errcode==0、switchBoards 大小）
    let switch_board_list = service
        .call_data_service()
        .expect("通话数据服务存在")
        .get_switch_board_list()
        .await
        .expect("总机号列表成功");
    assert_eq!(switch_board_list.base.errcode, 0);
    let switch_boards = switch_board_list
        .data
        .as_ref()
        .expect("data 非空")
        .switch_boards();
    assert_eq!(switch_boards.len(), 2, "总机号数量");

    // 3. IVR 外呼（对应 Java：code==0、返回 callid）
    let mut ivr_dial = IVRDialRequest::default();
    ivr_dial.phone_number = Some("18434399105".to_string());
    ivr_dial.ivr_id = Some(ivr_id.to_string());
    ivr_dial.corp_phone_list = Some(switch_boards.clone());
    let ivr_dial_response = service
        .dial_service()
        .expect("话务服务存在")
        .ivr_dial(&ivr_dial)
        .await
        .expect("外呼成功");
    assert_eq!(ivr_dial_response.base.code, 0);
    assert_eq!(ivr_dial_response.callid.as_deref(), Some("call-001"));

    // 请求体线格式（对应 Java `IVRDialRequest.toJson()`，Gson 字段名原样：
    // 下划线风格；null 字段省略；默认值 loc_pref_on=1、skip_restrict=false）
    let body = server.last_body();
    assert!(
        body.contains(r#""phone_number":"18434399105""#),
        "body: {body}"
    );
    assert!(body.contains(r#""ivr_id":"433""#), "body: {body}");
    assert!(
        body.contains(r#""corp_phone_list":["01012345678","01087654321"]"#),
        "body: {body}"
    );
    assert!(body.contains(r#""loc_pref_on":1"#), "body: {body}");
    assert!(body.contains(r#""skip_restrict":false"#), "body: {body}");
    // 未设置字段省略（对应 Gson 默认省略 null）
    assert!(!body.contains("backup_corp_phone_list"), "body: {body}");
}

/// 追加语义测试：IVRDialRequest 序列化默认值（无 HTTP）。
#[test]
fn test_ivr_dial_request_to_json_defaults() {
    let request = IVRDialRequest::default();
    let json = request.to_json();
    // 未设置字段省略，默认字段参与序列化
    assert!(!json.contains("phone_number"), "json: {json}");
    assert!(!json.contains("ivr_id"), "json: {json}");
    assert!(json.contains(r#""loc_pref_on":1"#), "json: {json}");
    assert!(json.contains(r#""skip_restrict":false"#), "json: {json}");
}
