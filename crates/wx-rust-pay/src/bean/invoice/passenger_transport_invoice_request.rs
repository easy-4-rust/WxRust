use super::buyer_information::BuyerInformation;
use serde::{Deserialize, Serialize};

/// 服务商开具旅客运输行业电子发票请求。
/// 购买方手机号、邮箱以及出行人证件号码由调用方按微信支付文档加密。
/// 对应 Java: com.github.binarywang.wxpay.bean.invoice.PassengerTransportInvoiceRequest
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassengerTransportInvoiceRequest {
    /// 子商户号。微信支付分配的子商户号，必填，最长32个字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,
    /// 发票申请单号。唯一标识一次开票行为并关联唯一的购买方信息，必填，最长32个字符；
    /// 微信支付账单开票场景下填写微信支付交易单号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fapiao_apply_id: Option<String>,
    /// 购买方信息，即发票抬头，必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_information: Option<BuyerInformation>,
    /// 需要开具的旅客运输行业数电发票信息，必填。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fapiao_information: Option<FapiaoInformation>,
}

/// 需要开具的旅客运输行业数电发票信息。
/// 对应 Java: com.github.binarywang.wxpay.bean.invoice.PassengerTransportInvoiceRequest.FapiaoInformation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct FapiaoInformation {
    /// 商户发票单号，必填，最长32个字符，在每个商户下必须唯一。
    /// 仅支持字母、数字、中划线、下划线、竖线和星号；开票失败或发票已冲红时可更换后重试，
    /// 同一发票申请单最多支持5个商户发票单号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fapiao_id: Option<String>,
    /// 总价税合计，必填，单位：分。值为所有发票行单行金额合计之和，且全部发票的总价税合计
    /// 不能超过交易总金额。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i64>,
    /// 发票行信息，必填；单张发票最多包含8行。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<InvoiceItem>>,
    /// 出口业务适用政策代码。可选值：1（退税政策）、2（免税政策）、3（征税政策）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_business_policy_code: Option<i64>,
    /// 增值税即征即退代码。可选值：1（软件产品）、2（资源综合利用产品）、3（管道运输服务）、
    /// 4（有形动产融资租赁服务）、5（有形动产融资性售后回租服务）、6（新型墙体材料）、
    /// 7（风力发电产品）、8（光伏发电产品）、9（动漫软件产品）、10（飞机维修劳务）、
    /// 11（黄金）、12（铂金）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_refund_levy_code: Option<i64>,
    /// 开票人ID，必填，最长64个字符，为税局乐企系统登记的开票人ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_person_id: Option<String>,
    /// 开票人名称，最长64个字符，为税局乐企系统登记的脱敏后名称；格式为脱敏姓名、空格和身份证后四位。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_person: Option<String>,
    /// 发票类型，必填。可选值：COMM_FAPIAO（增值税普通发票）、VAT_FAPIAO（增值税专用发票）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fapiao_bill_type: Option<String>,
    /// 发票对应的交易信息，必填，最多支持10条。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_information: Option<Vec<TransactionInformation>>,
    /// 发票备注。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

/// 发票行信息。
/// 对应 Java: com.github.binarywang.wxpay.bean.invoice.PassengerTransportInvoiceRequest.InvoiceItem
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InvoiceItem {
    /// 税局侧规定的货物或应税劳务、服务税收分类编码，必填，长度为19个字符；
    /// 旅客运输业务仅支持以301开头的税收分类编码。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,
    /// 货物或应税劳务、服务名称，必填，由商户自定义，最长128个字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    /// 规格型号，展示在发票规格型号列，最长20个字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specification: Option<String>,
    /// 单位，展示在发票单位列；折扣行不填写。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /// 数量，单位为10的负8次方，100000000表示数量1。非折扣行不填写时默认为100000000；
    /// 折扣行不填写。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    /// 单行金额合计，必填，单位：分。折扣行为负数，非折扣行为正数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<i64>,
    /// 税率，必填，单位为万分之一，例如1300表示13%。当前支持0、1%、1.5%、3%、5%、6%、
    /// 9%、10%、11%、13%、16%和17%。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<i64>,
    /// 是否为折扣行，必填；折扣行必须紧跟在被折扣行之后。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<bool>,
    /// 优惠政策标识。可选值：1（简易征收）、2（稀土产品）、3（免税）、4（不征税）、
    /// 5（先征后退）、6（100%先征后退）、7（50%先征后退）、8（按3%简易征收）、
    /// 9（按5%简易征收）、10（按5%简易征收减按1.5%计征）、11（即征即退30%）、
    /// 12（即征即退50%）、13（即征即退70%）、14（即征即退100%）、
    /// 15（超税负3%即征即退）、16（超税负8%即征即退）、17（超税负12%即征即退）、
    /// 18（超税负6%即征即退）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferential_policy_code: Option<i64>,
    /// 出行人额外信息，可选；传入时其内部必填字段必须完整填写。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passenger_information: Option<PassengerInformation>,
}

/// 出行人额外信息。证件号码由调用方按微信支付文档加密。
/// 对应 Java: com.github.binarywang.wxpay.bean.invoice.PassengerTransportInvoiceRequest.PassengerInformation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassengerInformation {
    /// 出行人姓名；填写出行人额外信息时必填，最长20个字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 出行人证件类型；填写出行人额外信息时必填。可选值：IDENTITY_CARD（居民身份证）、
    /// PASSPORT（护照）、HONG_KONG_PERMIT（中国香港居民来往内地通行证）、
    /// MACAO_PERMIT（中国澳门居民来往内地通行证）、TAIWAN_PERMIT（中国台湾居民来往大陆通行证）、
    /// FOREIGNER_RESIDENCE_PERMIT（外国人居留证）、HONG_KONG_RESIDENT_CARD（香港居民证）、
    /// MACAO_RESIDENT_CARD（澳门居民证）、TAIWAN_RESIDENT_CARD（台湾居民证）、
    /// MILITARY_OFFICER_CARD（军官证）、ARMED_POLICE_OFFICER_CARD（武警警官证）、
    /// SOLDIER_CARD（士兵证）、HOMECOMING_CERT（港澳同胞回乡证）、
    /// TAIWAN_COMPATRIOT_CERT（台胞证）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_type: Option<String>,
    /// 出行人证件号码；填写出行人额外信息时必填。该字段为密文字段，调用方需使用微信支付公钥
    /// 或微信支付平台证书公钥加密后传入。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub certificate_number: Option<String>,
    /// 出行日期；填写出行人额外信息时必填，使用 RFC3339 格式：yyyy-MM-DDTHH:mm:ss+TIMEZONE。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_date: Option<String>,
    /// 出发地详细地址；填写出行人额外信息时必填，最长80个字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departure_place: Option<String>,
    /// 目的地详细地址；填写出行人额外信息时必填，最长80个字符。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    /// 交通工具类型；填写出行人额外信息时必填。可选值：LONG_DISTANCE_BUS（长途汽车）、
    /// PUBLIC_TRANSPORTATION（公共交通）、CAR（汽车）、SHIP（船舶）、
    /// OTHER_TRANSPORTATION（其他交通工具）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transportation_type: Option<String>,
    /// 交通工具等级。交通工具为 SHIP 时必填，可选值：SHIP_FIRST_CLASS_CABIN（船舶一等舱）、
    /// SHIP_SECOND_CLASS_CABIN（船舶二等舱）、SHIP_THIRD_CLASS_CABIN（船舶三等舱）；
    /// 其他交通工具不填写。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transportation_classes: Option<String>,
}

/// 发票对应的交易信息。
/// 对应 Java: com.github.binarywang.wxpay.bean.invoice.PassengerTransportInvoiceRequest.TransactionInformation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TransactionInformation {
    /// 支付渠道，必填。当前可选值：WECHAT_PAY（微信支付）。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pay_channel: Option<String>,
    /// 支付订单号，最长64个字符。支付渠道为 WECHAT_PAY 时，本字段与商户订单号至少填写一个。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// 支付商户订单号，最长64个字符。支付渠道为 WECHAT_PAY 时，本字段与支付订单号至少填写一个。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    /// 交易金额，必填，单位：分。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
