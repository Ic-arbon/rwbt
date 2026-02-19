//! SiFli RF front-end register byte offsets and field bit-position constants.
//!
//! These constants serve two purposes:
//! 1. **Register offsets** → used in RFC command `rd()`/`wr()` instructions.
//! 2. **Bit positions** → used in RFC command `or()`/`and()` instructions.
//!
//! The bit positions are extracted from the SiFli PAC (`sifli-pac` bt_rfc/regs.rs).
//! For boolean (1-bit) fields, the constant IS the bit number.
//! For multi-bit fields, the constant is the LSB position; individual bits can be
//! addressed as `FIELD + n` where `n < width`.

// ============================================================================
// Register byte offsets (for RFC command rd/wr instructions)
// ============================================================================

/// Register byte offsets within the BT_RFC peripheral.
///
/// Used as arguments to [`super::cmd::rd`] and [`super::cmd::wr`].
pub mod offset {
    pub const VCO_REG1: u16 = 0x00;
    pub const VCO_REG2: u16 = 0x04;
    pub const VCO_REG3: u16 = 0x08;
    pub const MISC_CTRL_REG: u16 = 0x0C;
    pub const RF_LODIST_REG: u16 = 0x10;
    pub const FBDV_REG1: u16 = 0x14;
    pub const FBDV_REG2: u16 = 0x18;
    pub const PFDCP_REG: u16 = 0x1C;
    pub const LPF_REG: u16 = 0x20;
    pub const EDR_CAL_REG1: u16 = 0x24;
    pub const OSLO_REG: u16 = 0x28;
    pub const TRF_REG1: u16 = 0x34;
    pub const TRF_REG2: u16 = 0x38;
    pub const TRF_EDR_REG1: u16 = 0x3C;
    pub const TRF_EDR_REG2: u16 = 0x40;
    pub const RRF_REG: u16 = 0x44;
    pub const RBB_REG1: u16 = 0x48;
    pub const RBB_REG2: u16 = 0x4C;
    pub const RBB_REG3: u16 = 0x50;
    pub const RBB_REG5: u16 = 0x58;
    pub const ADC_REG: u16 = 0x60;
    pub const TBB_REG: u16 = 0x64;
    /// Note: PAC defines AtstbufReg at 0x68, but the SDK RFC command sequences
    /// use 0x6C. We follow the SDK value (verified working).
    pub const ATSTBUF_REG: u16 = 0x6C;
    pub const INCCAL_REG1: u16 = 0x74;
    pub const IQ_PWR_REG1: u16 = 0xA8;
    pub const IQ_PWR_REG2: u16 = 0xAC;
}

// ============================================================================
// Per-register bit-position constants
// ============================================================================

/// VCO_REG1 (offset 0x00) — VCO control.
pub mod vco_reg1 {
    /// 2M modulation enable. (1-bit)
    pub const BRF_EN_2M_MOD_LV: u16 = 0;
    /// VCO varactor bias. (3-bit, base)
    pub const BRF_VCO_VAR_VVN_BM_LV: u16 = 1;
    /// VCO capacitor bank bias. (3-bit, base)
    pub const BRF_VCO_CBANK_VVN_BM_LV: u16 = 4;
    /// VCO filter enable. (1-bit)
    pub const BRF_VCO_FLT_EN_LV: u16 = 7;
    /// VCO LDO reference voltage. (4-bit, base)
    pub const BRF_VCO_LDO_VREF_LV: u16 = 8;
    /// 5GHz VCO enable (BLE). (1-bit)
    pub const BRF_VCO5G_EN_LV: u16 = 12;
    /// 3GHz VCO enable (EDR). (1-bit)
    pub const BRF_VCO3G_EN_LV: u16 = 13;
}

/// VCO_REG2 (offset 0x04) — VCO calibration control.
pub mod vco_reg2 {
    /// ACAL voltage low threshold select. (4-bit, base)
    pub const BRF_VCO_ACAL_VL_SEL_LV: u16 = 0;
    /// ACAL voltage high threshold select. (4-bit, base)
    pub const BRF_VCO_ACAL_VH_SEL_LV: u16 = 4;
    /// ACAL enable. (1-bit)
    pub const BRF_VCO_ACAL_EN_LV: u16 = 8;
    /// FKCAL enable. (1-bit)
    pub const BRF_VCO_FKCAL_EN_LV: u16 = 19;
}

/// RF_LODIST_REG (offset 0x10) — LO distribution and power control.
pub mod rf_lodist_reg {
    /// EDR LO distribution enable. (1-bit)
    pub const BRF_LODISTEDR_EN_LV: u16 = 0;
    /// EDR TX LO distribution enable. (1-bit)
    pub const BRF_LODIST5G_EDRTX_EN_LV: u16 = 7;
    /// BLE TX LO distribution enable. (1-bit)
    pub const BRF_LODIST5G_BLETX_EN_LV: u16 = 8;
    /// RX LO distribution enable. (1-bit)
    pub const BRF_LODIST5G_RX_EN_LV: u16 = 9;
    /// LO I-array enable. (1-bit)
    pub const BRF_LO_IARY_EN_LV: u16 = 16;
    /// RF bandgap enable. (1-bit)
    pub const BRF_EN_RFBG_LV: u16 = 17;
    /// VDD power switch enable. (1-bit)
    pub const BRF_EN_VDDPSW_LV: u16 = 18;
}

/// FBDV_REG1 (offset 0x14) — Frequency binary divider control.
pub mod fbdv_reg1 {
    /// FKCAL counter ready (read-only). (1-bit)
    pub const BRF_FKCAL_CNT_RDY_LV: u16 = 0;
    /// FKCAL counter reset-bar. (1-bit)
    pub const BRF_FKCAL_CNT_RSTB_LV: u16 = 1;
    /// FKCAL counter enable. (1-bit)
    pub const BRF_FKCAL_CNT_EN_LV: u16 = 2;
    /// SDM clock select. (1-bit)
    pub const BRF_SDM_CLK_SEL_LV: u16 = 3;
    /// FBDV modulator stage. (2-bit, base)
    pub const BRF_FBDV_MOD_STG_LV: u16 = 4;
    /// FBDV reset-bar. (1-bit)
    pub const BRF_FBDV_RSTB_LV: u16 = 7;
    /// FBDV enable. (1-bit)
    pub const BRF_FBDV_EN_LV: u16 = 12;
}

/// PFDCP_REG (offset 0x1C) — PFD and charge pump control.
pub mod pfdcp_reg {
    /// PFD/CP ICP setting. (4-bit, base at bit 11)
    pub const BRF_PFDCP_ICP_SET_LV: u16 = 11;
    /// PFD/CP enable. (1-bit)
    pub const BRF_PFDCP_EN_LV: u16 = 19;
}

/// EDR_CAL_REG1 (offset 0x24) — EDR VCO calibration result.
///
/// This register is also the target format for the BT TX calibration table
/// entries read by `RD_FULCAL` during BT_TXON sequences.
pub mod edr_cal_reg1 {
    /// EDR VCO capcode (PDX). (8-bit, base)
    pub const BRF_EDR_VCO_PDX_LV: u16 = 0;
    /// EDR VCO IDAC. (7-bit, base)
    pub const BRF_EDR_VCO_IDAC_LV: u16 = 8;
    /// OSLO frequency code. (3-bit, base)
    pub const BRF_OSLO_FC_LV: u16 = 16;
    /// OSLO bias current. (5-bit, base)
    pub const BRF_OSLO_BM_LV: u16 = 20;
    /// EDR TMXCAP select. (4-bit, base)
    pub const BRF_TRF_EDR_TMXCAP_SEL_LV: u16 = 28;
}

/// OSLO_REG (offset 0x28) — EDR output-stage LO control.
pub mod oslo_reg {
    /// OSLO enable. (1-bit)
    pub const BRF_OSLO_EN_LV: u16 = 11;
}

/// TRF_REG1 (offset 0x34) — TX RF front-end control 1.
pub mod trf_reg1 {
    /// PA cascode bypass. (1-bit)
    pub const BRF_PA_CAS_BP_LV: u16 = 0;
    /// PA power mode. (2-bit, base)
    pub const BRF_PA_PM_LV: u16 = 1;
    /// TX RF signal enable. (1-bit)
    pub const BRF_TRF_SIG_EN_LV: u16 = 16;
    /// PA output power-up. (1-bit)
    pub const BRF_PA_OUT_PU_LV: u16 = 21;
    /// PA buffer power-up. (1-bit)
    pub const BRF_PA_BUF_PU_LV: u16 = 22;
}

/// TRF_REG2 (offset 0x38) — TX RF front-end control 2.
pub mod trf_reg2 {
    /// PA TX/RX mode switch. (1-bit)
    pub const BRF_PA_TX_RX_LV: u16 = 9;
    /// PA matching cap. (1-bit)
    pub const BRF_PA_MCAP_LV: u16 = 10;
}

/// TRF_EDR_REG1 (offset 0x3C) — EDR TX front-end control 1.
pub mod trf_edr_reg1 {
    /// EDR PA power-up. (1-bit)
    pub const BRF_TRF_EDR_PA_PU_LV: u16 = 2;
    /// EDR TMX power-up. (1-bit)
    pub const BRF_TRF_EDR_TMX_PU_LV: u16 = 12;
    /// EDR TMX buffer power-up. (1-bit)
    pub const BRF_TRF_EDR_TMXBUF_PU_LV: u16 = 19;
    /// EDR I-array enable. (1-bit)
    pub const BRF_TRF_EDR_IARRAY_EN_LV: u16 = 20;
}

/// TRF_EDR_REG2 (offset 0x40) — EDR TX front-end control 2.
pub mod trf_edr_reg2 {
    /// Power meter enable. (1-bit)
    pub const BRF_TRF_EDR_PWRMTR_EN_LV: u16 = 10;
    /// PA transformer single-ended/differential. (1-bit)
    pub const BRF_TRF_EDR_PA_XFMR_SG_LV: u16 = 11;
    /// PA capacitor enable. (1-bit)
    pub const BRF_TRF_EDR_PACAP_EN_LV: u16 = 17;
}

/// RRF_REG (offset 0x44) — RX RF front-end control.
pub mod rrf_reg {
    /// Mixer power-up. (1-bit)
    pub const BRF_MX_PU_LV: u16 = 3;
    /// LNA shunt switch. (1-bit)
    pub const BRF_LNA_SHUNTSW_LV: u16 = 6;
    /// LNA power-up. (1-bit)
    pub const BRF_LNA_PU_LV: u16 = 17;
    /// RRF LDO 1.1V enable. (1-bit)
    pub const BRF_RRF_LDO11_EN_LV: u16 = 22;
}

/// RBB_REG1 (offset 0x48) — RX baseband control 1.
pub mod rbb_reg1 {
    /// RBB LDO enable. (1-bit)
    pub const BRF_EN_LDO_RBB_LV: u16 = 13;
}

/// RBB_REG2 (offset 0x4C) — RX baseband control 2.
pub mod rbb_reg2 {
    /// RVGA Q-channel enable. (1-bit)
    pub const BRF_EN_RVGA_Q_LV: u16 = 6;
    /// RVGA I-channel enable. (1-bit)
    pub const BRF_EN_RVGA_I_LV: u16 = 7;
    /// CBPF (channel bandpass filter) enable. (1-bit)
    pub const BRF_EN_CBPF_LV: u16 = 27;
}

/// RBB_REG3 (offset 0x50) — RX baseband control 3.
pub mod rbb_reg3 {
    /// Packet detector enable. (4-bit, base — set/clear individual bits 0-3)
    pub const BRF_EN_PKDET_LV: u16 = 0;
}

/// RBB_REG5 (offset 0x58) — RX baseband control 5.
pub mod rbb_reg5 {
    /// TX loopback enable (power meter → RVGA → ADC). (1-bit)
    pub const BRF_RVGA_TX_LPBK_EN_LV: u16 = 0;
    /// I-array enable. (1-bit)
    pub const BRF_EN_IARRAY_LV: u16 = 5;
    /// Offset DAC Q enable. (1-bit)
    pub const BRF_EN_OSDACQ_LV: u16 = 6;
    /// Offset DAC I enable. (1-bit)
    pub const BRF_EN_OSDACI_LV: u16 = 7;
}

/// ADC_REG (offset 0x60) — ADC control.
pub mod adc_reg {
    /// ADC reference LDO enable. (1-bit)
    pub const BRF_EN_LDO_ADCREF_LV: u16 = 4;
    /// ADC LDO enable. (1-bit)
    pub const BRF_EN_LDO_ADC_LV: u16 = 9;
    /// ADC Q-channel enable. (1-bit)
    pub const BRF_EN_ADC_Q_LV: u16 = 20;
    /// ADC I-channel enable. (1-bit)
    pub const BRF_EN_ADC_I_LV: u16 = 21;
}

/// TBB_REG (offset 0x64) — TX baseband (DAC) control.
pub mod tbb_reg {
    /// TBB I-array enable. (1-bit)
    pub const BRF_EN_TBB_IARRAY_LV: u16 = 8;
    /// DAC digital VDD LDO enable. (1-bit)
    pub const BRF_EN_LDO_DAC_DVDD_LV: u16 = 9;
    /// DAC analog VDD LDO enable. (1-bit)
    pub const BRF_EN_LDO_DAC_AVDD_LV: u16 = 10;
    /// DAC enable. (1-bit)
    pub const BRF_EN_DAC_LV: u16 = 11;
    /// DAC start. (1-bit)
    pub const BRF_DAC_START_LV: u16 = 12;
}

/// INCCAL_REG1 (offset 0x74) — Incremental calibration control.
pub mod inccal_reg1 {
    /// Start incremental calibration. (1-bit)
    pub const INCCAL_START: u16 = 29;
}

/// IQ_PWR_REG1 (offset 0xA8) — TX DC calibration coefficients.
///
/// Written by `RD_DCCAL1` during BT_TXON sequences.
pub mod iq_pwr_reg1 {
    /// TX DC calibration coefficient 0. (14-bit, base)
    pub const TX_DC_CAL_COEF0: u16 = 0;
    /// TX DC calibration coefficient 1. (14-bit, base)
    pub const TX_DC_CAL_COEF1: u16 = 14;
    /// EDR TMXBUF gain control (GFSK). (4-bit, base)
    pub const EDR_TMXBUF_GC_GFSK: u16 = 28;
}

/// IQ_PWR_REG2 (offset 0xAC) — TX DC calibration offsets.
///
/// Written by `RD_DCCAL2` during BT_TXON sequences.
pub mod iq_pwr_reg2 {
    /// TX DC calibration offset Q. (11-bit, base)
    pub const TX_DC_CAL_OFFSET_Q: u16 = 0;
    /// EDR PA bias current. (5-bit, base)
    pub const BRF_TRF_EDR_PA_BM_LV: u16 = 11;
    /// TX DC calibration offset I. (11-bit, base)
    pub const TX_DC_CAL_OFFSET_I: u16 = 16;
    /// EDR TMXBUF gain control (DPSK). (4-bit, base)
    pub const EDR_TMXBUF_GC_DPSK: u16 = 28;
}
