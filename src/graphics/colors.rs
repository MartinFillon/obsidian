#[derive(Debug, Clone, Copy)]
pub struct Colors {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Colors {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> Self {
        Self {
            red,
            green,
            blue,
            alpha,
        }
    }

    pub fn red() -> Self {
        Self {
            red: 1.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    pub fn green() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    pub fn blue() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    pub fn white() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }

    pub fn black() -> Self {
        Self {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    pub fn yellow() -> Self {
        Self {
            red: 1.0,
            green: 1.0,
            blue: 0.0,
            alpha: 1.0,
        }
    }

    pub fn cyan() -> Self {
        Self {
            red: 0.0,
            green: 1.0,
            blue: 1.0,
            alpha: 1.0,
        }
    }
}

impl From<&Colors> for [f32; 4] {
    fn from(color: &Colors) -> Self {
        [color.red, color.green, color.blue, color.alpha]
    }
}
