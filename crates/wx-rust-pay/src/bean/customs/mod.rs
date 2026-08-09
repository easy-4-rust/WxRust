//! 对应 Java `com.github.binarywang.wxpay.bean.customs` 包（生成）。

pub mod declaration_query_request;
pub mod declaration_query_result;
pub mod declaration_request;
pub mod declaration_result;
pub mod redeclare_request;
pub mod redeclare_result;
pub mod verify_certificate_request;
pub mod verify_certificate_result;

pub use declaration_query_request::DeclarationQueryRequest;
pub use declaration_query_result::DeclarationData;
pub use declaration_query_result::DeclarationQueryResult;
pub use declaration_request::DeclarationRequest;
pub use declaration_result::DeclarationResult;
pub use redeclare_request::RedeclareRequest;
pub use redeclare_result::RedeclareResult;
pub use verify_certificate_request::VerifyCertificateRequest;
pub use verify_certificate_result::VerifyCertificateResult;
