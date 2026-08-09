//! 对应 Java `com.github.binarywang.wxpay.bean.profitsharing.ReceiverList`。
//!
//! 分账接收人列表辅助类：`ArrayList<Receiver>` 的包装（Java 私有构造 +
//! `getInstance()` 工厂），用于组装 v2 分账请求 `receivers` 字段的 JSON。

use crate::bean::Receiver;

/// 分账接收人列表（对应 Java `ReceiverList`）。
///
/// `ADAPTED`：Java `ArrayList<Receiver>` 包装以 `Vec<Receiver>` 承载；
/// `add` 链式调用返回 `&mut Self`（Java 返回 `this`）。
#[derive(Debug, Default, Clone, PartialEq)]
pub struct ReceiverList {
    list: Vec<Receiver>,
}

impl ReceiverList {
    /// 获取一个实例（对应 Java `getInstance()`）。
    pub fn get_instance() -> Self {
        Self { list: Vec::new() }
    }

    /// 添加一个分账条目（对应 Java `add(Receiver)`）。
    ///
    /// 注意微信上限为 50 个。
    pub fn add(&mut self, receiver: Receiver) -> &mut Self {
        self.list.push(receiver);
        self
    }

    /// 转为 JSON 格式（对应 Java `toJSONString()`，Gson 序列化内部列表）。
    pub fn to_json_string(&self) -> String {
        serde_json::to_string(&self.list).unwrap_or_default()
    }

    /// 内部列表只读访问（`ADAPTED`：Java 无对应公开方法，供调用方组装使用）。
    pub fn receivers(&self) -> &[Receiver] {
        &self.list
    }
}
