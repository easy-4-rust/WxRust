//! Coverage boost: `wx_mp_template_industry_enum.rs` (142 lines, 0% covered).
//!
//! Exercises `first_class()`, `second_class()`, `code()`, `find_by_class()`,
//! `find_by_code()`, and `ALL` array for all 41 variants.

use wx_rust_mp::bean::template::WxMpTemplateIndustryEnum;

/// ALL array must have exactly 41 elements.
#[test]
fn all_array_length() {
    assert_eq!(WxMpTemplateIndustryEnum::ALL.len(), 41);
}

/// first_class / second_class / code for every variant are non-empty and unique codes.
#[test]
fn all_variants_have_first_class() {
    let mut codes = Vec::new();
    for &v in &WxMpTemplateIndustryEnum::ALL {
        assert!(!v.first_class().is_empty(), "empty first_class for {v:?}");
        assert!(!v.second_class().is_empty(), "empty second_class for {v:?}");
        let c = v.code();
        assert!((1..=41).contains(&c), "code out of range for {v:?}: {c}");
        codes.push(c);
    }
    codes.sort();
    codes.dedup();
    assert_eq!(codes.len(), 41, "codes must be unique 1..41");
}

/// Specific first_class/second_class spot checks for each category.
#[test]
fn first_class_it_tech() {
    assert_eq!(WxMpTemplateIndustryEnum::ECommerce.first_class(), "IT科技");
    assert_eq!(
        WxMpTemplateIndustryEnum::ItSoftwareAndServices.first_class(),
        "IT科技"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::ItHardwareAndEquipment.first_class(),
        "IT科技"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::ElectronicTechnique.first_class(),
        "IT科技"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::CommunicationAndOperator.first_class(),
        "IT科技"
    );
    assert_eq!(WxMpTemplateIndustryEnum::OnlineGame.first_class(), "IT科技");
}

#[test]
fn first_class_finance() {
    assert_eq!(WxMpTemplateIndustryEnum::Bank.first_class(), "金融业");
    assert_eq!(WxMpTemplateIndustryEnum::Fund.first_class(), "金融业");
    assert_eq!(WxMpTemplateIndustryEnum::Insurance.first_class(), "金融业");
}

#[test]
fn first_class_transport() {
    assert_eq!(
        WxMpTemplateIndustryEnum::Express.first_class(),
        "运输与仓储"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::Logistics.first_class(),
        "运输与仓储"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::Storage.first_class(),
        "运输与仓储"
    );
}

#[test]
fn first_class_education() {
    assert_eq!(WxMpTemplateIndustryEnum::Cultivate.first_class(), "教育");
    assert_eq!(WxMpTemplateIndustryEnum::Academy.first_class(), "教育");
}

#[test]
fn first_class_gov() {
    assert_eq!(
        WxMpTemplateIndustryEnum::AcademicResearch.first_class(),
        "政府与公共事业"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::TrafficPolice.first_class(),
        "政府与公共事业"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::Museum.first_class(),
        "政府与公共事业"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::PublicWorksNonprofit.first_class(),
        "政府与公共事业"
    );
}

#[test]
fn first_class_medical() {
    assert_eq!(
        WxMpTemplateIndustryEnum::MedicalHealth.first_class(),
        "医药护理"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::CareAndBeauty.first_class(),
        "医药护理"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::HealthAndHygiene.first_class(),
        "医药护理"
    );
}

#[test]
fn first_class_vehicle() {
    assert_eq!(
        WxMpTemplateIndustryEnum::AutomotiveRelated.first_class(),
        "交通工具"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::MotorcycleCorrelation.first_class(),
        "交通工具"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::TheTrainRelated.first_class(),
        "交通工具"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::ThePlaneRelated.first_class(),
        "交通工具"
    );
}

#[test]
fn first_class_real_estate() {
    assert_eq!(
        WxMpTemplateIndustryEnum::Architecture.first_class(),
        "房地产"
    );
    assert_eq!(WxMpTemplateIndustryEnum::RealEstate.first_class(), "房地产");
}

#[test]
fn first_class_commercial() {
    assert_eq!(
        WxMpTemplateIndustryEnum::Legislation.first_class(),
        "商业服务"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::ConventionAndExhibition.first_class(),
        "商业服务"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::IntermediaryServices.first_class(),
        "商业服务"
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::Authentication.first_class(),
        "商业服务"
    );
    assert_eq!(WxMpTemplateIndustryEnum::Audit.first_class(), "商业服务");
}

#[test]
fn first_class_entertainment() {
    assert_eq!(
        WxMpTemplateIndustryEnum::MassMedia.first_class(),
        "文体娱乐"
    );
    assert_eq!(WxMpTemplateIndustryEnum::Sports.first_class(), "文体娱乐");
    assert_eq!(
        WxMpTemplateIndustryEnum::LeisureAndEntertainment.first_class(),
        "文体娱乐"
    );
}

#[test]
fn second_class_spot_checks() {
    assert_eq!(
        WxMpTemplateIndustryEnum::ECommerce.second_class(),
        "互联网|电子商务"
    );
    assert_eq!(WxMpTemplateIndustryEnum::Bank.second_class(), "银行");
    assert_eq!(WxMpTemplateIndustryEnum::Repast.second_class(), "餐饮");
    assert_eq!(WxMpTemplateIndustryEnum::Hotel.second_class(), "酒店");
    assert_eq!(WxMpTemplateIndustryEnum::Travel.second_class(), "旅游");
    assert_eq!(WxMpTemplateIndustryEnum::Express.second_class(), "快递");
    assert_eq!(WxMpTemplateIndustryEnum::Other.second_class(), "其他");
    assert_eq!(WxMpTemplateIndustryEnum::Printing.second_class(), "印刷");
}

#[test]
fn code_spot_checks() {
    assert_eq!(WxMpTemplateIndustryEnum::ECommerce.code(), 1);
    assert_eq!(WxMpTemplateIndustryEnum::Bank.code(), 7);
    assert_eq!(WxMpTemplateIndustryEnum::Repast.code(), 10);
    assert_eq!(WxMpTemplateIndustryEnum::Other.code(), 41);
}

/// find_by_class: exact match and contains semantics.
#[test]
fn find_by_class_exact() {
    let e = WxMpTemplateIndustryEnum::find_by_class("IT科技", "互联网|电子商务");
    assert_eq!(e, Some(WxMpTemplateIndustryEnum::ECommerce));
}

#[test]
fn find_by_class_contains() {
    // second_class is matched via .contains()
    let e = WxMpTemplateIndustryEnum::find_by_class("IT科技", "电子商务");
    assert_eq!(e, Some(WxMpTemplateIndustryEnum::ECommerce));
}

#[test]
fn find_by_class_not_found() {
    assert!(WxMpTemplateIndustryEnum::find_by_class("不存在", "也不存在").is_none());
}

#[test]
fn find_by_class_wrong_first() {
    assert!(WxMpTemplateIndustryEnum::find_by_class("金融业", "互联网|电子商务").is_none());
}

/// find_by_code: exact and not found.
#[test]
fn find_by_code_exact() {
    assert_eq!(
        WxMpTemplateIndustryEnum::find_by_code(7),
        Some(WxMpTemplateIndustryEnum::Bank)
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::find_by_code(1),
        Some(WxMpTemplateIndustryEnum::ECommerce)
    );
    assert_eq!(
        WxMpTemplateIndustryEnum::find_by_code(41),
        Some(WxMpTemplateIndustryEnum::Other)
    );
}

#[test]
fn find_by_code_not_found() {
    assert!(WxMpTemplateIndustryEnum::find_by_code(0).is_none());
    assert!(WxMpTemplateIndustryEnum::find_by_code(42).is_none());
    assert!(WxMpTemplateIndustryEnum::find_by_code(-1).is_none());
}

/// Iterate all 41 variants and verify find_by_code roundtrip.
#[test]
fn find_by_code_roundtrip_all() {
    for &v in &WxMpTemplateIndustryEnum::ALL {
        assert_eq!(WxMpTemplateIndustryEnum::find_by_code(v.code()), Some(v));
    }
}

/// Serde roundtrip for all 41 variants.
#[test]
fn serde_roundtrip_all() {
    for &v in &WxMpTemplateIndustryEnum::ALL {
        let json = serde_json::to_string(&v).unwrap();
        let back: WxMpTemplateIndustryEnum = serde_json::from_str(&json).unwrap();
        assert_eq!(v, back, "roundtrip failed for {v:?}");
    }
}

/// Clone/Copy/Debug/PartialEq/Eq traits.
#[test]
fn traits_work() {
    let v = WxMpTemplateIndustryEnum::Bank;
    let cloned = v;
    assert_eq!(v, cloned);
    let _debug = format!("{v:?}");
}
