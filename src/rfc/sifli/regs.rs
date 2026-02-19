//! Auto-generated from sifli-pac bt_rfc.yaml. Do not edit manually.
//!
//! Regenerate with:
//! ```sh
//! cargo run --features _gen --bin gen-sifli-regs -- <path-to-bt_rfc.yaml> > src/rfc/sifli/regs.rs
//! rustfmt src/rfc/sifli/regs.rs
//! ```
//!
//! These constants serve two purposes:
//! 1. **Register offsets** → used in RFC command `rd()`/`wr()` instructions.
//! 2. **Bit positions** → used in RFC command `or()`/`and()` instructions.
//!
//! For boolean (1-bit) fields, the constant IS the bit number.
//! For multi-bit fields, the constant is the LSB position; individual bits can be
//! addressed as `FIELD + n` where `n < width`.

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
    pub const ATEST_REG: u16 = 0x2C;
    pub const DTEST_REG: u16 = 0x30;
    pub const TRF_REG1: u16 = 0x34;
    pub const TRF_REG2: u16 = 0x38;
    pub const TRF_EDR_REG1: u16 = 0x3C;
    pub const TRF_EDR_REG2: u16 = 0x40;
    pub const RRF_REG: u16 = 0x44;
    pub const RBB_REG1: u16 = 0x48;
    pub const RBB_REG2: u16 = 0x4C;
    pub const RBB_REG3: u16 = 0x50;
    pub const RBB_REG4: u16 = 0x54;
    pub const RBB_REG5: u16 = 0x58;
    pub const RBB_REG6: u16 = 0x5C;
    pub const ADC_REG: u16 = 0x60;
    pub const TBB_REG: u16 = 0x64;
    pub const ATSTBUF_REG: u16 = 0x68;
    pub const INCCAL_REG1: u16 = 0x74;
    pub const INCCAL_REG2: u16 = 0x78;
    pub const ROSCAL_REG1: u16 = 0x7C;
    pub const ROSCAL_REG2: u16 = 0x80;
    pub const RCROSCAL_REG: u16 = 0x84;
    pub const PACAL_REG: u16 = 0x88;
    pub const CU_ADDR_REG1: u16 = 0x8C;
    pub const CU_ADDR_REG2: u16 = 0x90;
    pub const CU_ADDR_REG3: u16 = 0x94;
    pub const CAL_ADDR_REG1: u16 = 0x98;
    pub const CAL_ADDR_REG2: u16 = 0x9C;
    pub const CAL_ADDR_REG3: u16 = 0xA0;
    pub const AGC_REG: u16 = 0xA4;
    pub const IQ_PWR_REG1: u16 = 0xA8;
    pub const IQ_PWR_REG2: u16 = 0xAC;
}

/// VCO_REG1 (offset 0x00)
pub mod vco_reg1 {
    /// brf_en_2m_mod_lv — 1 bit (offset 0)
    pub const BRF_EN_2M_MOD_LV: u16 = 0;
    /// brf_vco_var_vvn_bm_lv — 3 bits (offset 1)
    pub const BRF_VCO_VAR_VVN_BM_LV: u16 = 1;
    /// brf_vco_cbank_vvn_bm_lv — 3 bits (offset 4)
    pub const BRF_VCO_CBANK_VVN_BM_LV: u16 = 4;
    /// brf_vco_flt_en_lv — 1 bit (offset 7)
    pub const BRF_VCO_FLT_EN_LV: u16 = 7;
    /// brf_vco_ldo_vref_lv — 4 bits (offset 8)
    pub const BRF_VCO_LDO_VREF_LV: u16 = 8;
    /// brf_vco5g_en_lv — 1 bit (offset 12)
    pub const BRF_VCO5G_EN_LV: u16 = 12;
    /// brf_vco3g_en_lv — 1 bit (offset 13)
    pub const BRF_VCO3G_EN_LV: u16 = 13;
}

/// VCO_REG2 (offset 0x04)
pub mod vco_reg2 {
    /// brf_vco_acal_vl_sel_lv — 4 bits (offset 0)
    pub const BRF_VCO_ACAL_VL_SEL_LV: u16 = 0;
    /// brf_vco_acal_vh_sel_lv — 4 bits (offset 4)
    pub const BRF_VCO_ACAL_VH_SEL_LV: u16 = 4;
    /// brf_vco_acal_en_lv — 1 bit (offset 8)
    pub const BRF_VCO_ACAL_EN_LV: u16 = 8;
    /// brf_vco_incfcal_vl_sel_lv — 3 bits (offset 9)
    pub const BRF_VCO_INCFCAL_VL_SEL_LV: u16 = 9;
    /// brf_vco_incfcal_vh_sel_lv — 3 bits (offset 12)
    pub const BRF_VCO_INCFCAL_VH_SEL_LV: u16 = 12;
    /// brf_vco_incfcal_en_lv — 1 bit (offset 15)
    pub const BRF_VCO_INCFCAL_EN_LV: u16 = 15;
    /// brf_vco_fkcal_vc_sel_lv — 3 bits (offset 16)
    pub const BRF_VCO_FKCAL_VC_SEL_LV: u16 = 16;
    /// brf_vco_fkcal_en_lv — 1 bit (offset 19)
    pub const BRF_VCO_FKCAL_EN_LV: u16 = 19;
    /// brf_en_mod_inphase_lv — 1 bit (offset 20)
    pub const BRF_EN_MOD_INPHASE_LV: u16 = 20;
    /// brf_vco3g_acal_up_lv — 1 bit (offset 21)
    pub const BRF_VCO3G_ACAL_UP_LV: u16 = 21;
    /// brf_vco3g_acal_incal_lv — 1 bit (offset 22)
    pub const BRF_VCO3G_ACAL_INCAL_LV: u16 = 22;
    /// brf_vco3g_incfcal_up_lv — 1 bit (offset 23)
    pub const BRF_VCO3G_INCFCAL_UP_LV: u16 = 23;
    /// brf_vco3g_incfcal_incal_lv — 1 bit (offset 24)
    pub const BRF_VCO3G_INCFCAL_INCAL_LV: u16 = 24;
    /// brf_vco5g_acal_up_lv — 1 bit (offset 25)
    pub const BRF_VCO5G_ACAL_UP_LV: u16 = 25;
    /// brf_vco5g_acal_incal_lv — 1 bit (offset 26)
    pub const BRF_VCO5G_ACAL_INCAL_LV: u16 = 26;
    /// brf_vco5g_incfcal_up_lv — 1 bit (offset 27)
    pub const BRF_VCO5G_INCFCAL_UP_LV: u16 = 27;
    /// brf_vco5g_incfcal_incal_lv — 1 bit (offset 28)
    pub const BRF_VCO5G_INCFCAL_INCAL_LV: u16 = 28;
}

/// VCO_REG3 (offset 0x08)
pub mod vco_reg3 {
    /// brf_vco_pdx_lv — 8 bits (offset 0)
    pub const BRF_VCO_PDX_LV: u16 = 0;
    /// brf_vco_idac_lv — 7 bits (offset 8)
    pub const BRF_VCO_IDAC_LV: u16 = 8;
    /// tx_kcal — 12 bits (offset 16)
    pub const TX_KCAL: u16 = 16;
}

/// MISC_CTRL_REG (offset 0x0C)
pub mod misc_ctrl_reg {
    /// pdx_force_en — 1 bit (offset 0)
    pub const PDX_FORCE_EN: u16 = 0;
    /// idac_force_en — 1 bit (offset 1)
    pub const IDAC_FORCE_EN: u16 = 1;
    /// xtal_ref_en — 1 bit (offset 2)
    pub const XTAL_REF_EN: u16 = 2;
    /// adc_clk_en — 1 bit (offset 3)
    pub const ADC_CLK_EN: u16 = 3;
    /// adc_fifo_clk_phase_sel — 1 bit (offset 4)
    pub const ADC_FIFO_CLK_PHASE_SEL: u16 = 4;
    /// unlock_flag_clr — 1 bit (offset 5)
    pub const UNLOCK_FLAG_CLR: u16 = 5;
    /// xtal_ref_en_frc_en — 1 bit (offset 6)
    pub const XTAL_REF_EN_FRC_EN: u16 = 6;
    /// adc_clk_en_frc_en — 1 bit (offset 7)
    pub const ADC_CLK_EN_FRC_EN: u16 = 7;
    /// adc_clk_sel — 1 bit (offset 8)
    pub const ADC_CLK_SEL: u16 = 8;
    /// adc_clk_sel_frc_en — 1 bit (offset 9)
    pub const ADC_CLK_SEL_FRC_EN: u16 = 9;
    /// cbpf_bw_frc_en — 1 bit (offset 10)
    pub const CBPF_BW_FRC_EN: u16 = 10;
    /// cbpf_wx2_stg1_frc_en — 1 bit (offset 11)
    pub const CBPF_WX2_STG1_FRC_EN: u16 = 11;
    /// cbpf_wx2_stg2_frc_en — 1 bit (offset 12)
    pub const CBPF_WX2_STG2_FRC_EN: u16 = 12;
    /// rvga_wx2_stg1_frc_en — 1 bit (offset 13)
    pub const RVGA_WX2_STG1_FRC_EN: u16 = 13;
    /// rvga_wx2_stg2_frc_en — 1 bit (offset 14)
    pub const RVGA_WX2_STG2_FRC_EN: u16 = 14;
    /// pkdet_en_early_off_en — 1 bit (offset 15)
    pub const PKDET_EN_EARLY_OFF_EN: u16 = 15;
    /// xtal_rfch_sel_en — 1 bit (offset 16)
    pub const XTAL_RFCH_SEL_EN: u16 = 16;
    /// iq_swap_en — 1 bit (offset 17)
    pub const IQ_SWAP_EN: u16 = 17;
    /// bt_xtal_ref_en — 1 bit (offset 18)
    pub const BT_XTAL_REF_EN: u16 = 18;
    /// dac_clk_en — 1 bit (offset 19)
    pub const DAC_CLK_EN: u16 = 19;
    /// edr_unlock_flag_clr — 1 bit (offset 20)
    pub const EDR_UNLOCK_FLAG_CLR: u16 = 20;
    /// edr_xtal_ref_en — 1 bit (offset 21)
    pub const EDR_XTAL_REF_EN: u16 = 21;
    /// edr_xtal_ref_en_frc_en — 1 bit (offset 22)
    pub const EDR_XTAL_REF_EN_FRC_EN: u16 = 22;
    /// dac_clk_en_frc_en — 1 bit (offset 23)
    pub const DAC_CLK_EN_FRC_EN: u16 = 23;
    /// bypass_dac_fifo — 1 bit (offset 24)
    pub const BYPASS_DAC_FIFO: u16 = 24;
    /// dac_wclk_edge_sel — 1 bit (offset 25)
    pub const DAC_WCLK_EDGE_SEL: u16 = 25;
    /// adc_q_en_frc_en — 1 bit (offset 26)
    pub const ADC_Q_EN_FRC_EN: u16 = 26;
    /// en_2m_mod_frc_en — 1 bit (offset 27)
    pub const EN_2M_MOD_FRC_EN: u16 = 27;
}

/// RF_LODIST_REG (offset 0x10)
pub mod rf_lodist_reg {
    /// brf_lodistedr_en_lv — 1 bit (offset 0)
    pub const BRF_LODISTEDR_EN_LV: u16 = 0;
    /// brf_lodistedr_ldo_vref_lv — 4 bits (offset 1)
    pub const BRF_LODISTEDR_LDO_VREF_LV: u16 = 1;
    /// brf_lodistedr_tx_sel_lv — 2 bits (offset 5)
    pub const BRF_LODISTEDR_TX_SEL_LV: u16 = 5;
    /// brf_lodist5g_edrtx_en_lv — 1 bit (offset 7)
    pub const BRF_LODIST5G_EDRTX_EN_LV: u16 = 7;
    /// brf_lodist5g_bletx_en_lv — 1 bit (offset 8)
    pub const BRF_LODIST5G_BLETX_EN_LV: u16 = 8;
    /// brf_lodist5g_rx_en_lv — 1 bit (offset 9)
    pub const BRF_LODIST5G_RX_EN_LV: u16 = 9;
    /// brf_lodist5g_fbdv_str_lv — 2 bits (offset 10)
    pub const BRF_LODIST5G_FBDV_STR_LV: u16 = 10;
    /// brf_lodist5g_tx_str_lv — 2 bits (offset 12)
    pub const BRF_LODIST5G_TX_STR_LV: u16 = 12;
    /// brf_lodist5g_rx_str_lv — 2 bits (offset 14)
    pub const BRF_LODIST5G_RX_STR_LV: u16 = 14;
    /// brf_lo_iary_en_lv — 1 bit (offset 16)
    pub const BRF_LO_IARY_EN_LV: u16 = 16;
    /// brf_en_rfbg_lv — 1 bit (offset 17)
    pub const BRF_EN_RFBG_LV: u16 = 17;
    /// brf_en_vddpsw_lv — 1 bit (offset 18)
    pub const BRF_EN_VDDPSW_LV: u16 = 18;
}

/// FBDV_REG1 (offset 0x14)
pub mod fbdv_reg1 {
    /// brf_fkcal_cnt_rdy_lv — 1 bit (offset 0)
    pub const BRF_FKCAL_CNT_RDY_LV: u16 = 0;
    /// brf_fkcal_cnt_rstb_lv — 1 bit (offset 1)
    pub const BRF_FKCAL_CNT_RSTB_LV: u16 = 1;
    /// brf_fkcal_cnt_en_lv — 1 bit (offset 2)
    pub const BRF_FKCAL_CNT_EN_LV: u16 = 2;
    /// brf_sdm_clk_sel_lv — 1 bit (offset 3)
    pub const BRF_SDM_CLK_SEL_LV: u16 = 3;
    /// brf_fbdv_mod_stg_lv — 2 bits (offset 4)
    pub const BRF_FBDV_MOD_STG_LV: u16 = 4;
    /// brf_fbdv_rstb_sync_en_lv — 1 bit (offset 6)
    pub const BRF_FBDV_RSTB_SYNC_EN_LV: u16 = 6;
    /// brf_fbdv_rstb_lv — 1 bit (offset 7)
    pub const BRF_FBDV_RSTB_LV: u16 = 7;
    /// brf_fbdv_ldo_vref_lv — 4 bits (offset 8)
    pub const BRF_FBDV_LDO_VREF_LV: u16 = 8;
    /// brf_fbdv_en_lv — 1 bit (offset 12)
    pub const BRF_FBDV_EN_LV: u16 = 12;
}

/// FBDV_REG2 (offset 0x18)
pub mod fbdv_reg2 {
    /// brf_fkcal_cnt_divn_lv — 16 bits (offset 0)
    pub const BRF_FKCAL_CNT_DIVN_LV: u16 = 0;
    /// brf_fkcal_cnt_op_lv — 16 bits (offset 16)
    pub const BRF_FKCAL_CNT_OP_LV: u16 = 16;
}

/// PFDCP_REG (offset 0x1C)
pub mod pfdcp_reg {
    /// brf_csd_dn_lv — 1 bit (offset 0)
    pub const BRF_CSD_DN_LV: u16 = 0;
    /// brf_csd_up_lv — 1 bit (offset 1)
    pub const BRF_CSD_UP_LV: u16 = 1;
    /// brf_lo_unlock_lv — 1 bit (offset 2)
    pub const BRF_LO_UNLOCK_LV: u16 = 2;
    /// brf_pfdcp_csd_reset_lv — 1 bit (offset 3)
    pub const BRF_PFDCP_CSD_RESET_LV: u16 = 3;
    /// brf_pfdcp_csd_en_lv — 1 bit (offset 4)
    pub const BRF_PFDCP_CSD_EN_LV: u16 = 4;
    /// brf_pfdcp_icp_os_lv — 6 bits (offset 5)
    pub const BRF_PFDCP_ICP_OS_LV: u16 = 5;
    /// brf_pfdcp_icp_set_lv — 4 bits (offset 11)
    pub const BRF_PFDCP_ICP_SET_LV: u16 = 11;
    /// brf_pfdcp_ldo_vref_lv — 4 bits (offset 15)
    pub const BRF_PFDCP_LDO_VREF_LV: u16 = 15;
    /// brf_pfdcp_en_lv — 1 bit (offset 19)
    pub const BRF_PFDCP_EN_LV: u16 = 19;
}

/// LPF_REG (offset 0x20)
pub mod lpf_reg {
    /// brf_lpf_rz_sel_lv — 3 bits (offset 0)
    pub const BRF_LPF_RZ_SEL_LV: u16 = 0;
    /// brf_lpf_rp4_sel_lv — 3 bits (offset 3)
    pub const BRF_LPF_RP4_SEL_LV: u16 = 3;
    /// brf_lpf_cz_sel_lv — 3 bits (offset 6)
    pub const BRF_LPF_CZ_SEL_LV: u16 = 6;
    /// brf_lpf_cp4_sel_lv — 2 bits (offset 9)
    pub const BRF_LPF_CP4_SEL_LV: u16 = 9;
    /// brf_lpf_cp3_sel_lv — 3 bits (offset 11)
    pub const BRF_LPF_CP3_SEL_LV: u16 = 11;
    /// brf_lo_open_lv — 1 bit (offset 14)
    pub const BRF_LO_OPEN_LV: u16 = 14;
}

/// EDR_CAL_REG1 (offset 0x24)
pub mod edr_cal_reg1 {
    /// brf_edr_vco_pdx_lv — 8 bits (offset 0)
    pub const BRF_EDR_VCO_PDX_LV: u16 = 0;
    /// brf_edr_vco_idac_lv — 7 bits (offset 8)
    pub const BRF_EDR_VCO_IDAC_LV: u16 = 8;
    /// brf_oslo_fc_lv — 3 bits (offset 16)
    pub const BRF_OSLO_FC_LV: u16 = 16;
    /// brf_oslo_bm_lv — 5 bits (offset 20)
    pub const BRF_OSLO_BM_LV: u16 = 20;
    /// brf_trf_edr_tmxcap_sel_lv — 4 bits (offset 28)
    pub const BRF_TRF_EDR_TMXCAP_SEL_LV: u16 = 28;
}

/// OSLO_REG (offset 0x28)
pub mod oslo_reg {
    /// brf_oslo_acal_cmp_lv — 1 bit (offset 0)
    pub const BRF_OSLO_ACAL_CMP_LV: u16 = 0;
    /// brf_oslo_pkdet_en_lv — 1 bit (offset 1)
    pub const BRF_OSLO_PKDET_EN_LV: u16 = 1;
    /// brf_oslo_ngm_en_lv — 1 bit (offset 2)
    pub const BRF_OSLO_NGM_EN_LV: u16 = 2;
    /// brf_oslo_fcal_en_lv — 1 bit (offset 3)
    pub const BRF_OSLO_FCAL_EN_LV: u16 = 3;
    /// brf_oslo_pkdet_vref_lv — 3 bits (offset 4)
    pub const BRF_OSLO_PKDET_VREF_LV: u16 = 4;
    /// brf_oslo_ldo_vref_lv — 4 bits (offset 7)
    pub const BRF_OSLO_LDO_VREF_LV: u16 = 7;
    /// brf_oslo_en_lv — 1 bit (offset 11)
    pub const BRF_OSLO_EN_LV: u16 = 11;
}

/// ATEST_REG (offset 0x2C)
pub mod atest_reg {
    /// brf_dc_tr_lv — 3 bits (offset 0)
    pub const BRF_DC_TR_LV: u16 = 0;
    /// brf_dc_br_lv — 3 bits (offset 3)
    pub const BRF_DC_BR_LV: u16 = 3;
    /// brf_dc_mr_lv — 3 bits (offset 6)
    pub const BRF_DC_MR_LV: u16 = 6;
}

/// DTEST_REG (offset 0x30)
pub mod dtest_reg {
    /// brf_fbdv_dtest_tr_lv — 4 bits (offset 0)
    pub const BRF_FBDV_DTEST_TR_LV: u16 = 0;
    /// brf_fbdv_dtest_en_lv — 1 bit (offset 4)
    pub const BRF_FBDV_DTEST_EN_LV: u16 = 4;
}

/// TRF_REG1 (offset 0x34)
pub mod trf_reg1 {
    /// brf_pa_cas_bp_lv — 1 bit (offset 0)
    pub const BRF_PA_CAS_BP_LV: u16 = 0;
    /// brf_pa_pm_lv — 2 bits (offset 1)
    pub const BRF_PA_PM_LV: u16 = 1;
    /// brf_pa_vc_lv — 6 bits (offset 3)
    pub const BRF_PA_VC_LV: u16 = 3;
    /// brf_pa_rstn_lv — 1 bit (offset 9)
    pub const BRF_PA_RSTN_LV: u16 = 9;
    /// brf_pa_setbc_lv — 4 bits (offset 10)
    pub const BRF_PA_SETBC_LV: u16 = 10;
    /// brf_pa_setsgn_lv — 1 bit (offset 14)
    pub const BRF_PA_SETSGN_LV: u16 = 14;
    /// brf_pa_bcsel_lv — 1 bit (offset 15)
    pub const BRF_PA_BCSEL_LV: u16 = 15;
    /// brf_trf_sig_en_lv — 1 bit (offset 16)
    pub const BRF_TRF_SIG_EN_LV: u16 = 16;
    /// brf_trf_ldo_vref_sel_lv — 4 bits (offset 17)
    pub const BRF_TRF_LDO_VREF_SEL_LV: u16 = 17;
    /// brf_pa_out_pu_lv — 1 bit (offset 21)
    pub const BRF_PA_OUT_PU_LV: u16 = 21;
    /// brf_pa_buf_pu_lv — 1 bit (offset 22)
    pub const BRF_PA_BUF_PU_LV: u16 = 22;
}

/// TRF_REG2 (offset 0x38)
pub mod trf_reg2 {
    /// brf_pa_atten_gain_lv — 4 bits (offset 0)
    pub const BRF_PA_ATTEN_GAIN_LV: u16 = 0;
    /// brf_pa_atten_en_lv — 1 bit (offset 4)
    pub const BRF_PA_ATTEN_EN_LV: u16 = 4;
    /// brf_pa_match2_lv — 2 bits (offset 5)
    pub const BRF_PA_MATCH2_LV: u16 = 5;
    /// brf_pa_match1_lv — 2 bits (offset 7)
    pub const BRF_PA_MATCH1_LV: u16 = 7;
    /// brf_pa_tx_rx_lv — 1 bit (offset 9)
    pub const BRF_PA_TX_RX_LV: u16 = 9;
    /// brf_pa_mcap_lv — 1 bit (offset 10)
    pub const BRF_PA_MCAP_LV: u16 = 10;
    /// brf_pa_unit_sel_lv — 5 bits (offset 11)
    pub const BRF_PA_UNIT_SEL_LV: u16 = 11;
    /// brf_pa_bufload_sel_lv — 2 bits (offset 16)
    pub const BRF_PA_BUFLOAD_SEL_LV: u16 = 16;
    /// brf_pa_bm_lv — 2 bits (offset 18)
    pub const BRF_PA_BM_LV: u16 = 18;
}

/// TRF_EDR_REG1 (offset 0x3C)
pub mod trf_edr_reg1 {
    /// brf_trf_edr_pacas_bm_lv — 2 bits (offset 0)
    pub const BRF_TRF_EDR_PACAS_BM_LV: u16 = 0;
    /// brf_trf_edr_pa_pu_lv — 1 bit (offset 2)
    pub const BRF_TRF_EDR_PA_PU_LV: u16 = 2;
    /// brf_trf_edr_tmxcap_bm_lv — 2 bits (offset 3)
    pub const BRF_TRF_EDR_TMXCAP_BM_LV: u16 = 3;
    /// brf_trf_edr_tmxcap_sel_lv — 4 bits (offset 5)
    pub const BRF_TRF_EDR_TMXCAP_SEL_LV: u16 = 5;
    /// brf_trf_edr_tmxcas_bm_lv — 2 bits (offset 9)
    pub const BRF_TRF_EDR_TMXCAS_BM_LV: u16 = 9;
    /// brf_trf_edr_tmxcas_sel_lv — 1 bit (offset 11)
    pub const BRF_TRF_EDR_TMXCAS_SEL_LV: u16 = 11;
    /// brf_trf_edr_tmx_pu_lv — 1 bit (offset 12)
    pub const BRF_TRF_EDR_TMX_PU_LV: u16 = 12;
    /// brf_trf_edr_lobias_bm_lv — 2 bits (offset 13)
    pub const BRF_TRF_EDR_LOBIAS_BM_LV: u16 = 13;
    /// brf_trf_edr_tmxbuf_ibld_lv — 4 bits (offset 15)
    pub const BRF_TRF_EDR_TMXBUF_IBLD_LV: u16 = 15;
    /// brf_trf_edr_tmxbuf_pu_lv — 1 bit (offset 19)
    pub const BRF_TRF_EDR_TMXBUF_PU_LV: u16 = 19;
    /// brf_trf_edr_iarray_en_lv — 1 bit (offset 20)
    pub const BRF_TRF_EDR_IARRAY_EN_LV: u16 = 20;
}

/// TRF_EDR_REG2 (offset 0x40)
pub mod trf_edr_reg2 {
    /// brf_trf_edr_pwrmtr_os_pn_lv — 1 bit (offset 0)
    pub const BRF_TRF_EDR_PWRMTR_OS_PN_LV: u16 = 0;
    /// brf_trf_edr_pwrmtr_os_lv — 4 bits (offset 1)
    pub const BRF_TRF_EDR_PWRMTR_OS_LV: u16 = 1;
    /// brf_trf_edr_pwrmtr_gc_lv — 2 bits (offset 5)
    pub const BRF_TRF_EDR_PWRMTR_GC_LV: u16 = 5;
    /// brf_trf_edr_pwrmtr_bm_lv — 3 bits (offset 7)
    pub const BRF_TRF_EDR_PWRMTR_BM_LV: u16 = 7;
    /// brf_trf_edr_pwrmtr_en_lv — 1 bit (offset 10)
    pub const BRF_TRF_EDR_PWRMTR_EN_LV: u16 = 10;
    /// brf_trf_edr_pa_xfmr_sg_lv — 1 bit (offset 11)
    pub const BRF_TRF_EDR_PA_XFMR_SG_LV: u16 = 11;
    /// brf_trf_edr_papmos_bm_lv — 3 bits (offset 12)
    pub const BRF_TRF_EDR_PAPMOS_BM_LV: u16 = 12;
    /// brf_trf_edr_pacap_bm_lv — 2 bits (offset 15)
    pub const BRF_TRF_EDR_PACAP_BM_LV: u16 = 15;
    /// brf_trf_edr_pacap_en_lv — 1 bit (offset 17)
    pub const BRF_TRF_EDR_PACAP_EN_LV: u16 = 17;
}

/// RRF_REG (offset 0x44)
pub mod rrf_reg {
    /// brf_mx_bm_lv — 3 bits (offset 0)
    pub const BRF_MX_BM_LV: u16 = 0;
    /// brf_mx_pu_lv — 1 bit (offset 3)
    pub const BRF_MX_PU_LV: u16 = 3;
    /// brf_lna_match_lv — 2 bits (offset 4)
    pub const BRF_LNA_MATCH_LV: u16 = 4;
    /// brf_lna_shuntsw_lv — 1 bit (offset 6)
    pub const BRF_LNA_SHUNTSW_LV: u16 = 6;
    /// brf_lna_fbrtrim_lv — 3 bits (offset 7)
    pub const BRF_LNA_FBRTRIM_LV: u16 = 7;
    /// brf_lna_gc_lv — 4 bits (offset 10)
    pub const BRF_LNA_GC_LV: u16 = 10;
    /// brf_lna_bm_lv — 3 bits (offset 14)
    pub const BRF_LNA_BM_LV: u16 = 14;
    /// brf_lna_pu_lv — 1 bit (offset 17)
    pub const BRF_LNA_PU_LV: u16 = 17;
    /// brf_rrf_ldo_vref_sel_lv — 4 bits (offset 18)
    pub const BRF_RRF_LDO_VREF_SEL_LV: u16 = 18;
    /// brf_rrf_ldo11_en_lv — 1 bit (offset 22)
    pub const BRF_RRF_LDO11_EN_LV: u16 = 22;
}

/// RBB_REG1 (offset 0x48)
pub mod rbb_reg1 {
    /// brf_cbpf_fc_lv_2m — 2 bits (offset 0)
    pub const BRF_CBPF_FC_LV_2M: u16 = 0;
    /// brf_cbpf_bm_lv_2m — 3 bits (offset 2)
    pub const BRF_CBPF_BM_LV_2M: u16 = 2;
    /// brf_cbpf_cc_lv_2m — 4 bits (offset 5)
    pub const BRF_CBPF_CC_LV_2M: u16 = 5;
    /// brf_sel_ldovref_rbb_lv — 4 bits (offset 9)
    pub const BRF_SEL_LDOVREF_RBB_LV: u16 = 9;
    /// brf_en_ldo_rbb_lv — 1 bit (offset 13)
    pub const BRF_EN_LDO_RBB_LV: u16 = 13;
    /// brf_pkdet_vth2q_bt — 4 bits (offset 14)
    pub const BRF_PKDET_VTH2Q_BT: u16 = 14;
    /// brf_pkdet_vth2i_bt — 4 bits (offset 18)
    pub const BRF_PKDET_VTH2I_BT: u16 = 18;
    /// brf_pkdet_vth1q_bt — 4 bits (offset 22)
    pub const BRF_PKDET_VTH1Q_BT: u16 = 22;
    /// brf_pkdet_vth1i_bt — 4 bits (offset 26)
    pub const BRF_PKDET_VTH1I_BT: u16 = 26;
}

/// RBB_REG2 (offset 0x4C)
pub mod rbb_reg2 {
    /// brf_rvga_man_cfsel_lv — 1 bit (offset 0)
    pub const BRF_RVGA_MAN_CFSEL_LV: u16 = 0;
    /// brf_rvga_gc_lv — 5 bits (offset 1)
    pub const BRF_RVGA_GC_LV: u16 = 1;
    /// brf_en_rvga_q_lv — 1 bit (offset 6)
    pub const BRF_EN_RVGA_Q_LV: u16 = 6;
    /// brf_en_rvga_i_lv — 1 bit (offset 7)
    pub const BRF_EN_RVGA_I_LV: u16 = 7;
    /// brf_cbpf_w2x_stg2_lv — 1 bit (offset 8)
    pub const BRF_CBPF_W2X_STG2_LV: u16 = 8;
    /// brf_cbpf_w2x_stg1_lv — 1 bit (offset 9)
    pub const BRF_CBPF_W2X_STG1_LV: u16 = 9;
    /// brf_cbpf_gc_lv — 2 bits (offset 10)
    pub const BRF_CBPF_GC_LV: u16 = 10;
    /// brf_cbpf_en_rc — 1 bit (offset 12)
    pub const BRF_CBPF_EN_RC: u16 = 12;
    /// brf_cbpf_fc_lv — 2 bits (offset 13)
    pub const BRF_CBPF_FC_LV: u16 = 13;
    /// brf_cbpf_bw_lv — 1 bit (offset 15)
    pub const BRF_CBPF_BW_LV: u16 = 15;
    /// brf_cbpf_vstart_lv — 2 bits (offset 16)
    pub const BRF_CBPF_VSTART_LV: u16 = 16;
    /// brf_cbpf_vcmref_lv — 2 bits (offset 18)
    pub const BRF_CBPF_VCMREF_LV: u16 = 18;
    /// brf_cbpf_bm_lv — 3 bits (offset 20)
    pub const BRF_CBPF_BM_LV: u16 = 20;
    /// brf_cbpf_cc_lv — 4 bits (offset 23)
    pub const BRF_CBPF_CC_LV: u16 = 23;
    /// brf_en_cbpf_lv — 1 bit (offset 27)
    pub const BRF_EN_CBPF_LV: u16 = 27;
}

/// RBB_REG3 (offset 0x50)
pub mod rbb_reg3 {
    /// brf_en_pkdet_lv — 4 bits (offset 0)
    pub const BRF_EN_PKDET_LV: u16 = 0;
    /// brf_rvga_w2x_stg2_lv — 1 bit (offset 4)
    pub const BRF_RVGA_W2X_STG2_LV: u16 = 4;
    /// brf_rvga_w2x_stg1_lv — 1 bit (offset 5)
    pub const BRF_RVGA_W2X_STG1_LV: u16 = 5;
    /// brf_rvga_vstart_lv — 2 bits (offset 6)
    pub const BRF_RVGA_VSTART_LV: u16 = 6;
    /// brf_rvga_vcmref_lv — 2 bits (offset 8)
    pub const BRF_RVGA_VCMREF_LV: u16 = 8;
    /// brf_rvga_bm_lv — 3 bits (offset 10)
    pub const BRF_RVGA_BM_LV: u16 = 10;
    /// brf_rvga_rz_lv — 3 bits (offset 13)
    pub const BRF_RVGA_RZ_LV: u16 = 13;
    /// brf_rvga_cc_lv — 4 bits (offset 16)
    pub const BRF_RVGA_CC_LV: u16 = 16;
    /// brf_rvga_cfman_lv — 3 bits (offset 20)
    pub const BRF_RVGA_CFMAN_LV: u16 = 20;
}

/// RBB_REG4 (offset 0x54)
pub mod rbb_reg4 {
    /// brf_pkdet_vth2q_lv — 4 bits (offset 0)
    pub const BRF_PKDET_VTH2Q_LV: u16 = 0;
    /// brf_pkdet_vth2i_lv — 4 bits (offset 4)
    pub const BRF_PKDET_VTH2I_LV: u16 = 4;
    /// brf_pkdet_vth1q_lv — 4 bits (offset 8)
    pub const BRF_PKDET_VTH1Q_LV: u16 = 8;
    /// brf_pkdet_vth1i_lv — 4 bits (offset 12)
    pub const BRF_PKDET_VTH1I_LV: u16 = 12;
    /// brf_dos_q_lv — 7 bits (offset 16)
    pub const BRF_DOS_Q_LV: u16 = 16;
    /// brf_dos_i_lv — 7 bits (offset 23)
    pub const BRF_DOS_I_LV: u16 = 23;
}

/// RBB_REG5 (offset 0x58)
pub mod rbb_reg5 {
    /// brf_rvga_tx_lpbk_en_lv — 1 bit (offset 0)
    pub const BRF_RVGA_TX_LPBK_EN_LV: u16 = 0;
    /// brf_cbpf_bt_en_lv — 1 bit (offset 1)
    pub const BRF_CBPF_BT_EN_LV: u16 = 1;
    /// brf_iary_bm_lv — 3 bits (offset 2)
    pub const BRF_IARY_BM_LV: u16 = 2;
    /// brf_en_iarray_lv — 1 bit (offset 5)
    pub const BRF_EN_IARRAY_LV: u16 = 5;
    /// brf_en_osdacq_lv — 1 bit (offset 6)
    pub const BRF_EN_OSDACQ_LV: u16 = 6;
    /// brf_en_osdaci_lv — 1 bit (offset 7)
    pub const BRF_EN_OSDACI_LV: u16 = 7;
    /// brf_rstb_rccal_lv — 1 bit (offset 8)
    pub const BRF_RSTB_RCCAL_LV: u16 = 8;
    /// brf_cbpf_capman_lv — 5 bits (offset 9)
    pub const BRF_CBPF_CAPMAN_LV: u16 = 9;
    /// brf_rccal_mancap_lv — 1 bit (offset 14)
    pub const BRF_RCCAL_MANCAP_LV: u16 = 14;
    /// brf_rccal_selxo_lv — 1 bit (offset 15)
    pub const BRF_RCCAL_SELXO_LV: u16 = 15;
    /// brf_en_rccal_lv — 1 bit (offset 16)
    pub const BRF_EN_RCCAL_LV: u16 = 16;
    /// brf_pkdet_bm_lv — 3 bits (offset 17)
    pub const BRF_PKDET_BM_LV: u16 = 17;
    /// brf_cbpf_bt_en_frc_en — 1 bit (offset 20)
    pub const BRF_CBPF_BT_EN_FRC_EN: u16 = 20;
}

/// RBB_REG6 (offset 0x5C)
pub mod rbb_reg6 {
    /// brf_cbpf_fc_lv_br — 2 bits (offset 0)
    pub const BRF_CBPF_FC_LV_BR: u16 = 0;
    /// brf_cbpf_bm_lv_br — 3 bits (offset 2)
    pub const BRF_CBPF_BM_LV_BR: u16 = 2;
    /// brf_cbpf_cc_lv_br — 4 bits (offset 5)
    pub const BRF_CBPF_CC_LV_BR: u16 = 5;
    /// brf_cbpf_fc_lv_edr — 2 bits (offset 9)
    pub const BRF_CBPF_FC_LV_EDR: u16 = 9;
    /// brf_cbpf_bm_lv_edr — 3 bits (offset 11)
    pub const BRF_CBPF_BM_LV_EDR: u16 = 11;
    /// brf_cbpf_cc_lv_edr — 4 bits (offset 14)
    pub const BRF_CBPF_CC_LV_EDR: u16 = 14;
    /// brf_rvga_w2x_stg2_lv_br — 1 bit (offset 18)
    pub const BRF_RVGA_W2X_STG2_LV_BR: u16 = 18;
    /// brf_rvga_w2x_stg1_lv_br — 1 bit (offset 19)
    pub const BRF_RVGA_W2X_STG1_LV_BR: u16 = 19;
    /// brf_rvga_w2x_stg2_lv_edr — 1 bit (offset 20)
    pub const BRF_RVGA_W2X_STG2_LV_EDR: u16 = 20;
    /// brf_rvga_w2x_stg1_lv_edr — 1 bit (offset 21)
    pub const BRF_RVGA_W2X_STG1_LV_EDR: u16 = 21;
    /// brf_cbpf_w2x_stg2_lv_br — 1 bit (offset 22)
    pub const BRF_CBPF_W2X_STG2_LV_BR: u16 = 22;
    /// brf_cbpf_w2x_stg1_lv_br — 1 bit (offset 23)
    pub const BRF_CBPF_W2X_STG1_LV_BR: u16 = 23;
    /// brf_cbpf_w2x_stg2_lv_edr — 1 bit (offset 24)
    pub const BRF_CBPF_W2X_STG2_LV_EDR: u16 = 24;
    /// brf_cbpf_w2x_stg1_lv_edr — 1 bit (offset 25)
    pub const BRF_CBPF_W2X_STG1_LV_EDR: u16 = 25;
    /// brf_cbpf_bw_lv_br — 1 bit (offset 26)
    pub const BRF_CBPF_BW_LV_BR: u16 = 26;
    /// brf_cbpf_bw_lv_edr — 1 bit (offset 27)
    pub const BRF_CBPF_BW_LV_EDR: u16 = 27;
}

/// ADC_REG (offset 0x60)
pub mod adc_reg {
    /// brf_sel_ldovref_adcref_lv — 4 bits (offset 0)
    pub const BRF_SEL_LDOVREF_ADCREF_LV: u16 = 0;
    /// brf_en_ldo_adcref_lv — 1 bit (offset 4)
    pub const BRF_EN_LDO_ADCREF_LV: u16 = 4;
    /// brf_sel_ldovref_adc_lv — 4 bits (offset 5)
    pub const BRF_SEL_LDOVREF_ADC_LV: u16 = 5;
    /// brf_en_ldo_adc_lv — 1 bit (offset 9)
    pub const BRF_EN_LDO_ADC_LV: u16 = 9;
    /// brf_rstb_adc_lv — 1 bit (offset 10)
    pub const BRF_RSTB_ADC_LV: u16 = 10;
    /// brf_adc_vsp_lv — 2 bits (offset 11)
    pub const BRF_ADC_VSP_LV: u16 = 11;
    /// brf_adc_cmpcl_lv — 3 bits (offset 13)
    pub const BRF_ADC_CMPCL_LV: u16 = 13;
    /// brf_adc_cmm_lv — 4 bits (offset 16)
    pub const BRF_ADC_CMM_LV: u16 = 16;
    /// brf_en_adc_q_lv — 1 bit (offset 20)
    pub const BRF_EN_ADC_Q_LV: u16 = 20;
    /// brf_en_adc_i_lv — 1 bit (offset 21)
    pub const BRF_EN_ADC_I_LV: u16 = 21;
}

/// TBB_REG (offset 0x64)
pub mod tbb_reg {
    /// brf_sel_ldovref_dac_dvdd_lv — 4 bits (offset 0)
    pub const BRF_SEL_LDOVREF_DAC_DVDD_LV: u16 = 0;
    /// brf_sel_ldovref_dac_avdd_lv — 4 bits (offset 4)
    pub const BRF_SEL_LDOVREF_DAC_AVDD_LV: u16 = 4;
    /// brf_en_tbb_iarray_lv — 1 bit (offset 8)
    pub const BRF_EN_TBB_IARRAY_LV: u16 = 8;
    /// brf_en_ldo_dac_dvdd_lv — 1 bit (offset 9)
    pub const BRF_EN_LDO_DAC_DVDD_LV: u16 = 9;
    /// brf_en_ldo_dac_avdd_lv — 1 bit (offset 10)
    pub const BRF_EN_LDO_DAC_AVDD_LV: u16 = 10;
    /// brf_en_dac_lv — 1 bit (offset 11)
    pub const BRF_EN_DAC_LV: u16 = 11;
    /// brf_dac_start_lv — 1 bit (offset 12)
    pub const BRF_DAC_START_LV: u16 = 12;
    /// brf_dac_sel_clk_bar_lv — 1 bit (offset 13)
    pub const BRF_DAC_SEL_CLK_BAR_LV: u16 = 13;
    /// brf_dac_lsb_cnt_lv — 2 bits (offset 14)
    pub const BRF_DAC_LSB_CNT_LV: u16 = 14;
}

/// ATSTBUF_REG (offset 0x68)
pub mod atstbuf_reg {
    /// brf_atstbuf_w2x_stg2_lv — 1 bit (offset 0)
    pub const BRF_ATSTBUF_W2X_STG2_LV: u16 = 0;
    /// brf_atstbuf_w2x_stg1_lv — 1 bit (offset 1)
    pub const BRF_ATSTBUF_W2X_STG1_LV: u16 = 1;
    /// brf_atstbuf_vstart_lv — 2 bits (offset 2)
    pub const BRF_ATSTBUF_VSTART_LV: u16 = 2;
    /// brf_atstbuf_vcmref_lv — 2 bits (offset 4)
    pub const BRF_ATSTBUF_VCMREF_LV: u16 = 4;
    /// brf_atstbuf_bm_lv — 3 bits (offset 6)
    pub const BRF_ATSTBUF_BM_LV: u16 = 6;
    /// brf_atstbuf_rz_lv — 3 bits (offset 9)
    pub const BRF_ATSTBUF_RZ_LV: u16 = 9;
    /// brf_atstbuf_cc_lv — 4 bits (offset 12)
    pub const BRF_ATSTBUF_CC_LV: u16 = 12;
    /// brf_atstbuf_cfman_lv — 3 bits (offset 16)
    pub const BRF_ATSTBUF_CFMAN_LV: u16 = 16;
    /// brf_atstbuf_man_cfsel_lv — 1 bit (offset 19)
    pub const BRF_ATSTBUF_MAN_CFSEL_LV: u16 = 19;
    /// brf_atstbuf_gc_lv — 5 bits (offset 20)
    pub const BRF_ATSTBUF_GC_LV: u16 = 20;
    /// brf_atstbuf_ch_sel_lv — 1 bit (offset 25)
    pub const BRF_ATSTBUF_CH_SEL_LV: u16 = 25;
    /// brf_en_atstbuf_lv — 1 bit (offset 26)
    pub const BRF_EN_ATSTBUF_LV: u16 = 26;
}

/// INCCAL_REG1 (offset 0x74)
pub mod inccal_reg1 {
    /// vco3g_auto_incacal_en — 1 bit (offset 0)
    pub const VCO3G_AUTO_INCACAL_EN: u16 = 0;
    /// vco3g_auto_incfcal_en — 1 bit (offset 1)
    pub const VCO3G_AUTO_INCFCAL_EN: u16 = 1;
    /// vco3g_incacal_wait_time — 6 bits (offset 2)
    pub const VCO3G_INCACAL_WAIT_TIME: u16 = 2;
    /// vco3g_incfcal_wait_time — 6 bits (offset 8)
    pub const VCO3G_INCFCAL_WAIT_TIME: u16 = 8;
    /// vco3g_idac_offset — 7 bits (offset 14)
    pub const VCO3G_IDAC_OFFSET: u16 = 14;
    /// vco3g_pdx_offset — 8 bits (offset 21)
    pub const VCO3G_PDX_OFFSET: u16 = 21;
    /// inccal_start — 1 bit (offset 29)
    pub const INCCAL_START: u16 = 29;
    /// frc_inccal_clk_on — 1 bit (offset 30)
    pub const FRC_INCCAL_CLK_ON: u16 = 30;
}

/// INCCAL_REG2 (offset 0x78)
pub mod inccal_reg2 {
    /// vco5g_auto_incacal_en — 1 bit (offset 0)
    pub const VCO5G_AUTO_INCACAL_EN: u16 = 0;
    /// vco5g_auto_incfcal_en — 1 bit (offset 1)
    pub const VCO5G_AUTO_INCFCAL_EN: u16 = 1;
    /// vco5g_incacal_wait_time — 6 bits (offset 2)
    pub const VCO5G_INCACAL_WAIT_TIME: u16 = 2;
    /// vco5g_incfcal_wait_time — 6 bits (offset 8)
    pub const VCO5G_INCFCAL_WAIT_TIME: u16 = 8;
    /// vco5g_idac_offset — 7 bits (offset 14)
    pub const VCO5G_IDAC_OFFSET: u16 = 14;
    /// vco5g_pdx_offset — 8 bits (offset 21)
    pub const VCO5G_PDX_OFFSET: u16 = 21;
}

/// ROSCAL_REG1 (offset 0x7C)
pub mod roscal_reg1 {
    /// roscal_start — 1 bit (offset 0)
    pub const ROSCAL_START: u16 = 0;
    /// roscal_bypass — 1 bit (offset 1)
    pub const ROSCAL_BYPASS: u16 = 1;
    /// en_rosdac_i — 1 bit (offset 2)
    pub const EN_ROSDAC_I: u16 = 2;
    /// en_rosdac_q — 1 bit (offset 3)
    pub const EN_ROSDAC_Q: u16 = 3;
    /// roscal_ta — 9 bits (offset 4)
    pub const ROSCAL_TA: u16 = 4;
    /// roscal_tb — 4 bits (offset 13)
    pub const ROSCAL_TB: u16 = 13;
    /// roscal_tc — 7 bits (offset 17)
    pub const ROSCAL_TC: u16 = 17;
}

/// ROSCAL_REG2 (offset 0x80)
pub mod roscal_reg2 {
    /// roscal_done — 1 bit (offset 0)
    pub const ROSCAL_DONE: u16 = 0;
    /// dos_i_sw — 7 bits (offset 1)
    pub const DOS_I_SW: u16 = 1;
    /// dos_q_sw — 7 bits (offset 8)
    pub const DOS_Q_SW: u16 = 8;
}

/// RCROSCAL_REG (offset 0x84)
pub mod rcroscal_reg {
    /// ros_adc_q — 10 bits (offset 0)
    pub const ROS_ADC_Q: u16 = 0;
    /// ros_adc_i — 10 bits (offset 10)
    pub const ROS_ADC_I: u16 = 10;
    /// rccal_done — 1 bit (offset 20)
    pub const RCCAL_DONE: u16 = 20;
    /// rccal_start — 1 bit (offset 21)
    pub const RCCAL_START: u16 = 21;
    /// rc_capcode_offset — 4 bits (offset 22)
    pub const RC_CAPCODE_OFFSET: u16 = 22;
    /// rc_capcode — 5 bits (offset 26)
    pub const RC_CAPCODE: u16 = 26;
}

/// PACAL_REG (offset 0x88)
pub mod pacal_reg {
    /// pacal_start — 1 bit (offset 0)
    pub const PACAL_START: u16 = 0;
    /// pacal_done — 1 bit (offset 1)
    pub const PACAL_DONE: u16 = 1;
    /// sgn_cal_rslt — 1 bit (offset 2)
    pub const SGN_CAL_RSLT: u16 = 2;
    /// bc_cal_rslt — 4 bits (offset 3)
    pub const BC_CAL_RSLT: u16 = 3;
    /// pacal_rdy — 1 bit (offset 7)
    pub const PACAL_RDY: u16 = 7;
    /// pa_rstb_frc_en — 1 bit (offset 8)
    pub const PA_RSTB_FRC_EN: u16 = 8;
    /// pacal_clk_en — 1 bit (offset 9)
    pub const PACAL_CLK_EN: u16 = 9;
}

/// CU_ADDR_REG1 (offset 0x8C)
pub mod cu_addr_reg1 {
    /// rxon_cfg_addr — 12 bits (offset 0)
    pub const RXON_CFG_ADDR: u16 = 0;
    /// rxoff_cfg_addr — 12 bits (offset 16)
    pub const RXOFF_CFG_ADDR: u16 = 16;
}

/// CU_ADDR_REG2 (offset 0x90)
pub mod cu_addr_reg2 {
    /// txon_cfg_addr — 12 bits (offset 0)
    pub const TXON_CFG_ADDR: u16 = 0;
    /// txoff_cfg_addr — 12 bits (offset 16)
    pub const TXOFF_CFG_ADDR: u16 = 16;
}

/// CU_ADDR_REG3 (offset 0x94)
pub mod cu_addr_reg3 {
    /// bt_txon_cfg_addr — 12 bits (offset 0)
    pub const BT_TXON_CFG_ADDR: u16 = 0;
    /// bt_txoff_cfg_addr — 12 bits (offset 16)
    pub const BT_TXOFF_CFG_ADDR: u16 = 16;
}

/// CAL_ADDR_REG1 (offset 0x98)
pub mod cal_addr_reg1 {
    /// ble_rx_cal_addr — 12 bits (offset 0)
    pub const BLE_RX_CAL_ADDR: u16 = 0;
    /// bt_rx_cal_addr — 12 bits (offset 16)
    pub const BT_RX_CAL_ADDR: u16 = 16;
}

/// CAL_ADDR_REG2 (offset 0x9C)
pub mod cal_addr_reg2 {
    /// ble_tx_cal_addr — 12 bits (offset 0)
    pub const BLE_TX_CAL_ADDR: u16 = 0;
    /// bt_tx_cal_addr — 12 bits (offset 16)
    pub const BT_TX_CAL_ADDR: u16 = 16;
}

/// CAL_ADDR_REG3 (offset 0xA0)
pub mod cal_addr_reg3 {
    /// txdc_cal_addr — 12 bits (offset 0)
    pub const TXDC_CAL_ADDR: u16 = 0;
}

/// AGC_REG (offset 0xA4)
pub mod agc_reg {
    /// lna_gain_frc_en — 1 bit (offset 0)
    pub const LNA_GAIN_FRC_EN: u16 = 0;
    /// cbpf_gain_frc_en — 1 bit (offset 1)
    pub const CBPF_GAIN_FRC_EN: u16 = 1;
    /// vga_gain_frc_en — 1 bit (offset 2)
    pub const VGA_GAIN_FRC_EN: u16 = 2;
    /// lna_gc — 4 bits (offset 3)
    pub const LNA_GC: u16 = 3;
    /// cbpf_gc — 2 bits (offset 7)
    pub const CBPF_GC: u16 = 7;
    /// vga_gc — 5 bits (offset 9)
    pub const VGA_GC: u16 = 9;
}

/// IQ_PWR_REG1 (offset 0xA8)
pub mod iq_pwr_reg1 {
    /// tx_dc_cal_coef0 — 14 bits (offset 0)
    pub const TX_DC_CAL_COEF0: u16 = 0;
    /// tx_dc_cal_coef1 — 14 bits (offset 14)
    pub const TX_DC_CAL_COEF1: u16 = 14;
    /// edr_tmxbuf_gc_gfsk — 4 bits (offset 28)
    pub const EDR_TMXBUF_GC_GFSK: u16 = 28;
}

/// IQ_PWR_REG2 (offset 0xAC)
pub mod iq_pwr_reg2 {
    /// tx_dc_cal_offset_q — 11 bits (offset 0)
    pub const TX_DC_CAL_OFFSET_Q: u16 = 0;
    /// brf_trf_edr_pa_bm_lv — 5 bits (offset 11)
    pub const BRF_TRF_EDR_PA_BM_LV: u16 = 11;
    /// tx_dc_cal_offset_i — 11 bits (offset 16)
    pub const TX_DC_CAL_OFFSET_I: u16 = 16;
    /// edr_lpf_bypass — 1 bit (offset 27)
    pub const EDR_LPF_BYPASS: u16 = 27;
    /// edr_tmxbuf_gc_dpsk — 4 bits (offset 28)
    pub const EDR_TMXBUF_GC_DPSK: u16 = 28;
}
