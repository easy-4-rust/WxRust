//! 视频号小店接口地址常量（对应 Java `WxChannelApiUrlConstants`）。
//!

//! 常量值即完整 URL（域名为 `https://api.weixin.qq.com`）；自定义域名
//! 替换由执行引擎在 token 注入时统一处理（Java `executeInternal` 中
//! `uri.replace("https://api.weixin.qq.com", apiHostUrl)` 语义）。

/// 获取账户余额（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BALANCE_URL`）。
pub const GET_BALANCE_URL: &str = "https://api.weixin.qq.com/channels/ec/funds/getbalance";

/// 获取结算账户（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BANK_ACCOUNT_URL`）。
pub const GET_BANK_ACCOUNT_URL: &str = "https://api.weixin.qq.com/channels/ec/funds/getbankacct";

/// 获取资金流水详情（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BALANCE_FLOW_DETAIL_URL`）。
pub const GET_BALANCE_FLOW_DETAIL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/funds/getfundsflowdetail";

/// 获取资金流水列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BALANCE_FLOW_LIST_URL`）。
pub const GET_BALANCE_FLOW_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/funds/getfundsflowlist";

/// 获取提现记录（对应 Java `WxChannelApiUrlConstants` 常量 `GET_WITHDRAW_DETAIL_URL`）。
pub const GET_WITHDRAW_DETAIL_URL: &str =
    "https://api.weixin.qq.com/channels/ec/funds/getwithdrawdetail";

/// 获取提现记录列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_WITHDRAW_LIST_URL`）。
pub const GET_WITHDRAW_LIST_URL: &str =
    "https://api.weixin.qq.com/channels/ec/funds/getwithdrawlist";

/// 修改结算账户（对应 Java `WxChannelApiUrlConstants` 常量 `SET_BANK_ACCOUNT_URL`）。
pub const SET_BANK_ACCOUNT_URL: &str = "https://api.weixin.qq.com/channels/ec/funds/setbankacct";

/// 商户提现（对应 Java `WxChannelApiUrlConstants` 常量 `WITHDRAW_URL`）。
pub const WITHDRAW_URL: &str = "https://api.weixin.qq.com/channels/ec/funds/submitwithdraw";

/// 根据卡号查银行信息（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BANK_BY_NUM_URL`）。
pub const GET_BANK_BY_NUM_URL: &str = "https://api.weixin.qq.com/shop/funds/getbankbynum";

/// 搜索银行列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_BANK_LIST_URL`）。
pub const GET_BANK_LIST_URL: &str = "https://api.weixin.qq.com/shop/funds/getbanklist";

/// 查询城市列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_CITY_URL`）。
pub const GET_CITY_URL: &str = "https://api.weixin.qq.com/shop/funds/getcity";

/// 查询大陆银行省份列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_PROVINCE_URL`）。
pub const GET_PROVINCE_URL: &str = "https://api.weixin.qq.com/shop/funds/getprovince";

/// 查询支行列表（对应 Java `WxChannelApiUrlConstants` 常量 `GET_SUB_BANK_URL`）。
pub const GET_SUB_BANK_URL: &str = "https://api.weixin.qq.com/shop/funds/getsubbranch";

/// 获取二维码（对应 Java `WxChannelApiUrlConstants` 常量 `GET_QRCODE_URL`）。
pub const GET_QRCODE_URL: &str = "https://api.weixin.qq.com/shop/funds/qrcode/get";

/// 查询扫码状态（对应 Java `WxChannelApiUrlConstants` 常量 `CHECK_QRCODE_URL`）。
pub const CHECK_QRCODE_URL: &str = "https://api.weixin.qq.com/shop/funds/qrcode/check";
