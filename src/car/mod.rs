use arduino_hal::Peripherals;
use arduino_hal::port::PinOps;
use arduino_hal::simple_pwm::{Timer0Pwm, IntoPwmPin, Prescaler, Timer2Pwm, PwmPinOps};
use arduino_hal::spi::Settings;
use embedded_nrf24l01::setup::spi_mode;
use iron::drivers::generics::antenna::*;
use iron::drivers::specifics::l298n::L298N;
use embedded_nrf24l01::{NRF24L01, Configuration, DataRate, CrcMode};
use iron::drivers::specifics::l298n::*;
use iron::math::useful_functions::*;

use crate::shared::*;

pub fn run(dp: Peripherals) -> !
{
    let pins = arduino_hal::pins!(dp);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());
    
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let timer2 = Timer2Pwm::new(dp.TC2, Prescaler::Prescale64);

    let mut motors = L298N::new(
        pins.d3.into_output().into_pwm(&timer2), 
        pins.d2.into_output(), 
        pins.d4.into_output(), 
        pins.d5.into_output().into_pwm(&timer0), 
        pins.d6.into_output(), 
        pins.d7.into_output()
    );

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
    antenna.set_pipes_rx_enable(&[false, true, false, false, false, false]).unwrap();
    antenna.set_auto_ack(&[false; 6]).unwrap();
    antenna.set_crc(CrcMode::Disabled).unwrap();
    antenna.set_rx_addr(1, ADDRESS).unwrap();
    antenna.set_pipes_rx_lengths(&[None, Some(<Command as SendWithAntenna>::PACK_SIZE as u8), None, None, None, None]);

    let mut antenna = AntennaContainer::new(antenna.rx().unwrap());
    
    loop {
        fn should_move(coord: f32) -> bool
        {
            abs(coord) >= 0.1
        }

        if let Some(controller_data) = unsafe { antenna.try_read::<Command>() }
        {
            let [x, y] = controller_data.move_direction;
            
            fn set_motor_speed<IN1: PinOps, IN2: PinOps, EN: PwmPinOps<TC>, TC>(motor: &mut L298NMotor<IN1, IN2, EN, TC>, mut coordinate: f32)
            {
                if !should_move(coordinate) {
                    motor.turn_off()
                }
                else
                {
                    if coordinate.is_sign_positive() {
                        motor.set_forward_direction();

                        motor.set_speed(map_clamping(coordinate, 0.0..=1.0, 0.0..=255.0) as u8);
                    }
                    else
                    {
                        motor.set_backward_direction();

                        motor.set_speed(map_clamping(-coordinate, 0.0..=1.0, 0.0..=255.0) as u8);
                    }
                }
            }
        
            set_motor_speed(motors.a(), y - x);
            set_motor_speed(motors.b(), y + x);
        }

        arduino_hal::delay_ms(20);
    }
}