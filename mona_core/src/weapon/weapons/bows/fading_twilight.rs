use crate::attribute::{Attribute, AttributeName};
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

pub struct FadingTwilightEffect {
    pub state: usize,
}

impl<A: Attribute> WeaponEffect<A> for FadingTwilightEffect {
    fn apply(&self, data: &WeaponCommonData, attribute: &mut A) {
        let refine = data.refine as f64;
        let value = if self.state == 0 {
            refine * 0.015 + 0.045
        } else if self.state == 1 {
            refine * 0.025 + 0.075
        } else {
            refine * 0.035 + 0.105
        };

        attribute.set_value_by(AttributeName::BonusBase, "「落霞」被动", value);
    }
}

pub struct FadingTwilight;

impl WeaponTrait for FadingTwilight {
    const META_DATA: WeaponStaticData = WeaponStaticData {
        name: WeaponName::FadingTwilight,
        internal_name: "Bow_Fallensun",
        weapon_type: WeaponType::Bow,
        weapon_sub_stat: Some(WeaponSubStatFamily::Recharge67),
        weapon_base: WeaponBaseATKFamily::ATK565,
        star: 4,
        #[cfg(not(target_family = "wasm"))]
        effect: Some("具有夕暮、流霞、朝晖三种状态，分别能使造成的伤害提升(6/7.5/9/10.5/12)%:(10/12.5/15/17.5/20)%:(14/17.5/21/24.5/28)%。攻击命中敌人后，将转换为下一种状态，每7秒至多转换一次状态。装备该武器的角色处于队伍后台时该效果依旧能触发转换。"),
        #[cfg(not(target_family = "wasm"))]
        chs: "落霞"
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG_DATA: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "state",
            title: "w7",
            config: ItemConfigType::Option { options: "夕暮,流霞,朝晖", default: 2 }
        }
    ]);

    fn get_effect<A: Attribute>(character: &CharacterCommonData, config: &WeaponConfig) -> Option<Box<dyn WeaponEffect<A>>> {
        let state = match *config {
            WeaponConfig::FadingTwilight { state } => state,
            _ => 2
        };

        Some(Box::new(FadingTwilightEffect {
            state
        }))
    }
}
