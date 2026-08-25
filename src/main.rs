#![no_std]
#![no_main]

mod button;
mod led;

use cortex_m::delay::Delay;
use cortex_m_rt::entry;
use panic_halt as _;

use waveshare_rp2040_zero::{
    Pins,
    hal::{
        Sio, Watchdog,
        clocks::{Clock, init_clocks_and_plls},
        pac,
    },
};

use crate::{
    button::{Button, InputEvent},
    led::Led,
};

const XOSC_CRYSTAL_FREQ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    // Peripheral
    let mut pac = pac::Peripherals::take().unwrap();
    let core = cortex_m::Peripherals::take().unwrap();

    // Watchdog
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    // Clock
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Delay
    let mut delay = Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    // GPIO
    let sio = Sio::new(pac.SIO);

    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // LED
    let led_red_pin = pins.gp13.into_push_pull_output();
    let led_green_pin = pins.gp14.into_push_pull_output();
    let led_yellow_pin = pins.gp15.into_push_pull_output();

    let mut led_red = Led::new(led_red_pin);
    let mut led_green = Led::new(led_green_pin);
    let mut led_yellow = Led::new(led_yellow_pin);

    // Button
    let bt_red_pin = pins.gp10.into_pull_up_input();
    let bt_green_pin = pins.gp11.into_pull_up_input();
    let bt_yellow_pin = pins.gp12.into_pull_up_input();

    let mut red_button = Button::new(bt_red_pin);
    let mut green_button = Button::new(bt_green_pin);
    let mut yellow_button = Button::new(bt_yellow_pin);

    loop {
        // LED定期処理
        led_red.process();
        led_green.process();

        // Button定期処理
        let red_event = red_button.process();
        let green_event = green_button.process();
        let yellow_event = yellow_button.process();

        // 動作確認
        if red_event == InputEvent::Press {
            led_red.switch_on_off();
        }

        if green_event == InputEvent::Press {
            led_green.switch_on_off();
        }

        if yellow_event == InputEvent::Press {
            led_yellow.switch_on_off();
        }

        // Button debounce / LED blink の基準周期
        delay.delay_ms(1);
    }
}
