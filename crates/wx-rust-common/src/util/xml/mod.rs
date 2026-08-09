//! XML 序列化工具（`PLATFORM_NA` 说明）。
//!
//! 对应 Java `me.chanjar.weixin.common.util.xml` 包（XStream 转换器）。
//! Java 使用 XStream 定制消息 XML 序列化；WxRust 以 `quick-xml` + serde 派生
//! 替代，线格式经 golden 夹具验证。通用 XML→Map 能力见 [`super::xml_utils`]。
