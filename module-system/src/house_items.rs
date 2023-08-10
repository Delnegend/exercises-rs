#![allow(dead_code)]
pub mod properties {
    pub enum KíchThước {
        BìnhThường,
        To,
        Nhỏ,
    }

    pub enum KiểuDáng {
        HiệnĐại,
        CổĐiển,
        MộcMạc,
    }
}

use properties::{KíchThước, KiểuDáng};

pub struct Door {
    chất_liệu: String,
    kích_thước: KíchThước,
}

impl Door {
    pub fn new(material: &str, size: KíchThước) -> Door {
        Door {
            chất_liệu: material.to_string(),
            kích_thước: size,
        }
    }
    pub fn material(&self) -> &str {
        &self.chất_liệu
    }
    pub fn size(&self) -> &str {
        match self.kích_thước {
            KíchThước::BìnhThường => "normal",
            KíchThước::To => "big",
            KíchThước::Nhỏ => "small",
        }
    }
}

pub struct Window {
    material: String,
    size: KíchThước,
    style: KiểuDáng,
}

impl Window {
    pub fn new(material: &str, size: KíchThước, style: KiểuDáng) -> Window {
        Window {
            material: material.to_string(),
            size,
            style,
        }
    }
    pub fn material(&self) -> &str {
        &self.material
    }
    pub fn size(&self) -> &str {
        match self.size {
            KíchThước::BìnhThường => "normal",
            KíchThước::To => "big",
            KíchThước::Nhỏ => "small",
        }
    }
    pub fn style(&self) -> &str {
        match self.style {
            KiểuDáng::HiệnĐại => "modern",
            KiểuDáng::CổĐiển => "classic",
            KiểuDáng::MộcMạc => "rustic",
        }
    }
}
