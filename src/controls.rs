use stm32g4xx_hal as hal;

use hal::{
    adc::{config::SampleTime, Adc, Configured},
    stm32::ADC2,
};

use crate::constants::*;

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

pub struct Controls {
    adc: Adc<ADC2, Configured>,

    left_x: LeftXPin,
    left_y: LeftYPin,
    right_x: RightXPin,
    right_y: RightYPin,
    left_trigger: LeftTriggerPin,
    right_trigger: RightTriggerPin,

    a: APin,
    b: BPin,
    x: XPin,
    y: YPin,
    start: StartPin,
    back: BackPin,
    left_bumper: LeftBumperPin,
    right_bumper: RightBumperPin,
}

impl Controls {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        adc: Adc<ADC2, Configured>,
        left_x: LeftXPin,
        left_y: LeftYPin,
        right_x: RightXPin,
        right_y: RightYPin,
        left_trigger: LeftTriggerPin,
        right_trigger: RightTriggerPin,
        a: APin,
        b: BPin,
        x: XPin,
        y: YPin,
        start: StartPin,
        back: BackPin,
        left_bumper: LeftBumperPin,
        right_bumper: RightBumperPin,
    ) -> Self {
        Self {
            adc,
            left_x,
            left_y,
            right_x,
            right_y,
            left_trigger,
            right_trigger,
            a,
            b,
            x,
            y,
            start,
            back,
            left_bumper,
            right_bumper,
        }
    }

    pub fn read(&mut self) -> State {
        let left_x = self.adc.convert(&self.left_x, SampleTime::Cycles_640_5);
        let left_y = self.adc.convert(&self.left_y, SampleTime::Cycles_640_5);
        let right_x = self.adc.convert(&self.right_x, SampleTime::Cycles_640_5);
        let right_y = self.adc.convert(&self.right_y, SampleTime::Cycles_640_5);
        let left_trigger = self
            .adc
            .convert(&self.left_trigger, SampleTime::Cycles_640_5);
        let right_trigger = self
            .adc
            .convert(&self.right_trigger, SampleTime::Cycles_640_5);

        State {
            buttons_lo: self.buttons_lo(),
            buttons_hi: self.buttons_hi(),
            left_trigger: scale_trigger(left_trigger, ADC_MIN, ADC_MAX, false),
            right_trigger: scale_trigger(right_trigger, ADC_MIN, ADC_MAX, false),
            left_x: scale_axis(left_x, ADC_MIN, ADC_CENTER, ADC_MAX, LEFT_X_INVERT),
            left_y: scale_axis(left_y, ADC_MIN, ADC_CENTER, ADC_MAX, LEFT_Y_INVERT),
            right_x: scale_axis(right_x, ADC_MIN, ADC_CENTER, ADC_MAX, RIGHT_X_INVERT),
            right_y: scale_axis(right_y, ADC_MIN, ADC_CENTER, ADC_MAX, RIGHT_Y_INVERT),
        }
    }

    fn buttons_lo(&self) -> u8 {
        let mut buttons = 0;

        if self.start.is_low() {
            buttons |= START;
        }
        if self.back.is_low() {
            buttons |= BACK;
        }

        buttons
    }

    fn buttons_hi(&self) -> u8 {
        let mut buttons = 0;

        if self.left_bumper.is_low() {
            buttons |= LEFT_BUMPER;
        }
        if self.right_bumper.is_low() {
            buttons |= RIGHT_BUMPER;
        }
        if self.a.is_low() {
            buttons |= A;
        }
        if self.b.is_low() {
            buttons |= B;
        }
        if self.x.is_low() {
            buttons |= X;
        }
        if self.y.is_low() {
            buttons |= Y;
        }

        buttons
    }
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
