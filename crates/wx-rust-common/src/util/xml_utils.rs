//! XML 转换工具类。
//!
//! 对应 Java `me.chanjar.weixin.common.util.XmlUtils`（基于 dom4j 的 xml2Map）。
//! Rust 侧使用 `quick-xml` 实现相同语义：XML → `Map<String, Object>`。

use std::collections::HashMap;

use quick_xml::Reader;
use quick_xml::events::Event;

/// XML 转换工具。
pub struct XmlUtils;

impl XmlUtils {
    /// 将 XML 字符串转换为键值 Map。
    ///
    /// 语义与 Java `xml2Map` 一致：顶层元素下的每个子元素作为键，
    /// 元素文本作为值；同名元素保留最后一个（Java 实现用 HashMap）。
    ///
    /// # 参数
    /// - `xml_string`：XML 字符串
    ///
    /// # 返回
    /// 键值 Map；解析失败时返回错误。
    pub fn xml_2_map(xml_string: &str) -> Result<HashMap<String, String>, String> {
        let mut reader = Reader::from_str(xml_string);
        reader.config_mut().trim_text(true);

        let mut map = HashMap::new();
        let mut depth = 0usize;
        let mut current_tag: Option<String> = None;
        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(e)) => {
                    depth += 1;
                    if depth == 2 {
                        current_tag = Some(String::from_utf8_lossy(e.name().as_ref()).to_string());
                    }
                }
                Ok(Event::Text(t)) => {
                    if let Some(tag) = &current_tag {
                        let text = t.decode().map_err(|e| e.to_string())?;
                        map.insert(tag.clone(), text.to_string());
                    }
                }
                Ok(Event::CData(t)) => {
                    if let Some(tag) = &current_tag {
                        let text = t.decode().map_err(|e| e.to_string())?;
                        map.insert(tag.clone(), text.to_string());
                    }
                }
                Ok(Event::End(_)) => {
                    if depth >= 2 {
                        depth -= 1;
                        if depth == 1 {
                            current_tag = None;
                        }
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(format!("XML 解析失败: {e}")),
                _ => {}
            }
            buf.clear();
        }
        Ok(map)
    }
}
