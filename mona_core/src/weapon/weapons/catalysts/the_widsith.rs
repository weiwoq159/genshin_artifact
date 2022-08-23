use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct TheWidsithEffect {
    r1: f64,
    r2: f64,
    r3: f64,
}

impl TheWidsithEffect {
    pub fn new(config: &WeaponConfig) -> TheWidsithEffect {
        let (r1, r2, r3) = match *config {
            WeaponConfig::TheWidsith { t1_rate, t2_rate, t3_rate } => (t1_rate, t2_rate, t3_rate),
            _ => (0.0, 0.0, 0.0)
        };

        Self {
            r1, r2, r3
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for TheWidsithEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;

        attribute.add_atk_percentage("流浪乐章被动等效（宣叙调）", (refine * 0.15 + 0.45) * self.r1);
        attribute.add_elemental_bonus("流浪乐章被动等效（咏叹调）", (refine * 0.12 + 0.36) * self.r2);
        attribute.set_value_by(AttributeName::ElementalMastery, "流浪乐章被动等效（间奏曲）", (refine * 60.0 + 180.0) * self.r3);
    }
}

pub struct TheWidsith;

impl WeaponTrait for TheWidsith {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::TheWidsith,
        internal_name: "Catalyst_Troupe",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage120),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("登场乐：角色登场时，随机获得一个主题曲，持续10秒。每30秒只能触发一次。宣叙调：攻击力提升60%/75%/90%/105%/120%；咏叹调：全元素伤害提升48%/60％/72%/84%/96%；间奏曲：元素精通提升240/300/360/420/480。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "流浪乐章"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "t1_rate",
            title: "w16",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "t2_rate",
            title: "w17",
            config: ItemConfig::RATE01_TYPE
        },
        ItemConfig {
            name: "t3_rate",
            title: "w18",
            config: ItemConfig::RATE01_TYPE
        },
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(TheWidsithEffect::new(config)))
    }
}
