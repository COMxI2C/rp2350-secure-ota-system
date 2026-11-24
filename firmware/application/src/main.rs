#![no_std]
#![no_main]

//! RP2350 secure OTA firmware application entry point.

use embassy_executor::Spawner;
use embassy_rp as hal;
use embassy_rp::block::ImageDef;
use embassy_rp::gpio::{Level, Output};

use {panic_probe as _};
use defmt_rtt as _;

mod app;

/// Tell the RP2350 Boot ROM that this is a secure executable.
#[unsafe(link_section = ".start_block")]
#[used]
pub static IMAGE_DEF: ImageDef = hal::block::ImageDef::secure_exe();

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = hal::init(Default::default());
    let status_led = Output::new(peripherals.PIN_6, Level::Low);

    app::run(status_led).await;
}

/// Program metadata exposed through picotool.
#[unsafe(link_section = ".bi_entries")]
#[used]
pub static PICOTOOL_ENTRIES: [embassy_rp::binary_info::EntryAddr; 4] = [
    embassy_rp::binary_info::rp_program_name!(c"rp2350-secure-ota-system"),
    embassy_rp::binary_info::rp_program_description!(
        c"RP2350 firmware platform with hardware-accelerated SHA-256"
    ),
    embassy_rp::binary_info::rp_cargo_version!(),
    embassy_rp::binary_info::rp_program_build_attribute!(),
];
