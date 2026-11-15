use wx_rust_common::bean::result::WxMediaUploadResult;

#[test]
fn media_upload_result_uses_wechat_wire_names_and_java_bean_serialization_names() {
    let result: WxMediaUploadResult =
        serde_json::from_str(r#"{"type":"image","media_id":"MEDIA-001","created_at":1794614400}"#)
            .expect("snake_case WeChat response should deserialize");

    assert_eq!(result.r#type.as_deref(), Some("image"));
    assert_eq!(result.media_id.as_deref(), Some("MEDIA-001"));
    assert_eq!(result.thumb_media_id, None);
    assert_eq!(result.url, None);
    assert_eq!(result.created_at, 1_794_614_400);

    let serialized = serde_json::to_value(result).expect("result should serialize");
    assert_eq!(serialized["mediaId"], "MEDIA-001");
    assert_eq!(serialized["createdAt"], 1_794_614_400);
    assert!(serialized["thumbMediaId"].is_null());
    assert!(serialized["url"].is_null());
}

#[test]
fn media_upload_result_accepts_java_camel_case_and_preserves_null_fields() {
    let result: WxMediaUploadResult = serde_json::from_str(
        r#"{"url":null,"type":"image","mediaId":"MEDIA-002","thumbMediaId":null,"createdAt":1388561359}"#,
    )
    .expect("Java camelCase representation should deserialize");
    assert_eq!(result.media_id.as_deref(), Some("MEDIA-002"));
    assert_eq!(result.created_at, 1_388_561_359);
    assert!(result.url.is_none());
    assert!(result.thumb_media_id.is_none());
}
