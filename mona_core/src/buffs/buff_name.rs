use num_derive::FromPrimitive;
use strum_macros::Display;
use serde::{Serialize, Deserialize};
use mona_derive::{BuffData, EnumLen};
use crate::buffs::buff::{BuffMeta};

#[derive(Copy, Clone)]
#[derive(Serialize, Deserialize)]
#[derive(BuffData, EnumLen, FromPrimitive, Display)]
pub enum BuffName {
    // common
    ATKPercentage,
    DEFPercentage,
    HPPercentage,
    ATKFixed,
    DEFFixed,
    HPFixed,
    Critical,
    CriticalDamage,
    CustomBonus,
    ElementalMastery,
    Recharge,
    DEFMinus,
    ResMinus,
    HealingBonus,
    BaseDmg,

    // character
    AlbedoTalent2,
    AlbedoC4,
    AlbedoC6,
    AloyTalent1,
    AratakiIttoC4,
    BeidouC6,
    BennettQ,
    BennettC6,
    ChongyunTalent2,
    DionaC6G50,
    EulaE,
    GanyuTalent2,
    GanyuC1,
    GorouE1,
    GorouE3,
    GorouTalent1,
    GorouC6,
    HuTaoTalent1,
    JeanC4,
    KaedeharaKazuhaTalent2,
    KaedeharaKazuhaC2,
    KamisatoAyakaC4,
    KleeC2,
    KleeC6,
    KujouSaraEOrQ,
    LisaTalent2,
    MonaQ,
    MonaC1,
    NingguangTalent2,
    RaidenShogunE,
    RaidenShogunC4,
    RazorC4,
    RosariaTalent2,
    RosariaC6,
    ShenheE,
    ShenheQ,
    ShenheTalent1,
    ShenheTalent2,
    SucroseTalent1,
    SucroseTalent2,
    SucroseC6,
    ThomaTalent1,
    ThomaC6,
    VentiC2,
    VentiC6,
    XianglingTalent2,
    XianglingC1,
    XianglingC6,
    XingqiuC2,
    XinyanC4,
    XinyanTalent2,
    YaeMikoC4,
    YoimiyaTalent2,
    YunjinQ,
    YunjinC2,
    ZhongliShield,
    YelanTalent2,
    YelanC4,
    KamisatoAyatoQ,
    ShikanoinHeizouTalent2,
    TighnariC4,
    DoriC4,

    // weapon
    FreedomSworn,
    SongOfBrokenPines,
    WolfsGravestone,
    ThrillingTalesOfDragonSlayers,
    ElegyOfTheEnd,
    HakushinRing,
    SapwoodBlade,
    Moonpiercer,

    ResonancePyro2,
    ResonanceCryo2,
    ResonanceGeo2,
    ResonanceHydro2,
    ResonanceDendro2,

    Instructor4,
    NoblesseOblige4,
    ArchaicPetra4,
    ViridescentVenerer4,
    TenacityOfTheMillelith4,
}
