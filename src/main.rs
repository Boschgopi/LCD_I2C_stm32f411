#![no_std]
#![no_main]

// Imports
// use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{
    i2c::Mode,
    pac::{self},
    prelude::*,
    serial::{config::Config, SerialExt},
};

#[entry]
fn main() -> ! {
    // Setup handler for device peripherals
    let dp = pac::Peripherals::take().unwrap();

    // I2C Config steps:
    // 1) Need to configure the system clocks
    // - Promote RCC structure to HAL to be able to configure clocks
    let rcc = dp.RCC.constrain();
    // - Configure system clocks
    // 8 MHz must be used for the Nucleo-F401RE board according to the manual
    let clocks = rcc.cfgr.use_hse(8.MHz()).freeze();
    // 2) Configure/Define SCL and SDA pins
    let gpiob = dp.GPIOB.split();
    let scl = gpiob.pb8;
    let sda = gpiob.pb9;
    // 3) Configure I2C peripheral channel
    // We're going to use I2C1 since its pins are the ones connected to the I2C interface we're using
    // To configure/instantiate serial peripheral channel we have two options:
    // Use the i2c device peripheral handle and instantiate a transmitter instance using an extension trait
    let mut i2c = dp.I2C1.i2c(
        (scl, sda),
        Mode::Standard {
            frequency: 100.kHz(),
        },
        &clocks,
    );
    // Or use the I2C abstraction
    // let mut i2c = I2c::new(
    //     dp.I2C1,
    //     (scl, sda),
    //     Mode::Standard {
    //         frequency: 300.kHz(),
    //     },
    //     &clocks,
    // );

    // Serial config steps:
    // 1) Need to configure the system clocks
    // Already done earlier for the I2C module
    // 2) Configure/Define TX pin
    // Use PA2 as it is connected to the host serial interface
    //let gpioa = dp.GPIOA.split();
    //let tx_pin = gpioa.pa2.into_alternate();
    // 3) Configure Serial peripheral channel
    // We're going to use USART2 since its pins are the ones connected to the USART host interface
    // To configure/instantiate the serial peripheral channel we have two options:
    // Use the device peripheral handle to directly access USART2 and instantiate a transmitter instance
    //let _tx:stm32f4xx_hal::serial::Tx<stm32f4xx_hal::pac::USART2> = dp
    //    .USART2
    //    .tx(
    //        tx_pin,
    //        Config::default()
    //            .baudrate(9600.bps())
    //            .wordlength_8()
    //            .parity_none(),
    //        &clocks,
    //    )
    //    .unwrap();

      let mut delay = dp.TIM1.delay_ms(&clocks);

    // PCF8574 I2C Address
    const PCF8574_ADDR: u8 = 0x20;    

    // Application Loop
    loop {
        // Set all pins of the PCF8574 as outputs
        // Each bit of the data byte corresponds to a pin on the PCF8574.
        // Set a bit to 0 to configure the corresponding pin as an output.
        // In this example, all pins are set as outputs, so we set the data byte to 0x00.
        let output_config: u8 = 0x00;
        i2c.write(PCF8574_ADDR, &[output_config]).unwrap();
        delay.delay_ms(1000_u32); // Wait for 1 second

        // Toggle all pins of the PCF8574
        // To toggle the pins, we first read the current state of the GPIO pins,
        // then complement the bits (1s to 0s and 0s to 1s) and write back the new state.
        let mut input_buffer: [u8; 1] = [0];
        i2c.read(PCF8574_ADDR, &mut input_buffer).unwrap();
        let current_state = input_buffer[0];
        let new_state = !current_state;
        i2c.write(PCF8574_ADDR, &[new_state]).unwrap();
        delay.delay_ms(1000_u32); // Wait for 1 second
    }
}

