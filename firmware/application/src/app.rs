//! Main application behavior for the RP2350 firmware.

use defmt::info;
use embassy_rp::gpio::Output;
use embassy_time::Timer;

use platform_rp2350::drivers::sha256::Sha256;

/// Runs the firmware application.
///
/// Receives the initialized status LED and manages the SHA-256
/// hardware accelerator through the RP2350 platform crate.
pub async fn run(mut led: Output<'static>) {
    let mut sha256 = Sha256::new();

    loop {
        let message = b"abc";

        info!("Computing SHA-256 digest for reference message");

        let mut hasher = sha256.start();
        hasher.update(message);
        let digest = hasher.finalize();

        info!(
            "SHA-256: {:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}{:08x}",
            digest[0], digest[1], digest[2], digest[3],
            digest[4], digest[5], digest[6], digest[7]
        );

        led.toggle();
        Timer::after_secs(1).await;
    }
}
