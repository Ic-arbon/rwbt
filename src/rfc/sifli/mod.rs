/// RFC register byte offsets and field bit-position constants for SiFli RF front-end.
pub mod regs;

/// Calibration table SRAM packing/unpacking for SiFli RF front-end.
///
/// The RFC hardware reads per-channel VCO and per-power-level TXDC
/// calibration data from SRAM tables addressed by `CAL_ADDR_REG1/2/3`.
pub mod cal_table;
