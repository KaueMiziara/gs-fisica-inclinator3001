#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_println::println;
use hal::{
    clock::ClockControl, i2c, peripherals::Peripherals, prelude::*, timer::TimerGroup, Delay, Rtc,
    IO,
};
use crate::accelerometer::AccelerometerAngles;
use mpu6050::Mpu6050;

mod accelerometer;
mod conversion;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    let mut delay = Delay::new(&clocks);

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c = i2c::I2C::new(
        peripherals.I2C0,
        io.pins.gpio21,
        io.pins.gpio22,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );
    delay.delay_ms(255u8);

    let mut mpu = Mpu6050::new(i2c);
    mpu.init(&mut delay)
        .expect("Ocorreu um erro ao inicializar o MPU6050!");

    loop {
        let accelerometer_angles = AccelerometerAngles::new(&mut mpu);

        accelerometer_angles.print_angles();

        delay.delay_ms(750u32);

        println!();
    }
}
