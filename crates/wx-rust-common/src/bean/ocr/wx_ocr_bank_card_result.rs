//! 对应 Java `me.chanjar.weixin.common.bean.ocr.WxOcrBankCardResult`（由 gen_bean_structs.py 生成）。

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WxOcrBankCardResult {
    /// number
    #[serde(rename = "number", default)]
    pub number: String,
}
