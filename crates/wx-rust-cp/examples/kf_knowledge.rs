use wx_rust_common::error::WxErrorException;
use wx_rust_cp::api::{WxCpKfKnowledgeService, WxCpService};

async fn read_all_groups(service: &dyn WxCpService) -> Result<(), WxErrorException> {
    let mut cursor = None;
    loop {
        let page = service
            .list_knowledge_group(cursor.as_deref(), Some(100), None)
            .await?;
        // 在此处理 page.group_list；has_more 是整数，不是布尔值。
        if page.has_more.unwrap_or(0) == 0 {
            break;
        }
        cursor = page.next_cursor;
        if cursor.as_deref().is_none_or(str::is_empty) {
            break;
        }
    }
    Ok(())
}

fn main() {
    // 使用现有 WxCpService 客户端调用；示例本身不会访问企业微信。
    let _ = read_all_groups;
}
