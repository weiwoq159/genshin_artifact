use crate::attribute::Attribute;
use crate::character::character_common_data::CharacterCommonData;
use crate::common::WeaponType;
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_effect::WeaponEffect;

pub struct FavoniusLance;

impl WeaponTrait for FavoniusLance {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FavoniusLance,
        internal_name: "Pole_Zephyrus",
        weapon_type: WeaponType::Polearm,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("顺风而行：攻击造成暴击时，有60%/70%/80%/90%/100%的几率产生少量元素微粒，能为角色恢复6点元素能量。该效果每12/10.5/9/7.5/6秒只能触发一次。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "西风长枪"
    };

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        None
    }
}
