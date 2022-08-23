use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum WeaponConfig {
    NoConfig,

    MistsplitterReforged { emblem_level: usize },
    SummitShaper { stack: f64, shield_rate: f64 },
    FreedomSworn { rate: f64 },
    TheAlleyFlash { rate: f64 },
    PrototypeRancour { stack: f64 },
    LionsRoar { rate: f64 },
    IronSting { stack: f64 },
    CinnabarSpindle { rate: f64 },
    BlackcliffLongsword { stack: f64 },
    HarbingerOfDawn { rate: f64 },
    SkyriderSword { rate: f64 },
    DarkIronSword { rate: f64 },
    CoolSteel { rate: f64 },
    HaranGeppakuFutsu { stack: f64 },
    CursedBlade { rate: f64 },
    SapwoodBlade { rate: f64 },

    // claymore
    WolfsGravestone { rate: f64 },
    TheUnforged { stack: f64, shield_rate: f64 },
    SongOfBrokenPines { rate: f64 },
    Akuoumaru { energy: usize },
    Whiteblind { stack: f64 },
    TheBell { rate: f64 },
    SerpentSpine { stack: f64 },
    BlackcliffSlasher { stack: f64 },
    Rainslasher { rate: f64 },
    LithicBlade { liyue_count: usize },
    SkyriderGreatsword { stack: f64 },
    BloodstainedGreatsword { rate: f64 },
    FerrousShadow { rate: f64 },
    ForestRegalia { rate: f64 },

    // Polearm
    EngulfingLightning { rate: f64 },
    PrimordialJadeWingedSpear { stack: f64, full_rate: f64 },
    CalamityQueller { stack: f64, backend_rate: f64 },
    StaffOfHoma { be50_rate: f64 },
    VortexVanquisher { stack: f64, shield_rate: f64 },
    PrototypeStarglitter { stack: f64 },
    LithicSpear { liyue_count: usize },
    DragonsBane { rate: f64 },
    Deathmatch { ge2: bool },
    BlackcliffPole { stack: f64 },
    WavebreakersFin { energy: usize },
    BlackTassel { rate: f64 },
    Moonpiercer { rate: f64 },

    // Catalyst
    LostPrayerToTheSacredWinds { stack: f64 },
    MemoryOfDust { stack: f64, shield_rate: f64 },
    WindAndSong { rate: f64 },
    TheWidsith { t1_rate: f64, t2_rate: f64, t3_rate: f64 },
    SolarPearl { rate1: f64, rate2: f64 },
    MappaMare { stack: f64 },
    DodocoTales { rate1: f64, rate2: f64 },
    BlackcliffAgate { stack: f64 },
    OathswornEye { rate: f64 },
    KagurasVerity { stack: f64, full_rate: f64 },
    MagicGuide { rate: f64 },
    EmeraldOrb { rate: f64 },
    TwinNephrite { rate: f64 },
    FruitOfFulfillment { stack: f64 },

    // Bow
    PolarStar { stack: usize },
    ThunderingPulse { stack: usize },
    ElegyOfTheEnd { rate: f64 },
    AmosBow { stack: f64 },
    AlleyHunter { stack: f64 },
    Predator { stack: f64 },
    PrototypeCrescent { rate: f64 },
    MouunsMoon { energy: usize },
    MitternachtsWaltz { rate1: f64, rate2: f64 },
    Hamayumi { rate: f64 },
    CompoundBow { stack: f64 },
    BlackcliffWarbow { stack: f64 },
    WindblumeOde { rate: f64 },
    RavenBow { rate: f64 },
    SharpshootersOath { rate: f64 },
    Slingshot { is_effect: bool, rate: f64 },
    AquaSimulacra { is_enemy_around: bool },
    FadingTwilight { state: usize },
    HuntersPath { rate: f64 },
    KingsSquire { rate: f64 },
}

impl Default for WeaponConfig {
    fn default() -> Self {
        WeaponConfig::NoConfig
    }
}
