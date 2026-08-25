use embedded_hal::digital::OutputPin;

#[derive(PartialEq, Eq)]
enum LedStatus {
    On,
    Off,
}

#[derive(PartialEq, Eq)]
enum LedMode {
    Default,
    Blink,
}

pub struct Led<PIN> {
    pin: PIN,
    status: LedStatus,
    mode: LedMode,
    blink_count: u32,
    blink_threshold: u32,
}

impl<PIN> Led<PIN>
where
    PIN: OutputPin,
{
    pub fn new(mut pin: PIN) -> Self {
        let _ = pin.set_low();
        Self {
            pin,
            status: LedStatus::Off,
            mode: LedMode::Default,
            blink_count: 0,
            blink_threshold: 1000,
        }
    }

    pub fn on(&mut self) {
        if self.status == LedStatus::Off {
            let _ = self.pin.set_high();
            self.status = LedStatus::On;
        }
    }

    pub fn off(&mut self) {
        if self.status == LedStatus::On {
            let _ = self.pin.set_low();
            self.status = LedStatus::Off;
        }
    }

    pub fn is_on(&mut self) -> bool {
        if self.status == LedStatus::On {
            return true;
        }
        false
    }

    pub fn switch_on_off(&mut self) {
        self.mode = LedMode::Default;
        self.blink_count = 0;

        match self.status {
            LedStatus::On => self.off(),
            LedStatus::Off => self.on(),
        }
    }

    pub fn start_blink(&mut self, th: Option<u32>) {
        self.mode = LedMode::Blink;
        self.blink_count = 0;
        if let Some(threshold) = th {
            self.blink_threshold = threshold;
        }
    }

    pub fn end_blink(&mut self) {
        self.mode = LedMode::Default;
        self.blink_count = 0;
        self.off();
    }

    pub fn switch_blink(&mut self, th: Option<u32>) {
        match self.mode {
            LedMode::Default => self.start_blink(th),
            LedMode::Blink => self.end_blink(),
        }
    }

    pub fn process(&mut self) {
        match self.mode {
            LedMode::Default => {}
            LedMode::Blink => self.blink(),
        }
    }

    fn blink(&mut self) {
        self.blink_count += 1;
        if self.blink_count >= self.blink_threshold {
            if self.status == LedStatus::On {
                self.off();
            } else {
                // == LedStatus::off
                self.on()
            }
            self.blink_count = 0;
        }
    }
}
