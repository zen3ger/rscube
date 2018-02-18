extern crate svgdom;

mod dom;

use std::default;
pub use self::dom::Dom;
pub use self::svgdom::WriteBuffer;

pub struct ColorScheme {
    u: svgdom::Color,
    d: svgdom::Color,
    r: svgdom::Color,
    l: svgdom::Color,
    f: svgdom::Color,
    b: svgdom::Color,
}

impl ColorScheme {
    pub fn u(&self) -> svgdom::Color {
        self.u
    }

    pub fn d(&self) -> svgdom::Color {
        self.d
    }

    pub fn r(&self) -> svgdom::Color {
        self.r
    }

    pub fn l(&self) -> svgdom::Color {
        self.l
    }

    pub fn f(&self) -> svgdom::Color {
        self.f
    }

    pub fn b(&self) -> svgdom::Color {
        self.b
    }
}

impl default::Default for ColorScheme {
    fn default() -> Self {
        Self {
            u: svgdom::Color::new(255, 255, 255),
            d: svgdom::Color::new(255, 255, 0),
            r: svgdom::Color::new(255, 0, 0),
            l: svgdom::Color::new(255, 127, 0),
            f: svgdom::Color::new(0, 255, 31),
            b: svgdom::Color::new(0, 80, 255),
        }
    }
}
