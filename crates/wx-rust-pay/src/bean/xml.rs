//! XML 解析辅助（v2 报文线格式，对应 Java `BaseWxPayResult` 的
//! dom4j/XPath `toMap()` 语义与 XStream 输出格式化）。

use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::XmlVersion;
use quick_xml::events::Event;

/// 解析 XML 根元素（`<xml>`）的全部直接子节点为「名称 → 文本」map。
///
/// 对应 Java `BaseWxPayResult.toMap()` 的 `XPath /xml/*` 语义：子节点文本
/// 取首个出现（v2 报文中元素名唯一；同名重复元素以首个为准），子节点内含
/// 嵌套元素时取全部后代文本的拼接（与 `Node.getTextContent()` 一致）。
pub fn root_children_map(xml: &str) -> Result<HashMap<String, String>, String> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    // 定位根元素起始标签（跳过前导文本/声明）
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => break,
            Ok(Event::Eof) => return Err("XML 解析失败: 缺少根元素".to_string()),
            Ok(Event::Text(_))
            | Ok(Event::CData(_))
            | Ok(Event::Decl(_))
            | Ok(Event::Comment(_)) => {}
            Err(e) => return Err(format!("XML 解析失败: {e}")),
            _ => {}
        }
        buf.clear();
    }
    let mut fields = HashMap::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name().as_ref().to_string();
                let text = collect_element_text(&mut reader)?;
                fields.entry(name).or_insert(text);
            }
            Ok(Event::Empty(e)) => {
                let name = e.name().as_ref().to_string();
                fields.entry(name).or_insert_with(String::new);
            }
            Ok(Event::End(_)) => break, // 根元素结束
            Ok(Event::Eof) => return Err("XML 解析失败: 根元素未闭合".to_string()),
            Err(e) => return Err(format!("XML 解析失败: {e}")),
            _ => {}
        }
        buf.clear();
    }
    Ok(fields)
}

/// 递归收集当前元素的全部后代文本（Text/CData 拼接；嵌套元素内容计入）。
/// 调用时当前元素的 Start 已消费，消费至匹配的 End。
fn collect_element_text(reader: &mut Reader<&[u8]>) -> Result<String, String> {
    let mut text = String::new();
    let mut buf = Vec::new();
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(_)) => {
                text.push_str(&collect_element_text(reader)?);
            }
            Ok(Event::Text(t)) => {
                text.push_str(&t.xml_content(XmlVersion::Implicit1_0));
            }
            Ok(Event::CData(t)) => {
                text.push_str(&t.xml_content(XmlVersion::Implicit1_0));
            }
            Ok(Event::End(_)) => break,
            Ok(Event::Eof) => return Err("XML 解析失败: 元素未闭合".to_string()),
            Err(e) => return Err(format!("XML 解析失败: {e}")),
            _ => {}
        }
        buf.clear();
    }
    Ok(text)
}

/// 将 `quick_xml` 序列化的空元素 `<x/>` 归一为 `<x></x>`。
///
/// Java XStream 对空字符串字段输出 `<x></x>`；quick-xml serde 输出
/// `<x/>`（解析语义等价）。v2 报文无属性，`/>` 只可能出现在空元素处，
/// 归一保证与 Java `toXML()` 逐字节一致。
pub fn expand_empty_elements(xml: &str) -> String {
    let mut out = String::with_capacity(xml.len() + 8);
    let mut rest = xml;
    while let Some(pos) = rest.find("/>") {
        // 回找该自闭合标签的起始 `<`
        let start = rest[..pos].rfind('<');
        match start {
            Some(s) => {
                let name = &rest[s + 1..pos];
                if !name.is_empty()
                    && name
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
                {
                    out.push_str(&rest[..s]);
                    out.push('<');
                    out.push_str(name);
                    out.push_str("></");
                    out.push_str(name);
                    out.push('>');
                    rest = &rest[pos + 2..];
                    continue;
                }
                out.push_str(&rest[..pos]);
                out.push_str("/>");
                rest = &rest[pos + 2..];
            }
            None => {
                out.push_str(&rest[..pos]);
                out.push_str("/>");
                rest = &rest[pos + 2..];
            }
        }
    }
    out.push_str(rest);
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_root_children_map() {
        let xml = concat!(
            "<xml>\n  <appid><![CDATA[wx2421b1c4370ec43b]]></appid>\n",
            "  <total_fee>1</total_fee>\n  <transaction_id></transaction_id>\n</xml>"
        );
        let map = root_children_map(xml).unwrap();
        assert_eq!(
            map.get("appid").map(String::as_str),
            Some("wx2421b1c4370ec43b")
        );
        assert_eq!(map.get("total_fee").map(String::as_str), Some("1"));
        assert_eq!(map.get("transaction_id").map(String::as_str), Some(""));
    }

    #[test]
    fn test_expand_empty_elements() {
        assert_eq!(
            expand_empty_elements("<xml><a>1</a><b/></xml>"),
            "<xml><a>1</a><b></b></xml>"
        );
        assert_eq!(expand_empty_elements("<xml></xml>"), "<xml></xml>");
    }
}
