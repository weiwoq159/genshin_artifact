use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;

pub struct HuntersBow;

impl WeaponTrait for HuntersBow {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::HuntersBow,
        internal_name: "Bow_Hunters",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: None,
        weapon_base: WeaponBaseATKFamily::ATK185,
        star: 1,
        #[cfg(not(target_family = "wasm"))]
        effect: None,
        #[cfg(not(target_family = "wasm"))]
        chs: "猎弓"
    };

    fn get_effect<A: Attribute>(_character: &CharacterCommonData, _config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
