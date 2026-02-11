use crate::input::source::Cpx;

pub fn interpolate_nums(from: u8, to: u8, ratio: f32) -> u8 {
    let diff = ((to as i16) - (from as i16)) as f32;
    (from as i16 + ((ratio * diff) as i16)) as u8
}

pub fn color_to_number(s: &str) -> (u8, u8, u8) {
    let r = u8::from_str_radix(&s[1..3], 16).unwrap();
    let g = u8::from_str_radix(&s[3..5], 16).unwrap();
    let b = u8::from_str_radix(&s[5..7], 16).unwrap();
    (r, g, b)
}

pub fn numbers_to_color(nums: (u8, u8, u8)) -> String {
    let (r, g, b) = nums;
    format!("#{:x}{:x}{:x}", r, g, b)
}

pub fn interpolate_colors(from: &str, to: &str, ratio: f32) -> String {
    let (fr, fg, fb) = color_to_number(from);
    let (tr, tg, tb) = color_to_number(to);
    let r = interpolate_nums(fr, tr, ratio);
    let g = interpolate_nums(fg, tg, ratio);
    let b = interpolate_nums(fb, tb, ratio);
    numbers_to_color((r, g, b))
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Color {
    Gray,
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Magenta,
    Cyan,
    Lime,
}

impl Color {
    pub fn list() -> Vec<Color> {
        vec![
            Color::Gray,
            Color::Red,
            Color::Blue,
            Color::Green,
            Color::Yellow,
            Color::Orange,
            Color::Magenta,
            Color::Cyan,
            Color::Lime,
        ]
    }

    pub fn name(&self) -> String {
        match self {
            Self::Gray => "gray",
            Self::Red => "red",
            Self::Blue => "blue",
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Orange => "orange",
            Self::Magenta => "magenta",
            Self::Cyan => "cyan",
            Self::Lime => "lime",
        }
        .into()
    }

    pub fn from_str(str: &str) -> Self {
        match str {
            "gray" => Self::Gray,
            "red" => Self::Red,
            "blue" => Self::Blue,
            "green" => Self::Green,
            "yellow" => Self::Yellow,
            "orange" => Self::Orange,
            "magenta" => Self::Magenta,
            "cyan" => Self::Cyan,
            "lime" => Self::Lime,
            _ => panic!("color name {} not found", str),
        }
    }

    pub fn hex(&self) -> String {
        match self {
            Self::Gray => "#bebebe",
            Self::Red => "#ff0000",
            Self::Blue => "#0000ff",
            Self::Green => "#006400",
            Self::Yellow => "#ffd700",
            Self::Orange => "#ff8c00",
            Self::Magenta => "#ee82ee",
            Self::Cyan => "#40e0d0",
            Self::Lime => "#00ff00",
        }
        .into()
    }

    fn white_ratio(&self) -> f32 {
        match self {
            Self::Gray => 0.5,
            Self::Red => 0.5,
            Self::Blue => 0.5,
            Self::Green => 0.5,
            Self::Yellow => 0.3,
            Self::Orange => 0.6,
            Self::Magenta => 0.6,
            Self::Cyan => 0.6,
            Self::Lime => 0.6,
        }
    }

    pub fn light(&self) -> String {
        interpolate_colors(&self.hex(), "#ffffff", self.white_ratio())
    }

    pub fn tikz(&self) -> String {
        let (r, g, b) = color_to_number(&self.hex());
        format!("{{rgb,255:red,{};green,{};blue,{}}}", r, g, b)
    }
}

pub enum SimpleRelation {
    Arrow,
    NotArrow,
    Unknown,
}

pub struct DirectedRelation {
    from: String,
    to: String,
    from_to: SimpleRelation,
    to_from: SimpleRelation,
}

pub fn relation_color(relation: DirectedRelation) -> Color {
    match (relation.from_to, relation.to_from) {
        (SimpleRelation::Arrow, SimpleRelation::Arrow) => Color::Yellow,
        (SimpleRelation::Arrow, SimpleRelation::NotArrow) => Color::Green,
        (SimpleRelation::Arrow, SimpleRelation::Unknown) => Color::Lime,
        (SimpleRelation::NotArrow, SimpleRelation::Arrow) => Color::Red,
        (SimpleRelation::NotArrow, SimpleRelation::NotArrow) => Color::Blue,
        (SimpleRelation::NotArrow, SimpleRelation::Unknown) => Color::Magenta,
        (SimpleRelation::Unknown, SimpleRelation::Arrow) => Color::Orange,
        (SimpleRelation::Unknown, SimpleRelation::NotArrow) => Color::Cyan,
        (SimpleRelation::Unknown, SimpleRelation::Unknown) => Color::Gray,
    }
}
