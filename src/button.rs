use embedded_hal::digital::InputPin;

const CHATTERING_COUNT: u8 = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputEvent {
    Press,
    Release,
    None,
}

#[derive(PartialEq)]
enum ButtonStatus {
    Pressed,
    Released,
}

pub struct Button<PIN> {
    pin: PIN,
    status: ButtonStatus,
    count: u8,
}

impl<PIN> Button<PIN>
where
    PIN: InputPin,
{
    pub fn new(pin: PIN) -> Self {
        Self {
            pin,
            status: ButtonStatus::Released,
            count: 0,
        }
    }

    pub fn process(&mut self) -> InputEvent {
        // off -> on
        if self.status == ButtonStatus::Released && self.pin.is_low().unwrap_or(false) {
            self.count += 1;
            if self.count >= CHATTERING_COUNT {
                self.status = ButtonStatus::Pressed;
                self.count = 0;
                return InputEvent::Press;
            }
        } else if self.status == ButtonStatus::Pressed && self.pin.is_high().unwrap_or(false) {
            self.count += 1;
            if self.count >= CHATTERING_COUNT {
                self.status = ButtonStatus::Released;
                self.count = 0;
                return InputEvent::Release;
            }
        } else {
            self.count = 0;
        }

        InputEvent::None
    }
}
