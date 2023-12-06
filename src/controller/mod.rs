use arduino_hal::{Peripherals, spi::Settings, default_serial};
use embedded_nrf24l01::{NRF24L01, Configuration, DataRate, CrcMode, setup::spi_mode};
use iron::{drivers::generics::{joystick::Joystick, antenna::AntennaContainer}, math::useful_functions::map};

use crate::shared::*;

pub fn run(dp: Peripherals) -> !
{
    let pins = arduino_hal::pins!(dp);
    let mut serial = default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let joystick = Joystick::new(
        pins.a0.into_analog_input(&mut adc), 
        pins.a1.into_analog_input(&mut adc), 
        pins.a2.into_pull_up_input());
    
    let mut antenna = NRF24L01::new(
        pins.d8.into_output(), 
        pins.d9.into_output(),
        arduino_hal::Spi::new
        (
            dp.SPI,
            pins.d13.into_output(),
            pins.d11.into_output(),
            pins.d12.into_pull_up_input(),
            pins.d10.into_output(),
            Settings {
                mode: spi_mode(),
                ..Default::default()
            },
        ).0
    ).unwrap();

    antenna.set_frequency(CHANNEL).unwrap();
    antenna.set_auto_retransmit(0, 15).unwrap();
    antenna.set_rf(&DataRate::R250Kbps, 3).unwrap();
    antenna.set_auto_ack(&[false; 6]).unwrap();
    antenna.set_crc(CrcMode::Disabled).unwrap();
    antenna.set_tx_addr(ADDRESS).unwrap();
    
    let mut antenna = AntennaContainer::new(antenna.tx().unwrap());
    
    loop {
        if antenna.internal_mut().can_send().unwrap_or(false) {
            let coords = joystick.coords(&mut adc);
            let x = map(coords.x as f32, 0.0..=1023.0, -1.0..=1.0);
            let y = map(coords.y as f32, 0.0..=1023.0, -1.0..=1.0);
            let command = Command {
                move_direction: [x, y]
            };

            antenna.send(&command);
        }

        arduino_hal::delay_ms(20);
    }
}