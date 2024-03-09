
pub fn interpolate_nums(from: u8, to: u8, ratio: f32) -> u8 {
    let diff = ((to as i16)-(from as i16)) as f32;
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

