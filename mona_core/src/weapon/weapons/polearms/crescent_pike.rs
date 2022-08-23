use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct CrescentPike;

impl WeaponTrait for CrescentPike {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::CrescentPike,
        internal_name: "Pole_Exotic",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::PhysicalBonus75),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("注能之针：获得元素微粒或元素晶球后的5秒内，普通攻击和重击额外造成20%/25%/30%/35%/40%攻击力伤害。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "流月针"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
