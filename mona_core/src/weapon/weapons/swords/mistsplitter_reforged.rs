use crate::attribute::{AttributeName, Attribute};
use crate::common::{Element, WeaponType};
use super::super::super::weapon_effect::WeaponEffect;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::character::Character;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::WeaponName;
use super::super::super::weapon_config::WeaponConfig;

pub struct MistsplitterReforgedEffect {
    pub level: i32,
    pub element: Element,
}

impl<T: Attribute> WeaponEffect<T> for MistsplitterReforgedEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value1 = data.refine as f64 * 0.03 + 0.09;
        let key = "雾切之回光被动";
        attribute.set_value_by(AttributeName::BonusElectro, key, value1);
        attribute.set_value_by(AttributeName::BonusHydro, key, value1);
        attribute.set_value_by(AttributeName::BonusAnemo, key, value1);
        attribute.set_value_by(AttributeName::BonusPyro, key, value1);
        attribute.set_value_by(AttributeName::BonusCryo, key, value1);
        attribute.set_value_by(AttributeName::BonusDendro, key, value1);
        attribute.set_value_by(AttributeName::BonusGeo, key, value1);

        let value2 = if self.level == 1 {
            0.02 * data.refine as f64 + 0.06
        } else if self.level == 2 {
            0.04 * data.refine as f64 + 0.12
        } else if self.level == 3 {
            0.07 * data.refine as f64 + 0.21
        } else {
            0.0
        };

        let attribute_name = AttributeName::bonus_name_by_element(self.element);
        attribute.set_value_by(attribute_name, key, value2);
    }
}

impl MistsplitterReforgedEffect {
    pub fn new(config: &WeaponConfig, element: Element) -> MistsplitterReforgedEffect {
        let level = match *config {
            WeaponConfig::MistsplitterReforged { emblem_level } => emblem_level,
            _ => 0
        };

        MistsplitterReforgedEffect {
            element,
            level: level as i32
        }
    }
}

pub struct MistsplitterReforged;

impl WeaponTrait for MistsplitterReforged {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MistsplitterReforged,
        internal_name: "Sword_Narukami",
        weapon_type: WeaponType::Sword,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage96),
        weapon_base: WeaponBaseATKFamily::ATK674,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("获得12/15/18/21/24%所有元素伤害加成，并能获得「雾切之巴印」的威势。雾切之巴印：持有1/2/3层雾切之巴印时，获得(8/10/12/14/16%)/(16/20/24/28/32%)/(28/35/42/49/56)%自己的元素类型的元素伤害加成。在下列情况下，角色将各获得1层雾切之巴印：普通攻击造成元素伤害时，持续5秒；施放元素爆发时，持续10秒；此外，角色元素能量低于100%时，将获得1层雾切之巴印，此雾切之巴印会在角色的元素能量充满时消失。每层雾切之巴印的持续时间独立计算。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "雾切之回光"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "emblem_level",
            title: "w26",
            config: ItemConfigType::Int {
                min: 0,
                max: 3,
                default: 3
            }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(MistsplitterReforgedEffect::new(config, character.static_data.element)))
    }
}
