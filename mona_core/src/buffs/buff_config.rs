use serde::{Serialize, Deserialize};
use crate::attribute::Attribute;
use crate::buffs::Buff;
use crate::buffs::buffs::get_buff;
use crate::common::Element;

#[derive(Serialize, Deserialize)]
pub enum BuffConfig {
    ATKPercentage { p: f64 },
    DEFPercentage { p: f64 },
    HPPercentage { p: f64 },
    ATKFixed { value: f64 },
    DEFFixed { value: f64 },
    HPFixed { value: f64 },
    Critical { p: f64 },
    CriticalDamage { p: f64 },
    CustomBonus { p: f64 },
    ElementalMastery { value: f64 },
    Recharge { p: f64 },
    DEFMinus { p: f64 },
    ResMinus { p: f64 },
    HealingBonus { p: f64 },
    BaseDmg { value: f64 },

    BennettQ { base_atk: f64, c1: bool, skill3: usize },
    EulaE { skill2: usize },
    GorouE1 { skill2: usize },
    GorouC6 { level: usize },
    KaedeharaKazuhaTalent2 { element: Element, em: f64 },
    KujouSaraEOrQ { c6: bool, base_atk: f64, skill2: usize },
    MonaQ { c4: bool, skill3: usize },
    RaidenShogunE { energy: usize, skill2: usize },
    RosariaTalent2 { crit: f64 },
    ShenheE { atk: f64, skill2: usize },
    ShenheQ { skill3: usize },
    ShenheTalent1 { c2: bool },
    ShenheTalent2 { t: usize },
    SucroseTalent2 { em: f64 },
    SucroseC6 { element: Element },
    ThomaTalent1 { stack: f64 },
    VentiC2 { levitating: bool },
    VentiC6 { is_convert: bool, element: Element },
    YoimiyaTalent2 { talent1_stack: usize },
    YunjinQ { talent2: bool, skill3: usize, def: f64, ele_count: usize },
    YelanTalent2 { secs: usize },
    YelanC4 { count: usize },
    KamisatoAyatoQ { skill_level: usize },
    TighnariC4 { after_reaction: bool },
    DoriC4 { hp_below50: bool, energy_below50: bool },
    NilouTalent2 { hp: f64 },
    CandaceTalent2 { hp: f64 },

    FreedomSworn { refine: usize },
    SongOfBrokenPines { refine: usize },
    WolfsGravestone { refine: usize },
    ThrillingTalesOfDragonSlayers { refine: usize },
    ElegyOfTheEnd { refine: usize },
    HakushinRing { refine: usize, element: Element },
    SapwoodBlade { refine: usize, rate: f64 },
    Moonpiercer { refine: usize },
    XiphosMoonlight { refine: usize, em: f64, },
    MakhairaAquamarine { refine: usize, em: f64 },
    KeyOfKhajNisut { refine: usize, hp: f64 },

    ResonanceCryo2 { rate: f64 },
    ResonanceGeo2 { rate1: f64, rate2: f64 },
    ResonanceDendro2 { rate1: f64, rate2: f64 },

    ArchaicPetra4 { element: Element },
    ViridescentVenerer4 { element: Element },

    NoConfig,
}
