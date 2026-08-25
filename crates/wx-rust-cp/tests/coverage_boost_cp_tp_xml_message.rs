//! Coverage boost: `wx_cp_tp_xml_message.rs` (140 lines, 0%).
//!
//! Exercises `WxCpTpXmlMessage::from_xml` with various XML inputs covering
//! all field types (string, int, long, double, node array, nested node).

use wx_rust_cp::bean::message::WxCpTpXmlMessage;

#[test]
fn from_xml_minimal() {
    let xml = r#"<xml>
        <SuiteId>s1</SuiteId>
        <InfoType>change_auth</InfoType>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.suite_id.as_deref(), Some("s1"));
    assert_eq!(msg.info_type.as_deref(), Some("change_auth"));
    assert!(msg.all_fields_map.is_some());
}

#[test]
fn from_xml_full_contact_change() {
    let xml = r#"<xml>
        <InfoType>change_contact</InfoType>
        <ChangeType>create_user</ChangeType>
        <UserID>user_001</UserID>
        <Department>1</Department>
        <Department>2</Department>
        <MainDepartment>1</MainDepartment>
        <IsLeaderInDept>0</IsLeaderInDept>
        <IsLeaderInDept>1</IsLeaderInDept>
        <Mobile>13800138000</Mobile>
        <Position>Engineer</Position>
        <Gender>1</Gender>
        <Email>test@example.com</Email>
        <Status>1</Status>
        <Avatar>https://img.example.com/avatar.png</Avatar>
        <Alias>alias1</Alias>
        <Telephone>010-12345678</Telephone>
        <Id>42</Id>
        <Name>Test Corp</Name>
        <ParentId>10</ParentId>
        <Order>5</Order>
        <TagId>tag_1</TagId>
        <AddUserItems>u1,u2</AddUserItems>
        <DelUserItems>u3</DelUserItems>
        <AddPartyItems>p1</AddPartyItems>
        <DelPartyItems>p2</DelPartyItems>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.info_type.as_deref(), Some("change_contact"));
    assert_eq!(msg.change_type.as_deref(), Some("create_user"));
    assert_eq!(msg.user_id.as_deref(), Some("user_001"));
    assert_eq!(msg.departments, vec![1, 2]);
    assert_eq!(msg.main_department, Some(1));
    assert_eq!(msg.is_leader_in_dept, vec![0, 1]);
    assert_eq!(msg.mobile.as_deref(), Some("13800138000"));
    assert_eq!(msg.position.as_deref(), Some("Engineer"));
    assert_eq!(msg.gender, Some(1));
    assert_eq!(msg.email.as_deref(), Some("test@example.com"));
    assert_eq!(msg.status.as_deref(), Some("1"));
    assert_eq!(msg.avatar.as_deref(), Some("https://img.example.com/avatar.png"));
    assert_eq!(msg.alias.as_deref(), Some("alias1"));
    assert_eq!(msg.telephone.as_deref(), Some("010-12345678"));
    assert_eq!(msg.id.as_deref(), Some("42"));
    assert_eq!(msg.name.as_deref(), Some("Test Corp"));
    assert_eq!(msg.parent_id.as_deref(), Some("10"));
    assert_eq!(msg.order, Some(5));
    assert_eq!(msg.tag_id.as_deref(), Some("tag_1"));
    assert_eq!(msg.add_user_items.as_deref(), Some("u1,u2"));
    assert_eq!(msg.del_user_items.as_deref(), Some("u3"));
    assert_eq!(msg.add_party_items.as_deref(), Some("p1"));
    assert_eq!(msg.del_party_items.as_deref(), Some("p2"));
}

#[test]
fn from_xml_suite_ticket() {
    let xml = r#"<xml>
        <SuiteId>s1</SuiteId>
        <InfoType>suite_ticket</InfoType>
        <SuiteTicket>ticker_abc</SuiteTicket>
        <TimeStamp>1234567890</TimeStamp>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.suite_ticket.as_deref(), Some("ticker_abc"));
    assert_eq!(msg.time_stamp.as_deref(), Some("1234567890"));
}

#[test]
fn from_xml_change_auth() {
    let xml = r#"<xml>
        <SuiteId>s1</SuiteId>
        <InfoType>change_auth</InfoType>
        <AuthCode>auth_code_xyz</AuthCode>
        <AuthCorpId>corp_abc</AuthCorpId>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.auth_code.as_deref(), Some("auth_code_xyz"));
    assert_eq!(msg.auth_corp_id.as_deref(), Some("corp_abc"));
}

#[test]
fn from_xml_batch_job() {
    let xml = r#"<xml>
        <InfoType>batch_job_result</InfoType>
        <BatchJob>
            <JobId>job_1</JobId>
            <JobType>sync_user</JobType>
            <ErrCode>0</ErrCode>
            <ErrMsg>ok</ErrMsg>
        </BatchJob>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.batch_job.job_id.as_deref(), Some("job_1"));
    assert_eq!(msg.batch_job.job_type.as_deref(), Some("sync_user"));
    assert_eq!(msg.batch_job.err_code, Some(0));
    assert_eq!(msg.batch_job.err_msg.as_deref(), Some("ok"));
}

#[test]
fn from_xml_contact_sync() {
    let xml = r#"<xml>
        <InfoType>register_corp</InfoType>
        <ContactSync>
            <AccessToken>at_123</AccessToken>
            <ExpiresIn>7200</ExpiresIn>
        </ContactSync>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.contact_sync.access_token.as_deref(), Some("at_123"));
    assert_eq!(msg.contact_sync.expires_in, Some(7200));
}

#[test]
fn from_xml_auth_user_info() {
    let xml = r#"<xml>
        <InfoType>cancel_auth</InfoType>
        <AuthUserInfo>
            <UserId>user_xyz</UserId>
        </AuthUserInfo>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.auth_user_info.user_id.as_deref(), Some("user_xyz"));
}

#[test]
fn from_xml_text_message() {
    let xml = r#"<xml>
        <ToUserName>toUser</ToUserName>
        <FromUserName>fromUser</FromUserName>
        <CreateTime>1348831860</CreateTime>
        <MsgType>text</MsgType>
        <Content>Hello World</Content>
        <MsgId>1234567890123456</MsgId>
        <AgentID>1000002</AgentID>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.to_user_name.as_deref(), Some("toUser"));
    assert_eq!(msg.from_user_name.as_deref(), Some("fromUser"));
    assert_eq!(msg.create_time, Some(1348831860));
    assert_eq!(msg.msg_type.as_deref(), Some("text"));
    assert_eq!(msg.content.as_deref(), Some("Hello World"));
    assert_eq!(msg.msg_id.as_deref(), Some("1234567890123456"));
    assert_eq!(msg.agent_id.as_deref(), Some("1000002"));
}

#[test]
fn from_xml_location_event() {
    let xml = r#"<xml>
        <MsgType>event</MsgType>
        <Event>LOCATION</Event>
        <Latitude>23.099994</Latitude>
        <Longitude>113.324520</Longitude>
        <Precision>119.364</Precision>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.latitude, Some(23.099994));
    assert_eq!(msg.longitude, Some(113.324520));
    assert_eq!(msg.precision, Some(119.364));
}

#[test]
fn from_xml_image_message() {
    let xml = r#"<xml>
        <MsgType>image</MsgType>
        <PicUrl>https://img.example.com/pic.jpg</PicUrl>
        <MediaId>media_id_1</MediaId>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.pic_url.as_deref(), Some("https://img.example.com/pic.jpg"));
    assert_eq!(msg.media_id.as_deref(), Some("media_id_1"));
}

#[test]
fn from_xml_video_message() {
    let xml = r#"<xml>
        <MsgType>video</MsgType>
        <MediaId>vid_1</MediaId>
        <ThumbMediaId>thumb_1</ThumbMediaId>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.thumb_media_id.as_deref(), Some("thumb_1"));
}

#[test]
fn from_xml_location_message() {
    let xml = r#"<xml>
        <MsgType>location</MsgType>
        <Location_X>23.09</Location_X>
        <Location_Y>113.32</Location_Y>
        <Scale>15</Scale>
        <Label>Guangzhou</Label>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.location_x, Some(23.09));
    assert_eq!(msg.location_y, Some(113.32));
    assert_eq!(msg.scale, Some(15.0));
    assert_eq!(msg.label.as_deref(), Some("Guangzhou"));
}

#[test]
fn from_xml_link_message() {
    let xml = r#"<xml>
        <MsgType>link</MsgType>
        <Title>Link Title</Title>
        <Description>Link Desc</Description>
        <Url>https://example.com</Url>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.title.as_deref(), Some("Link Title"));
    assert_eq!(msg.description.as_deref(), Some("Link Desc"));
    assert_eq!(msg.url.as_deref(), Some("https://example.com"));
}

#[test]
fn from_xml_external_contact_event() {
    let xml = r#"<xml>
        <MsgType>event</MsgType>
        <Event>change_external_contact</Event>
        <ChangeType>add_external_contact</ChangeType>
        <ExternalUserID>ext_user_1</ExternalUserID>
        <State>state_1</State>
        <Source>source_1</Source>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.external_user_id.as_deref(), Some("ext_user_1"));
    assert_eq!(msg.state.as_deref(), Some("state_1"));
    assert_eq!(msg.source.as_deref(), Some("source_1"));
}

#[test]
fn from_xml_tag_event() {
    let xml = r#"<xml>
        <MsgType>event</MsgType>
        <Event>change_contact</Event>
        <ChangeType>update_tag</ChangeType>
        <TagId>tag_1</TagId>
        <AddUserItems>a1,a2</AddUserItems>
        <DelUserItems>d1</DelUserItems>
        <AddPartyItems>p1</AddPartyItems>
        <DelPartyItems>p2</DelPartyItems>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    assert_eq!(msg.tag_id.as_deref(), Some("tag_1"));
}

#[test]
fn from_xml_invalid_root() {
    let xml = "just a scalar";
    let result = WxCpTpXmlMessage::from_xml(xml);
    assert!(result.is_err());
}

#[test]
fn from_xml_empty_fields() {
    let xml = r#"<xml>
        <SuiteId></SuiteId>
        <InfoType></InfoType>
    </xml>"#;
    let msg = WxCpTpXmlMessage::from_xml(xml).unwrap();
    // Empty strings are parsed as Some("") for str_field
    assert_eq!(msg.suite_id.as_deref(), Some(""));
}

#[test]
fn default_message() {
    let msg = WxCpTpXmlMessage::default();
    assert!(msg.suite_id.is_none());
    assert!(msg.departments.is_empty());
    assert!(msg.is_leader_in_dept.is_empty());
}
