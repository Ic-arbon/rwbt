#![no_std]

pub mod common;

/// BT MAC register blocks (DM / BT / BLE) generated from RivieraWaves YODA CSV.
pub mod mac;

/// RFC (Radio Frequency Controller) command sequencer and register definitions.
///
/// The RivieraWaves BT MAC uses an RFC command sequencer to control the RF
/// front-end during TX/RX operations. Command sequences are stored in RFC SRAM
/// and executed automatically by the MAC hardware.
pub mod rfc;
