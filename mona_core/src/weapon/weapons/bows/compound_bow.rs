use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_trait::WeaponTrait;

pub struct CompoundBowEffect {
    stack: f64
}

impl CompoundBowEffect {
    pub fn new(config: &WeaponConfig) -> CompoundBowEffect {
        match *config {
            WeaponConfig::CompoundBow { stack } => CompoundBowEffect {
                stack
            },
            _ => CompoundBowEffect {
                stack: 0.0
            }
        }
    }
}

impl<T: Attribute> WeaponEffect<T> for CompoundBowEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut T) {
        let refine = data.refine as f64;
        let atk_bonus = (refine * 0.01 + 0.03) * self.stack;
        let speed_bonus = (refine * 0.003 + 0.009) * self.stack;
        attribute.add_atk_percentage("钢轮弓被动等效", atk_bonus);
        attribute.set_value_by(AttributeName::SpeedNormalAttack, "钢轮弓被动等效", speed_bonus);
    }
}

pub struct CompoundBow;

impl WeaponTrait for CompoundBow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CompoundBow,
        internal_name: "Bow_Exotic",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus150),
        weapon_base: WeaponBaseATKFamily::ATK454,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("注能之矢：普通攻击和瞄准攻击命中时，提升4%/5%/6%/7%/8%攻击力与1.2%/1.5%/1.8%/2.1%/2.4%普通攻击速度。该效果持续6秒，最多可以叠加4层，每0.3秒只能触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "钢轮弓"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::STACK04
    ]);

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        Some(Box::new(CompoundBowEffect::new(config)))
    }
}
