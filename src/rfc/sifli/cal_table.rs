//! Calibration table SRAM packing/unpacking.
//!
//! The RFC hardware reads per-channel VCO and per-power-level TXDC
//! calibration data from SRAM tables addressed by `CAL_ADDR_REG1/2/3`.
//! The bit layout of these table entries is vendor-specific.
//!
//! # Table formats
//!
//! ## VCO RX table (addressed by `CAL_ADDR_REG1`)
//!
//! Each 32-bit word packs two 16-bit halfwords. For BLE RX, the low half
//! is the 1M PHY entry and the high half is the 2M PHY entry. For BT RX,
//! each half is a separate channel (ch0=low, ch1=high).
//!
//! Halfword format: `capcode[7:0] | idac[15:8]`
//!
//! ## VCO TX table (addressed by `CAL_ADDR_REG2`)
//!
//! Each 32-bit word: `capcode[7:0] | idac[14:8] | kcal[31:16]`
//!
//! ## BT TX table (EDR, also addressed by `CAL_ADDR_REG2`)
//!
//! Each 32-bit word matches [`EDR_CAL_REG1`](super::regs::edr_cal_reg1)
//! register layout, loaded by the `RD_FULCAL` command during `BT_TXON`.
//! See [`pack_edr_cal`] for the full bit layout including scattered DPSK
//! gain bits.
//!
//! ## TXDC table (addressed by `CAL_ADDR_REG3`)
//!
//! Two 32-bit words per power level:
//! - Word 1 matches [`IQ_PWR_REG1`](super::regs::iq_pwr_reg1), loaded by `RD_DCCAL1`.
//! - Word 2 matches [`IQ_PWR_REG2`](super::regs::iq_pwr_reg2), loaded by `RD_DCCAL2`.

// ============================================================================
// VCO calibration table packing
// ============================================================================

/// Pack a VCO RX halfword: `capcode[7:0] | idac[15:8]`.
///
/// Used in both BLE RX and BT RX tables. Two halfwords are combined
/// with [`pack_vco_rx_word`] to form a 32-bit SRAM entry.
#[inline]
pub const fn pack_vco_rx_half(capcode: u8, idac: u8) -> u16 {
    (capcode as u16) | ((idac as u16) << 8)
}

/// Pack two VCO RX halfwords into a 32-bit SRAM word.
///
/// - For BLE RX: `low` = 1M PHY, `high` = 2M PHY.
/// - For BT RX: `low` = even channel, `high` = odd channel.
#[inline]
pub const fn pack_vco_rx_word(low: u16, high: u16) -> u32 {
    (low as u32) | ((high as u32) << 16)
}

/// Pack a VCO TX calibration word: `capcode[7:0] | idac[14:8] | kcal[31:16]`.
///
/// One word per channel (79 channels for BLE TX, 79 for BT TX).
/// The `RD_FULCAL` command during `TXON` loads this into the VCO control
/// registers.
#[inline]
pub const fn pack_vco_tx(capcode: u8, idac: u8, kcal: u16) -> u32 {
    (capcode as u32) | ((idac as u32) << 8) | ((kcal as u32) << 16)
}

// ============================================================================
// TXDC calibration table packing
// ============================================================================

/// Packed TXDC calibration entry (two 32-bit words per power level).
pub struct TxdcCalEntry {
    /// Word 1: matches `IQ_PWR_REG1` layout. Loaded by `RD_DCCAL1`.
    ///
    /// `coef0[13:0] | coef1[27:14] | tmxbuf_gc_gfsk[31:28]`
    pub word1: u32,
    /// Word 2: matches `IQ_PWR_REG2` layout. Loaded by `RD_DCCAL2`.
    ///
    /// `offset_q[10:0] | edr_pa_bm[15:11] | offset_i[26:16] | tmxbuf_gc_dpsk[31:28]`
    pub word2: u32,
}

/// Pack a TXDC calibration table entry.
///
/// - `coef0`, `coef1`: 14-bit DC calibration coefficients.
/// - `offset_i`, `offset_q`: 11-bit DC offset values.
/// - `tmxbuf_gc`: 4-bit TX mixer buffer gain control (same value in both words).
/// - `edr_pa_bm`: 5-bit EDR PA bias current.
#[inline]
pub const fn pack_txdc(
    coef0: u16,
    coef1: u16,
    offset_i: u16,
    offset_q: u16,
    tmxbuf_gc: u8,
    edr_pa_bm: u8,
) -> TxdcCalEntry {
    let word1 = (coef0 as u32 & 0x3FFF)
        | (((coef1 as u32) & 0x3FFF) << 14)
        | (((tmxbuf_gc as u32) & 0xF) << 28);
    let word2 = (offset_q as u32 & 0x7FF)
        | (((edr_pa_bm as u32) & 0x1F) << 11)
        | (((offset_i as u32) & 0x7FF) << 16)
        | (((tmxbuf_gc as u32) & 0xF) << 28);
    TxdcCalEntry { word1, word2 }
}

// ============================================================================
// EDR_CAL_REG1 calibration table packing
// ============================================================================

/// Pack an EDR calibration table word matching the `EDR_CAL_REG1` register layout.
///
/// This format is used in the BT TX calibration table (addressed by
/// `CAL_ADDR_REG2.bt_tx_cal_addr`). The MAC's `RD_FULCAL` command during
/// `BT_TXON` sequences loads this word directly into `EDR_CAL_REG1`.
///
/// # Bit layout
///
/// ```text
/// [7:0]   capcode (brf_edr_vco_pdx_lv)
/// [14:8]  idac    (brf_edr_vco_idac_lv)
/// [15]    dpsk_gain scattered bit 0
/// [18:16] oslo_fc (brf_oslo_fc_lv)
/// [19]    dpsk_gain scattered bit 1
/// [24:20] oslo_bm (brf_oslo_bm_lv)
/// [27:25] dpsk_gain scattered bits 4:2
/// [31:28] tmxcap  (brf_trf_edr_tmxcap_sel_lv)
/// ```
///
/// # DPSK gain scattering
///
/// The 6-bit `dpsk_gain` value is right-shifted by 1 (dropping bit 0),
/// then the remaining 5 bits are scattered into the "holes" between
/// the named register fields:
///
/// - `(dpsk_gain >> 1) & 0x01` → bit 15
/// - `(dpsk_gain >> 1) & 0x02` → bit 19
/// - `(dpsk_gain >> 1) & 0x1C` → bits 27:25
#[inline]
pub const fn pack_edr_cal(
    capcode: u8,
    idac: u8,
    oslo_fc: u8,
    oslo_bm: u8,
    tmxcap: u8,
    dpsk_gain: u8,
) -> u32 {
    let mut word: u32 = 0;
    word |= capcode as u32; // [7:0]
    word |= (idac as u32) << 8; // [14:8]
    word |= ((oslo_fc as u32) & 0x7) << 16; // [18:16]
    word |= ((oslo_bm as u32) & 0x1F) << 20; // [24:20]
    word |= ((tmxcap as u32) & 0xF) << 28; // [31:28]

    // Scatter DPSK gain bits into holes
    let dg = (dpsk_gain >> 1) as u32;
    word |= (dg & 0x01) << 15; // dg[0] → bit 15
    word |= (dg & 0x02) << 18; // dg[1] → bit 19
    word |= (dg & 0x1C) << 23; // dg[4:2] → bits 27:25

    word
}
