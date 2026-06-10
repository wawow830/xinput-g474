#[derive(Default)]
pub struct State {
    pub buttons_lo: u8,
    pub buttons_hi: u8,
    pub left_trigger: u8,
    pub right_trigger: u8,
    pub left_x: i16,
    pub left_y: i16,
    pub right_x: i16,
    pub right_y: i16,
}

pub fn scale_axis(raw: u16, min: u16, center: u16, max: u16, invert: bool) -> i16 {
    let raw = raw as i32;
    let min = min as i32;
    let center = center as i32;
    let max = max as i32;

    let negative_span = (center - min).max(1);
    let positive_span = (max - center).max(1);

    let mut value = if raw >= center {
        ((raw - center) * 32_767) / positive_span
    } else {
        ((raw - center) * 32_768) / negative_span
    };

    if invert {
        value = -value;
    }

    value.clamp(-32_768, 32_767) as i16
}

pub fn scale_trigger(raw: u16, min: u16, max: u16, invert: bool) -> u8 {
    let raw = raw as i32;
    let min = min as i32;
    let max = max as i32;
    let span = (max - min).abs().max(1);

    let mut value = if max >= min {
        ((raw - min) * 255) / span
    } else {
        ((min - raw) * 255) / span
    };

    value = value.clamp(0, 255);

    if invert {
        value = 255 - value;
    }

    value as u8
}

pub struct Controls;

impl Controls {
    pub const fn new() -> Self {
        Self
    }

    pub fn read(&mut self) -> State {
        State::default()
    }
}
