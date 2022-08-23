use crate::attribute::{Attribute, AttributeName};
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

pub struct  SerpentSpineEffect {
    stack: f64
}

impl SerpentSpineEffect {
    pub fn new(config: &WeaponConfig) -> SerpentSpineEffect {
        match *config {
            WeaponConfig::SerpentSpine { stack } => SerpentSpineEffect {
                stack
            },
            _ => SerpentSpineEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for SerpentSpineEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let value = (data.refine as f64 * 0.01 + 0.05) * self.stack;
        attribute.set_value_by(AttributeName::BonusBase, "螭骨剑被动等效", value);
    }
}

pub struct SerpentSpine;

impl WeaponTrait for SerpentSpine {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::SerpentSpine,
        internal_name: "Claymore_Kione",
        weapon_type: WeaponType::Claymore,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalRate60),
        weapon_base: WeaponBaseATKFamily::ATK510,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("破浪：角色在场上时，每4秒提升6%/7%/8%/9%/10%造成的伤害，3%/2.7%/2.4%/2.2%/2%受到的伤害。该效果最多叠加5层，不随角色退场重置，受到伤害后会减少1层效果"),
        #[cfg(not(target_family = "wasm"))]
        chs: "螭骨剑"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "stack",
            title: "w1",
            config: ItemConfigType::Float { min: 0.0, max: 5.0, default: 0.0 },
        }
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(SerpentSpineEffect::new(config)))
    }
}
