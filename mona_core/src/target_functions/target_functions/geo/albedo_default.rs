use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::{ArtifactEffectConfig, ArtifactEffectConfigBuilder, ConfigLevel, ConfigRate};
use crate::attribute::SimpleAttributeGraph2;
use crate::character::{Character, CharacterName};
use crate::character::character_common_data::CharacterCommonData;
use crate::character::traits::{CharacterTrait};
use crate::character::characters::Albedo;
use crate::character::skill_config::CharacterSkillConfig;
use crate::common::item_config_type::ItemConfig;
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_meta::{TargetFunctionFor, TargetFunctionMeta, TargetFunctionMetaImage};
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::{TargetFunction, TargetFunctionConfig, TargetFunctionName};
use crate::target_functions::target_function::TargetFunctionMetaTrait;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;
use crate::weapon::weapon_common_data::WeaponCommonData;

pub struct AlbedoDefaultTargetFunction;

impl TargetFunctionMetaTrait for AlbedoDefaultTargetFunction {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: TargetFunctionMeta = TargetFunctionMeta {
        name: TargetFunctionName::AlbedoDefault,
        chs: "阿贝多-白垩之子",
        description: "普通副C阿贝多",
        tags: "输出",
        four: TargetFunctionFor::SomeWho(CharacterName::Albedo),
        image: TargetFunctionMetaImage::Avatar,
    };

    fn create(_character: &CharacterCommonData, _weapon: &WeaponCommonData, _config: &TargetFunctionConfig) -> Box<dyn TargetFunction> {
        Box::new(AlbedoDefaultTargetFunction)
    }
}

impl TargetFunction for AlbedoDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.1,
            atk_percentage: 1.0,
            hp_fixed: 0.0,
            hp_percentage: 0.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 0.2,
            elemental_mastery: 0.3,
            critical: 1.0,
            critical_damage: 1.0,
            healing_bonus: 0.0,
            bonus_electro: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 2.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::ATKPercentage,
                StatName::DEFPercentage,
            ],
            goblet_main_stats: vec![
                StatName::GeoBonus,
                StatName::ATKPercentage,
                StatName::DEFPercentage,
            ],
            head_main_stats: vec![
                StatName::CriticalRate,
                StatName::CriticalDamage,
                StatName::DEFPercentage,
                StatName::ATKPercentage,
            ],
            set_names: Some(vec![
                ArtifactSetName::HuskOfOpulentDreams,
                ArtifactSetName::DefendersWill,
                ArtifactSetName::Gambler,
                ArtifactSetName::ArchaicPetra,
            ]),
            very_critical_set_names: None,
            normal_threshold: TargetFunctionOptConfig::DEFAULT_NORMAL_THRESHOLD,
            critical_threshold: TargetFunctionOptConfig::DEFAULT_CRITICAL_THRESHOLD,
            very_critical_threshold: TargetFunctionOptConfig::DEFAULT_VERY_CRITICAL_THRESHOLD
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfigBuilder::new()
            .husk_of_opulent_dreams(4.0)
            .build()
    }

    fn target(
        &self,
        attribute: &SimpleAttributeGraph2,
        character: &Character<SimpleAttributeGraph2>,
        _weapon: &Weapon<SimpleAttributeGraph2>,
        _artifacts: &[&Artifact],
        enemy: &Enemy) -> f64 {
        let context = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };

        type SkillEnum = <Albedo as CharacterTrait>::DamageEnumType;

        let config = CharacterSkillConfig::Albedo { fatal_count: 4 };

        let damage_transient_blossom = Albedo::damage::<SimpleDamageBuilder>(&context, SkillEnum::ETransientBlossom, &config, None);
        let damage_q_blossom = Albedo::damage::<SimpleDamageBuilder>(&context, SkillEnum::QFatalBlossom, &config, None);
        let damage_q1 = Albedo::damage::<SimpleDamageBuilder>(&context, SkillEnum::Q1, &config, None);

        damage_transient_blossom.normal.expectation * 14.0
        + damage_q1.normal.expectation * 1.0
        + damage_q_blossom.normal.expectation * 7.0
    }
}
