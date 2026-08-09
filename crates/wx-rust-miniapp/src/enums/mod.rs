//! 小程序枚举。

pub mod url_business;
pub mod url_core;

pub use url_business::{internet, link, msg, qrcode, sec_check, subscribe, user};
pub use url_core::{
    get_access_token_url, get_paid_union_id_url, get_stable_access_token_url,
    js_code_to_session_url, set_dynamic_data_url,
};

// 内容服务组（G2）URL 常量注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g2_urls;
pub use g2_urls::*;

// 核心服务组（G1）URL 常量注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g1_urls;
pub use g1_urls::*;

// 电商服务组（G3）URL 常量注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g3_urls;
pub use g3_urls::*;

// 能力服务组（G4）URL 常量注册（Wave 2，并发追加行由 Wave 3 补齐）。
pub mod g4_urls;
pub use g4_urls::*;
