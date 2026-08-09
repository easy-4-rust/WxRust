//! 模版消息行业枚举。
//!
//! 对应 Java `me.chanjar.weixin.mp.bean.template.WxMpTemplateIndustryEnum`。

/// 模版消息行业枚举。
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WxMpTemplateIndustryEnum {
    /// IT科技 - 互联网|电子商务
    ECommerce,
    /// IT科技 - IT软件与服务
    ItSoftwareAndServices,
    /// IT科技 - IT硬件与设备
    ItHardwareAndEquipment,
    /// IT科技 - 电子技术
    ElectronicTechnique,
    /// IT科技 - 通信与运营商
    CommunicationAndOperator,
    /// IT科技 - 网络游戏
    OnlineGame,
    /// 金融业 - 银行
    Bank,
    /// 金融业 - 证券基金理财信托
    Fund,
    /// 金融业 - 保险
    Insurance,
    /// 餐饮 - 餐饮
    Repast,
    /// 酒店旅游 - 酒店
    Hotel,
    /// 酒店旅游 - 旅游
    Travel,
    /// 运输与仓储 - 快递
    Express,
    /// 运输与仓储 - 物流
    Logistics,
    /// 运输与仓储 - 仓储
    Storage,
    /// 教育 - 培训
    Cultivate,
    /// 教育 - 院校
    Academy,
    /// 政府与公共事业 - 学术科研
    AcademicResearch,
    /// 政府与公共事业 - 交警
    TrafficPolice,
    /// 政府与公共事业 - 博物馆
    Museum,
    /// 政府与公共事业 - 公共事业非盈利机构
    PublicWorksNonprofit,
    /// 医药护理 - 医药医疗
    MedicalHealth,
    /// 医药护理 - 护理美容
    CareAndBeauty,
    /// 医药护理 - 保健与卫生
    HealthAndHygiene,
    /// 交通工具 - 汽车相关
    AutomotiveRelated,
    /// 交通工具 - 摩托车相关
    MotorcycleCorrelation,
    /// 交通工具 - 火车相关
    TheTrainRelated,
    /// 交通工具 - 飞机相关
    ThePlaneRelated,
    /// 房地产 - 建筑
    Architecture,
    /// 房地产 - 物业
    RealEstate,
    /// 消费品 - 消费品
    ConsumerGoods,
    /// 商业服务 - 法律
    Legislation,
    /// 商业服务 - 会展
    ConventionAndExhibition,
    /// 商业服务 - 中介服务
    IntermediaryServices,
    /// 商业服务 - 认证
    Authentication,
    /// 商业服务 - 审计
    Audit,
    /// 文体娱乐 - 传媒
    MassMedia,
    /// 文体娱乐 - 体育
    Sports,
    /// 文体娱乐 - 娱乐休闲
    LeisureAndEntertainment,
    /// 印刷 - 印刷
    Printing,
    /// 其他 - 其他
    Other,
}

impl WxMpTemplateIndustryEnum {
    /// 全部枚举项（保持 Java 声明顺序）。
    pub const ALL: [Self; 41] = [
        Self::ECommerce,
        Self::ItSoftwareAndServices,
        Self::ItHardwareAndEquipment,
        Self::ElectronicTechnique,
        Self::CommunicationAndOperator,
        Self::OnlineGame,
        Self::Bank,
        Self::Fund,
        Self::Insurance,
        Self::Repast,
        Self::Hotel,
        Self::Travel,
        Self::Express,
        Self::Logistics,
        Self::Storage,
        Self::Cultivate,
        Self::Academy,
        Self::AcademicResearch,
        Self::TrafficPolice,
        Self::Museum,
        Self::PublicWorksNonprofit,
        Self::MedicalHealth,
        Self::CareAndBeauty,
        Self::HealthAndHygiene,
        Self::AutomotiveRelated,
        Self::MotorcycleCorrelation,
        Self::TheTrainRelated,
        Self::ThePlaneRelated,
        Self::Architecture,
        Self::RealEstate,
        Self::ConsumerGoods,
        Self::Legislation,
        Self::ConventionAndExhibition,
        Self::IntermediaryServices,
        Self::Authentication,
        Self::Audit,
        Self::MassMedia,
        Self::Sports,
        Self::LeisureAndEntertainment,
        Self::Printing,
        Self::Other,
    ];

    /// 主行业（一级行业）。
    pub fn first_class(self) -> &'static str {
        match self {
            Self::ECommerce => "IT科技",
            Self::ItSoftwareAndServices => "IT科技",
            Self::ItHardwareAndEquipment => "IT科技",
            Self::ElectronicTechnique => "IT科技",
            Self::CommunicationAndOperator => "IT科技",
            Self::OnlineGame => "IT科技",
            Self::Bank => "金融业",
            Self::Fund => "金融业",
            Self::Insurance => "金融业",
            Self::Repast => "餐饮",
            Self::Hotel => "酒店旅游",
            Self::Travel => "酒店旅游",
            Self::Express => "运输与仓储",
            Self::Logistics => "运输与仓储",
            Self::Storage => "运输与仓储",
            Self::Cultivate => "教育",
            Self::Academy => "教育",
            Self::AcademicResearch => "政府与公共事业",
            Self::TrafficPolice => "政府与公共事业",
            Self::Museum => "政府与公共事业",
            Self::PublicWorksNonprofit => "政府与公共事业",
            Self::MedicalHealth => "医药护理",
            Self::CareAndBeauty => "医药护理",
            Self::HealthAndHygiene => "医药护理",
            Self::AutomotiveRelated => "交通工具",
            Self::MotorcycleCorrelation => "交通工具",
            Self::TheTrainRelated => "交通工具",
            Self::ThePlaneRelated => "交通工具",
            Self::Architecture => "房地产",
            Self::RealEstate => "房地产",
            Self::ConsumerGoods => "消费品",
            Self::Legislation => "商业服务",
            Self::ConventionAndExhibition => "商业服务",
            Self::IntermediaryServices => "商业服务",
            Self::Authentication => "商业服务",
            Self::Audit => "商业服务",
            Self::MassMedia => "文体娱乐",
            Self::Sports => "文体娱乐",
            Self::LeisureAndEntertainment => "文体娱乐",
            Self::Printing => "印刷",
            Self::Other => "其他",
        }
    }

    /// 副行业（二级行业）。
    pub fn second_class(self) -> &'static str {
        match self {
            Self::ECommerce => "互联网|电子商务",
            Self::ItSoftwareAndServices => "IT软件与服务",
            Self::ItHardwareAndEquipment => "IT硬件与设备",
            Self::ElectronicTechnique => "电子技术",
            Self::CommunicationAndOperator => "通信与运营商",
            Self::OnlineGame => "网络游戏",
            Self::Bank => "银行",
            Self::Fund => "证券基金理财信托",
            Self::Insurance => "保险",
            Self::Repast => "餐饮",
            Self::Hotel => "酒店",
            Self::Travel => "旅游",
            Self::Express => "快递",
            Self::Logistics => "物流",
            Self::Storage => "仓储",
            Self::Cultivate => "培训",
            Self::Academy => "院校",
            Self::AcademicResearch => "学术科研",
            Self::TrafficPolice => "交警",
            Self::Museum => "博物馆",
            Self::PublicWorksNonprofit => "公共事业非盈利机构",
            Self::MedicalHealth => "医药医疗",
            Self::CareAndBeauty => "护理美容",
            Self::HealthAndHygiene => "保健与卫生",
            Self::AutomotiveRelated => "汽车相关",
            Self::MotorcycleCorrelation => "摩托车相关",
            Self::TheTrainRelated => "火车相关",
            Self::ThePlaneRelated => "飞机相关",
            Self::Architecture => "建筑",
            Self::RealEstate => "物业",
            Self::ConsumerGoods => "消费品",
            Self::Legislation => "法律",
            Self::ConventionAndExhibition => "会展",
            Self::IntermediaryServices => "中介服务",
            Self::Authentication => "认证",
            Self::Audit => "审计",
            Self::MassMedia => "传媒",
            Self::Sports => "体育",
            Self::LeisureAndEntertainment => "娱乐休闲",
            Self::Printing => "印刷",
            Self::Other => "其他",
        }
    }

    /// 行业代码。
    pub fn code(self) -> i32 {
        match self {
            Self::ECommerce => 1,
            Self::ItSoftwareAndServices => 2,
            Self::ItHardwareAndEquipment => 3,
            Self::ElectronicTechnique => 4,
            Self::CommunicationAndOperator => 5,
            Self::OnlineGame => 6,
            Self::Bank => 7,
            Self::Fund => 8,
            Self::Insurance => 9,
            Self::Repast => 10,
            Self::Hotel => 11,
            Self::Travel => 12,
            Self::Express => 13,
            Self::Logistics => 14,
            Self::Storage => 15,
            Self::Cultivate => 16,
            Self::Academy => 17,
            Self::AcademicResearch => 18,
            Self::TrafficPolice => 19,
            Self::Museum => 20,
            Self::PublicWorksNonprofit => 21,
            Self::MedicalHealth => 22,
            Self::CareAndBeauty => 23,
            Self::HealthAndHygiene => 24,
            Self::AutomotiveRelated => 25,
            Self::MotorcycleCorrelation => 26,
            Self::TheTrainRelated => 27,
            Self::ThePlaneRelated => 28,
            Self::Architecture => 29,
            Self::RealEstate => 30,
            Self::ConsumerGoods => 31,
            Self::Legislation => 32,
            Self::ConventionAndExhibition => 33,
            Self::IntermediaryServices => 34,
            Self::Authentication => 35,
            Self::Audit => 36,
            Self::MassMedia => 37,
            Self::Sports => 38,
            Self::LeisureAndEntertainment => 39,
            Self::Printing => 40,
            Self::Other => 41,
        }
    }

    /// 查找行业（对应 Java `findByClass`：一级行业相等 + 二级行业包含）。
    pub fn find_by_class(first_class: &str, second_class: &str) -> Option<Self> {
        Self::ALL
            .into_iter()
            .find(|e| e.first_class() == first_class && e.second_class().contains(second_class))
    }

    /// 按行业编码查找（对应 Java `findByCode`）。
    pub fn find_by_code(code: i32) -> Option<Self> {
        Self::ALL.into_iter().find(|e| e.code() == code)
    }
}
