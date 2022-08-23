use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};

pub struct FreedomSwornEffect {
    rate: f64,
}

impl FreedomSwornEffect {
    pub fn new(config: &WeaponConfig) -> FreedomSwornEffect {
        match *config {
            WeaponConfig::FreedomSworn { rate } => FreedomSwornEffect {
                rate,
            },
            _ => FreedomSwornEffect {
                rate: 0.0,
            },
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for FreedomSwornEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine;

        attribute.set_value_by(AttributeName::BonusBase, "苍古自由之誓被动", refine as f64 * 0.025 + 0.075);
        let dmg_bonus = (refine as f64 * 0.04 + 0.12) * self.rate;
        let atk_bonus = (refine as f64 * 0.05 + 0.15) * self.rate;
        attribute.set_value_by(AttributeName::BonusNormalAttack, "苍古自由之誓被动等效", dmg_bonus);
        attribute.set_value_by(AttributeName::BonusChargedAttack, "苍古自由之誓被动等效", dmg_bonus);
        attribute.set_value_by(AttributeName::BonusPlungingAttack, "苍古自由之誓被动等效", dmg_bonus);
        attribute.add_atk_percentage("苍古自由之誓被动等效", atk_bonus);
    }
}

pub struct FreedomSworn;

impl WeaponTrait for FreedomSworn {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FreedomSworn,
        internal_name: "Sword_Widsith",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM43),
        weapon_base: WeaponBaseATKFamily::ATK608,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("飘游风中的「千年的大乐章」的一部分。伤害提升10%/12.5%/15%/17.5%/20%；触发元素反应时，角色获得一枚奋起之符，每0.5秒内至多触发一次，角色处于队伍后台也能触发。拥有2枚奋起之符时，将消耗所有奋起之符，使附近队伍中所有角色获得持续12秒的「千年的大乐章·抗争之歌」效果：普通攻击、重击、下落攻击造成的伤害提升16%/20%/24%/28%/32%，攻击力提升20%/25%/30%/35%/40%。触发后20秒内，无法再次获得奋起之符。「千年的大乐章」触发的多种数值效果中，同类数值效果不可叠加。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "苍古自由之誓"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "w2",
            config: ItemConfigType::Float {
                min: 0.0,
                max: 1.0,
                default: 0.0
            }
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(FreedomSwornEffect::new(config)))
    }
}
