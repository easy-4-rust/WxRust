//! 对应 Java `bean.invoice.merchant.MakeOutInvoiceRequest`。
//!
//! 由 `scripts/gen_mp_bean_structs.py` 从 Java 数据类生成（@SerializedName 覆盖保留）。

#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MakeOutInvoiceRequest {
    #[serde(rename = "invoiceinfo", default)]
    pub invoiceinfo: InvoiceInfo,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceInfo {
    #[serde(rename = "wxopenid", default)]
    pub wxopenid: String,
    #[serde(rename = "ddh", default)]
    pub ddh: String,
    #[serde(rename = "fpqqlsh", default)]
    pub fpqqlsh: String,
    #[serde(rename = "nsrsbh", default)]
    pub nsrsbh: String,
    #[serde(rename = "nsrmc", default)]
    pub nsrmc: String,
    #[serde(rename = "nsrdz", default)]
    pub nsrdz: String,
    #[serde(rename = "nsrdh", default)]
    pub nsrdh: String,
    #[serde(rename = "nsrbank", default)]
    pub nsrbank: String,
    #[serde(rename = "nsrbankid", default)]
    pub nsrbankid: String,
    #[serde(rename = "ghfnsrsbh", default)]
    pub ghfnsrsbh: String,
    #[serde(rename = "ghfmc", default)]
    pub ghfmc: String,
    #[serde(rename = "ghfdz", default)]
    pub ghfdz: String,
    #[serde(rename = "ghfdh", default)]
    pub ghfdh: String,
    #[serde(rename = "ghfbank", default)]
    pub ghfbank: String,
    #[serde(rename = "ghfbankid", default)]
    pub ghfbankid: String,
    #[serde(rename = "kpr", default)]
    pub kpr: String,
    #[serde(rename = "skr", default)]
    pub skr: String,
    #[serde(rename = "fhr", default)]
    pub fhr: String,
    #[serde(rename = "jshj", default)]
    pub jshj: String,
    #[serde(rename = "hjje", default)]
    pub hjje: String,
    #[serde(rename = "hjse", default)]
    pub hjse: String,
    #[serde(rename = "bz", default)]
    pub bz: String,
    #[serde(rename = "hylx", default)]
    pub hylx: String,
    #[serde(rename = "invoicedetailList", default)]
    pub invoicedetail_list: Vec<InvoiceDetailItem>,
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct InvoiceDetailItem {
    #[serde(rename = "fphxz", default)]
    pub fphxz: String,
    #[serde(rename = "spbm", default)]
    pub spbm: String,
    #[serde(rename = "xmmc", default)]
    pub xmmc: String,
    #[serde(rename = "dw", default)]
    pub dw: String,
    #[serde(rename = "ggxh", default)]
    pub ggxh: String,
    #[serde(rename = "xmsl", default)]
    pub xmsl: String,
    #[serde(rename = "xmdj", default)]
    pub xmdj: String,
    #[serde(rename = "xmje", default)]
    pub xmje: String,
    #[serde(rename = "sl", default)]
    pub sl: String,
    #[serde(rename = "se", default)]
    pub se: String,
}
