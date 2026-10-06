/// 路由上下文。对应 Java: Map<String, Object>。
pub type RouteContext = std::collections::HashMap<String, Box<dyn std::any::Any + Send>>;
