use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use esp_idf_hal::i2c::*;
use esp_idf_hal::prelude::*;
use esp_idf_hal::peripherals::Peripherals;

use embedded_graphics::{
    prelude::*,
    text::{Baseline, Text},
    pixelcolor::BinaryColor,
    mono_font::{ascii::FONT_6X10, MonoTextStyleBuilder},
    primitives::{PrimitiveStyleBuilder, Rectangle}
};

fn main() -> anyhow::Result<()>
{
    // It is necessary to call this function once. Otherwise, some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take()?;
    let scl = peripherals.pins.gpio22;
    let sda = peripherals.pins.gpio21;
    let i2c0 = peripherals.i2c0;


    let config = I2cConfig::new().baudrate(100.kHz().into());
    let i2c = I2cDriver::new(i2c0, sda, scl, &config)?;
    let i2c_display_interface = I2CDisplayInterface::new(i2c);


    let display_size = DisplaySize128x64;
    let display_rotation = DisplayRotation::Rotate0;

    let mut display = Ssd1306::new(i2c_display_interface, display_size, display_rotation)
        .into_buffered_graphics_mode();

    let on = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_6X10)
        .text_color(BinaryColor::On)
        .build();

    display.init().unwrap();

    Rectangle::new(Point::new(0, 0), Size::new(127, 63))
        .into_styled(on)
        .draw(&mut display)
        .unwrap();

    Text::with_baseline("Hello world!", Point::new(0, 16), text_style, Baseline::Top)
        .draw(&mut display)
        .unwrap();

    loop {
        display.flush().unwrap();
        log::info!("Hello, world!");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}