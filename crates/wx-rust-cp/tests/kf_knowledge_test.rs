use serde_json::json;
use wx_rust_cp::bean::kf::WxCpKfKnowledgeIntent;

#[test]
fn knowledge_preserves_both_question_shapes_and_all_attachments() {
    let nested = json!({"text":{"content":"问题"}, "similar_questions":{"items":[{"text":{"content":"相似问法"}}]},"answers":[{"text":{"content":"答案"},"attachments":[
        {"msgtype":"image","image":{"media_id":"m1","name":"图片"}},
        {"msgtype":"video","video":{"media_id":"m2","name":"视频"}},
        {"msgtype":"link","link":{"title":"标题","pic_url":"pic","desc":"说明","url":"url"}},
        {"msgtype":"miniprogram","miniprogram":{"title":"程序","thumb_media_id":"thumb","appid":"app","pagepath":"page"}}
    ]}]});
    let input = json!({"group_id":"g","intent_id":"i","question":nested,"similar_questions":nested["similar_questions"],"answers":nested["answers"]});
    let model: WxCpKfKnowledgeIntent = serde_json::from_value(input.clone()).unwrap();
    assert_eq!(serde_json::to_value(model).unwrap(), input);
}
