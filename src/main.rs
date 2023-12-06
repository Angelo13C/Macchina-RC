#![no_std]
#![no_main]

mod shared;

#[cfg(feature = "car")]
mod car;
#[cfg(feature = "controller")]
mod controller;

use iron::drivers::{specifics::hc_sr04::HcSr04, generics};
use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! 
{
    let dp = arduino_hal::Peripherals::take().unwrap();

    #[cfg(not(any(feature = "car", feature = "controller")))]
    {
        compile_error!("Select a feature between `car` and `controller`")
    }
    #[cfg(all(feature = "car", feature = "controller"))]
    {
        compile_error!("You can't compile for both a car and a controller");
    }
    #[cfg(feature = "car")]
    {
        car::run(dp)
    }
    #[cfg(feature = "controller")]
    {
        controller::run(dp)
    }
}