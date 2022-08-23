use crate::attribute::{Attribute, AttributeName};
use crate::buffs::{Buff, BuffConfig};
use crate::buffs::buff::BuffMeta;
use crate::buffs::buff_meta::{BuffFrom, BuffGenre, BuffImage, BuffMetaData};
use crate::buffs::buff_name::BuffName;
use crate::common::item_config_type::{ItemConfig, ItemConfigType};
use crate::enemies::Enemy;
use crate::weapon::WeaponName;

pub struct BuffSapwoodBlade {
    rate: f64,
    refine: usize,
}

impl<A: Attribute> Buff<A> for BuffSapwoodBlade {
    fn change_attribute(&self, attribute: &mut A) {
        let value = self.refine as f64 * 15.0 + 45.0;
        attribute.set_value_by(AttributeName::ElementalMastery, "BUFF: 「种识之叶」", self.rate * value);
    }
}

impl BuffMeta for BuffSapwoodBlade {
    #[cfg(not(target_family = "wasm"))]
    const META_DATA: BuffMetaData = BuffMetaData {
        name: BuffName::SapwoodBlade,
        chs: "",
        image: BuffImage::Weapon(WeaponName::SapwoodBlade),
        genre: BuffGenre::Weapon,
        description: Some(""),
        from: BuffFrom::Weapon(WeaponName::SapwoodBlade)
    };

    #[cfg(not(target_family = "wasm"))]
    const CONFIG: Option<&'static [ItemConfig]> = Some(&[
        ItemConfig::REFINE,
        ItemConfig {
            name: "rate",
            title: "b41",
            config: ItemConfigType::Float { min: 0.0, max: 1.0, default: 1.0 }
        }
    ]);

    fn create<A: Attribute>(b: &BuffConfig) -> Box<dyn Buff<A>> {
        let (refine, rate) = match *b {
            BuffConfig::SapwoodBlade { refine, rate } => (refine, rate),
            _ => (1, 0.0)
        };

        Box::new(BuffSapwoodBlade {
            rate, refine
        })
    }
}