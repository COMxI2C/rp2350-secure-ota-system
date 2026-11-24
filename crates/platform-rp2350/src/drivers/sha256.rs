//! ============================================================================
//! RP2350 SHA-256 Hardware Driver
//! ----------------------------------------------------------------------------
//! Minimal and safe hardware abstraction for the RP2350 SHA-256 accelerator.
//!
//! This driver communicates directly with the RP2350 dedicated SHA-256
//! peripheral. The hash computation is performed entirely in hardware;
//! the CPU is only responsible for feeding message data and reading the
//! resulting digest.
//!
//! Features
//! --------
//! - Hardware accelerated SHA-256
//! - no_std compatible
//! - Zero heap allocation
//! - Ownership-based API
//! - Blocking implementation
//! ============================================================================

use core::marker::PhantomData;
use embassy_rp::pac;

/// Type-state marker indicating that the peripheral has not been initialized.
pub struct Disabled;

/// Type-state marker indicating that the peripheral is ready for use.
pub struct Enabled;

/// RP2350 SHA-256 peripheral.
///
/// The generic parameter represents the initialization state of the
/// peripheral, preventing invalid usage at compile time.
pub struct Sha256<State> {
    csr: pac::sha256::Sha256,
    _state: PhantomData<State>,
}

/// Active SHA-256 hashing session.
///
/// A `Hasher` owns exclusive access to the peripheral until the digest
/// is finalized or the object is dropped.
pub struct Hasher<'a> {
    sha256: &'a mut Sha256<Enabled>,
    cache: [u8; 4],
    count: usize,
}

//=============================================================================
// Peripheral initialization
//=============================================================================

impl Sha256<Disabled> {
    /// Initializes the RP2350 SHA-256 hardware accelerator.
    ///
    /// This function:
    /// - Releases the peripheral from reset.
    /// - Configures the SHA engine.
    /// - Returns an initialized driver instance.
    pub fn new() -> Sha256<Enabled> {
        let csr = pac::SHA256;
        let resets = pac::RESETS;

        // Release the SHA256 peripheral from reset.
        resets.reset().modify(|w| w.set_sha256(true));
        resets.reset().modify(|w| w.set_sha256(false));

        while !resets.reset_done().read().sha256() {}

        // Prepare the hardware for a new hashing session.
        csr.csr().modify(|w| {
            w.set_start(true);
            w.set_bswap(false);
        });

        Sha256 {
            csr,
            _state: PhantomData,
        }
    }
}

impl Sha256<Enabled> {
    /// Starts a new SHA-256 hashing session.
    ///
    /// The returned `Hasher` provides a streaming interface used to feed
    /// data into the hardware accelerator.
    pub fn start(&mut self) -> Hasher<'_> {
        Hasher {
            sha256: self,
            cache: [0; 4],
            count: 0,
        }
    }
}

//=============================================================================
// Public Hasher API
//=============================================================================

impl Hasher<'_> {
    /// Writes a byte slice into the SHA-256 engine.
    ///
    /// Data is internally packed into 32-bit words before being sent
    /// to the hardware peripheral.
    pub fn update(&mut self, input: &[u8]) {
        for &byte in input {
            self.write_u8(byte);
        }
    }

    /// Finalizes the SHA-256 computation.
    ///
    /// Performs the mandatory SHA-256 message padding, waits until the
    /// hardware accelerator finishes the computation, and returns the
    /// resulting 256-bit digest.
    pub fn finalize(mut self) -> [u32; 8] {
        let cache_idx = self.count % 4;

        // Append the mandatory SHA-256 padding bit.
        self.cache[cache_idx] = 0x80;
        self.write_word(u32::from_be_bytes(self.cache));

        // Pad the message to the next 512-bit boundary, leaving space
        // for the final 64-bit message length.
        let msg_words = (self.count + 4) / 4;
        let total_words = (msg_words + 2 + 15) & !15;
        let zeros = total_words - (msg_words + 2);

        for _ in 0..zeros {
            self.write_word(0);
        }

        // Append the original message length in bits.
        let bit_count = self.count as u64 * 8;

        self.write_word((bit_count >> 32) as u32);
        self.write_word(bit_count as u32);

        // Wait until the hardware reports that the digest is valid.
        let csr = &self.sha256.csr;

        while !csr.csr().read().sum_vld() {
            core::hint::spin_loop();
        }

        [
            csr.sum0().read(),
            csr.sum1().read(),
            csr.sum2().read(),
            csr.sum3().read(),
            csr.sum4().read(),
            csr.sum5().read(),
            csr.sum6().read(),
            csr.sum7().read(),
        ]
    }
}

//=============================================================================
// Private implementation
//=============================================================================

impl Hasher<'_> {
    /// Buffers incoming bytes until a complete 32-bit word is available.
    fn write_u8(&mut self, byte: u8) {
        let idx = self.count % 4;

        self.cache[idx] = byte;
        self.count += 1;

        if idx == 3 {
            self.write_word(u32::from_be_bytes(self.cache));
            self.cache = [0; 4];
        }
    }

    /// Writes one 32-bit word to the SHA-256 peripheral.
    ///
    /// Blocks until the hardware indicates it is ready to receive data.
    fn write_word(&mut self, word: u32) {
        while !self.sha256.csr.csr().read().wdata_rdy() {
            core::hint::spin_loop();
        }

        self.sha256.csr.wdata().write_value(word);
    }
}

//=============================================================================
// Automatic cleanup
//=============================================================================

impl Drop for Hasher<'_> {
    /// Resets the SHA-256 peripheral when the hashing session ends.
    ///
    /// This guarantees that every new hashing session starts from a
    /// clean hardware state, even if `finalize()` was never called.
    fn drop(&mut self) {
        self.sha256.csr.csr().modify(|w| {
            w.set_start(true);
            w.set_bswap(false);
        });
    }
}