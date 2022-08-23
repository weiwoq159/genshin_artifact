use crate::attribute::{Attribute, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct MagicGuideEffect {
    pub rate: f64,
}

impl MagicGuideEffect {
    pub fn new(rate: f64) -> Self {
        Self { rate }
    }
}

impl<A: Attribute> WeaponEffect<A> for MagicGuideEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = (refine * 0.03 + 0.09) * self.rate;
        attribute.set_value_by(AttributeName::BonusBase, "魔导绪论被动等效", value);
    }
}

pub struct MagicGuide;

impl WeaponTrait for MagicGuide {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::MagicGuide,
        internal_name: "Catalyst_Intro",
        weapon_type: WeaponType::Catalyst,
        weapon_sub_stat: Some(WeaponSubStatFamily::EM41),
        weapon_base: WeaponBaseATKFamily::ATK354,
        star: 3,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("止水息雷：对处于水元素或雷元素影响下的敌人，造成的伤害提高12%/15%/18%/21%/24%。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "魔导绪论"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::RATE01
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let rate = match *config {
            WeaponConfig::MagicGuide { rate } => rate,
            _ => 0.0
        };

        Some(Box::new(MagicGuideEffect::new(rate)))
    }
}
