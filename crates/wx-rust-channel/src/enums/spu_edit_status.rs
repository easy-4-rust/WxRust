//! 视频号小店 枚举（对应 Java `SpuEditStatus`）。

/// SpuEditStatus（对应 Java `me.chanjar.weixin.channel.enums.SpuEditStatus`）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SpuEditStatus {
    /// 0 初始值
    Init,
    /// 1 编辑中
    Submit,
    /// 2 审核中
    Ing,
    /// 3 审核失败
    Fail,
    /// 4 审核成功
    Success,
    /// 5 商品信息写入中
    Writing,
    /// 7 商品异步提交，上传中（处于该状态的商品调用上架商品接口会返回10020067）
    AsyncWriting,
    /// 8 商品异步提交，上传失败（请重新提交）
    AsyncFail,
}

impl SpuEditStatus {
    /// 枚举 key（对应 Java `getKey()`）。
    pub fn key(&self) -> i32 {
        match self {
            SpuEditStatus::Init => 0,
            SpuEditStatus::Submit => 1,
            SpuEditStatus::Ing => 2,
            SpuEditStatus::Fail => 3,
            SpuEditStatus::Success => 4,
            SpuEditStatus::Writing => 5,
            SpuEditStatus::AsyncWriting => 7,
            SpuEditStatus::AsyncFail => 8,
        }
    }

    /// 枚举中文说明（对应 Java `getVal()`）。
    pub fn val(&self) -> &'static str {
        match self {
            SpuEditStatus::Init => "初始值",
            SpuEditStatus::Submit => "编辑中",
            SpuEditStatus::Ing => "审核中",
            SpuEditStatus::Fail => "审核失败",
            SpuEditStatus::Success => "审核成功",
            SpuEditStatus::Writing => "商品信息写入中",
            SpuEditStatus::AsyncWriting => "商品异步提交，上传中",
            SpuEditStatus::AsyncFail => "商品异步提交，上传失败",
        }
    }
}
