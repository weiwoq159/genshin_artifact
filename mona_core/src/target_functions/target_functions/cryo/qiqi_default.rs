use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::characters::qiqi::Qiqi;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct QiqiDefaultTargetFunction {
    pub recharge_demand: f64
}

impl QiqiDefaultTargetFunction {
    pub fn new(config: &TargetFunctionConfig) -> QiqiDefaultTargetFunction {
        let recharge_demand = match *config {
            TargetFunctionConfig::QiqiDefault { recharge_demand } => recharge_demand,
            _ => 1.0
        };
        QiqiDefaultTargetFunction {
            recharge_demand
        }
    }
}

impl TargetFunctionMetaTrait for QiqiDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::QiqiDefault,
        chs: "七七-冻冻回魂夜",
        description: "普通治疗辅助七七",
        tags: "治疗",
        four: TargetFunctionFor::SomeWho(CharacterName::Qiqi),
        image: TargetFunctionMetaImage::Avatar
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig {
            name: "recharge_demand",
            title: ItemConfig::DEFAULT_RECHARGE_TITLE,
            config: ItemConfigType::Float { min: 1.0, max: 3.0, default: 1.6 }
        }
    ]);

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(QiqiDefaultTargetFunction::new(config))
    }
}

impl TargetFunction for QiqiDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 1.0,
            elemental_mastery: 0.0,
            critical: 0.0,
            critical_damage: 0.0,
            healing_bonus: 0.0,
            bonus_electro: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::Recharge,
            ],
            goblet_main_stats: vec![
                StatName::ATKPercentage,
            ],
            head_main_stats: vec![
                StatName::HealingBonus,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::MaidenBeloved,
                ArtifactSetName::ShimenawasReminiscence,
                ArtifactSetName::GladiatorsFinale,
                ArtifactSetName::EmblemOfSeveredFate,
                ArtifactSetName::OceanHuedClam,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        Default::default()
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, _artifacts: &[&Artifact], enemy: &Enemy) -> f64 {
        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute, enemy
        };

        type S = <Qiqi as CharacterTrait>::DamageEnumType;
        let heal_e = Qiqi::damage::<SimpleDamageBuilder>(&context, S::EHeal2, &CharacterSkillConfig::NoConfig, None).normal.expectation;

        let r = attribute.get_value(AttributeName::Recharge).min(self.recharge_demand);

        r * heal_e
    }
}
