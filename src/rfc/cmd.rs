//! RFC command ISA encoding and sequence builder.
//!
//! The RivieraWaves BT MAC controller uses an RFC (Radio Frequency Controller)
//! command sequencer to automate RF front-end control during TX/RX operations.
//! Commands are 16-bit instructions packed two per 32-bit word, stored in RFC
//! SRAM, and their start addresses registered via `CU_ADDR_REG1/2/3`.
//!
//! This module provides the command encoding and a builder for constructing
//! command sequences. The ISA is standard across all RivieraWaves licensees.

// ============================================================================
// RFC Command Opcodes
// ============================================================================

/// Read an RFC register (by byte offset) into the sequencer's working register.
#[inline]
pub const fn rd(reg_offset: u16) -> u16 {
    0x1800 + reg_offset
}

/// Write the sequencer's working register back to an RFC register (by byte offset).
#[inline]
pub const fn wr(reg_offset: u16) -> u16 {
    0x2800 + reg_offset
}

/// Clear a single bit in the sequencer's working register.
#[inline]
pub const fn and(bit: u16) -> u16 {
    0x3000 + bit
}

/// Set a single bit in the sequencer's working register.
#[inline]
pub const fn or(bit: u16) -> u16 {
    0x4000 + bit
}

/// Wait for the specified number of microseconds.
#[inline]
pub const fn wait(us: u16) -> u16 {
    0x5000 + us
}

/// Read VCO frequency calibration result from the per-channel table
/// (addressed by `CAL_ADDR_REG1/2`).
pub const RD_FULCAL: u16 = 0x6000;

/// Read TXDC calibration coefficients (coef0/coef1) from the per-power table
/// (addressed by `CAL_ADDR_REG3`).
pub const RD_DCCAL1: u16 = 0x7000;

/// Read TXDC calibration offsets (offset_i/offset_q) from the per-power table
/// (addressed by `CAL_ADDR_REG3`).
pub const RD_DCCAL2: u16 = 0x8000;

/// End of command sequence.
pub const END: u16 = 0xF000;

// ============================================================================
// Command Sequence Builder
// ============================================================================

/// Maximum number of 16-bit commands in a single sequence.
const MAX_CMDS: usize = 128;

/// Builder for RFC command sequences.
///
/// Commands are 16-bit values accumulated in a buffer. When written to SRAM,
/// they are packed two per 32-bit word (low halfword first).
pub struct CmdBuilder {
    buf: [u16; MAX_CMDS],
    len: usize,
}

impl CmdBuilder {
    /// Create a new empty command builder.
    pub const fn new() -> Self {
        Self {
            buf: [0; MAX_CMDS],
            len: 0,
        }
    }

    /// Append a 16-bit command.
    #[inline]
    pub fn push(&mut self, cmd: u16) {
        self.buf[self.len] = cmd;
        self.len += 1;
    }

    /// Pad to even length (commands are packed as pairs into 32-bit words).
    pub fn pad_even(&mut self) {
        if self.len % 2 != 0 {
            self.push(END);
        }
    }

    /// Number of 16-bit commands in the sequence.
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }

    /// Whether the sequence is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Write the command sequence to RFC SRAM.
    ///
    /// - `base`: RFC SRAM base address (e.g., `0x4008_2000` on SiFli).
    /// - `offset`: Byte offset within SRAM to start writing.
    ///
    /// Returns the next available byte offset after the last written word.
    ///
    /// # Safety
    ///
    /// Caller must ensure `base + offset` points to valid RFC SRAM and that
    /// the sequence fits within the SRAM region.
    pub unsafe fn write_to_sram(&self, base: u32, offset: u32) -> u32 {
        let mut addr = offset;
        for i in (0..self.len).step_by(2) {
            let lo = self.buf[i] as u32;
            let hi = self.buf[i + 1] as u32;
            let word = lo | (hi << 16);
            core::ptr::write_volatile((base + addr) as *mut u32, word);
            addr += 4;
        }
        addr
    }
}
