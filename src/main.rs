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
use defmt_rtt;
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
    // );.



    // PCF8574 I2C Address
    const PCF8574_ADDR: u8 = 0x27;    
    let output_config: u8 = 0x00;
    let rs_pin_mask: u8 = 0b00000001;
    const RS: u8 = 0x01;
    const RW: u8 = 0x02;
    const EN_MASK: u8 = 0x04;
    const D4: u8 = 0x10;
    const D5: u8 = 0x20;
    const D6: u8 = 0x40;
    const D7: u8 = 0x80;
    i2c.write(PCF8574_ADDR, &[!rs_pin_mask]).unwrap();
    i2c.write(PCF8574_ADDR, &[EN_MASK]).unwrap();




    let mut delay = dp.TIM1.delay_ms(&clocks);
  

//RS => 1, characters
//RS => 0, numbeers

    // Application Loop
    loop {
        // Set all pins of the PCF8574 as outputs
        // Each bit of the data byte corresponds to a pin on the PCF8574.
        // Set a bit to 0 to configure the corresponding pin as an output.
        // In this example, all pins are set as outputs, so we set the data byte to 0x00.
        i2c.write(PCF8574_ADDR, &[rs_pin_mask]).unwrap();
        defmt::println!("check-1");
        
        // i2c.write(PCF8574_ADDR, &[output_config]).unwrap();
        defmt::println!("check-2");
        delay.delay_ms(1000_u32); // Wait for 1 second

        // Toggle all pins of the PCF8574
        // To toggle the pins, we first read the current state of the GPIO pins,
        // then complement the bits (1s to 0s and 0s to 1s) and write back the new state.
        let mut input_buffer=[0;1];
        defmt::println!("check-3");
        i2c.read(PCF8574_ADDR, &mut input_buffer).unwrap();

        defmt::println!("{:x}",input_buffer);

        let current_state = input_buffer[0];
        let new_state = !current_state;
        i2c.write(PCF8574_ADDR, &[new_state]).unwrap();
        delay.delay_ms(1000_u32); // Wait for 1 second

        let init_commands: [u8; 5] = [0x00, 0x38, 0x00, 0x06, 0x0C];
        i2c.write(PCF8574_ADDR, &init_commands).unwrap();
    

        let clear_display: [u8; 2] = [0x00, 0x01];
        i2c.write(PCF8574_ADDR, &clear_display).unwrap();
        defmt::println!("check 4- GOPI");
        // Your name "Santosh"
        let name: [u8; 12] = [0x40, b'S', b'a', b'n', b't', b'o', b's', b'h',b' ',b' ',b'i',b's'];
    
        // Send your name to the LCD
        i2c.write(PCF8574_ADDR, &name).unwrap();

        // Set cursor to the second row
        let set_cursor_row2: [u8; 2] = [0x00, 0xC0];
        i2c.write(PCF8574_ADDR, &set_cursor_row2).unwrap();
        defmt::println!("check 5- vemula naga gopi");
        // Display text on the second row
        let text_row2: [u8; 9] = [0x40, b'I', b'n', b'n', b'o', b'c', b'e', b'n',b't'];
        i2c.write(PCF8574_ADDR, &text_row2).unwrap();
        defmt::println!("finish");
    }
}



