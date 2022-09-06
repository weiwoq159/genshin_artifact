use crate::attribute::{Attribute, AttributeName};
use crate::common::{ChangeAttribute, Element, SkillType, StatName, WeaponType};
// use strum_macros::{EnumCount as EnumCountMacro, EnumString};
// use num_derive::FromPrimitive;
// use strum::EnumCount;
use crate::character::character_common_data::CharacterCommonData;
use crate::character::{CharacterConfig, CharacterName, CharacterStaticData};
use crate::character::character_sub_stat::CharacterSubStatFamily;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::{CharacterSkillMap, CharacterSkillMapItem, CharacterTrait};
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::damage::damage_builder::DamageBuilder;
use crate::damage::DamageContext;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::macros::{skill_type, damage_enum};

// pub struct CynoSkillType {
//     pub normal_dmg1: [f64; 15],
//     pub normal_dmg2: [f64; 15],
//     pub normal_dmg3: [f64; 15],
//     pub normal_dmg4: [f64; 15],
//     pub charged_dmg1: [f64; 15],
//     pub charged_dmg2: [f64; 15],
//     pub charged_dmg3: [f64; 15],
//     pub charged_dmg4: [f64; 15],
//     pub plunging_dmg1: [f64; 15],
//     pub plunging_dmg2: [f64; 15],
//     pub plunging_dmg3: [f64; 15],
//
//     pub e1: [f64; 15],
//
//     pub q1: [f64; 15],
//     pub q2: [f64; 15],
// }


//
skill_type!(CynoSkillType normal_dmg1 normal_dmg2 normal_dmg3 normal_dmg4 charged_dmg plunging_dmg1 plunging_dmg2 plunging_dmg3 e1 e2 qnormal1 qnormal2 qnormal3 qnormal4 qnormal5 qcharged qplunging1 qplunging2 qplunging3);

pub const CYNO_SKILL: CynoSkillType = CynoSkillType {
    normal_dmg1: [0.4926,0.5327,0.5728,0.63,0.6701,0.716,0.779,0.842,0.905,0.9737,1.0424,1.1112,1.1799,1.2486,1.3173],
    normal_dmg2: [0.4792,0.5182,0.5572,0.6129,0.6519,0.6965,0.7578,0.8191,0.8804,0.9473,1.0141,1.081,1.1479,1.2147,1.2816],
    normal_dmg3: [0.2931,0.3169,0.3408,0.3748,0.3987,0.426,0.4634,0.5009,0.5384,0.5793,0.6202,0.6611,0.702,0.7429,0.7838],
    normal_dmg4: [0.7589,0.8207,0.8824,0.9707,1.0325,1.1031,1.2001,1.2972,1.3943,1.5002,1.6061,1.712,1.8178,1.9237,2.0296],
    charged_dmg: [1.2238,1.3234,1.423,1.5653,1.6649,1.7787,1.9353,2.0918,2.2483,2.4191,2.5899,2.7606,2.9314,3.1021,3.2729],
    plunging_dmg1: [0.6393,0.6914,0.7434,0.8177,0.8698,0.9293,1.011,1.0928,1.1746,1.2638,1.353,1.4422,1.5314,1.6206,1.7098],
    plunging_dmg2: [1.2784,1.3824,1.4865,1.6351,1.7392,1.8581,2.0216,2.1851,2.3486,2.527,2.7054,2.8838,3.0622,3.2405,3.4189],
    plunging_dmg3: [1.5968,1.7267,1.8567,2.0424,2.1723,2.3209,2.5251,2.7293,2.9336,3.1564,3.3792,3.602,3.8248,4.0476,4.2704],

    e1: [1.304,1.4018,1.4996,1.63,1.7278,1.8256,1.956,2.0864,2.2168,2.3472,2.4776,2.608,2.771,2.934,3.097],
    e2: [1.568,1.6856,1.8032,1.96,2.0776,2.1952,2.352,2.5088,2.6656,2.8224,2.9792,3.136,3.332,3.528,3.724],

    qnormal1: [0.7828,0.8466,0.9103,1.0013,1.065,1.1378,1.238,1.3381,1.4382,1.5475,1.6567,1.7659,1.8752,1.9844,2.0936],
    qnormal2: [0.8247,0.8918,0.9589,1.0548,1.122,1.1987,1.3042,1.4096,1.5151,1.6302,1.7453,1.8603,1.9754,2.0905,2.2056],
    qnormal3: [1.0463,1.1315,1.2167,1.3383,1.4235,1.5208,1.6547,1.7885,1.9223,2.0683,2.2143,2.3603,2.5063,2.6523,2.7983],
    qnormal4: [0.5169,0.559,0.6011,0.6612,0.7033,0.7514,0.8175,0.8836,0.9497,1.0219,1.094,1.1661,1.2383,1.3104,1.3825],
    qnormal5: [1.3084,1.4149,1.5215,1.6736,1.7801,1.9018,2.0692,2.2365,2.4039,2.5865,2.769,2.9516,3.1342,3.3168,3.4993],
    qcharged: [1.0105,1.0927,1.175,1.2925,1.3748,1.4688,1.598,1.7272,1.8565,1.9975,2.1385,2.2795,2.4205,2.5615,2.7025],
    qplunging1: [0.6393,0.6914,0.7434,0.8177,0.8698,0.9293,1.011,1.0928,1.1746,1.2638,1.353,1.4422,1.5314,1.6206,1.7098],
    qplunging2: [1.2784,1.3824,1.4865,1.6351,1.7392,1.8581,2.0216,2.1851,2.3486,2.527,2.7054,2.8838,3.0622,3.2405,3.4189],
    qplunging3: [1.5968,1.7267,1.8567,2.0424,2.1723,2.3209,2.5251,2.7293,2.9336,3.1564,3.3792,3.602,3.8248,4.0476,4.2704],

};

pub struct CynoEffect {
    pub has_talent1: bool,
    pub has_talent2: bool,
    pub after_q:bool,
    pub c2: bool,
    pub c2_stack: f64,
}

impl<A: Attribute> ChangeAttribute<A> for CynoEffect {
    fn change_attribute(&self, attribute: &mut A) {
        if self.c2 {
            attribute.set_value_by(AttributeName::BonusElectro, "赛诺2命：「令仪·引谒归灵」", 0.1 * self.c2_stack);
        }
        if self.after_q {
            attribute.set_value_by(AttributeName::ElementalMastery, "「启途誓使」精通加成", 100.0);
        }
    }
}

// #[derive(Copy, Clone, Eq, PartialEq, EnumString)]
// #[derive(FromPrimitive, EnumCountMacro)]
// pub enum CynoDamageEnum {
//     Normal1,
//     Normal2,
//     Normal3,
//     Normal4,
//     Charged1,
//     Charged2,
//     Charged3,
//     Charged4,
//     Plunging1,
//     Plunging2,
//     Plunging3,
//     E1,
//     Q1,
//     Q2
// }
//
// impl Into<usize> for CynoDamageEnum {
//     fn into(self) -> usize {
//         self as usize
//     }
// }

damage_enum!(
    CynoDamageEnum
    Normal1
    Normal2
    Normal3
    Normal4
    Charged1
    Plunging1
    Plunging2
    Plunging3
    E1
    E2
    E3
    QNormal1
    QNormal2
    QNormal3
    QNormal4
    QNormal5
    QCharged1
    QPlunging1
    QPlunging2
    QPlunging3
);

#[derive(Copy, Clone, Eq, PartialEq, EnumString)]
#[derive(FromPrimitive, EnumCountMacro)]
pub enum CynoSkillgroupEnum {
    NoQ,
    E1,
    E2,
    E3,
    Q
}

impl CynoDamageEnum {
    pub fn get_element(&self) -> Element {
        use CynoDamageEnum::*;
        match *self {
            E1 | E2 | E3 | QNormal1 | QNormal2 | QNormal3 | QNormal4 |QNormal5| QCharged1| QPlunging1 | QPlunging2| QPlunging3 => Element::Electro,
            _ => Element::Physical
        }
    }

    pub fn get_skill_type(&self) -> SkillType {
        use CynoDamageEnum::*;
        match *self {
            Normal1 | Normal2 | Normal3 | Normal4 | QNormal1 | QNormal2 | QNormal3 | QNormal4 | QNormal5 => SkillType::NormalAttack,
            Charged1 | QCharged1 => SkillType::ChargedAttack,
            E1 | E2 | E3 => SkillType::ElementalSkill,
            Plunging1 | Plunging2 | Plunging3 | QPlunging1 | QPlunging2 | QPlunging3 => SkillType::PlungingAttack,
            //None => SkillType::ElementalBurst
        }
    }

    pub fn get_skill_group(&self) -> CynoSkillgroupEnum {
        use CynoDamageEnum::*;
        match *self {
            Normal1|Normal2|Normal3|Normal4|Plunging1|Plunging2|Plunging3|Charged1 => CynoSkillgroupEnum::NoQ,
            E1 => CynoSkillgroupEnum::E1,
            E2 => CynoSkillgroupEnum::E2,
            E3 => CynoSkillgroupEnum::E3,
            QNormal1|QNormal2|QNormal3|QNormal4|QNormal5|QPlunging1|QPlunging2|QPlunging3|QCharged1 => CynoSkillgroupEnum::Q,
        }
    }
}

pub struct Cyno;

impl CharacterTrait for Cyno {
    const STATIC_DATA: CharacterStaticData = CharacterStaticData {
        name: CharacterName::Cyno,
        internal_name: "Cyno",
        chs: "赛诺",
        element: Element::Electro,
        hp: [972,2522,3356,5022,5614,6459,7249,8103,8695,9557,10149,11020,11613,12491],
        atk: [25,64,85,128,143,165,185,206,221,243,258,281,296,318],
        def: [67,174,231,345,386,444,499,557,598,657,698,758,799,859],
        sub_stat: CharacterSubStatFamily::CriticalDamage384,
        weapon_type: WeaponType::Polearm,
        star: 5,
        skill_name1: "普通攻击·七圣枪术",
        skill_name2: "秘仪·律渊渡魂",
        skill_name3: "圣仪·煟煌随狼行"
    };
    type SkillType = CynoSkillType;
    const SKILL: Self::SkillType = CYNO_SKILL;
    type DamageEnumType = CynoDamageEnum;
    type RoleEnum = ();

    #[cfg(not(target_family = "wasm"))]
    const SKILL_MAP: CharacterSkillMap = CharacterSkillMap {
        skill1: Some(&[
            CharacterSkillMapItem { index: CynoDamageEnum::Normal1 as usize, chs: "一段伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::Normal2 as usize, chs: "二段伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::Normal3 as usize, chs: "三段伤害/2" },
            CharacterSkillMapItem { index: CynoDamageEnum::Normal4 as usize, chs: "四段伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::Charged1 as usize, chs: "重击伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::Plunging1 as usize, chs: "下坠期间伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::Plunging2 as usize, chs: "低空坠地冲击伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::Plunging3 as usize, chs: "高空坠地冲击伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::QNormal1 as usize, chs: "启途誓使：一段伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::QNormal2 as usize, chs: "启途誓使：二段伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::QNormal3 as usize, chs: "启途誓使：三段伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::QNormal4 as usize, chs: "启途誓使：四段伤害/2" },
            CharacterSkillMapItem { index: CynoDamageEnum::QNormal5 as usize, chs: "启途誓使：五段伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::QCharged1 as usize, chs: "启途誓使：重击伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::QPlunging1 as usize, chs: "启途誓使：下坠期间伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::QPlunging2 as usize, chs: "启途誓使：低空坠地冲击伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::QPlunging3 as usize, chs: "启途誓使：高空坠地冲击伤害" },
        ]),
        skill2: Some(&[
            CharacterSkillMapItem { index: CynoDamageEnum::E1 as usize, chs: "技能伤害" },
            CharacterSkillMapItem { index: CynoDamageEnum::E2 as usize, chs: "启途誓使：冥祭" },
            CharacterSkillMapItem { index: CynoDamageEnum::E3 as usize, chs: "启途誓使：渡荒之雷" }
        ]),
        skill3: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "c2_stack",
            title: "c43",
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 4.0 },
        },
        ItemConfig {
            name: "after_q",
            title: "c41",
            config: ItemConfigType::Bool { default: true }
        },
    ]);

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_SKILL: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "under_judication",
            title: "c42",
            config: ItemConfigType::Bool { default: true }
        },
        // ItemConfig {
        //     name: "after_q",
        //     title: "c41",
        //     config: ItemConfigType::Bool { default: true }
        // },
    ]);

    fn damage_internal<D: DamageBuilder>(context: &DamageContext<'_, D::AttributeType>, s: usize, config: &CharacterSkillConfig, fumo: Option<Element>) -> D::Result {
        let s: CynoDamageEnum = num::FromPrimitive::from_usize(s).unwrap();
        let (s1, s2, s3) = context.character_common_data.get_3_skill();

        use CynoDamageEnum::*;
        let ratio = match s {
            Normal1 => CYNO_SKILL.normal_dmg1[s1],
            Normal2 => CYNO_SKILL.normal_dmg2[s1],
            Normal3 => CYNO_SKILL.normal_dmg3[s1],
            Normal4 => CYNO_SKILL.normal_dmg4[s1],
            Charged1 => CYNO_SKILL.charged_dmg[s1],
            Plunging1 => CYNO_SKILL.plunging_dmg1[s1],
            Plunging2 => CYNO_SKILL.plunging_dmg2[s1],
            Plunging3 => CYNO_SKILL.plunging_dmg3[s1],
            QNormal1 => CYNO_SKILL.qnormal1[s3],
            QNormal2 => CYNO_SKILL.qnormal2[s3],
            QNormal3 => CYNO_SKILL.qnormal3[s3],
            QNormal4 => CYNO_SKILL.qnormal4[s3],
            QNormal5 => CYNO_SKILL.qnormal5[s3],
            QCharged1 => CYNO_SKILL.qcharged[s3],
            QPlunging1 => CYNO_SKILL.qplunging1[s3],
            QPlunging2 => CYNO_SKILL.qplunging2[s3],
            QPlunging3 => CYNO_SKILL.qplunging3[s3],
            E1 => CYNO_SKILL.e1[s2],
            E2 => CYNO_SKILL.e2[s2],
            E3 => 1.0,
        };

        let mut builder = D::new();
        builder.add_atk_ratio("技能倍率", ratio);

        let skill_type = s.get_skill_type();
        let skill_group = s.get_skill_group();
        let has_talent1=context.character_common_data.has_talent1;
        let has_talent2=context.character_common_data.has_talent2;
        let (under_judication/* ,after_q */)= match *config {
            CharacterSkillConfig::Cyno {under_judication/* , after_q */ } => (under_judication/* ,after_q */),
            _ => (false/* ,false */)
        };
        // if context.character_common_data.has_talent2 && (skill_type == SkillType::ChargedAttack || skill_type == SkillType::ElementalBurst) {
        //     let em = context.attribute.get_value(AttributeName::ElementalMastery);
        //     let bonus = 0.6_f64.min(0.0006 * em);
        //     builder.add_extra_bonus("提纳里天赋「诸叶辨通」", bonus);
        // }

        //q em bonus
        // if after_q {
        //     builder.add_extra_em("启途誓使精通加成", 100.0);
        // }
        //talent2: add 35 dmg bonus to e
        if skill_group==CynoSkillgroupEnum::E2 && has_talent1 && under_judication {
            builder.add_extra_bonus("「裁定」效果",0.35);
        }

        //normal attacks under q talent3
        if has_talent2 {
            let em = context.attribute.get_value(AttributeName::ElementalMastery);
            if skill_group == CynoSkillgroupEnum::Q && skill_type == SkillType::NormalAttack {
                builder.add_extra_damage("天赋3：「九弓的执命」加成",em*1.5);
            }
            if skill_group == CynoSkillgroupEnum::E3 && skill_type == SkillType::ElementalSkill {
                builder.add_extra_damage("天赋3：「九弓的执命」加成",em*2.5);
            }

        }

        builder.damage(
            &context.attribute,
            &context.enemy,
            s.get_element(),
            skill_type,
            context.character_common_data.level,
            fumo,
        )
    }

    fn new_effect<A: Attribute>(common_data: &CharacterCommonData, config: &CharacterConfig) -> Option<Box<dyn ChangeAttribute<A>>> {
        // let (talent1_ratio, c2_ratio) = match *config {
        //     CharacterConfig::Cyno { talent1_ratio, c2_ratio } => (talent1_ratio, c2_ratio),
        //     _ => (0.0, 0.0)
        // };
        let (c2_stack,after_q)= match *config {
            CharacterConfig::Cyno {c2_stack ,after_q} => (c2_stack,after_q),
            _ => (0.0,false)
        };

        Some(Box::new(CynoEffect {
            has_talent1: common_data.has_talent1,
            has_talent2: common_data.has_talent2,
            after_q,
            c2: common_data.constellation >= 2,
            c2_stack
        }))
    }

    fn get_target_function_by_role(role_index: usize, team: &TeamQuantization, c: &CharacterCommonData, w: &WeaponCommonData) -> Box<dyn TargetFunction> {
        unimplemented!()
    }
}