use crate::attribute::{Attribute, AttributeCommon, AttributeName};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::WeaponType;
use crate::weapon::weapon_common_data::WeaponCommonData;
use crate::weapon::weapon_effect::WeaponEffect;
use crate::weapon::weapon_static_data::WeaponStaticData;
use crate::weapon::weapon_trait::WeaponTrait;
use crate::weapon::{WeaponConfig, WeaponName};
use crate::weapon::weapon_base_atk::WeaponBaseATKFamily;
use crate::weapon::weapon_sub_stat::WeaponSubStatFamily;

pub struct AquaSimulacraEffect {
    pub is_enemy_around: bool
}

impl<A: Attribute> WeaponEffect<A> for AquaSimulacraEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let v1 = refine * 0.04 + 0.12;
        attribute.add_hp_percentage("「若水」被动", v1);

        if self.is_enemy_around {
            let v2 = refine * 0.05 + 0.15;
            attribute.set_value_by(AttributeName::BonusBase, "「若水」被动", v2);
        }
    }
}

pub struct AquaSimulacra;

impl WeaponTrait for AquaSimulacra {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::AquaSimulacra,
        internal_name: "Bow_Kirin",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::CriticalDamage192),
        weapon_base: WeaponBaseATKFamily::ATK542,
        star: 5,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("生命值提升16/20/24/28/32%。周围存在敌人时，装备该武器的角色造成的伤害都会提升20/25/30/35/40%，不论该角色处于场上或是处于队伍后台。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "若水"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "is_enemy_around",
            title: "w6",
            config: ItemConfigType::Bool { default: true }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let is_enemy_around = match *config {
            WeaponConfig::AquaSimulacra { is_enemy_around } => is_enemy_around,
            _ => true
        };

        Some(Box::new(AquaSimulacraEffect {
            is_enemy_around
        }))
    }
}