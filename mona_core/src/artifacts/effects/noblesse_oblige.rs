use crate::artifacts::artifact_trait::{ArtifactMetaData, ArtifactTrait};
use crate::artifacts::ArtifactSetName;
use super::super::effect::ArtifactEffect;
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, AttributeCommon};
use crate::character::character_common_data::CharacterCommonData;
use crate::common::item_config_type::ItemConfig;

pub struct NoblesseObligeEffect {
    pub rate: f64,
}

impl NoblesseObligeEffect {
    pub fn new(config: &ArtifactEffectConfig) -> NoblesseObligeEffect {
        NoblesseObligeEffect {
            rate: config.config_noblesse_oblige.rate,
        }
    }
}

impl<T: Attribute> ArtifactEffect<T> for NoblesseObligeEffect {
    fn effect2(&self, attribute: &mut T) {
        attribute.set_value_by(AttributeName::BonusElementalBurst, "昔日宗室之仪2", 0.2);
    }

    fn effect4(&self, attribute: &mut T) {
        attribute.add_atk_percentage("昔日宗室之仪4", self.rate * 0.2);
        attribute.set_value_by(AttributeName::ATKBonusForOther, "昔日宗室之仪4", self.rate * 0.2);
    }
}

pub struct NoblesseOblige;

impl ArtifactTrait for NoblesseOblige {
    fn create_effect<A: Attribute>(config: &ArtifactEffectConfig, _character_common_data: &CharacterCommonData) -> Box<dyn ArtifactEffect<A>> {
        Box::new(NoblesseObligeEffect::new(config))
    }

    #[cfg(not(target_family = "wasm"))]
    const META_DATA: ArtifactMetaData = ArtifactMetaData {
        name: ArtifactSetName::NoblesseOblige,
        name_mona: "noblesseOblige",
        chs: "昔日宗室之仪",
        flower: Some("宗室之花"),
        feather: Some("宗室之翎"),
        sand: Some("宗室时计"),
        goblet: Some("宗室银瓮"),
        head: Some("宗室面具"),
        star: (4, 5),
        effect1: None,
        effect2: Some("元素爆发造成的伤害提升20％。"),
        effect3: None,
        effect4: Some("施放元素爆发后，队伍中所有角色攻击力提升20％，持续12秒。该效果不可叠加。"),
        effect5: None
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG4: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "rate",
            title: "a2",
            config: ItemConfig::RATE01_TYPE
        }
    ]);
}
