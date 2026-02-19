// Auto-generated from RivieraWaves YODA CSV. Do not edit.

pub mod dm {
    #[allow(unused_imports)]
    use crate::common::{Reg, R, RW, W};

    /// DM core block.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Dm {
        ptr: *mut u8,
    }

    unsafe impl Send for Dm {}
    unsafe impl Sync for Dm {}

    impl Dm {
        /// # Safety
        /// `ptr` must point to a valid DM core register block.
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self {
                ptr: ptr as *mut u8,
            }
        }

        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as *mut ()
        }

        /// VERSION (0x04)
        #[inline(always)]
        pub const fn version(self) -> Reg<regs::Version, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }

        /// INTCNTL0 (0x0C)
        #[inline(always)]
        pub const fn intcntl0(self) -> Reg<regs::Intcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x0Cusize) as _) }
        }

        /// INTSTAT0 (0x10)
        #[inline(always)]
        pub const fn intstat0(self) -> Reg<regs::Intstat0, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }

        /// INTACK0 (0x14)
        #[inline(always)]
        pub const fn intack0(self) -> Reg<regs::Intack0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }

        /// INTCNTL1 (0x18)
        #[inline(always)]
        pub const fn intcntl1(self) -> Reg<regs::Intcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }

        /// INTSTAT1 (0x1C)
        #[inline(always)]
        pub const fn intstat1(self) -> Reg<regs::Intstat1, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1Cusize) as _) }
        }

        /// INTACK1 (0x20)
        #[inline(always)]
        pub const fn intack1(self) -> Reg<regs::Intack1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }

        /// ACTFIFOSTAT (0x24)
        #[inline(always)]
        pub const fn actfifostat(self) -> Reg<regs::Actfifostat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }

        /// ETPTR (0x2C)
        #[inline(always)]
        pub const fn etptr(self) -> Reg<regs::Etptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x2Cusize) as _) }
        }

        /// DEEPSLCNTL (0x30)
        #[inline(always)]
        pub const fn deepslcntl(self) -> Reg<regs::Deepslcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }

        /// DEEPSLWKUP (0x34)
        #[inline(always)]
        pub const fn deepslwkup(self) -> Reg<regs::Deepslwkup, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }

        /// DEEPSLSTAT (0x38)
        #[inline(always)]
        pub const fn deepslstat(self) -> Reg<regs::Deepslstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }

        /// ENBPRESET (0x3C)
        #[inline(always)]
        pub const fn enbpreset(self) -> Reg<regs::Enbpreset, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x3Cusize) as _) }
        }

        /// FINECNTCORR (0x40)
        #[inline(always)]
        pub const fn finecntcorr(self) -> Reg<regs::Finecntcorr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }

        /// CLKNCNTCORR (0x44)
        #[inline(always)]
        pub const fn clkncntcorr(self) -> Reg<regs::Clkncntcorr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x44usize) as _) }
        }

        /// DIAGCNTL (0x50)
        #[inline(always)]
        pub const fn diagcntl(self) -> Reg<regs::Diagcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x50usize) as _) }
        }

        /// DIAGSTAT (0x54)
        #[inline(always)]
        pub const fn diagstat(self) -> Reg<regs::Diagstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x54usize) as _) }
        }

        /// DEBUGADDMAX (0x58)
        #[inline(always)]
        pub const fn debugaddmax(self) -> Reg<regs::Debugaddmax, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x58usize) as _) }
        }

        /// DEBUGADDMIN (0x5C)
        #[inline(always)]
        pub const fn debugaddmin(self) -> Reg<regs::Debugaddmin, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x5Cusize) as _) }
        }

        /// ERRORTYPESTAT (0x60)
        #[inline(always)]
        pub const fn errortypestat(self) -> Reg<regs::Errortypestat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x60usize) as _) }
        }

        /// SWPROFILING (0x64)
        #[inline(always)]
        pub const fn swprofiling(self) -> Reg<regs::Swprofiling, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x64usize) as _) }
        }

        /// RADIOCNTL0 (0x70)
        #[inline(always)]
        pub const fn radiocntl0(self) -> Reg<regs::Radiocntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }

        /// RADIOCNTL1 (0x74)
        #[inline(always)]
        pub const fn radiocntl1(self) -> Reg<regs::Radiocntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }

        /// AESCNTL (0xB0)
        #[inline(always)]
        pub const fn aescntl(self) -> Reg<regs::Aescntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB0usize) as _) }
        }

        /// AESKEY31_0 (0xB4)
        #[inline(always)]
        pub const fn aeskey31_0(self) -> Reg<regs::Aeskey310, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB4usize) as _) }
        }

        /// AESKEY63_32 (0xB8)
        #[inline(always)]
        pub const fn aeskey63_32(self) -> Reg<regs::Aeskey6332, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB8usize) as _) }
        }

        /// AESKEY95_64 (0xBC)
        #[inline(always)]
        pub const fn aeskey95_64(self) -> Reg<regs::Aeskey9564, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xBCusize) as _) }
        }

        /// AESKEY127_96 (0xC0)
        #[inline(always)]
        pub const fn aeskey127_96(self) -> Reg<regs::Aeskey12796, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC0usize) as _) }
        }

        /// AESPTR (0xC4)
        #[inline(always)]
        pub const fn aesptr(self) -> Reg<regs::Aesptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC4usize) as _) }
        }

        /// TXMICVAL (0xC8)
        #[inline(always)]
        pub const fn txmicval(self) -> Reg<regs::Txmicval, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC8usize) as _) }
        }

        /// RXMICVAL (0xCC)
        #[inline(always)]
        pub const fn rxmicval(self) -> Reg<regs::Rxmicval, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xCCusize) as _) }
        }

        /// TIMGENCNTL (0xE0)
        #[inline(always)]
        pub const fn timgencntl(self) -> Reg<regs::Timgencntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE0usize) as _) }
        }

        /// FINETIMTGT (0xE4)
        #[inline(always)]
        pub const fn finetimtgt(self) -> Reg<regs::Finetimtgt, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE4usize) as _) }
        }

        /// CLKNTGT1 (0xE8)
        #[inline(always)]
        pub const fn clkntgt1(self) -> Reg<regs::Clkntgt1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE8usize) as _) }
        }

        /// HMICROSECTGT1 (0xEC)
        #[inline(always)]
        pub const fn hmicrosectgt1(self) -> Reg<regs::Hmicrosectgt1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xECusize) as _) }
        }

        /// CLKNTGT2 (0xF0)
        #[inline(always)]
        pub const fn clkntgt2(self) -> Reg<regs::Clkntgt2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF0usize) as _) }
        }

        /// HMICROSECTGT2 (0xF4)
        #[inline(always)]
        pub const fn hmicrosectgt2(self) -> Reg<regs::Hmicrosectgt2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF4usize) as _) }
        }

        /// CLKNTGT3 (0xF8)
        #[inline(always)]
        pub const fn clkntgt3(self) -> Reg<regs::Clkntgt3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF8usize) as _) }
        }

        /// HMICROSECTGT3 (0xFC)
        #[inline(always)]
        pub const fn hmicrosectgt3(self) -> Reg<regs::Hmicrosectgt3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xFCusize) as _) }
        }

        /// SLOTCLK (0x100)
        #[inline(always)]
        pub const fn slotclk(self) -> Reg<regs::Slotclk, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x100usize) as _) }
        }

        /// FINETIMECNT (0x104)
        #[inline(always)]
        pub const fn finetimecnt(self) -> Reg<regs::Finetimecnt, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x104usize) as _) }
        }

        /// ACTSCHCNTL (0x110)
        #[inline(always)]
        pub const fn actschcntl(self) -> Reg<regs::Actschcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x110usize) as _) }
        }

        /// DFANCNTL (0x194)
        #[inline(always)]
        pub const fn dfancntl(self) -> Reg<regs::Dfancntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x194usize) as _) }
        }
    }

    pub mod regs {

        /// VERSION
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Version(pub u32);

        impl Version {
            /// BUILD — 8 bits (offset 0)
            #[inline(always)]
            pub const fn build(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_build(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// UPG — 8 bits (offset 8)
            #[inline(always)]
            pub const fn upg(&self) -> u8 {
                ((self.0 >> 8) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_upg(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 8)) | ((val as u32 & 0xFF) << 8);
            }

            /// REL — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rel(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rel(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// TYP — 8 bits (offset 24)
            #[inline(always)]
            pub const fn typ(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_typ(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// INTCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intcntl0(pub u32);

        impl Intcntl0 {
            /// ERRORINTMSK — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintmsk(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTSTAT0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intstat0(pub u32);

        impl Intstat0 {
            /// ERRORINTSTAT — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintstat(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTACK0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intack0(pub u32);

        impl Intack0 {
            /// ERRORINTACK — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintack(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intcntl1(pub u32);

        impl Intcntl1 {
            /// CLKNINTMSK — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintmsk(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTMSK — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintmsk(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTMSK — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintmsk(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTMSK — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintmsk(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTMSK — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintmsk(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTMSK — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intmsk(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTMSK — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intmsk(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTMSK — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intmsk(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTMSK — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointmsk(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// CLKNINTSRVAL — 4 bits (offset 24)
            #[inline(always)]
            pub const fn clknintsrval(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_clknintsrval(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// CLKNINTSRMSK — 3 bits (offset 28)
            #[inline(always)]
            pub const fn clknintsrmsk(&self) -> u8 {
                ((self.0 >> 28) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_clknintsrmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 28)) | ((val as u32 & 0x7) << 28);
            }
        }

        /// INTSTAT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intstat1(pub u32);

        impl Intstat1 {
            /// CLKNINTSTAT — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintstat(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintstat(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTSTAT — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintstat(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTSTAT — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintstat(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTSTAT — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintstat(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTSTAT — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintstat(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTSTAT — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intstat(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTSTAT — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intstat(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTSTAT — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intstat(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTSTAT — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointstat(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }
        }

        /// INTACK1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intack1(pub u32);

        impl Intack1 {
            /// CLKNINTACK — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintack(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintack(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTACK — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintack(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTACK — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintack(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTACK — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintack(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTACK — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintack(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTACK — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intack(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTACK — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intack(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTACK — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intack(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTACK — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointack(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }
        }

        /// ACTFIFOSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Actfifostat(pub u32);

        impl Actfifostat {
            /// STARTACTINTSTAT — 1 bit (offset 0)
            #[inline(always)]
            pub const fn startactintstat(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_startactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// ENDACTINTSTAT — 1 bit (offset 1)
            #[inline(always)]
            pub const fn endactintstat(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_endactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SKIPACTINTSTAT — 1 bit (offset 2)
            #[inline(always)]
            pub const fn skipactintstat(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_skipactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// TXINTSTAT — 1 bit (offset 3)
            #[inline(always)]
            pub const fn txintstat(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// RXINTSTAT — 1 bit (offset 4)
            #[inline(always)]
            pub const fn rxintstat(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// ISOTXINTSTAT — 1 bit (offset 5)
            #[inline(always)]
            pub const fn isotxintstat(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isotxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// ISORXINTSTAT — 1 bit (offset 6)
            #[inline(always)]
            pub const fn isorxintstat(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isorxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// ACTFLAG — 1 bit (offset 15)
            #[inline(always)]
            pub const fn actflag(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_actflag(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// CURRENT_ET_IDX — 4 bits (offset 24)
            #[inline(always)]
            pub const fn current_et_idx(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_current_et_idx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// SKIP_ET_IDX — 4 bits (offset 28)
            #[inline(always)]
            pub const fn skip_et_idx(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_skip_et_idx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// ETPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Etptr(pub u32);

        impl Etptr {
            /// ETPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn etptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_etptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// DEEPSLCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslcntl(pub u32);

        impl Deepslcntl {
            /// OSC_SLEEP_EN — 1 bit (offset 0)
            #[inline(always)]
            pub const fn osc_sleep_en(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_osc_sleep_en(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// RADIO_SLEEP_EN — 1 bit (offset 1)
            #[inline(always)]
            pub const fn radio_sleep_en(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_radio_sleep_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// DEEP_SLEEP_ON — 1 bit (offset 2)
            #[inline(always)]
            pub const fn deep_sleep_on(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_on(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// DEEP_SLEEP_CORR_EN — 1 bit (offset 3)
            #[inline(always)]
            pub const fn deep_sleep_corr_en(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_corr_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// DEEP_SLEEP_STAT — 1 bit (offset 15)
            #[inline(always)]
            pub const fn deep_sleep_stat(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_stat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// EXTWKUPDSB — 1 bit (offset 31)
            #[inline(always)]
            pub const fn extwkupdsb(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_extwkupdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DEEPSLWKUP
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslwkup(pub u32);

        impl Deepslwkup {
            /// DEEPSLTIME — 32 bits (offset 0)
            #[inline(always)]
            pub const fn deepsltime(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_deepsltime(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// DEEPSLSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslstat(pub u32);

        impl Deepslstat {
            /// DEEPSLDUR — 32 bits (offset 0)
            #[inline(always)]
            pub const fn deepsldur(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_deepsldur(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// ENBPRESET
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Enbpreset(pub u32);

        impl Enbpreset {
            /// TWRM — 10 bits (offset 0)
            #[inline(always)]
            pub const fn twrm(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_twrm(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }

            /// TWOSC — 11 bits (offset 10)
            #[inline(always)]
            pub const fn twosc(&self) -> u16 {
                ((self.0 >> 10) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_twosc(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 10)) | ((val as u32 & 0x7FF) << 10);
            }

            /// TWEXT — 11 bits (offset 21)
            #[inline(always)]
            pub const fn twext(&self) -> u16 {
                ((self.0 >> 21) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_twext(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 21)) | ((val as u32 & 0x7FF) << 21);
            }
        }

        /// FINECNTCORR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finecntcorr(pub u32);

        impl Finecntcorr {
            /// FINECNTCORR — 10 bits (offset 0)
            #[inline(always)]
            pub const fn finecntcorr(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_finecntcorr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNCNTCORR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkncntcorr(pub u32);

        impl Clkncntcorr {
            /// CLKNCNTCORR — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkncntcorr(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkncntcorr(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }

            /// ABS_DELTA — 1 bit (offset 31)
            #[inline(always)]
            pub const fn abs_delta(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abs_delta(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DIAGCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Diagcntl(pub u32);

        impl Diagcntl {
            /// DIAG0 — 6 bits (offset 0)
            #[inline(always)]
            pub const fn diag0(&self) -> u8 {
                (self.0 & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_diag0(&mut self, val: u8) {
                self.0 = (self.0 & !0x3F) | (val as u32 & 0x3F);
            }

            /// DIAG0_EN — 1 bit (offset 7)
            #[inline(always)]
            pub const fn diag0_en(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag0_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// DIAG1 — 6 bits (offset 8)
            #[inline(always)]
            pub const fn diag1(&self) -> u8 {
                ((self.0 >> 8) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_diag1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 8)) | ((val as u32 & 0x3F) << 8);
            }

            /// DIAG1_EN — 1 bit (offset 15)
            #[inline(always)]
            pub const fn diag1_en(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag1_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// DIAG2 — 6 bits (offset 16)
            #[inline(always)]
            pub const fn diag2(&self) -> u8 {
                ((self.0 >> 16) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_diag2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 16)) | ((val as u32 & 0x3F) << 16);
            }

            /// DIAG2_EN — 1 bit (offset 23)
            #[inline(always)]
            pub const fn diag2_en(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag2_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }

            /// DIAG3 — 6 bits (offset 24)
            #[inline(always)]
            pub const fn diag3(&self) -> u8 {
                ((self.0 >> 24) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_diag3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 24)) | ((val as u32 & 0x3F) << 24);
            }

            /// DIAG3_EN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn diag3_en(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag3_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DIAGSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Diagstat(pub u32);

        impl Diagstat {
            /// DIAG0STAT — 8 bits (offset 0)
            #[inline(always)]
            pub const fn diag0stat(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag0stat(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// DIAG1STAT — 8 bits (offset 8)
            #[inline(always)]
            pub const fn diag1stat(&self) -> u8 {
                ((self.0 >> 8) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag1stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 8)) | ((val as u32 & 0xFF) << 8);
            }

            /// DIAG2STAT — 8 bits (offset 16)
            #[inline(always)]
            pub const fn diag2stat(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag2stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// DIAG3STAT — 8 bits (offset 24)
            #[inline(always)]
            pub const fn diag3stat(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag3stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// DEBUGADDMAX
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Debugaddmax(pub u32);

        impl Debugaddmax {
            /// EM_ADDMAX — 16 bits (offset 0)
            #[inline(always)]
            pub const fn em_addmax(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_em_addmax(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// REG_ADDMAX — 16 bits (offset 16)
            #[inline(always)]
            pub const fn reg_addmax(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_reg_addmax(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// DEBUGADDMIN
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Debugaddmin(pub u32);

        impl Debugaddmin {
            /// EM_ADDMIN — 16 bits (offset 0)
            #[inline(always)]
            pub const fn em_addmin(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_em_addmin(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// REG_ADDMIN — 16 bits (offset 16)
            #[inline(always)]
            pub const fn reg_addmin(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_reg_addmin(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// ERRORTYPESTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Errortypestat(pub u32);

        impl Errortypestat {
            /// RADIO_EMACC_ERROR — 1 bit (offset 0)
            #[inline(always)]
            pub const fn radio_emacc_error(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_radio_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// FIFOWRITEERR — 1 bit (offset 1)
            #[inline(always)]
            pub const fn fifowriteerr(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifowriteerr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// ACT_SCHDL_ENTRY_ERROR — 1 bit (offset 2)
            #[inline(always)]
            pub const fn act_schdl_entry_error(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_act_schdl_entry_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// ACT_SCHDL_APFM_ERROR — 1 bit (offset 3)
            #[inline(always)]
            pub const fn act_schdl_apfm_error(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_act_schdl_apfm_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }
        }

        /// SWPROFILING
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Swprofiling(pub u32);

        impl Swprofiling {
            /// SWPROF0 — 1 bit (offset 0)
            #[inline(always)]
            pub const fn swprof0(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof0(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SWPROF1 — 1 bit (offset 1)
            #[inline(always)]
            pub const fn swprof1(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SWPROF2 — 1 bit (offset 2)
            #[inline(always)]
            pub const fn swprof2(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWPROF3 — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swprof3(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// SWPROF4 — 1 bit (offset 4)
            #[inline(always)]
            pub const fn swprof4(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// SWPROF5 — 1 bit (offset 5)
            #[inline(always)]
            pub const fn swprof5(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// SWPROF6 — 1 bit (offset 6)
            #[inline(always)]
            pub const fn swprof6(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// SWPROF7 — 1 bit (offset 7)
            #[inline(always)]
            pub const fn swprof7(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// SWPROF8 — 1 bit (offset 8)
            #[inline(always)]
            pub const fn swprof8(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// SWPROF9 — 1 bit (offset 9)
            #[inline(always)]
            pub const fn swprof9(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// SWPROF10 — 1 bit (offset 10)
            #[inline(always)]
            pub const fn swprof10(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// SWPROF11 — 1 bit (offset 11)
            #[inline(always)]
            pub const fn swprof11(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// SWPROF12 — 1 bit (offset 12)
            #[inline(always)]
            pub const fn swprof12(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// SWPROF13 — 1 bit (offset 13)
            #[inline(always)]
            pub const fn swprof13(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// SWPROF14 — 1 bit (offset 14)
            #[inline(always)]
            pub const fn swprof14(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// SWPROF15 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn swprof15(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// SWPROF16 — 1 bit (offset 16)
            #[inline(always)]
            pub const fn swprof16(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof16(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// SWPROF17 — 1 bit (offset 17)
            #[inline(always)]
            pub const fn swprof17(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof17(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// SWPROF18 — 1 bit (offset 18)
            #[inline(always)]
            pub const fn swprof18(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof18(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// SWPROF19 — 1 bit (offset 19)
            #[inline(always)]
            pub const fn swprof19(&self) -> bool {
                (self.0 >> 19) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof19(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 19)) | ((val as u32) << 19);
            }

            /// SWPROF20 — 1 bit (offset 20)
            #[inline(always)]
            pub const fn swprof20(&self) -> bool {
                (self.0 >> 20) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof20(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 20)) | ((val as u32) << 20);
            }

            /// SWPROF21 — 1 bit (offset 21)
            #[inline(always)]
            pub const fn swprof21(&self) -> bool {
                (self.0 >> 21) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof21(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 21)) | ((val as u32) << 21);
            }

            /// SWPROF22 — 1 bit (offset 22)
            #[inline(always)]
            pub const fn swprof22(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof22(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// SWPROF23 — 1 bit (offset 23)
            #[inline(always)]
            pub const fn swprof23(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof23(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }

            /// SWPROF24 — 1 bit (offset 24)
            #[inline(always)]
            pub const fn swprof24(&self) -> bool {
                (self.0 >> 24) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof24(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 24)) | ((val as u32) << 24);
            }

            /// SWPROF25 — 1 bit (offset 25)
            #[inline(always)]
            pub const fn swprof25(&self) -> bool {
                (self.0 >> 25) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof25(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 25)) | ((val as u32) << 25);
            }

            /// SWPROF26 — 1 bit (offset 26)
            #[inline(always)]
            pub const fn swprof26(&self) -> bool {
                (self.0 >> 26) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof26(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 26)) | ((val as u32) << 26);
            }

            /// SWPROF27 — 1 bit (offset 27)
            #[inline(always)]
            pub const fn swprof27(&self) -> bool {
                (self.0 >> 27) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof27(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 27)) | ((val as u32) << 27);
            }

            /// SWPROF28 — 1 bit (offset 28)
            #[inline(always)]
            pub const fn swprof28(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof28(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// SWPROF29 — 1 bit (offset 29)
            #[inline(always)]
            pub const fn swprof29(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof29(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// SWPROF30 — 1 bit (offset 30)
            #[inline(always)]
            pub const fn swprof30(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof30(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// SWPROF31 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn swprof31(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof31(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// RADIOCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl0(pub u32);

        impl Radiocntl0 {
            /// SPICOMP — 1 bit (offset 1)
            #[inline(always)]
            pub const fn spicomp(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_spicomp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SPIFREQ — 2 bits (offset 4)
            #[inline(always)]
            pub const fn spifreq(&self) -> u8 {
                ((self.0 >> 4) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_spifreq(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 4)) | ((val as u32 & 0x3) << 4);
            }

            /// SPICFG — 1 bit (offset 7)
            #[inline(always)]
            pub const fn spicfg(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_spicfg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// SPIPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn spiptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_spiptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// RADIOCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl1(pub u32);

        impl Radiocntl1 {
            /// SUBVERSION — 4 bits (offset 0)
            #[inline(always)]
            pub const fn subversion(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_subversion(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// XRFSEL — 6 bits (offset 4)
            #[inline(always)]
            pub const fn xrfsel(&self) -> u8 {
                ((self.0 >> 4) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_xrfsel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 4)) | ((val as u32 & 0x3F) << 4);
            }

            /// JEF_SELECT — 1 bit (offset 12)
            #[inline(always)]
            pub const fn jef_select(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_jef_select(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// DPCORR_EN — 1 bit (offset 13)
            #[inline(always)]
            pub const fn dpcorr_en(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_dpcorr_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// SYNC_PULSE_SRC — 1 bit (offset 14)
            #[inline(always)]
            pub const fn sync_pulse_src(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sync_pulse_src(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// SYNC_PULSE_MODE — 1 bit (offset 15)
            #[inline(always)]
            pub const fn sync_pulse_mode(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sync_pulse_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// FORCEAGC_LENGTH — 12 bits (offset 16)
            #[inline(always)]
            pub const fn forceagc_length(&self) -> u16 {
                ((self.0 >> 16) & 0xFFF) as u16
            }

            #[inline(always)]
            pub fn set_forceagc_length(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFF << 16)) | ((val as u32 & 0xFFF) << 16);
            }

            /// TXDNSL — 1 bit (offset 28)
            #[inline(always)]
            pub const fn txdnsl(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txdnsl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// RXDNSL — 1 bit (offset 29)
            #[inline(always)]
            pub const fn rxdnsl(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxdnsl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// FORCEIQ — 1 bit (offset 30)
            #[inline(always)]
            pub const fn forceiq(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_forceiq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// FORCEAGC_EN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn forceagc_en(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_forceagc_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// AESCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aescntl(pub u32);

        impl Aescntl {
            /// AES_START — 1 bit (offset 0)
            #[inline(always)]
            pub const fn aes_start(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aes_start(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// AES_MODE — 1 bit (offset 1)
            #[inline(always)]
            pub const fn aes_mode(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aes_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }
        }

        /// AESKEY31_0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey310(pub u32);

        impl Aeskey310 {
            /// AESKEY31_0 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey31_0(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey31_0(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY63_32
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey6332(pub u32);

        impl Aeskey6332 {
            /// AESKEY63_32 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey63_32(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey63_32(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY95_64
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey9564(pub u32);

        impl Aeskey9564 {
            /// AESKEY95_64 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey95_64(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey95_64(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY127_96
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey12796(pub u32);

        impl Aeskey12796 {
            /// AESKEY127_96 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey127_96(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey127_96(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aesptr(pub u32);

        impl Aesptr {
            /// AESPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn aesptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_aesptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// TXMICVAL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Txmicval(pub u32);

        impl Txmicval {
            /// TXMICVAL — 32 bits (offset 0)
            #[inline(always)]
            pub const fn txmicval(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_txmicval(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// RXMICVAL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rxmicval(pub u32);

        impl Rxmicval {
            /// RXMICVAL — 32 bits (offset 0)
            #[inline(always)]
            pub const fn rxmicval(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_rxmicval(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// TIMGENCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Timgencntl(pub u32);

        impl Timgencntl {
            /// PREFETCH_TIME — 9 bits (offset 0)
            #[inline(always)]
            pub const fn prefetch_time(&self) -> u16 {
                (self.0 & 0x1FF) as u16
            }

            #[inline(always)]
            pub fn set_prefetch_time(&mut self, val: u16) {
                self.0 = (self.0 & !0x1FF) | (val as u32 & 0x1FF);
            }

            /// PREFETCHABORT_TIME — 10 bits (offset 16)
            #[inline(always)]
            pub const fn prefetchabort_time(&self) -> u16 {
                ((self.0 >> 16) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_prefetchabort_time(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 16)) | ((val as u32 & 0x3FF) << 16);
            }
        }

        /// FINETIMTGT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finetimtgt(pub u32);

        impl Finetimtgt {
            /// FINETARGET — 28 bits (offset 0)
            #[inline(always)]
            pub const fn finetarget(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_finetarget(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// CLKNTGT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt1(pub u32);

        impl Clkntgt1 {
            /// CLKNTGT1 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt1(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt1(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt1(pub u32);

        impl Hmicrosectgt1 {
            /// HMICROSECTGT1 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt1(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt1(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNTGT2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt2(pub u32);

        impl Clkntgt2 {
            /// CLKNTGT2 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt2(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt2(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt2(pub u32);

        impl Hmicrosectgt2 {
            /// HMICROSECTGT2 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt2(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt2(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNTGT3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt3(pub u32);

        impl Clkntgt3 {
            /// CLKNTGT3 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt3(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt3(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt3(pub u32);

        impl Hmicrosectgt3 {
            /// HMICROSECTGT3 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt3(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt3(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// SLOTCLK
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Slotclk(pub u32);

        impl Slotclk {
            /// SCLK — 28 bits (offset 0)
            #[inline(always)]
            pub const fn sclk(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_sclk(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }

            /// CLKN_UPD — 1 bit (offset 30)
            #[inline(always)]
            pub const fn clkn_upd(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clkn_upd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// SAMP — 1 bit (offset 31)
            #[inline(always)]
            pub const fn samp(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_samp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// FINETIMECNT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finetimecnt(pub u32);

        impl Finetimecnt {
            /// FINECNT — 10 bits (offset 0)
            #[inline(always)]
            pub const fn finecnt(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_finecnt(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// ACTSCHCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Actschcntl(pub u32);

        impl Actschcntl {
            /// ENTRY_IDX — 4 bits (offset 0)
            #[inline(always)]
            pub const fn entry_idx(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_entry_idx(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// START_ACT — 1 bit (offset 31)
            #[inline(always)]
            pub const fn start_act(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_start_act(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DFANCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Dfancntl(pub u32);

        impl Dfancntl {
            /// LETXPRIMANTID — 7 bits (offset 0)
            #[inline(always)]
            pub const fn letxprimantid(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_letxprimantid(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// LETXPRIMIDCNTLEN — 1 bit (offset 7)
            #[inline(always)]
            pub const fn letxprimidcntlen(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_letxprimidcntlen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// LERXPRIMANTID — 7 bits (offset 8)
            #[inline(always)]
            pub const fn lerxprimantid(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_lerxprimantid(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// LERXPRIMIDCNTLEN — 1 bit (offset 15)
            #[inline(always)]
            pub const fn lerxprimidcntlen(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_lerxprimidcntlen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// BTTXPRIMANTID — 7 bits (offset 16)
            #[inline(always)]
            pub const fn bttxprimantid(&self) -> u8 {
                ((self.0 >> 16) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_bttxprimantid(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 16)) | ((val as u32 & 0x7F) << 16);
            }

            /// BTTXPRIMIDCNTLEN — 1 bit (offset 23)
            #[inline(always)]
            pub const fn bttxprimidcntlen(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_bttxprimidcntlen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }

            /// BTRXPRIMANTID — 7 bits (offset 24)
            #[inline(always)]
            pub const fn btrxprimantid(&self) -> u8 {
                ((self.0 >> 24) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_btrxprimantid(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 24)) | ((val as u32 & 0x7F) << 24);
            }

            /// BTRXPRIMIDCNTLEN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn btrxprimidcntlen(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_btrxprimidcntlen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }
    }
}

pub mod bt {
    #[allow(unused_imports)]
    use crate::common::{Reg, R, RW, W};

    /// BT core block.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Bt {
        ptr: *mut u8,
    }

    unsafe impl Send for Bt {}
    unsafe impl Sync for Bt {}

    impl Bt {
        /// # Safety
        /// `ptr` must point to a valid BT core register block.
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self {
                ptr: ptr as *mut u8,
            }
        }

        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as *mut ()
        }

        /// RWBTCNTL (0x00)
        #[inline(always)]
        pub const fn rwbtcntl(self) -> Reg<regs::Rwbtcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x00usize) as _) }
        }

        /// VERSION (0x04)
        #[inline(always)]
        pub const fn version(self) -> Reg<regs::Version, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }

        /// RWBTCONF (0x08)
        #[inline(always)]
        pub const fn rwbtconf(self) -> Reg<regs::Rwbtconf, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }

        /// INTCNTL0 (0x0C)
        #[inline(always)]
        pub const fn intcntl0(self) -> Reg<regs::Intcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x0Cusize) as _) }
        }

        /// INTSTAT0 (0x10)
        #[inline(always)]
        pub const fn intstat0(self) -> Reg<regs::Intstat0, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }

        /// INTACK0 (0x14)
        #[inline(always)]
        pub const fn intack0(self) -> Reg<regs::Intack0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }

        /// INTCNTL1 (0x18)
        #[inline(always)]
        pub const fn intcntl1(self) -> Reg<regs::Intcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }

        /// INTSTAT1 (0x1C)
        #[inline(always)]
        pub const fn intstat1(self) -> Reg<regs::Intstat1, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1Cusize) as _) }
        }

        /// INTACK1 (0x20)
        #[inline(always)]
        pub const fn intack1(self) -> Reg<regs::Intack1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }

        /// ACTFIFOSTAT (0x24)
        #[inline(always)]
        pub const fn actfifostat(self) -> Reg<regs::Actfifostat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }

        /// CURRENTRXDESCPTR (0x28)
        #[inline(always)]
        pub const fn currentrxdescptr(self) -> Reg<regs::Currentrxdescptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }

        /// ETPTR (0x2C)
        #[inline(always)]
        pub const fn etptr(self) -> Reg<regs::Etptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x2Cusize) as _) }
        }

        /// DEEPSLCNTL (0x30)
        #[inline(always)]
        pub const fn deepslcntl(self) -> Reg<regs::Deepslcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }

        /// DEEPSLWKUP (0x34)
        #[inline(always)]
        pub const fn deepslwkup(self) -> Reg<regs::Deepslwkup, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }

        /// DEEPSLSTAT (0x38)
        #[inline(always)]
        pub const fn deepslstat(self) -> Reg<regs::Deepslstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }

        /// ENBPRESET (0x3C)
        #[inline(always)]
        pub const fn enbpreset(self) -> Reg<regs::Enbpreset, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x3Cusize) as _) }
        }

        /// FINECNTCORR (0x40)
        #[inline(always)]
        pub const fn finecntcorr(self) -> Reg<regs::Finecntcorr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }

        /// CLKNCNTCORR (0x44)
        #[inline(always)]
        pub const fn clkncntcorr(self) -> Reg<regs::Clkncntcorr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x44usize) as _) }
        }

        /// DIAGCNTL (0x50)
        #[inline(always)]
        pub const fn diagcntl(self) -> Reg<regs::Diagcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x50usize) as _) }
        }

        /// DIAGSTAT (0x54)
        #[inline(always)]
        pub const fn diagstat(self) -> Reg<regs::Diagstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x54usize) as _) }
        }

        /// DEBUGADDMAX (0x58)
        #[inline(always)]
        pub const fn debugaddmax(self) -> Reg<regs::Debugaddmax, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x58usize) as _) }
        }

        /// DEBUGADDMIN (0x5C)
        #[inline(always)]
        pub const fn debugaddmin(self) -> Reg<regs::Debugaddmin, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x5Cusize) as _) }
        }

        /// ERRORTYPESTAT (0x60)
        #[inline(always)]
        pub const fn errortypestat(self) -> Reg<regs::Errortypestat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x60usize) as _) }
        }

        /// SWPROFILING (0x64)
        #[inline(always)]
        pub const fn swprofiling(self) -> Reg<regs::Swprofiling, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x64usize) as _) }
        }

        /// RADIOCNTL0 (0x70)
        #[inline(always)]
        pub const fn radiocntl0(self) -> Reg<regs::Radiocntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }

        /// RADIOCNTL1 (0x74)
        #[inline(always)]
        pub const fn radiocntl1(self) -> Reg<regs::Radiocntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }

        /// RADIOCNTL2 (0x78)
        #[inline(always)]
        pub const fn radiocntl2(self) -> Reg<regs::Radiocntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }

        /// RADIOCNTL3 (0x7C)
        #[inline(always)]
        pub const fn radiocntl3(self) -> Reg<regs::Radiocntl3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x7Cusize) as _) }
        }

        /// RADIOPWRUPDN (0x8C)
        #[inline(always)]
        pub const fn radiopwrupdn(self) -> Reg<regs::Radiopwrupdn, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x8Cusize) as _) }
        }

        /// RADIOTXRXTIM (0x90)
        #[inline(always)]
        pub const fn radiotxrxtim(self) -> Reg<regs::Radiotxrxtim, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x90usize) as _) }
        }

        /// SPIPTRCNTL0 (0xA0)
        #[inline(always)]
        pub const fn spiptrcntl0(self) -> Reg<regs::Spiptrcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xA0usize) as _) }
        }

        /// SPIPTRCNTL1 (0xA4)
        #[inline(always)]
        pub const fn spiptrcntl1(self) -> Reg<regs::Spiptrcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xA4usize) as _) }
        }

        /// SPIPTRCNTL2 (0xA8)
        #[inline(always)]
        pub const fn spiptrcntl2(self) -> Reg<regs::Spiptrcntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xA8usize) as _) }
        }

        /// SPIPTRCNTL3 (0xAC)
        #[inline(always)]
        pub const fn spiptrcntl3(self) -> Reg<regs::Spiptrcntl3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xACusize) as _) }
        }

        /// AESCNTL (0xB0)
        #[inline(always)]
        pub const fn aescntl(self) -> Reg<regs::Aescntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB0usize) as _) }
        }

        /// AESKEY31_0 (0xB4)
        #[inline(always)]
        pub const fn aeskey31_0(self) -> Reg<regs::Aeskey310, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB4usize) as _) }
        }

        /// AESKEY63_32 (0xB8)
        #[inline(always)]
        pub const fn aeskey63_32(self) -> Reg<regs::Aeskey6332, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB8usize) as _) }
        }

        /// AESKEY95_64 (0xBC)
        #[inline(always)]
        pub const fn aeskey95_64(self) -> Reg<regs::Aeskey9564, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xBCusize) as _) }
        }

        /// AESKEY127_96 (0xC0)
        #[inline(always)]
        pub const fn aeskey127_96(self) -> Reg<regs::Aeskey12796, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC0usize) as _) }
        }

        /// AESPTR (0xC4)
        #[inline(always)]
        pub const fn aesptr(self) -> Reg<regs::Aesptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC4usize) as _) }
        }

        /// TXMICVAL (0xC8)
        #[inline(always)]
        pub const fn txmicval(self) -> Reg<regs::Txmicval, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC8usize) as _) }
        }

        /// RXMICVAL (0xCC)
        #[inline(always)]
        pub const fn rxmicval(self) -> Reg<regs::Rxmicval, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xCCusize) as _) }
        }

        /// RFTESTCNTL (0xD0)
        #[inline(always)]
        pub const fn rftestcntl(self) -> Reg<regs::Rftestcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xD0usize) as _) }
        }

        /// RFTESTFREQ (0xD4)
        #[inline(always)]
        pub const fn rftestfreq(self) -> Reg<regs::Rftestfreq, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xD4usize) as _) }
        }

        /// RFTESTTXSTAT (0xD8)
        #[inline(always)]
        pub const fn rftesttxstat(self) -> Reg<regs::Rftesttxstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xD8usize) as _) }
        }

        /// RFTESTRXSTAT (0xDC)
        #[inline(always)]
        pub const fn rftestrxstat(self) -> Reg<regs::Rftestrxstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xDCusize) as _) }
        }

        /// TIMGENCNTL (0xE0)
        #[inline(always)]
        pub const fn timgencntl(self) -> Reg<regs::Timgencntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE0usize) as _) }
        }

        /// FINETIMTGT (0xE4)
        #[inline(always)]
        pub const fn finetimtgt(self) -> Reg<regs::Finetimtgt, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE4usize) as _) }
        }

        /// CLKNTGT1 (0xE8)
        #[inline(always)]
        pub const fn clkntgt1(self) -> Reg<regs::Clkntgt1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE8usize) as _) }
        }

        /// HMICROSECTGT1 (0xEC)
        #[inline(always)]
        pub const fn hmicrosectgt1(self) -> Reg<regs::Hmicrosectgt1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xECusize) as _) }
        }

        /// CLKNTGT2 (0xF0)
        #[inline(always)]
        pub const fn clkntgt2(self) -> Reg<regs::Clkntgt2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF0usize) as _) }
        }

        /// HMICROSECTGT2 (0xF4)
        #[inline(always)]
        pub const fn hmicrosectgt2(self) -> Reg<regs::Hmicrosectgt2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF4usize) as _) }
        }

        /// CLKNTGT3 (0xF8)
        #[inline(always)]
        pub const fn clkntgt3(self) -> Reg<regs::Clkntgt3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF8usize) as _) }
        }

        /// HMICROSECTGT3 (0xFC)
        #[inline(always)]
        pub const fn hmicrosectgt3(self) -> Reg<regs::Hmicrosectgt3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xFCusize) as _) }
        }

        /// SLOTCLK (0x100)
        #[inline(always)]
        pub const fn slotclk(self) -> Reg<regs::Slotclk, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x100usize) as _) }
        }

        /// FINETIMECNT (0x104)
        #[inline(always)]
        pub const fn finetimecnt(self) -> Reg<regs::Finetimecnt, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x104usize) as _) }
        }

        /// ACTSCHCNTL (0x110)
        #[inline(always)]
        pub const fn actschcntl(self) -> Reg<regs::Actschcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x110usize) as _) }
        }

        /// STARTFRMCLKNTS (0x114)
        #[inline(always)]
        pub const fn startfrmclknts(self) -> Reg<regs::Startfrmclknts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x114usize) as _) }
        }

        /// STARTFRMFINECNTTS (0x118)
        #[inline(always)]
        pub const fn startfrmfinecntts(self) -> Reg<regs::Startfrmfinecntts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x118usize) as _) }
        }

        /// ENDFRMCLKNTS (0x11C)
        #[inline(always)]
        pub const fn endfrmclknts(self) -> Reg<regs::Endfrmclknts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x11Cusize) as _) }
        }

        /// ENDFRMFINECNTTS (0x120)
        #[inline(always)]
        pub const fn endfrmfinecntts(self) -> Reg<regs::Endfrmfinecntts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x120usize) as _) }
        }

        /// SKIPFRMCLKNTS (0x124)
        #[inline(always)]
        pub const fn skipfrmclknts(self) -> Reg<regs::Skipfrmclknts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x124usize) as _) }
        }

        /// SKIPFRMFINECNTTS (0x128)
        #[inline(always)]
        pub const fn skipfrmfinecntts(self) -> Reg<regs::Skipfrmfinecntts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x128usize) as _) }
        }

        /// ABTRAINCNTL (0x130)
        #[inline(always)]
        pub const fn abtraincntl(self) -> Reg<regs::Abtraincntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x130usize) as _) }
        }

        /// EDRCNTL (0x134)
        #[inline(always)]
        pub const fn edrcntl(self) -> Reg<regs::Edrcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x134usize) as _) }
        }

        /// PCACNTL0 (0x140)
        #[inline(always)]
        pub const fn pcacntl0(self) -> Reg<regs::Pcacntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x140usize) as _) }
        }

        /// PCACNTL1 (0x144)
        #[inline(always)]
        pub const fn pcacntl1(self) -> Reg<regs::Pcacntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x144usize) as _) }
        }

        /// PCASTAT (0x148)
        #[inline(always)]
        pub const fn pcastat(self) -> Reg<regs::Pcastat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x148usize) as _) }
        }

        /// COEXIFCNTL0 (0x150)
        #[inline(always)]
        pub const fn coexifcntl0(self) -> Reg<regs::Coexifcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x150usize) as _) }
        }

        /// COEXIFCNTL1 (0x154)
        #[inline(always)]
        pub const fn coexifcntl1(self) -> Reg<regs::Coexifcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x154usize) as _) }
        }

        /// COEXIFCNTL2 (0x158)
        #[inline(always)]
        pub const fn coexifcntl2(self) -> Reg<regs::Coexifcntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x158usize) as _) }
        }

        /// BTMPRIO0 (0x160)
        #[inline(always)]
        pub const fn btmprio0(self) -> Reg<regs::Btmprio0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x160usize) as _) }
        }

        /// BTMPRIO1 (0x164)
        #[inline(always)]
        pub const fn btmprio1(self) -> Reg<regs::Btmprio1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x164usize) as _) }
        }

        /// BTMPRIO2 (0x168)
        #[inline(always)]
        pub const fn btmprio2(self) -> Reg<regs::Btmprio2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x168usize) as _) }
        }

        /// MWSPTABLE0 (0x170)
        #[inline(always)]
        pub const fn mwsptable0(self) -> Reg<regs::Mwsptable0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x170usize) as _) }
        }

        /// MWSPTIMING00 (0x174)
        #[inline(always)]
        pub const fn mwsptiming00(self) -> Reg<regs::Mwsptiming00, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x174usize) as _) }
        }

        /// MWSPTIMING01 (0x178)
        #[inline(always)]
        pub const fn mwsptiming01(self) -> Reg<regs::Mwsptiming01, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x178usize) as _) }
        }

        /// MWSPTIMING02 (0x17C)
        #[inline(always)]
        pub const fn mwsptiming02(self) -> Reg<regs::Mwsptiming02, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x17Cusize) as _) }
        }

        /// MWSPTABLE1 (0x180)
        #[inline(always)]
        pub const fn mwsptable1(self) -> Reg<regs::Mwsptable1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x180usize) as _) }
        }

        /// MWSPTIMING10 (0x184)
        #[inline(always)]
        pub const fn mwsptiming10(self) -> Reg<regs::Mwsptiming10, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x184usize) as _) }
        }

        /// MWSPTIMING11 (0x188)
        #[inline(always)]
        pub const fn mwsptiming11(self) -> Reg<regs::Mwsptiming11, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x188usize) as _) }
        }

        /// MWSPTIMING12 (0x18C)
        #[inline(always)]
        pub const fn mwsptiming12(self) -> Reg<regs::Mwsptiming12, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x18Cusize) as _) }
        }

        /// MWSPTABLE2 (0x190)
        #[inline(always)]
        pub const fn mwsptable2(self) -> Reg<regs::Mwsptable2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x190usize) as _) }
        }

        /// MWSPTIMING20 (0x194)
        #[inline(always)]
        pub const fn mwsptiming20(self) -> Reg<regs::Mwsptiming20, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x194usize) as _) }
        }

        /// MWSPTIMING21 (0x198)
        #[inline(always)]
        pub const fn mwsptiming21(self) -> Reg<regs::Mwsptiming21, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x198usize) as _) }
        }

        /// MWSPTIMING22 (0x19C)
        #[inline(always)]
        pub const fn mwsptiming22(self) -> Reg<regs::Mwsptiming22, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x19Cusize) as _) }
        }

        /// MWSIFSTAT (0x1A0)
        #[inline(always)]
        pub const fn mwsifstat(self) -> Reg<regs::Mwsifstat, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1A0usize) as _) }
        }

        /// MWSTXTABLE0 (0x1B0)
        #[inline(always)]
        pub const fn mwstxtable0(self) -> Reg<regs::Mwstxtable0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1B0usize) as _) }
        }

        /// MWSRXTABLE0 (0x1B4)
        #[inline(always)]
        pub const fn mwsrxtable0(self) -> Reg<regs::Mwsrxtable0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1B4usize) as _) }
        }

        /// MWSSFTABLE1 (0x1B8)
        #[inline(always)]
        pub const fn mwssftable1(self) -> Reg<regs::Mwssftable1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1B8usize) as _) }
        }

        /// MWSSFTABLE2 (0x1BC)
        #[inline(always)]
        pub const fn mwssftable2(self) -> Reg<regs::Mwssftable2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1BCusize) as _) }
        }

        /// MWSSFTABLE3 (0x1C0)
        #[inline(always)]
        pub const fn mwssftable3(self) -> Reg<regs::Mwssftable3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1C0usize) as _) }
        }

        /// MWSSFTABLE4 (0x1C4)
        #[inline(always)]
        pub const fn mwssftable4(self) -> Reg<regs::Mwssftable4, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1C4usize) as _) }
        }

        /// MWSSFTABLE5 (0x1C8)
        #[inline(always)]
        pub const fn mwssftable5(self) -> Reg<regs::Mwssftable5, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1C8usize) as _) }
        }

        /// MWSSFTABLE6 (0x1CC)
        #[inline(always)]
        pub const fn mwssftable6(self) -> Reg<regs::Mwssftable6, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1CCusize) as _) }
        }

        /// MWSSFTABLE7 (0x1D0)
        #[inline(always)]
        pub const fn mwssftable7(self) -> Reg<regs::Mwssftable7, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1D0usize) as _) }
        }

        /// MWSSFTABLE8 (0x1D4)
        #[inline(always)]
        pub const fn mwssftable8(self) -> Reg<regs::Mwssftable8, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1D4usize) as _) }
        }

        /// MWSFRSYNCOFFSET (0x1E0)
        #[inline(always)]
        pub const fn mwsfrsyncoffset(self) -> Reg<regs::Mwsfrsyncoffset, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1E0usize) as _) }
        }

        /// MWSTXOFFSET (0x1E4)
        #[inline(always)]
        pub const fn mwstxoffset(self) -> Reg<regs::Mwstxoffset, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1E4usize) as _) }
        }

        /// MWSRXOFFSET (0x1E8)
        #[inline(always)]
        pub const fn mwsrxoffset(self) -> Reg<regs::Mwsrxoffset, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1E8usize) as _) }
        }

        /// MWSWCICNTL0 (0x1F0)
        #[inline(always)]
        pub const fn mwswcicntl0(self) -> Reg<regs::Mwswcicntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1F0usize) as _) }
        }

        /// MWSWCICNTL1 (0x1F4)
        #[inline(always)]
        pub const fn mwswcicntl1(self) -> Reg<regs::Mwswcicntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1F4usize) as _) }
        }

        /// MWSWCITXCNTL (0x1F8)
        #[inline(always)]
        pub const fn mwswcitxcntl(self) -> Reg<regs::Mwswcitxcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1F8usize) as _) }
        }

        /// MWSWCIRXCNTL (0x1FC)
        #[inline(always)]
        pub const fn mwswcirxcntl(self) -> Reg<regs::Mwswcirxcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1FCusize) as _) }
        }

        /// LSAMCNTL0 (0x200)
        #[inline(always)]
        pub const fn lsamcntl0(self) -> Reg<regs::Lsamcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x200usize) as _) }
        }

        /// LSAMCNTL1 (0x204)
        #[inline(always)]
        pub const fn lsamcntl1(self) -> Reg<regs::Lsamcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x204usize) as _) }
        }

        /// eSCOCHANCNTL0 (0x210)
        #[inline(always)]
        pub const fn escochancntl0(self) -> Reg<regs::Escochancntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x210usize) as _) }
        }

        /// eSCOMUTECNTL0 (0x214)
        #[inline(always)]
        pub const fn escomutecntl0(self) -> Reg<regs::Escomutecntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x214usize) as _) }
        }

        /// eSCOCURRENTTXPTR0 (0x218)
        #[inline(always)]
        pub const fn escocurrenttxptr0(self) -> Reg<regs::Escocurrenttxptr0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x218usize) as _) }
        }

        /// eSCOCURRENTRXPTR0 (0x21C)
        #[inline(always)]
        pub const fn escocurrentrxptr0(self) -> Reg<regs::Escocurrentrxptr0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x21Cusize) as _) }
        }

        /// eSCOLTCNTL0 (0x220)
        #[inline(always)]
        pub const fn escoltcntl0(self) -> Reg<regs::Escoltcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x220usize) as _) }
        }

        /// eSCOTRCNTL0 (0x224)
        #[inline(always)]
        pub const fn escotrcntl0(self) -> Reg<regs::Escotrcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x224usize) as _) }
        }

        /// eSCODAYCNT0 (0x228)
        #[inline(always)]
        pub const fn escodaycnt0(self) -> Reg<regs::Escodaycnt0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x228usize) as _) }
        }

        /// eSCOCHANCNTL1 (0x230)
        #[inline(always)]
        pub const fn escochancntl1(self) -> Reg<regs::Escochancntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x230usize) as _) }
        }

        /// eSCOMUTECNTL1 (0x234)
        #[inline(always)]
        pub const fn escomutecntl1(self) -> Reg<regs::Escomutecntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x234usize) as _) }
        }

        /// eSCOCURRENTTXPTR1 (0x238)
        #[inline(always)]
        pub const fn escocurrenttxptr1(self) -> Reg<regs::Escocurrenttxptr1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x238usize) as _) }
        }

        /// eSCOCURRENTRXPTR1 (0x23C)
        #[inline(always)]
        pub const fn escocurrentrxptr1(self) -> Reg<regs::Escocurrentrxptr1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x23Cusize) as _) }
        }

        /// eSCOLTCNTL1 (0x240)
        #[inline(always)]
        pub const fn escoltcntl1(self) -> Reg<regs::Escoltcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x240usize) as _) }
        }

        /// eSCOTRCNTL1 (0x244)
        #[inline(always)]
        pub const fn escotrcntl1(self) -> Reg<regs::Escotrcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x244usize) as _) }
        }

        /// eSCODAYCNT1 (0x248)
        #[inline(always)]
        pub const fn escodaycnt1(self) -> Reg<regs::Escodaycnt1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x248usize) as _) }
        }

        /// eSCOCHANCNTL2 (0x250)
        #[inline(always)]
        pub const fn escochancntl2(self) -> Reg<regs::Escochancntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x250usize) as _) }
        }

        /// eSCOMUTECNTL2 (0x254)
        #[inline(always)]
        pub const fn escomutecntl2(self) -> Reg<regs::Escomutecntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x254usize) as _) }
        }

        /// eSCOCURRENTTXPTR2 (0x258)
        #[inline(always)]
        pub const fn escocurrenttxptr2(self) -> Reg<regs::Escocurrenttxptr2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x258usize) as _) }
        }

        /// eSCOCURRENTRXPTR2 (0x25C)
        #[inline(always)]
        pub const fn escocurrentrxptr2(self) -> Reg<regs::Escocurrentrxptr2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x25Cusize) as _) }
        }

        /// eSCOLTCNTL2 (0x260)
        #[inline(always)]
        pub const fn escoltcntl2(self) -> Reg<regs::Escoltcntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x260usize) as _) }
        }

        /// eSCOTRCNTL2 (0x264)
        #[inline(always)]
        pub const fn escotrcntl2(self) -> Reg<regs::Escotrcntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x264usize) as _) }
        }

        /// eSCODAYCNT2 (0x268)
        #[inline(always)]
        pub const fn escodaycnt2(self) -> Reg<regs::Escodaycnt2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x268usize) as _) }
        }

        /// AUDIOCNTL0 (0x270)
        #[inline(always)]
        pub const fn audiocntl0(self) -> Reg<regs::Audiocntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x270usize) as _) }
        }

        /// AUDIOCNTL1 (0x274)
        #[inline(always)]
        pub const fn audiocntl1(self) -> Reg<regs::Audiocntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x274usize) as _) }
        }

        /// AUDIOCNTL2 (0x278)
        #[inline(always)]
        pub const fn audiocntl2(self) -> Reg<regs::Audiocntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x278usize) as _) }
        }

        /// PCMGENCNTL (0x280)
        #[inline(always)]
        pub const fn pcmgencntl(self) -> Reg<regs::Pcmgencntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x280usize) as _) }
        }

        /// PCMPHYSCNTL0 (0x284)
        #[inline(always)]
        pub const fn pcmphyscntl0(self) -> Reg<regs::Pcmphyscntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x284usize) as _) }
        }

        /// PCMPHYSCNTL1 (0x288)
        #[inline(always)]
        pub const fn pcmphyscntl1(self) -> Reg<regs::Pcmphyscntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x288usize) as _) }
        }

        /// PCMPADDING (0x28C)
        #[inline(always)]
        pub const fn pcmpadding(self) -> Reg<regs::Pcmpadding, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x28Cusize) as _) }
        }

        /// PCMPLLCNTL0 (0x290)
        #[inline(always)]
        pub const fn pcmpllcntl0(self) -> Reg<regs::Pcmpllcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x290usize) as _) }
        }

        /// PCMPLLCNTL1 (0x294)
        #[inline(always)]
        pub const fn pcmpllcntl1(self) -> Reg<regs::Pcmpllcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x294usize) as _) }
        }

        /// PCMPLLCNTL2 (0x298)
        #[inline(always)]
        pub const fn pcmpllcntl2(self) -> Reg<regs::Pcmpllcntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x298usize) as _) }
        }

        /// PCMSOURCEPTR (0x29C)
        #[inline(always)]
        pub const fn pcmsourceptr(self) -> Reg<regs::Pcmsourceptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x29Cusize) as _) }
        }

        /// PCMSINKPTR (0x2A0)
        #[inline(always)]
        pub const fn pcmsinkptr(self) -> Reg<regs::Pcmsinkptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x2A0usize) as _) }
        }
    }

    pub mod regs {

        /// RWBTCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rwbtcntl(pub u32);

        impl Rwbtcntl {
            /// NWINSIZE — 6 bits (offset 0)
            #[inline(always)]
            pub const fn nwinsize(&self) -> u8 {
                (self.0 & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_nwinsize(&mut self, val: u8) {
                self.0 = (self.0 & !0x3F) | (val as u32 & 0x3F);
            }

            /// RWBTEN — 1 bit (offset 8)
            #[inline(always)]
            pub const fn rwbten(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rwbten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// CX_DNABORT — 1 bit (offset 9)
            #[inline(always)]
            pub const fn cx_dnabort(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cx_dnabort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// CX_RXBSYENA — 1 bit (offset 10)
            #[inline(always)]
            pub const fn cx_rxbsyena(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cx_rxbsyena(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// CX_TXBSYENA — 1 bit (offset 11)
            #[inline(always)]
            pub const fn cx_txbsyena(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cx_txbsyena(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// SEQNDSB — 1 bit (offset 12)
            #[inline(always)]
            pub const fn seqndsb(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_seqndsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// ARQNDSB — 1 bit (offset 13)
            #[inline(always)]
            pub const fn arqndsb(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_arqndsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// FLOWDSB — 1 bit (offset 14)
            #[inline(always)]
            pub const fn flowdsb(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_flowdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// HOPDSB — 1 bit (offset 15)
            #[inline(always)]
            pub const fn hopdsb(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_hopdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// WHITDSB — 1 bit (offset 16)
            #[inline(always)]
            pub const fn whitdsb(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_whitdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// CRCDSB — 1 bit (offset 17)
            #[inline(always)]
            pub const fn crcdsb(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_crcdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// CRYPTDSB — 1 bit (offset 18)
            #[inline(always)]
            pub const fn cryptdsb(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// LMPFLOWDSB — 1 bit (offset 19)
            #[inline(always)]
            pub const fn lmpflowdsb(&self) -> bool {
                (self.0 >> 19) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_lmpflowdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 19)) | ((val as u32) << 19);
            }

            /// SNIFF_ABORT — 1 bit (offset 20)
            #[inline(always)]
            pub const fn sniff_abort(&self) -> bool {
                (self.0 >> 20) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sniff_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 20)) | ((val as u32) << 20);
            }

            /// PAGEINQ_ABORT — 1 bit (offset 21)
            #[inline(always)]
            pub const fn pageinq_abort(&self) -> bool {
                (self.0 >> 21) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_pageinq_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 21)) | ((val as u32) << 21);
            }

            /// RFTEST_ABORT — 1 bit (offset 22)
            #[inline(always)]
            pub const fn rftest_abort(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rftest_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// SCAN_ABORT — 1 bit (offset 23)
            #[inline(always)]
            pub const fn scan_abort(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_scan_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }
        }

        /// VERSION
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Version(pub u32);

        impl Version {
            /// BUILD — 8 bits (offset 0)
            #[inline(always)]
            pub const fn build(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_build(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// UPG — 8 bits (offset 8)
            #[inline(always)]
            pub const fn upg(&self) -> u8 {
                ((self.0 >> 8) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_upg(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 8)) | ((val as u32 & 0xFF) << 8);
            }

            /// REL — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rel(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rel(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// TYP — 8 bits (offset 24)
            #[inline(always)]
            pub const fn typ(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_typ(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// RWBTCONF
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rwbtconf(pub u32);

        impl Rwbtconf {
            /// ADDR_WIDTH — 5 bits (offset 0)
            #[inline(always)]
            pub const fn addr_width(&self) -> u8 {
                (self.0 & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_addr_width(&mut self, val: u8) {
                self.0 = (self.0 & !0x1F) | (val as u32 & 0x1F);
            }

            /// BUS_TYPE — 1 bit (offset 6)
            #[inline(always)]
            pub const fn bus_type(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_bus_type(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// INTMODE — 1 bit (offset 7)
            #[inline(always)]
            pub const fn intmode(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_intmode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// CLK_SEL — 6 bits (offset 8)
            #[inline(always)]
            pub const fn clk_sel(&self) -> u8 {
                ((self.0 >> 8) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_clk_sel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 8)) | ((val as u32 & 0x3F) << 8);
            }

            /// DECIPHER — 1 bit (offset 14)
            #[inline(always)]
            pub const fn decipher(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_decipher(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// USEDBG — 1 bit (offset 15)
            #[inline(always)]
            pub const fn usedbg(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_usedbg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// RFIF — 7 bits (offset 16)
            #[inline(always)]
            pub const fn rfif(&self) -> u8 {
                ((self.0 >> 16) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rfif(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 16)) | ((val as u32 & 0x7F) << 16);
            }

            /// PCM — 1 bit (offset 23)
            #[inline(always)]
            pub const fn pcm(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_pcm(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }

            /// VXPORTNB — 2 bits (offset 24)
            #[inline(always)]
            pub const fn vxportnb(&self) -> u8 {
                ((self.0 >> 24) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_vxportnb(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 24)) | ((val as u32 & 0x3) << 24);
            }

            /// MWSWCI1 — 1 bit (offset 26)
            #[inline(always)]
            pub const fn mwswci1(&self) -> bool {
                (self.0 >> 26) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswci1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 26)) | ((val as u32) << 26);
            }

            /// MWSWCI2 — 1 bit (offset 27)
            #[inline(always)]
            pub const fn mwswci2(&self) -> bool {
                (self.0 >> 27) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswci2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 27)) | ((val as u32) << 27);
            }

            /// CORRELATOR — 1 bit (offset 28)
            #[inline(always)]
            pub const fn correlator(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_correlator(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// WLANCOEX — 1 bit (offset 29)
            #[inline(always)]
            pub const fn wlancoex(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_wlancoex(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// MWSCOEX — 1 bit (offset 30)
            #[inline(always)]
            pub const fn mwscoex(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwscoex(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// DMMODE — 1 bit (offset 31)
            #[inline(always)]
            pub const fn dmmode(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_dmmode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// INTCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intcntl0(pub u32);

        impl Intcntl0 {
            /// STARTFRMINTMSK — 1 bit (offset 0)
            #[inline(always)]
            pub const fn startfrmintmsk(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_startfrmintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// ENDFRMINTMSK — 1 bit (offset 1)
            #[inline(always)]
            pub const fn endfrmintmsk(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_endfrmintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SKIPFRMINTMSK — 1 bit (offset 2)
            #[inline(always)]
            pub const fn skipfrmintmsk(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_skipfrmintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// RXINTMSK — 1 bit (offset 4)
            #[inline(always)]
            pub const fn rxintmsk(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// FRSYNCINTMSK — 1 bit (offset 8)
            #[inline(always)]
            pub const fn frsyncintmsk(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_frsyncintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// MTOFFINT0MSK — 1 bit (offset 9)
            #[inline(always)]
            pub const fn mtoffint0msk(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mtoffint0msk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// MTOFFINT1MSK — 1 bit (offset 10)
            #[inline(always)]
            pub const fn mtoffint1msk(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mtoffint1msk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// MWSWCITXINTMSK — 1 bit (offset 11)
            #[inline(always)]
            pub const fn mwswcitxintmsk(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswcitxintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// MWSWCIRXINTMSK — 1 bit (offset 12)
            #[inline(always)]
            pub const fn mwswcirxintmsk(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswcirxintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// AUDIOINT0MSK — 1 bit (offset 13)
            #[inline(always)]
            pub const fn audioint0msk(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint0msk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// AUDIOINT1MSK — 1 bit (offset 14)
            #[inline(always)]
            pub const fn audioint1msk(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint1msk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// AUDIOINT2MSK — 1 bit (offset 15)
            #[inline(always)]
            pub const fn audioint2msk(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint2msk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// ERRORINTMSK — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintmsk(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTSTAT0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intstat0(pub u32);

        impl Intstat0 {
            /// FRSYNCINTSTAT — 1 bit (offset 8)
            #[inline(always)]
            pub const fn frsyncintstat(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_frsyncintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// MTOFFINT0STAT — 1 bit (offset 9)
            #[inline(always)]
            pub const fn mtoffint0stat(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mtoffint0stat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// MTOFFINT1STAT — 1 bit (offset 10)
            #[inline(always)]
            pub const fn mtoffint1stat(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mtoffint1stat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// MWSWCITXINTSTAT — 1 bit (offset 11)
            #[inline(always)]
            pub const fn mwswcitxintstat(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswcitxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// MWSWCIRXINTSTAT — 1 bit (offset 12)
            #[inline(always)]
            pub const fn mwswcirxintstat(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswcirxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// AUDIOINT0STAT — 1 bit (offset 13)
            #[inline(always)]
            pub const fn audioint0stat(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint0stat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// AUDIOINT1STAT — 1 bit (offset 14)
            #[inline(always)]
            pub const fn audioint1stat(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint1stat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// AUDIOINT2STAT — 1 bit (offset 15)
            #[inline(always)]
            pub const fn audioint2stat(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint2stat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// ERRORINTSTAT — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintstat(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTACK0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intack0(pub u32);

        impl Intack0 {
            /// FRSYNCINTACK — 1 bit (offset 8)
            #[inline(always)]
            pub const fn frsyncintack(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_frsyncintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// MTOFFINT0ACK — 1 bit (offset 9)
            #[inline(always)]
            pub const fn mtoffint0ack(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mtoffint0ack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// MTOFFINT1ACK — 1 bit (offset 10)
            #[inline(always)]
            pub const fn mtoffint1ack(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mtoffint1ack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// MWSWCITXINTACK — 1 bit (offset 11)
            #[inline(always)]
            pub const fn mwswcitxintack(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswcitxintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// MWSWCIRXINTACK — 1 bit (offset 12)
            #[inline(always)]
            pub const fn mwswcirxintack(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswcirxintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// AUDIOINT0ACK — 1 bit (offset 13)
            #[inline(always)]
            pub const fn audioint0ack(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint0ack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// AUDIOINT1ACK — 1 bit (offset 14)
            #[inline(always)]
            pub const fn audioint1ack(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint1ack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// AUDIOINT2ACK — 1 bit (offset 15)
            #[inline(always)]
            pub const fn audioint2ack(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audioint2ack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// ERRORINTACK — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintack(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intcntl1(pub u32);

        impl Intcntl1 {
            /// CLKNINTMSK — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintmsk(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTMSK — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintmsk(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTMSK — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintmsk(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTMSK — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintmsk(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTMSK — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintmsk(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTMSK — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intmsk(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTMSK — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intmsk(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTMSK — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intmsk(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTMSK — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointmsk(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// CLKNINTSRVAL — 4 bits (offset 24)
            #[inline(always)]
            pub const fn clknintsrval(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_clknintsrval(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// CLKNINTSRMSK — 3 bits (offset 28)
            #[inline(always)]
            pub const fn clknintsrmsk(&self) -> u8 {
                ((self.0 >> 28) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_clknintsrmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 28)) | ((val as u32 & 0x7) << 28);
            }
        }

        /// INTSTAT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intstat1(pub u32);

        impl Intstat1 {
            /// CLKNINTSTAT — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintstat(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintstat(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTSTAT — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintstat(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTSTAT — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintstat(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTSTAT — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintstat(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTSTAT — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintstat(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTSTAT — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intstat(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTSTAT — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intstat(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTSTAT — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intstat(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTSTAT — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointstat(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }
        }

        /// INTACK1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intack1(pub u32);

        impl Intack1 {
            /// CLKNINTACK — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintack(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintack(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTACK — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintack(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTACK — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintack(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTACK — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintack(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTACK — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintack(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTACK — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intack(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTACK — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intack(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTACK — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intack(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTACK — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointack(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }
        }

        /// ACTFIFOSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Actfifostat(pub u32);

        impl Actfifostat {
            /// STARTACTINTSTAT — 1 bit (offset 0)
            #[inline(always)]
            pub const fn startactintstat(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_startactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// ENDACTINTSTAT — 1 bit (offset 1)
            #[inline(always)]
            pub const fn endactintstat(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_endactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SKIPACTINTSTAT — 1 bit (offset 2)
            #[inline(always)]
            pub const fn skipactintstat(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_skipactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// RXINTSTAT — 1 bit (offset 4)
            #[inline(always)]
            pub const fn rxintstat(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// ACTFLAG — 1 bit (offset 15)
            #[inline(always)]
            pub const fn actflag(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_actflag(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// CURRENT_ET_IDX — 4 bits (offset 24)
            #[inline(always)]
            pub const fn current_et_idx(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_current_et_idx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// SKIP_ET_IDX — 4 bits (offset 28)
            #[inline(always)]
            pub const fn skip_et_idx(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_skip_et_idx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// CURRENTRXDESCPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Currentrxdescptr(pub u32);

        impl Currentrxdescptr {
            /// CURRENTRXDESCPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn currentrxdescptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_currentrxdescptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// ETPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Etptr(pub u32);

        impl Etptr {
            /// ETPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn etptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_etptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// DEEPSLCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslcntl(pub u32);

        impl Deepslcntl {
            /// OSC_SLEEP_EN — 1 bit (offset 0)
            #[inline(always)]
            pub const fn osc_sleep_en(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_osc_sleep_en(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// RADIO_SLEEP_EN — 1 bit (offset 1)
            #[inline(always)]
            pub const fn radio_sleep_en(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_radio_sleep_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// DEEP_SLEEP_ON — 1 bit (offset 2)
            #[inline(always)]
            pub const fn deep_sleep_on(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_on(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// DEEP_SLEEP_CORR_EN — 1 bit (offset 3)
            #[inline(always)]
            pub const fn deep_sleep_corr_en(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_corr_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// DEEP_SLEEP_STAT — 1 bit (offset 15)
            #[inline(always)]
            pub const fn deep_sleep_stat(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_stat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// EXTWKUPDSB — 1 bit (offset 31)
            #[inline(always)]
            pub const fn extwkupdsb(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_extwkupdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DEEPSLWKUP
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslwkup(pub u32);

        impl Deepslwkup {
            /// DEEPSLTIME — 32 bits (offset 0)
            #[inline(always)]
            pub const fn deepsltime(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_deepsltime(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// DEEPSLSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslstat(pub u32);

        impl Deepslstat {
            /// DEEPSLDUR — 32 bits (offset 0)
            #[inline(always)]
            pub const fn deepsldur(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_deepsldur(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// ENBPRESET
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Enbpreset(pub u32);

        impl Enbpreset {
            /// TWRM — 10 bits (offset 0)
            #[inline(always)]
            pub const fn twrm(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_twrm(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }

            /// TWOSC — 11 bits (offset 10)
            #[inline(always)]
            pub const fn twosc(&self) -> u16 {
                ((self.0 >> 10) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_twosc(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 10)) | ((val as u32 & 0x7FF) << 10);
            }

            /// TWEXT — 11 bits (offset 21)
            #[inline(always)]
            pub const fn twext(&self) -> u16 {
                ((self.0 >> 21) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_twext(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 21)) | ((val as u32 & 0x7FF) << 21);
            }
        }

        /// FINECNTCORR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finecntcorr(pub u32);

        impl Finecntcorr {
            /// FINECNTCORR — 10 bits (offset 0)
            #[inline(always)]
            pub const fn finecntcorr(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_finecntcorr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNCNTCORR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkncntcorr(pub u32);

        impl Clkncntcorr {
            /// CLKNCNTCORR — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkncntcorr(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkncntcorr(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }

            /// ABS_DELTA — 1 bit (offset 31)
            #[inline(always)]
            pub const fn abs_delta(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abs_delta(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DIAGCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Diagcntl(pub u32);

        impl Diagcntl {
            /// DIAG0 — 7 bits (offset 0)
            #[inline(always)]
            pub const fn diag0(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_diag0(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// DIAG0_EN — 1 bit (offset 7)
            #[inline(always)]
            pub const fn diag0_en(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag0_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// DIAG1 — 7 bits (offset 8)
            #[inline(always)]
            pub const fn diag1(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_diag1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// DIAG1_EN — 1 bit (offset 15)
            #[inline(always)]
            pub const fn diag1_en(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag1_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// DIAG2 — 7 bits (offset 16)
            #[inline(always)]
            pub const fn diag2(&self) -> u8 {
                ((self.0 >> 16) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_diag2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 16)) | ((val as u32 & 0x7F) << 16);
            }

            /// DIAG2_EN — 1 bit (offset 23)
            #[inline(always)]
            pub const fn diag2_en(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag2_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }

            /// DIAG3 — 7 bits (offset 24)
            #[inline(always)]
            pub const fn diag3(&self) -> u8 {
                ((self.0 >> 24) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_diag3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 24)) | ((val as u32 & 0x7F) << 24);
            }

            /// DIAG3_EN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn diag3_en(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag3_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DIAGSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Diagstat(pub u32);

        impl Diagstat {
            /// DIAG0STAT — 8 bits (offset 0)
            #[inline(always)]
            pub const fn diag0stat(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag0stat(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// DIAG1STAT — 8 bits (offset 8)
            #[inline(always)]
            pub const fn diag1stat(&self) -> u8 {
                ((self.0 >> 8) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag1stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 8)) | ((val as u32 & 0xFF) << 8);
            }

            /// DIAG2STAT — 8 bits (offset 16)
            #[inline(always)]
            pub const fn diag2stat(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag2stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// DIAG3STAT — 8 bits (offset 24)
            #[inline(always)]
            pub const fn diag3stat(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag3stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// DEBUGADDMAX
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Debugaddmax(pub u32);

        impl Debugaddmax {
            /// EM_ADDMAX — 16 bits (offset 0)
            #[inline(always)]
            pub const fn em_addmax(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_em_addmax(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// REG_ADDMAX — 16 bits (offset 16)
            #[inline(always)]
            pub const fn reg_addmax(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_reg_addmax(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// DEBUGADDMIN
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Debugaddmin(pub u32);

        impl Debugaddmin {
            /// EM_ADDMIN — 16 bits (offset 0)
            #[inline(always)]
            pub const fn em_addmin(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_em_addmin(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// REG_ADDMIN — 16 bits (offset 16)
            #[inline(always)]
            pub const fn reg_addmin(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_reg_addmin(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// ERRORTYPESTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Errortypestat(pub u32);

        impl Errortypestat {
            /// TXCRYPT_ERROR — 1 bit (offset 0)
            #[inline(always)]
            pub const fn txcrypt_error(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txcrypt_error(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// RXCRYPT_ERROR — 1 bit (offset 1)
            #[inline(always)]
            pub const fn rxcrypt_error(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxcrypt_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTMODE_ERROR — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptmode_error(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptmode_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// PKTCNTL_EMACC_ERROR — 1 bit (offset 3)
            #[inline(always)]
            pub const fn pktcntl_emacc_error(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_pktcntl_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// RADIOCNTL_EMACC_ERROR — 1 bit (offset 4)
            #[inline(always)]
            pub const fn radiocntl_emacc_error(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_radiocntl_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// AUDIO_EMACC_ERROR — 1 bit (offset 5)
            #[inline(always)]
            pub const fn audio_emacc_error(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_audio_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// PCM_EMACC_ERROR — 1 bit (offset 6)
            #[inline(always)]
            pub const fn pcm_emacc_error(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_pcm_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// MWSCOEX_EMACC_ERROR — 1 bit (offset 7)
            #[inline(always)]
            pub const fn mwscoex_emacc_error(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwscoex_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// ACT_SCHDL_ENTRY_ERROR — 1 bit (offset 8)
            #[inline(always)]
            pub const fn act_schdl_entry_error(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_act_schdl_entry_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// ACT_SCHDL_APFM_ERROR — 1 bit (offset 9)
            #[inline(always)]
            pub const fn act_schdl_apfm_error(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_act_schdl_apfm_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// FRM_CNTL_APFM_ERROR — 1 bit (offset 10)
            #[inline(always)]
            pub const fn frm_cntl_apfm_error(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_frm_cntl_apfm_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// FRM_CNTL_EMACC_ERROR — 1 bit (offset 11)
            #[inline(always)]
            pub const fn frm_cntl_emacc_error(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_frm_cntl_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// FRM_CNTL_TIMER_ERROR — 1 bit (offset 12)
            #[inline(always)]
            pub const fn frm_cntl_timer_error(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_frm_cntl_timer_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// HOPUNDERRUN_ERROR — 1 bit (offset 13)
            #[inline(always)]
            pub const fn hopunderrun_error(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_hopunderrun_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// CHMAP_ERROR — 1 bit (offset 14)
            #[inline(always)]
            pub const fn chmap_error(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_chmap_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// CSFORMAT_ERROR — 1 bit (offset 15)
            #[inline(always)]
            pub const fn csformat_error(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_csformat_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// CSATTNB_ERROR — 1 bit (offset 16)
            #[inline(always)]
            pub const fn csattnb_error(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_csattnb_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// TXDESC_EMPTY_ERROR — 1 bit (offset 17)
            #[inline(always)]
            pub const fn txdesc_empty_error(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txdesc_empty_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// RXDESC_EMPTY_ERROR — 1 bit (offset 18)
            #[inline(always)]
            pub const fn rxdesc_empty_error(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxdesc_empty_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// TXBUF_PTR_ERROR — 1 bit (offset 19)
            #[inline(always)]
            pub const fn txbuf_ptr_error(&self) -> bool {
                (self.0 >> 19) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txbuf_ptr_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 19)) | ((val as u32) << 19);
            }

            /// RXBUF_PTR_ERROR — 1 bit (offset 20)
            #[inline(always)]
            pub const fn rxbuf_ptr_error(&self) -> bool {
                (self.0 >> 20) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxbuf_ptr_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 20)) | ((val as u32) << 20);
            }

            /// PEER_SAM_ERROR — 1 bit (offset 21)
            #[inline(always)]
            pub const fn peer_sam_error(&self) -> bool {
                (self.0 >> 21) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_peer_sam_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 21)) | ((val as u32) << 21);
            }

            /// LOCAL_SAM_ERROR — 1 bit (offset 22)
            #[inline(always)]
            pub const fn local_sam_error(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_local_sam_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// FIFOINTOVF — 1 bit (offset 23)
            #[inline(always)]
            pub const fn fifointovf(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointovf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }
        }

        /// SWPROFILING
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Swprofiling(pub u32);

        impl Swprofiling {
            /// SWPROF0 — 1 bit (offset 0)
            #[inline(always)]
            pub const fn swprof0(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof0(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SWPROF1 — 1 bit (offset 1)
            #[inline(always)]
            pub const fn swprof1(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SWPROF2 — 1 bit (offset 2)
            #[inline(always)]
            pub const fn swprof2(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWPROF3 — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swprof3(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// SWPROF4 — 1 bit (offset 4)
            #[inline(always)]
            pub const fn swprof4(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// SWPROF5 — 1 bit (offset 5)
            #[inline(always)]
            pub const fn swprof5(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// SWPROF6 — 1 bit (offset 6)
            #[inline(always)]
            pub const fn swprof6(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// SWPROF7 — 1 bit (offset 7)
            #[inline(always)]
            pub const fn swprof7(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// SWPROF8 — 1 bit (offset 8)
            #[inline(always)]
            pub const fn swprof8(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// SWPROF9 — 1 bit (offset 9)
            #[inline(always)]
            pub const fn swprof9(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// SWPROF10 — 1 bit (offset 10)
            #[inline(always)]
            pub const fn swprof10(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// SWPROF11 — 1 bit (offset 11)
            #[inline(always)]
            pub const fn swprof11(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// SWPROF12 — 1 bit (offset 12)
            #[inline(always)]
            pub const fn swprof12(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// SWPROF13 — 1 bit (offset 13)
            #[inline(always)]
            pub const fn swprof13(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// SWPROF14 — 1 bit (offset 14)
            #[inline(always)]
            pub const fn swprof14(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// SWPROF15 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn swprof15(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// SWPROF16 — 1 bit (offset 16)
            #[inline(always)]
            pub const fn swprof16(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof16(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// SWPROF17 — 1 bit (offset 17)
            #[inline(always)]
            pub const fn swprof17(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof17(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// SWPROF18 — 1 bit (offset 18)
            #[inline(always)]
            pub const fn swprof18(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof18(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// SWPROF19 — 1 bit (offset 19)
            #[inline(always)]
            pub const fn swprof19(&self) -> bool {
                (self.0 >> 19) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof19(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 19)) | ((val as u32) << 19);
            }

            /// SWPROF20 — 1 bit (offset 20)
            #[inline(always)]
            pub const fn swprof20(&self) -> bool {
                (self.0 >> 20) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof20(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 20)) | ((val as u32) << 20);
            }

            /// SWPROF21 — 1 bit (offset 21)
            #[inline(always)]
            pub const fn swprof21(&self) -> bool {
                (self.0 >> 21) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof21(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 21)) | ((val as u32) << 21);
            }

            /// SWPROF22 — 1 bit (offset 22)
            #[inline(always)]
            pub const fn swprof22(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof22(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// SWPROF23 — 1 bit (offset 23)
            #[inline(always)]
            pub const fn swprof23(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof23(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }

            /// SWPROF24 — 1 bit (offset 24)
            #[inline(always)]
            pub const fn swprof24(&self) -> bool {
                (self.0 >> 24) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof24(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 24)) | ((val as u32) << 24);
            }

            /// SWPROF25 — 1 bit (offset 25)
            #[inline(always)]
            pub const fn swprof25(&self) -> bool {
                (self.0 >> 25) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof25(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 25)) | ((val as u32) << 25);
            }

            /// SWPROF26 — 1 bit (offset 26)
            #[inline(always)]
            pub const fn swprof26(&self) -> bool {
                (self.0 >> 26) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof26(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 26)) | ((val as u32) << 26);
            }

            /// SWPROF27 — 1 bit (offset 27)
            #[inline(always)]
            pub const fn swprof27(&self) -> bool {
                (self.0 >> 27) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof27(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 27)) | ((val as u32) << 27);
            }

            /// SWPROF28 — 1 bit (offset 28)
            #[inline(always)]
            pub const fn swprof28(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof28(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// SWPROF29 — 1 bit (offset 29)
            #[inline(always)]
            pub const fn swprof29(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof29(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// SWPROF30 — 1 bit (offset 30)
            #[inline(always)]
            pub const fn swprof30(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof30(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// SWPROF31 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn swprof31(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof31(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// RADIOCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl0(pub u32);

        impl Radiocntl0 {
            /// SPICOMP — 1 bit (offset 1)
            #[inline(always)]
            pub const fn spicomp(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_spicomp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SPIFREQ — 2 bits (offset 4)
            #[inline(always)]
            pub const fn spifreq(&self) -> u8 {
                ((self.0 >> 4) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_spifreq(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 4)) | ((val as u32 & 0x3) << 4);
            }

            /// SPICFG — 1 bit (offset 7)
            #[inline(always)]
            pub const fn spicfg(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_spicfg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// SPIPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn spiptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_spiptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// RADIOCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl1(pub u32);

        impl Radiocntl1 {
            /// SUBVERSION — 4 bits (offset 0)
            #[inline(always)]
            pub const fn subversion(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_subversion(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// XRFSEL — 6 bits (offset 4)
            #[inline(always)]
            pub const fn xrfsel(&self) -> u8 {
                ((self.0 >> 4) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_xrfsel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 4)) | ((val as u32 & 0x3F) << 4);
            }

            /// JEF_SELECT — 1 bit (offset 12)
            #[inline(always)]
            pub const fn jef_select(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_jef_select(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// DPCORR_EN — 1 bit (offset 13)
            #[inline(always)]
            pub const fn dpcorr_en(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_dpcorr_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// SYNC_PULSE_SRC — 1 bit (offset 14)
            #[inline(always)]
            pub const fn sync_pulse_src(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sync_pulse_src(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// SYNC_PULSE_MODE — 1 bit (offset 15)
            #[inline(always)]
            pub const fn sync_pulse_mode(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sync_pulse_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// FORCEAGC_LENGTH — 12 bits (offset 16)
            #[inline(always)]
            pub const fn forceagc_length(&self) -> u16 {
                ((self.0 >> 16) & 0xFFF) as u16
            }

            #[inline(always)]
            pub fn set_forceagc_length(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFF << 16)) | ((val as u32 & 0xFFF) << 16);
            }

            /// TXDNSL — 1 bit (offset 28)
            #[inline(always)]
            pub const fn txdnsl(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txdnsl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// RXDNSL — 1 bit (offset 29)
            #[inline(always)]
            pub const fn rxdnsl(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxdnsl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// FORCEIQ — 1 bit (offset 30)
            #[inline(always)]
            pub const fn forceiq(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_forceiq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// FORCEAGC_EN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn forceagc_en(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_forceagc_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// RADIOCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl2(pub u32);

        impl Radiocntl2 {
            /// FREQTABLE_PTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn freqtable_ptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_freqtable_ptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// SYNCERR — 4 bits (offset 16)
            #[inline(always)]
            pub const fn syncerr(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_syncerr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// TRAILER_GATING_VAL — 3 bits (offset 24)
            #[inline(always)]
            pub const fn trailer_gating_val(&self) -> u8 {
                ((self.0 >> 24) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_trailer_gating_val(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 24)) | ((val as u32 & 0x7) << 24);
            }
        }

        /// RADIOCNTL3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl3(pub u32);

        impl Radiocntl3 {
            /// TXVALID_BEH — 2 bits (offset 0)
            #[inline(always)]
            pub const fn txvalid_beh(&self) -> u8 {
                (self.0 & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txvalid_beh(&mut self, val: u8) {
                self.0 = (self.0 & !0x3) | (val as u32 & 0x3);
            }

            /// TX_SERPAR_IF — 1 bit (offset 2)
            #[inline(always)]
            pub const fn tx_serpar_if(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_tx_serpar_if(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// TXRATE0CFG — 2 bits (offset 8)
            #[inline(always)]
            pub const fn txrate0cfg(&self) -> u8 {
                ((self.0 >> 8) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txrate0cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 8)) | ((val as u32 & 0x3) << 8);
            }

            /// TXRATE1CFG — 2 bits (offset 10)
            #[inline(always)]
            pub const fn txrate1cfg(&self) -> u8 {
                ((self.0 >> 10) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txrate1cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 10)) | ((val as u32 & 0x3) << 10);
            }

            /// TXRATE2CFG — 2 bits (offset 12)
            #[inline(always)]
            pub const fn txrate2cfg(&self) -> u8 {
                ((self.0 >> 12) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txrate2cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 12)) | ((val as u32 & 0x3) << 12);
            }

            /// RXVALID_BEH — 2 bits (offset 16)
            #[inline(always)]
            pub const fn rxvalid_beh(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxvalid_beh(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// RXSYNC_ROUTING — 1 bit (offset 18)
            #[inline(always)]
            pub const fn rxsync_routing(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxsync_routing(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// RX_SERPAR_IF — 1 bit (offset 19)
            #[inline(always)]
            pub const fn rx_serpar_if(&self) -> bool {
                (self.0 >> 19) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rx_serpar_if(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 19)) | ((val as u32) << 19);
            }

            /// GETRSSIDELAY — 3 bits (offset 20)
            #[inline(always)]
            pub const fn getrssidelay(&self) -> u8 {
                ((self.0 >> 20) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_getrssidelay(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 20)) | ((val as u32 & 0x7) << 20);
            }

            /// RXRATE0CFG — 2 bits (offset 24)
            #[inline(always)]
            pub const fn rxrate0cfg(&self) -> u8 {
                ((self.0 >> 24) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxrate0cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 24)) | ((val as u32 & 0x3) << 24);
            }

            /// RXRATE1CFG — 2 bits (offset 26)
            #[inline(always)]
            pub const fn rxrate1cfg(&self) -> u8 {
                ((self.0 >> 26) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxrate1cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 26)) | ((val as u32 & 0x3) << 26);
            }

            /// RXRATE2CFG — 2 bits (offset 28)
            #[inline(always)]
            pub const fn rxrate2cfg(&self) -> u8 {
                ((self.0 >> 28) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxrate2cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 28)) | ((val as u32 & 0x3) << 28);
            }
        }

        /// RADIOPWRUPDN
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiopwrupdn(pub u32);

        impl Radiopwrupdn {
            /// TXPWRUPCT — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txpwrupct(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txpwrupct(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// TXPWRDNCT — 7 bits (offset 8)
            #[inline(always)]
            pub const fn txpwrdnct(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpwrdnct(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// RXPWRUPCT — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rxpwrupct(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxpwrupct(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }
        }

        /// RADIOTXRXTIM
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiotxrxtim(pub u32);

        impl Radiotxrxtim {
            /// TXPATHDLY — 7 bits (offset 0)
            #[inline(always)]
            pub const fn txpathdly(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpathdly(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// RXPATHDLY — 7 bits (offset 8)
            #[inline(always)]
            pub const fn rxpathdly(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rxpathdly(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// SYNC_POSITION — 8 bits (offset 24)
            #[inline(always)]
            pub const fn sync_position(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_sync_position(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// SPIPTRCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Spiptrcntl0(pub u32);

        impl Spiptrcntl0 {
            /// TXONPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn txonptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_txonptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// TXOFFPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn txoffptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_txoffptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// SPIPTRCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Spiptrcntl1(pub u32);

        impl Spiptrcntl1 {
            /// RXONPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn rxonptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rxonptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// RXOFFPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn rxoffptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rxoffptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// SPIPTRCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Spiptrcntl2(pub u32);

        impl Spiptrcntl2 {
            /// RSSIPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn rssiptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rssiptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// RXLENGTHPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn rxlengthptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rxlengthptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// SPIPTRCNTL3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Spiptrcntl3(pub u32);

        impl Spiptrcntl3 {
            /// RXPKTTYPPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn rxpkttypptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rxpkttypptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// AESCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aescntl(pub u32);

        impl Aescntl {
            /// AES_START — 1 bit (offset 0)
            #[inline(always)]
            pub const fn aes_start(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aes_start(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// AES_MODE — 1 bit (offset 1)
            #[inline(always)]
            pub const fn aes_mode(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aes_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }
        }

        /// AESKEY31_0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey310(pub u32);

        impl Aeskey310 {
            /// AESKEY31_0 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey31_0(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey31_0(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY63_32
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey6332(pub u32);

        impl Aeskey6332 {
            /// AESKEY63_32 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey63_32(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey63_32(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY95_64
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey9564(pub u32);

        impl Aeskey9564 {
            /// AESKEY95_64 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey95_64(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey95_64(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY127_96
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey12796(pub u32);

        impl Aeskey12796 {
            /// AESKEY127_96 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey127_96(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey127_96(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aesptr(pub u32);

        impl Aesptr {
            /// AESPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn aesptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_aesptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// TXMICVAL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Txmicval(pub u32);

        impl Txmicval {
            /// TXMICVAL — 32 bits (offset 0)
            #[inline(always)]
            pub const fn txmicval(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_txmicval(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// RXMICVAL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rxmicval(pub u32);

        impl Rxmicval {
            /// RXMICVAL — 32 bits (offset 0)
            #[inline(always)]
            pub const fn rxmicval(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_rxmicval(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// RFTESTCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rftestcntl(pub u32);

        impl Rftestcntl {
            /// TXPKTCNTEN — 1 bit (offset 11)
            #[inline(always)]
            pub const fn txpktcnten(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txpktcnten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// TXPLDSRC — 1 bit (offset 12)
            #[inline(always)]
            pub const fn txpldsrc(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txpldsrc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// PRBSTYPE — 1 bit (offset 13)
            #[inline(always)]
            pub const fn prbstype(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_prbstype(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// INFINITETX — 1 bit (offset 15)
            #[inline(always)]
            pub const fn infinitetx(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_infinitetx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// HERRREN — 1 bit (offset 16)
            #[inline(always)]
            pub const fn herrren(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_herrren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// SSERRREN — 1 bit (offset 17)
            #[inline(always)]
            pub const fn sserrren(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sserrren(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// PERCOUNT_MODE — 3 bits (offset 24)
            #[inline(always)]
            pub const fn percount_mode(&self) -> u8 {
                ((self.0 >> 24) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_percount_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 24)) | ((val as u32 & 0x7) << 24);
            }

            /// RXPKTCNTEN — 1 bit (offset 27)
            #[inline(always)]
            pub const fn rxpktcnten(&self) -> bool {
                (self.0 >> 27) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxpktcnten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 27)) | ((val as u32) << 27);
            }

            /// INFINITERX — 1 bit (offset 31)
            #[inline(always)]
            pub const fn infiniterx(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_infiniterx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// RFTESTFREQ
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rftestfreq(pub u32);

        impl Rftestfreq {
            /// TXFREQ — 7 bits (offset 0)
            #[inline(always)]
            pub const fn txfreq(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txfreq(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// RXFREQ — 7 bits (offset 8)
            #[inline(always)]
            pub const fn rxfreq(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rxfreq(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// TESTMODEEN — 1 bit (offset 16)
            #[inline(always)]
            pub const fn testmodeen(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_testmodeen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// DIRECTLOOPBACKEN — 1 bit (offset 17)
            #[inline(always)]
            pub const fn directloopbacken(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_directloopbacken(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// LOOPBACKMODE — 1 bit (offset 18)
            #[inline(always)]
            pub const fn loopbackmode(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_loopbackmode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }
        }

        /// RFTESTTXSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rftesttxstat(pub u32);

        impl Rftesttxstat {
            /// TXPKTCNT — 32 bits (offset 0)
            #[inline(always)]
            pub const fn txpktcnt(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_txpktcnt(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// RFTESTRXSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rftestrxstat(pub u32);

        impl Rftestrxstat {
            /// RXPKTCNT — 32 bits (offset 0)
            #[inline(always)]
            pub const fn rxpktcnt(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_rxpktcnt(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// TIMGENCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Timgencntl(pub u32);

        impl Timgencntl {
            /// PREFETCH_TIME — 9 bits (offset 0)
            #[inline(always)]
            pub const fn prefetch_time(&self) -> u16 {
                (self.0 & 0x1FF) as u16
            }

            #[inline(always)]
            pub fn set_prefetch_time(&mut self, val: u16) {
                self.0 = (self.0 & !0x1FF) | (val as u32 & 0x1FF);
            }

            /// PREFETCHABORT_TIME — 10 bits (offset 16)
            #[inline(always)]
            pub const fn prefetchabort_time(&self) -> u16 {
                ((self.0 >> 16) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_prefetchabort_time(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 16)) | ((val as u32 & 0x3FF) << 16);
            }
        }

        /// FINETIMTGT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finetimtgt(pub u32);

        impl Finetimtgt {
            /// FINETARGET — 28 bits (offset 0)
            #[inline(always)]
            pub const fn finetarget(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_finetarget(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// CLKNTGT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt1(pub u32);

        impl Clkntgt1 {
            /// CLKNTGT1 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt1(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt1(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt1(pub u32);

        impl Hmicrosectgt1 {
            /// HMICROSECTGT1 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt1(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt1(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNTGT2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt2(pub u32);

        impl Clkntgt2 {
            /// CLKNTGT2 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt2(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt2(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt2(pub u32);

        impl Hmicrosectgt2 {
            /// HMICROSECTGT2 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt2(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt2(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNTGT3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt3(pub u32);

        impl Clkntgt3 {
            /// CLKNTGT3 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt3(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt3(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt3(pub u32);

        impl Hmicrosectgt3 {
            /// HMICROSECTGT3 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt3(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt3(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// SLOTCLK
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Slotclk(pub u32);

        impl Slotclk {
            /// SCLK — 28 bits (offset 0)
            #[inline(always)]
            pub const fn sclk(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_sclk(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }

            /// CLKN_UPD — 1 bit (offset 30)
            #[inline(always)]
            pub const fn clkn_upd(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clkn_upd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// SAMP — 1 bit (offset 31)
            #[inline(always)]
            pub const fn samp(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_samp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// FINETIMECNT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finetimecnt(pub u32);

        impl Finetimecnt {
            /// FINECNT — 10 bits (offset 0)
            #[inline(always)]
            pub const fn finecnt(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_finecnt(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// ACTSCHCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Actschcntl(pub u32);

        impl Actschcntl {
            /// ENTRY_IDX — 4 bits (offset 0)
            #[inline(always)]
            pub const fn entry_idx(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_entry_idx(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// START_ACT — 1 bit (offset 31)
            #[inline(always)]
            pub const fn start_act(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_start_act(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// STARTFRMCLKNTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Startfrmclknts(pub u32);

        impl Startfrmclknts {
            /// STARTFRMCLKNTS — 28 bits (offset 0)
            #[inline(always)]
            pub const fn startfrmclknts(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_startfrmclknts(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// STARTFRMFINECNTTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Startfrmfinecntts(pub u32);

        impl Startfrmfinecntts {
            /// STARTFRMFINECNTTS — 10 bits (offset 0)
            #[inline(always)]
            pub const fn startfrmfinecntts(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_startfrmfinecntts(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// ENDFRMCLKNTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Endfrmclknts(pub u32);

        impl Endfrmclknts {
            /// ENDFRMCLKNTS — 28 bits (offset 0)
            #[inline(always)]
            pub const fn endfrmclknts(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_endfrmclknts(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// ENDFRMFINECNTTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Endfrmfinecntts(pub u32);

        impl Endfrmfinecntts {
            /// ENDFRMFINECNTTS — 10 bits (offset 0)
            #[inline(always)]
            pub const fn endfrmfinecntts(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_endfrmfinecntts(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// SKIPFRMCLKNTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Skipfrmclknts(pub u32);

        impl Skipfrmclknts {
            /// SKIPFRMPCLKNTS — 28 bits (offset 0)
            #[inline(always)]
            pub const fn skipfrmpclknts(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_skipfrmpclknts(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// SKIPFRMFINECNTTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Skipfrmfinecntts(pub u32);

        impl Skipfrmfinecntts {
            /// SKIPFRMFINECNTTS — 10 bits (offset 0)
            #[inline(always)]
            pub const fn skipfrmfinecntts(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_skipfrmfinecntts(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// ABTRAINCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Abtraincntl(pub u32);

        impl Abtraincntl {
            /// ABTINQTIME — 11 bits (offset 0)
            #[inline(always)]
            pub const fn abtinqtime(&self) -> u16 {
                (self.0 & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_abtinqtime(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FF) | (val as u32 & 0x7FF);
            }

            /// ABTINQLOAD — 1 bit (offset 12)
            #[inline(always)]
            pub const fn abtinqload(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abtinqload(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// ABTINQSTARTVALUE — 1 bit (offset 14)
            #[inline(always)]
            pub const fn abtinqstartvalue(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abtinqstartvalue(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// ABTINQEN — 1 bit (offset 15)
            #[inline(always)]
            pub const fn abtinqen(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abtinqen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// ABTPAGETIME — 11 bits (offset 16)
            #[inline(always)]
            pub const fn abtpagetime(&self) -> u16 {
                ((self.0 >> 16) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_abtpagetime(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 16)) | ((val as u32 & 0x7FF) << 16);
            }

            /// ABTPAGELOAD — 1 bit (offset 28)
            #[inline(always)]
            pub const fn abtpageload(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abtpageload(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// ABTPAGESTARTVALUE — 1 bit (offset 30)
            #[inline(always)]
            pub const fn abtpagestartvalue(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abtpagestartvalue(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// ABTPAGEEN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn abtpageen(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abtpageen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// EDRCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Edrcntl(pub u32);

        impl Edrcntl {
            /// RXGRD_TIMEOUT — 6 bits (offset 0)
            #[inline(always)]
            pub const fn rxgrd_timeout(&self) -> u8 {
                (self.0 & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_rxgrd_timeout(&mut self, val: u8) {
                self.0 = (self.0 & !0x3F) | (val as u32 & 0x3F);
            }

            /// GB_TXQUAL_GEN_DSB — 1 bit (offset 6)
            #[inline(always)]
            pub const fn gb_txqual_gen_dsb(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_gb_txqual_gen_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// RXGUARDDSB — 1 bit (offset 7)
            #[inline(always)]
            pub const fn rxguarddsb(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxguarddsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// GUARD_BAND_TIME — 3 bits (offset 8)
            #[inline(always)]
            pub const fn guard_band_time(&self) -> u8 {
                ((self.0 >> 8) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_guard_band_time(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 8)) | ((val as u32 & 0x7) << 8);
            }

            /// TX_SWAP — 1 bit (offset 12)
            #[inline(always)]
            pub const fn tx_swap(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_tx_swap(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// RX_SWAP — 1 bit (offset 13)
            #[inline(always)]
            pub const fn rx_swap(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rx_swap(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// EDRBCAST — 1 bit (offset 15)
            #[inline(always)]
            pub const fn edrbcast(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_edrbcast(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// TXRATE_SWINSTANT — 1 bit (offset 16)
            #[inline(always)]
            pub const fn txrate_swinstant(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txrate_swinstant(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// PCACNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcacntl0(pub u32);

        impl Pcacntl0 {
            /// PHASE_SHIFT_EN — 1 bit (offset 0)
            #[inline(always)]
            pub const fn phase_shift_en(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_phase_shift_en(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SYNC_SOURCE — 1 bit (offset 1)
            #[inline(always)]
            pub const fn sync_source(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sync_source(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// FRSYNC_POL — 1 bit (offset 2)
            #[inline(always)]
            pub const fn frsync_pol(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_frsync_pol(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// BLINDCORR_EN — 1 bit (offset 3)
            #[inline(always)]
            pub const fn blindcorr_en(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_blindcorr_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// CORR_STEP — 4 bits (offset 4)
            #[inline(always)]
            pub const fn corr_step(&self) -> u8 {
                ((self.0 >> 4) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_corr_step(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 4)) | ((val as u32 & 0xF) << 4);
            }

            /// SLVLBL — 5 bits (offset 8)
            #[inline(always)]
            pub const fn slvlbl(&self) -> u8 {
                ((self.0 >> 8) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_slvlbl(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 8)) | ((val as u32 & 0x1F) << 8);
            }

            /// TARGET_OFFSET — 11 bits (offset 16)
            #[inline(always)]
            pub const fn target_offset(&self) -> u16 {
                ((self.0 >> 16) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_target_offset(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 16)) | ((val as u32 & 0x7FF) << 16);
            }
        }

        /// PCACNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcacntl1(pub u32);

        impl Pcacntl1 {
            /// CLOCK_SHIFT — 11 bits (offset 0)
            #[inline(always)]
            pub const fn clock_shift(&self) -> u16 {
                (self.0 & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_clock_shift(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FF) | (val as u32 & 0x7FF);
            }

            /// CLOCK_SHIFT_EN — 1 bit (offset 12)
            #[inline(always)]
            pub const fn clock_shift_en(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clock_shift_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// CORR_INTERVAL — 8 bits (offset 16)
            #[inline(always)]
            pub const fn corr_interval(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_corr_interval(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }
        }

        /// PCASTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcastat(pub u32);

        impl Pcastat {
            /// MOMENT_OFFSET — 11 bits (offset 0)
            #[inline(always)]
            pub const fn moment_offset(&self) -> u16 {
                (self.0 & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_moment_offset(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FF) | (val as u32 & 0x7FF);
            }

            /// SHIFT_PHASE — 11 bits (offset 16)
            #[inline(always)]
            pub const fn shift_phase(&self) -> u16 {
                ((self.0 >> 16) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_shift_phase(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 16)) | ((val as u32 & 0x7FF) << 16);
            }
        }

        /// COEXIFCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Coexifcntl0(pub u32);

        impl Coexifcntl0 {
            /// WLANCOEX_EN — 1 bit (offset 0)
            #[inline(always)]
            pub const fn wlancoex_en(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_wlancoex_en(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SYNCGEN_EN — 1 bit (offset 1)
            #[inline(always)]
            pub const fn syncgen_en(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_syncgen_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// MWSCOEX_EN — 1 bit (offset 2)
            #[inline(always)]
            pub const fn mwscoex_en(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwscoex_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// MWSWCI_EN — 1 bit (offset 3)
            #[inline(always)]
            pub const fn mwswci_en(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswci_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// WLANRXMSK — 2 bits (offset 4)
            #[inline(always)]
            pub const fn wlanrxmsk(&self) -> u8 {
                ((self.0 >> 4) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_wlanrxmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 4)) | ((val as u32 & 0x3) << 4);
            }

            /// WLANTXMSK — 2 bits (offset 6)
            #[inline(always)]
            pub const fn wlantxmsk(&self) -> u8 {
                ((self.0 >> 6) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_wlantxmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 6)) | ((val as u32 & 0x3) << 6);
            }

            /// MWSRXMSK — 2 bits (offset 8)
            #[inline(always)]
            pub const fn mwsrxmsk(&self) -> u8 {
                ((self.0 >> 8) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwsrxmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 8)) | ((val as u32 & 0x3) << 8);
            }

            /// MWSTXMSK — 2 bits (offset 10)
            #[inline(always)]
            pub const fn mwstxmsk(&self) -> u8 {
                ((self.0 >> 10) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwstxmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 10)) | ((val as u32 & 0x3) << 10);
            }

            /// MWSRXFRQMSK — 2 bits (offset 12)
            #[inline(always)]
            pub const fn mwsrxfrqmsk(&self) -> u8 {
                ((self.0 >> 12) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwsrxfrqmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 12)) | ((val as u32 & 0x3) << 12);
            }

            /// MWSTXFRQMSK — 2 bits (offset 14)
            #[inline(always)]
            pub const fn mwstxfrqmsk(&self) -> u8 {
                ((self.0 >> 14) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwstxfrqmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 14)) | ((val as u32 & 0x3) << 14);
            }

            /// WLCTXPRIOMODE — 2 bits (offset 16)
            #[inline(always)]
            pub const fn wlctxpriomode(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_wlctxpriomode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// WLCRXPRIOMODE — 2 bits (offset 18)
            #[inline(always)]
            pub const fn wlcrxpriomode(&self) -> u8 {
                ((self.0 >> 18) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_wlcrxpriomode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 18)) | ((val as u32 & 0x3) << 18);
            }

            /// MWSSCANFREQMSK — 2 bits (offset 20)
            #[inline(always)]
            pub const fn mwsscanfreqmsk(&self) -> u8 {
                ((self.0 >> 20) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwsscanfreqmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 20)) | ((val as u32 & 0x3) << 20);
            }

            /// PAGEKNUDGEINC — 4 bits (offset 24)
            #[inline(always)]
            pub const fn pageknudgeinc(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_pageknudgeinc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// INQKNUDGEINC — 4 bits (offset 28)
            #[inline(always)]
            pub const fn inqknudgeinc(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_inqknudgeinc(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// COEXIFCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Coexifcntl1(pub u32);

        impl Coexifcntl1 {
            /// WLCPDELAY — 7 bits (offset 0)
            #[inline(always)]
            pub const fn wlcpdelay(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_wlcpdelay(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// WLCPDURATION — 7 bits (offset 8)
            #[inline(always)]
            pub const fn wlcpduration(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_wlcpduration(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// WLCPTXTHR — 5 bits (offset 16)
            #[inline(always)]
            pub const fn wlcptxthr(&self) -> u8 {
                ((self.0 >> 16) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_wlcptxthr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 16)) | ((val as u32 & 0x1F) << 16);
            }

            /// WLCPRXTHR — 5 bits (offset 24)
            #[inline(always)]
            pub const fn wlcprxthr(&self) -> u8 {
                ((self.0 >> 24) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_wlcprxthr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 24)) | ((val as u32 & 0x1F) << 24);
            }
        }

        /// COEXIFCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Coexifcntl2(pub u32);

        impl Coexifcntl2 {
            /// TX_ANT_DELAY — 4 bits (offset 0)
            #[inline(always)]
            pub const fn tx_ant_delay(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_tx_ant_delay(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// RX_ANT_DELAY — 4 bits (offset 8)
            #[inline(always)]
            pub const fn rx_ant_delay(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_rx_ant_delay(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }
        }

        /// BTMPRIO0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Btmprio0(pub u32);

        impl Btmprio0 {
            /// BTM0 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn btm0(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm0(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// BTM1 — 4 bits (offset 4)
            #[inline(always)]
            pub const fn btm1(&self) -> u8 {
                ((self.0 >> 4) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 4)) | ((val as u32 & 0xF) << 4);
            }

            /// BTM2 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn btm2(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// BTM3 — 4 bits (offset 12)
            #[inline(always)]
            pub const fn btm3(&self) -> u8 {
                ((self.0 >> 12) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm3(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 12)) | ((val as u32 & 0xF) << 12);
            }

            /// BTM4 — 4 bits (offset 16)
            #[inline(always)]
            pub const fn btm4(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm4(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// BTM5 — 4 bits (offset 20)
            #[inline(always)]
            pub const fn btm5(&self) -> u8 {
                ((self.0 >> 20) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm5(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 20)) | ((val as u32 & 0xF) << 20);
            }

            /// BTM6 — 4 bits (offset 24)
            #[inline(always)]
            pub const fn btm6(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm6(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// BTM7 — 4 bits (offset 28)
            #[inline(always)]
            pub const fn btm7(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm7(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// BTMPRIO1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Btmprio1(pub u32);

        impl Btmprio1 {
            /// BTM8 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn btm8(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm8(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// BTM9 — 4 bits (offset 4)
            #[inline(always)]
            pub const fn btm9(&self) -> u8 {
                ((self.0 >> 4) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm9(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 4)) | ((val as u32 & 0xF) << 4);
            }

            /// BTM10 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn btm10(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm10(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// BTM11 — 4 bits (offset 12)
            #[inline(always)]
            pub const fn btm11(&self) -> u8 {
                ((self.0 >> 12) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm11(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 12)) | ((val as u32 & 0xF) << 12);
            }

            /// BTM12 — 4 bits (offset 16)
            #[inline(always)]
            pub const fn btm12(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm12(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// BTM13 — 4 bits (offset 20)
            #[inline(always)]
            pub const fn btm13(&self) -> u8 {
                ((self.0 >> 20) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm13(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 20)) | ((val as u32 & 0xF) << 20);
            }

            /// BTM14 — 4 bits (offset 24)
            #[inline(always)]
            pub const fn btm14(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm14(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// BTM15 — 4 bits (offset 28)
            #[inline(always)]
            pub const fn btm15(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm15(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// BTMPRIO2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Btmprio2(pub u32);

        impl Btmprio2 {
            /// BTM16 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn btm16(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm16(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// BTM17 — 4 bits (offset 4)
            #[inline(always)]
            pub const fn btm17(&self) -> u8 {
                ((self.0 >> 4) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm17(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 4)) | ((val as u32 & 0xF) << 4);
            }

            /// BTM18 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn btm18(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm18(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// BTM19 — 4 bits (offset 12)
            #[inline(always)]
            pub const fn btm19(&self) -> u8 {
                ((self.0 >> 12) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm19(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 12)) | ((val as u32 & 0xF) << 12);
            }

            /// BTM20 — 4 bits (offset 16)
            #[inline(always)]
            pub const fn btm20(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btm20(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// BTMDEFAULT — 4 bits (offset 28)
            #[inline(always)]
            pub const fn btmdefault(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_btmdefault(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// MWSPTABLE0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptable0(pub u32);

        impl Mwsptable0 {
            /// MWSPAT0_0 — 3 bits (offset 0)
            #[inline(always)]
            pub const fn mwspat0_0(&self) -> u8 {
                (self.0 & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat0_0(&mut self, val: u8) {
                self.0 = (self.0 & !0x7) | (val as u32 & 0x7);
            }

            /// MWSPAT0_1 — 3 bits (offset 4)
            #[inline(always)]
            pub const fn mwspat0_1(&self) -> u8 {
                ((self.0 >> 4) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat0_1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 4)) | ((val as u32 & 0x7) << 4);
            }

            /// MWSPAT0_2 — 3 bits (offset 8)
            #[inline(always)]
            pub const fn mwspat0_2(&self) -> u8 {
                ((self.0 >> 8) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat0_2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 8)) | ((val as u32 & 0x7) << 8);
            }

            /// MWSPAT0_3 — 3 bits (offset 12)
            #[inline(always)]
            pub const fn mwspat0_3(&self) -> u8 {
                ((self.0 >> 12) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat0_3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 12)) | ((val as u32 & 0x7) << 12);
            }

            /// MWSPAT0_4 — 3 bits (offset 16)
            #[inline(always)]
            pub const fn mwspat0_4(&self) -> u8 {
                ((self.0 >> 16) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat0_4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 16)) | ((val as u32 & 0x7) << 16);
            }

            /// MWSPAT0_5 — 3 bits (offset 20)
            #[inline(always)]
            pub const fn mwspat0_5(&self) -> u8 {
                ((self.0 >> 20) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat0_5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 20)) | ((val as u32 & 0x7) << 20);
            }
        }

        /// MWSPTIMING00
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming00(pub u32);

        impl Mwsptiming00 {
            /// MWSPTIM0_0_TO_1 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim0_0_to_1(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim0_0_to_1(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSPTIM0_1_TO_2 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsptim0_1_to_2(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim0_1_to_2(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSPTIMING01
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming01(pub u32);

        impl Mwsptiming01 {
            /// MWSPTIM0_2_TO_3 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim0_2_to_3(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim0_2_to_3(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSPTIM0_3_TO_4 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsptim0_3_to_4(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim0_3_to_4(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSPTIMING02
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming02(pub u32);

        impl Mwsptiming02 {
            /// MWSPTIM0_4_TO_5 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim0_4_to_5(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim0_4_to_5(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }
        }

        /// MWSPTABLE1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptable1(pub u32);

        impl Mwsptable1 {
            /// MWSPAT1_0 — 3 bits (offset 0)
            #[inline(always)]
            pub const fn mwspat1_0(&self) -> u8 {
                (self.0 & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat1_0(&mut self, val: u8) {
                self.0 = (self.0 & !0x7) | (val as u32 & 0x7);
            }

            /// MWSPAT1_1 — 3 bits (offset 4)
            #[inline(always)]
            pub const fn mwspat1_1(&self) -> u8 {
                ((self.0 >> 4) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat1_1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 4)) | ((val as u32 & 0x7) << 4);
            }

            /// MWSPAT1_2 — 3 bits (offset 8)
            #[inline(always)]
            pub const fn mwspat1_2(&self) -> u8 {
                ((self.0 >> 8) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat1_2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 8)) | ((val as u32 & 0x7) << 8);
            }

            /// MWSPAT1_3 — 3 bits (offset 12)
            #[inline(always)]
            pub const fn mwspat1_3(&self) -> u8 {
                ((self.0 >> 12) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat1_3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 12)) | ((val as u32 & 0x7) << 12);
            }

            /// MWSPAT1_4 — 3 bits (offset 16)
            #[inline(always)]
            pub const fn mwspat1_4(&self) -> u8 {
                ((self.0 >> 16) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat1_4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 16)) | ((val as u32 & 0x7) << 16);
            }

            /// MWSPAT1_5 — 3 bits (offset 20)
            #[inline(always)]
            pub const fn mwspat1_5(&self) -> u8 {
                ((self.0 >> 20) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat1_5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 20)) | ((val as u32 & 0x7) << 20);
            }
        }

        /// MWSPTIMING10
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming10(pub u32);

        impl Mwsptiming10 {
            /// MWSPTIM1_0_TO_1 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim1_0_to_1(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim1_0_to_1(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSPTIM1_1_TO_2 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsptim1_1_to_2(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim1_1_to_2(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSPTIMING11
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming11(pub u32);

        impl Mwsptiming11 {
            /// MWSPTIM1_2_TO_3 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim1_2_to_3(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim1_2_to_3(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSPTIM1_3_TO_4 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsptim1_3_to_4(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim1_3_to_4(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSPTIMING12
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming12(pub u32);

        impl Mwsptiming12 {
            /// MWSPTIM1_4_TO_5 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim1_4_to_5(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim1_4_to_5(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }
        }

        /// MWSPTABLE2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptable2(pub u32);

        impl Mwsptable2 {
            /// MWSPAT2_0 — 3 bits (offset 0)
            #[inline(always)]
            pub const fn mwspat2_0(&self) -> u8 {
                (self.0 & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat2_0(&mut self, val: u8) {
                self.0 = (self.0 & !0x7) | (val as u32 & 0x7);
            }

            /// MWSPAT2_1 — 3 bits (offset 4)
            #[inline(always)]
            pub const fn mwspat2_1(&self) -> u8 {
                ((self.0 >> 4) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat2_1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 4)) | ((val as u32 & 0x7) << 4);
            }

            /// MWSPAT2_2 — 3 bits (offset 8)
            #[inline(always)]
            pub const fn mwspat2_2(&self) -> u8 {
                ((self.0 >> 8) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat2_2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 8)) | ((val as u32 & 0x7) << 8);
            }

            /// MWSPAT2_3 — 3 bits (offset 12)
            #[inline(always)]
            pub const fn mwspat2_3(&self) -> u8 {
                ((self.0 >> 12) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat2_3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 12)) | ((val as u32 & 0x7) << 12);
            }

            /// MWSPAT2_4 — 3 bits (offset 16)
            #[inline(always)]
            pub const fn mwspat2_4(&self) -> u8 {
                ((self.0 >> 16) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat2_4(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 16)) | ((val as u32 & 0x7) << 16);
            }

            /// MWSPAT2_5 — 3 bits (offset 20)
            #[inline(always)]
            pub const fn mwspat2_5(&self) -> u8 {
                ((self.0 >> 20) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwspat2_5(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 20)) | ((val as u32 & 0x7) << 20);
            }
        }

        /// MWSPTIMING20
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming20(pub u32);

        impl Mwsptiming20 {
            /// MWSPTIM2_0_TO_1 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim2_0_to_1(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim2_0_to_1(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSPTIM2_1_TO_2 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsptim2_1_to_2(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim2_1_to_2(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSPTIMING21
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming21(pub u32);

        impl Mwsptiming21 {
            /// MWSPTIM2_2_TO_3 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim2_2_to_3(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim2_2_to_3(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSPTIM2_3_TO_4 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsptim2_3_to_4(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim2_3_to_4(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSPTIMING22
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsptiming22(pub u32);

        impl Mwsptiming22 {
            /// MWSPTIM2_4_TO_5 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsptim2_4_to_5(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsptim2_4_to_5(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }
        }

        /// MWSIFSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsifstat(pub u32);

        impl Mwsifstat {
            /// MWS_PATTERN_VAL — 2 bits (offset 0)
            #[inline(always)]
            pub const fn mws_pattern_val(&self) -> u8 {
                (self.0 & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mws_pattern_val(&mut self, val: u8) {
                self.0 = (self.0 & !0x3) | (val as u32 & 0x3);
            }

            /// MWS_PATTERN_START_IND_VAL — 1 bit (offset 2)
            #[inline(always)]
            pub const fn mws_pattern_start_ind_val(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mws_pattern_start_ind_val(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// MWS_INACTIVITY_DURATION_VAL — 5 bits (offset 4)
            #[inline(always)]
            pub const fn mws_inactivity_duration_val(&self) -> u8 {
                ((self.0 >> 4) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_mws_inactivity_duration_val(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 4)) | ((val as u32 & 0x1F) << 4);
            }

            /// MWS_IF_SAMP — 1 bit (offset 31)
            #[inline(always)]
            pub const fn mws_if_samp(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mws_if_samp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// MWSTXTABLE0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwstxtable0(pub u32);

        impl Mwstxtable0 {
            /// MWSTXFREQL — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwstxfreql(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwstxfreql(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSTXFREQH — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwstxfreqh(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwstxfreqh(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSRXTABLE0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsrxtable0(pub u32);

        impl Mwsrxtable0 {
            /// MWSRXFREQL — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsrxfreql(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsrxfreql(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSRXFREQH — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsrxfreqh(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsrxfreqh(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSSFTABLE1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwssftable1(pub u32);

        impl Mwssftable1 {
            /// MWSSCANFREQL1 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsscanfreql1(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreql1(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSSCANFREQH1 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsscanfreqh1(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreqh1(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSSFTABLE2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwssftable2(pub u32);

        impl Mwssftable2 {
            /// MWSSCANFREQL2 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsscanfreql2(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreql2(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSSCANFREQH2 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsscanfreqh2(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreqh2(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSSFTABLE3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwssftable3(pub u32);

        impl Mwssftable3 {
            /// MWSSCANFREQL3 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsscanfreql3(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreql3(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSSCANFREQH3 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsscanfreqh3(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreqh3(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSSFTABLE4
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwssftable4(pub u32);

        impl Mwssftable4 {
            /// MWSSCANFREQL4 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsscanfreql4(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreql4(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSSCANFREQH4 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsscanfreqh4(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreqh4(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSSFTABLE5
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwssftable5(pub u32);

        impl Mwssftable5 {
            /// MWSSCANFREQL5 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsscanfreql5(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreql5(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSSCANFREQH5 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsscanfreqh5(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreqh5(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSSFTABLE6
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwssftable6(pub u32);

        impl Mwssftable6 {
            /// MWSSCANFREQL6 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsscanfreql6(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreql6(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSSCANFREQH6 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsscanfreqh6(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreqh6(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSSFTABLE7
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwssftable7(pub u32);

        impl Mwssftable7 {
            /// MWSSCANFREQL7 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsscanfreql7(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreql7(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSSCANFREQH7 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsscanfreqh7(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreqh7(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSSFTABLE8
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwssftable8(pub u32);

        impl Mwssftable8 {
            /// MWSSCANFREQL8 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mwsscanfreql8(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreql8(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// MWSSCANFREQH8 — 16 bits (offset 16)
            #[inline(always)]
            pub const fn mwsscanfreqh8(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsscanfreqh8(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// MWSFRSYNCOFFSET
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsfrsyncoffset(pub u32);

        impl Mwsfrsyncoffset {
            /// MWSFRSYNCASSERTDLY — 15 bits (offset 0)
            #[inline(always)]
            pub const fn mwsfrsyncassertdly(&self) -> u16 {
                (self.0 & 0x7FFF) as u16
            }

            #[inline(always)]
            pub fn set_mwsfrsyncassertdly(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FFF) | (val as u32 & 0x7FFF);
            }
        }

        /// MWSTXOFFSET
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwstxoffset(pub u32);

        impl Mwstxoffset {
            /// MWSTXASSERTDLY — 11 bits (offset 0)
            #[inline(always)]
            pub const fn mwstxassertdly(&self) -> u16 {
                (self.0 & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_mwstxassertdly(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FF) | (val as u32 & 0x7FF);
            }

            /// MWSTXDEASSERTDLY — 11 bits (offset 16)
            #[inline(always)]
            pub const fn mwstxdeassertdly(&self) -> u16 {
                ((self.0 >> 16) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_mwstxdeassertdly(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 16)) | ((val as u32 & 0x7FF) << 16);
            }
        }

        /// MWSRXOFFSET
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwsrxoffset(pub u32);

        impl Mwsrxoffset {
            /// MWSRXASSERTDLY — 11 bits (offset 0)
            #[inline(always)]
            pub const fn mwsrxassertdly(&self) -> u16 {
                (self.0 & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_mwsrxassertdly(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FF) | (val as u32 & 0x7FF);
            }

            /// MWSRXDEASSERTDLY — 11 bits (offset 16)
            #[inline(always)]
            pub const fn mwsrxdeassertdly(&self) -> u16 {
                ((self.0 >> 16) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_mwsrxdeassertdly(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 16)) | ((val as u32 & 0x7FF) << 16);
            }
        }

        /// MWSWCICNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwswcicntl0(pub u32);

        impl Mwswcicntl0 {
            /// MWSWCI_TXFRACTDIV — 3 bits (offset 0)
            #[inline(always)]
            pub const fn mwswci_txfractdiv(&self) -> u8 {
                (self.0 & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwswci_txfractdiv(&mut self, val: u8) {
                self.0 = (self.0 & !0x7) | (val as u32 & 0x7);
            }

            /// MWSWCI_TXINTDIV — 13 bits (offset 3)
            #[inline(always)]
            pub const fn mwswci_txintdiv(&self) -> u16 {
                ((self.0 >> 3) & 0x1FFF) as u16
            }

            #[inline(always)]
            pub fn set_mwswci_txintdiv(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1FFF << 3)) | ((val as u32 & 0x1FFF) << 3);
            }

            /// MWSWCI_RXFRACTDIV — 3 bits (offset 16)
            #[inline(always)]
            pub const fn mwswci_rxfractdiv(&self) -> u8 {
                ((self.0 >> 16) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_mwswci_rxfractdiv(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 16)) | ((val as u32 & 0x7) << 16);
            }

            /// MWSWCI_RXINTDIV — 13 bits (offset 19)
            #[inline(always)]
            pub const fn mwswci_rxintdiv(&self) -> u16 {
                ((self.0 >> 19) & 0x1FFF) as u16
            }

            #[inline(always)]
            pub fn set_mwswci_rxintdiv(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1FFF << 19)) | ((val as u32 & 0x1FFF) << 19);
            }
        }

        /// MWSWCICNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwswcicntl1(pub u32);

        impl Mwswcicntl1 {
            /// WCIIFSCNT — 4 bits (offset 0)
            #[inline(always)]
            pub const fn wciifscnt(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_wciifscnt(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// WCISEL — 1 bit (offset 15)
            #[inline(always)]
            pub const fn wcisel(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_wcisel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// SENDWCIMSG — 1 bit (offset 30)
            #[inline(always)]
            pub const fn sendwcimsg(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sendwcimsg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// RESEND_REAL_TIME — 1 bit (offset 31)
            #[inline(always)]
            pub const fn resend_real_time(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_resend_real_time(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// MWSWCITXCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwswcitxcntl(pub u32);

        impl Mwswcitxcntl {
            /// WCITXPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn wcitxptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_wcitxptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// WCITXNB — 6 bits (offset 16)
            #[inline(always)]
            pub const fn wcitxnb(&self) -> u8 {
                ((self.0 >> 16) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_wcitxnb(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 16)) | ((val as u32 & 0x3F) << 16);
            }
        }

        /// MWSWCIRXCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Mwswcirxcntl(pub u32);

        impl Mwswcirxcntl {
            /// WCIRXPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn wcirxptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_wcirxptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// WCIRXNB — 6 bits (offset 16)
            #[inline(always)]
            pub const fn wcirxnb(&self) -> u8 {
                ((self.0 >> 16) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_wcirxnb(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 16)) | ((val as u32 & 0x3F) << 16);
            }
        }

        /// LSAMCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Lsamcntl0(pub u32);

        impl Lsamcntl0 {
            /// LOCAL_SAM_PTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn local_sam_ptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_local_sam_ptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// LOCAL_SAM_LENGTH — 6 bits (offset 16)
            #[inline(always)]
            pub const fn local_sam_length(&self) -> u8 {
                ((self.0 >> 16) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_local_sam_length(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 16)) | ((val as u32 & 0x3F) << 16);
            }

            /// LOCAL_SAM_EN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn local_sam_en(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_local_sam_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// LSAMCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Lsamcntl1(pub u32);

        impl Lsamcntl1 {
            /// LOCAL_SAM_OFFSET — 6 bits (offset 0)
            #[inline(always)]
            pub const fn local_sam_offset(&self) -> u8 {
                (self.0 & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_local_sam_offset(&mut self, val: u8) {
                self.0 = (self.0 & !0x3F) | (val as u32 & 0x3F);
            }
        }

        /// eSCOCHANCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escochancntl0(pub u32);

        impl Escochancntl0 {
            /// TeSCO0 — 8 bits (offset 0)
            #[inline(always)]
            pub const fn tesco0(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_tesco0(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// INTDELAY0 — 5 bits (offset 8)
            #[inline(always)]
            pub const fn intdelay0(&self) -> u8 {
                ((self.0 >> 8) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_intdelay0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 8)) | ((val as u32 & 0x1F) << 8);
            }

            /// ITMODE0 — 1 bit (offset 13)
            #[inline(always)]
            pub const fn itmode0(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_itmode0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// eSCOCHANEN0 — 1 bit (offset 14)
            #[inline(always)]
            pub const fn escochanen0(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escochanen0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// eSCOCHANSWEN0 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn escochanswen0(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escochanswen0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// TOG0 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn tog0(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_tog0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// eSCOMUTECNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escomutecntl0(pub u32);

        impl Escomutecntl0 {
            /// MUTEPATT0 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mutepatt0(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mutepatt0(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// INVL0_0 — 2 bits (offset 16)
            #[inline(always)]
            pub const fn invl0_0(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_invl0_0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// INVL0_1 — 2 bits (offset 18)
            #[inline(always)]
            pub const fn invl0_1(&self) -> u8 {
                ((self.0 >> 18) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_invl0_1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 18)) | ((val as u32 & 0x3) << 18);
            }

            /// MUTE_SOURCE0 — 1 bit (offset 22)
            #[inline(always)]
            pub const fn mute_source0(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mute_source0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// MUTE_SINK0 — 1 bit (offset 23)
            #[inline(always)]
            pub const fn mute_sink0(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mute_sink0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }
        }

        /// eSCOCURRENTTXPTR0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escocurrenttxptr0(pub u32);

        impl Escocurrenttxptr0 {
            /// eSCO0PTRTX0 — 14 bits (offset 0)
            #[inline(always)]
            pub const fn esco0ptrtx0(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco0ptrtx0(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// eSCO0PTRTX1 — 14 bits (offset 16)
            #[inline(always)]
            pub const fn esco0ptrtx1(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco0ptrtx1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// eSCOCURRENTRXPTR0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escocurrentrxptr0(pub u32);

        impl Escocurrentrxptr0 {
            /// eSCO0PTRRX0 — 14 bits (offset 0)
            #[inline(always)]
            pub const fn esco0ptrrx0(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco0ptrrx0(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// eSCO0PTRRX1 — 14 bits (offset 16)
            #[inline(always)]
            pub const fn esco0ptrrx1(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco0ptrrx1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// eSCOLTCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escoltcntl0(pub u32);

        impl Escoltcntl0 {
            /// SYNLTADDR0 — 3 bits (offset 0)
            #[inline(always)]
            pub const fn synltaddr0(&self) -> u8 {
                (self.0 & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_synltaddr0(&mut self, val: u8) {
                self.0 = (self.0 & !0x7) | (val as u32 & 0x7);
            }

            /// SYNTYPE0 — 1 bit (offset 3)
            #[inline(always)]
            pub const fn syntype0(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_syntype0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// eSCOEDRTX0 — 1 bit (offset 4)
            #[inline(always)]
            pub const fn escoedrtx0(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escoedrtx0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// eSCOEDRRX0 — 1 bit (offset 5)
            #[inline(always)]
            pub const fn escoedrrx0(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escoedrrx0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// RETXNB0 — 8 bits (offset 16)
            #[inline(always)]
            pub const fn retxnb0(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_retxnb0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }
        }

        /// eSCOTRCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escotrcntl0(pub u32);

        impl Escotrcntl0 {
            /// RXTYPE0 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn rxtype0(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_rxtype0(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// RXLEN0 — 10 bits (offset 4)
            #[inline(always)]
            pub const fn rxlen0(&self) -> u16 {
                ((self.0 >> 4) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_rxlen0(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 4)) | ((val as u32 & 0x3FF) << 4);
            }

            /// TXTYPE0 — 4 bits (offset 16)
            #[inline(always)]
            pub const fn txtype0(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_txtype0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// TXLEN0 — 10 bits (offset 20)
            #[inline(always)]
            pub const fn txlen0(&self) -> u16 {
                ((self.0 >> 20) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_txlen0(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 20)) | ((val as u32 & 0x3FF) << 20);
            }

            /// TXSEQN0 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn txseqn0(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txseqn0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// eSCODAYCNT0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escodaycnt0(pub u32);

        impl Escodaycnt0 {
            /// DAYCOUNTER0 — 11 bits (offset 0)
            #[inline(always)]
            pub const fn daycounter0(&self) -> u16 {
                (self.0 & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_daycounter0(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FF) | (val as u32 & 0x7FF);
            }
        }

        /// eSCOCHANCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escochancntl1(pub u32);

        impl Escochancntl1 {
            /// TeSCO1 — 8 bits (offset 0)
            #[inline(always)]
            pub const fn tesco1(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_tesco1(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// INTDELAY1 — 5 bits (offset 8)
            #[inline(always)]
            pub const fn intdelay1(&self) -> u8 {
                ((self.0 >> 8) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_intdelay1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 8)) | ((val as u32 & 0x1F) << 8);
            }

            /// ITMODE1 — 1 bit (offset 13)
            #[inline(always)]
            pub const fn itmode1(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_itmode1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// eSCOCHANEN1 — 1 bit (offset 14)
            #[inline(always)]
            pub const fn escochanen1(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escochanen1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// eSCOCHANSWEN1 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn escochanswen1(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escochanswen1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// TOG1 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn tog1(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_tog1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// eSCOMUTECNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escomutecntl1(pub u32);

        impl Escomutecntl1 {
            /// MUTEPATT1 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mutepatt1(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mutepatt1(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// INVL1_0 — 2 bits (offset 16)
            #[inline(always)]
            pub const fn invl1_0(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_invl1_0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// INVL1_1 — 2 bits (offset 18)
            #[inline(always)]
            pub const fn invl1_1(&self) -> u8 {
                ((self.0 >> 18) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_invl1_1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 18)) | ((val as u32 & 0x3) << 18);
            }

            /// MUTE_SOURCE1 — 1 bit (offset 22)
            #[inline(always)]
            pub const fn mute_source1(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mute_source1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// MUTE_SINK1 — 1 bit (offset 23)
            #[inline(always)]
            pub const fn mute_sink1(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mute_sink1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }
        }

        /// eSCOCURRENTTXPTR1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escocurrenttxptr1(pub u32);

        impl Escocurrenttxptr1 {
            /// eSCO1PTRTX0 — 14 bits (offset 0)
            #[inline(always)]
            pub const fn esco1ptrtx0(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco1ptrtx0(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// eSCO1PTRTX1 — 14 bits (offset 16)
            #[inline(always)]
            pub const fn esco1ptrtx1(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco1ptrtx1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// eSCOCURRENTRXPTR1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escocurrentrxptr1(pub u32);

        impl Escocurrentrxptr1 {
            /// eSCO1PTRRX0 — 14 bits (offset 0)
            #[inline(always)]
            pub const fn esco1ptrrx0(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco1ptrrx0(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// eSCO1PTRRX1 — 14 bits (offset 16)
            #[inline(always)]
            pub const fn esco1ptrrx1(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco1ptrrx1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// eSCOLTCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escoltcntl1(pub u32);

        impl Escoltcntl1 {
            /// SYNLTADDR1 — 3 bits (offset 0)
            #[inline(always)]
            pub const fn synltaddr1(&self) -> u8 {
                (self.0 & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_synltaddr1(&mut self, val: u8) {
                self.0 = (self.0 & !0x7) | (val as u32 & 0x7);
            }

            /// SYNTYPE1 — 1 bit (offset 3)
            #[inline(always)]
            pub const fn syntype1(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_syntype1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// eSCOEDRTX1 — 1 bit (offset 4)
            #[inline(always)]
            pub const fn escoedrtx1(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escoedrtx1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// eSCOEDRRX1 — 1 bit (offset 5)
            #[inline(always)]
            pub const fn escoedrrx1(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escoedrrx1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// RETXNB1 — 8 bits (offset 16)
            #[inline(always)]
            pub const fn retxnb1(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_retxnb1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }
        }

        /// eSCOTRCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escotrcntl1(pub u32);

        impl Escotrcntl1 {
            /// RXTYPE1 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn rxtype1(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_rxtype1(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// RXLEN1 — 10 bits (offset 4)
            #[inline(always)]
            pub const fn rxlen1(&self) -> u16 {
                ((self.0 >> 4) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_rxlen1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 4)) | ((val as u32 & 0x3FF) << 4);
            }

            /// TXTYPE1 — 4 bits (offset 16)
            #[inline(always)]
            pub const fn txtype1(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_txtype1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// TXLEN1 — 10 bits (offset 20)
            #[inline(always)]
            pub const fn txlen1(&self) -> u16 {
                ((self.0 >> 20) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_txlen1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 20)) | ((val as u32 & 0x3FF) << 20);
            }

            /// TXSEQN1 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn txseqn1(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txseqn1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// eSCODAYCNT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escodaycnt1(pub u32);

        impl Escodaycnt1 {
            /// DAYCOUNTER1 — 11 bits (offset 0)
            #[inline(always)]
            pub const fn daycounter1(&self) -> u16 {
                (self.0 & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_daycounter1(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FF) | (val as u32 & 0x7FF);
            }
        }

        /// eSCOCHANCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escochancntl2(pub u32);

        impl Escochancntl2 {
            /// TeSCO2 — 8 bits (offset 0)
            #[inline(always)]
            pub const fn tesco2(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_tesco2(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// INTDELAY2 — 5 bits (offset 8)
            #[inline(always)]
            pub const fn intdelay2(&self) -> u8 {
                ((self.0 >> 8) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_intdelay2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 8)) | ((val as u32 & 0x1F) << 8);
            }

            /// ITMODE2 — 1 bit (offset 13)
            #[inline(always)]
            pub const fn itmode2(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_itmode2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// eSCOCHANEN2 — 1 bit (offset 14)
            #[inline(always)]
            pub const fn escochanen2(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escochanen2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// eSCOCHANSWEN2 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn escochanswen2(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escochanswen2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// TOG2 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn tog2(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_tog2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// eSCOMUTECNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escomutecntl2(pub u32);

        impl Escomutecntl2 {
            /// MUTEPATT2 — 16 bits (offset 0)
            #[inline(always)]
            pub const fn mutepatt2(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_mutepatt2(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// INVL2_0 — 2 bits (offset 16)
            #[inline(always)]
            pub const fn invl2_0(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_invl2_0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// INVL2_1 — 2 bits (offset 18)
            #[inline(always)]
            pub const fn invl2_1(&self) -> u8 {
                ((self.0 >> 18) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_invl2_1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 18)) | ((val as u32 & 0x3) << 18);
            }

            /// MUTE_SOURCE2 — 1 bit (offset 22)
            #[inline(always)]
            pub const fn mute_source2(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mute_source2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// MUTE_SINK2 — 1 bit (offset 23)
            #[inline(always)]
            pub const fn mute_sink2(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mute_sink2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }
        }

        /// eSCOCURRENTTXPTR2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escocurrenttxptr2(pub u32);

        impl Escocurrenttxptr2 {
            /// eSCO2PTRTX0 — 14 bits (offset 0)
            #[inline(always)]
            pub const fn esco2ptrtx0(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco2ptrtx0(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// eSCO2PTRTX1 — 14 bits (offset 16)
            #[inline(always)]
            pub const fn esco2ptrtx1(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco2ptrtx1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// eSCOCURRENTRXPTR2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escocurrentrxptr2(pub u32);

        impl Escocurrentrxptr2 {
            /// eSCO2PTRRX0 — 14 bits (offset 0)
            #[inline(always)]
            pub const fn esco2ptrrx0(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco2ptrrx0(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// eSCO2PTRRX1 — 14 bits (offset 16)
            #[inline(always)]
            pub const fn esco2ptrrx1(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_esco2ptrrx1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// eSCOLTCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escoltcntl2(pub u32);

        impl Escoltcntl2 {
            /// SYNLTADDR2 — 3 bits (offset 0)
            #[inline(always)]
            pub const fn synltaddr2(&self) -> u8 {
                (self.0 & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_synltaddr2(&mut self, val: u8) {
                self.0 = (self.0 & !0x7) | (val as u32 & 0x7);
            }

            /// SYNTYPE2 — 1 bit (offset 3)
            #[inline(always)]
            pub const fn syntype2(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_syntype2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// eSCOEDRTX2 — 1 bit (offset 4)
            #[inline(always)]
            pub const fn escoedrtx2(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escoedrtx2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// eSCOEDRRX2 — 1 bit (offset 5)
            #[inline(always)]
            pub const fn escoedrrx2(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_escoedrrx2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// RETXNB2 — 8 bits (offset 16)
            #[inline(always)]
            pub const fn retxnb2(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_retxnb2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }
        }

        /// eSCOTRCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escotrcntl2(pub u32);

        impl Escotrcntl2 {
            /// RXTYPE2 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn rxtype2(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_rxtype2(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// RXLEN2 — 10 bits (offset 4)
            #[inline(always)]
            pub const fn rxlen2(&self) -> u16 {
                ((self.0 >> 4) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_rxlen2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 4)) | ((val as u32 & 0x3FF) << 4);
            }

            /// TXTYPE2 — 4 bits (offset 16)
            #[inline(always)]
            pub const fn txtype2(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_txtype2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// TXLEN2 — 10 bits (offset 20)
            #[inline(always)]
            pub const fn txlen2(&self) -> u16 {
                ((self.0 >> 20) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_txlen2(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 20)) | ((val as u32 & 0x3FF) << 20);
            }

            /// TXSEQN2 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn txseqn2(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txseqn2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// eSCODAYCNT2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Escodaycnt2(pub u32);

        impl Escodaycnt2 {
            /// DAYCOUNTER2 — 11 bits (offset 0)
            #[inline(always)]
            pub const fn daycounter2(&self) -> u16 {
                (self.0 & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_daycounter2(&mut self, val: u16) {
                self.0 = (self.0 & !0x7FF) | (val as u32 & 0x7FF);
            }
        }

        /// AUDIOCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Audiocntl0(pub u32);

        impl Audiocntl0 {
            /// CVSD_BITORDER0 — 1 bit (offset 0)
            #[inline(always)]
            pub const fn cvsd_bitorder0(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cvsd_bitorder0(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// CVSDEN0 — 1 bit (offset 7)
            #[inline(always)]
            pub const fn cvsden0(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cvsden0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// AULAW_CODE0 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn aulaw_code0(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_aulaw_code0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// AULAWEN0 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn aulawen0(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aulawen0(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// SAMPLE_TYPE0 — 2 bits (offset 16)
            #[inline(always)]
            pub const fn sample_type0(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_sample_type0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// LINEAR_FORMAT0 — 2 bits (offset 20)
            #[inline(always)]
            pub const fn linear_format0(&self) -> u8 {
                ((self.0 >> 20) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_linear_format0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 20)) | ((val as u32 & 0x3) << 20);
            }
        }

        /// AUDIOCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Audiocntl1(pub u32);

        impl Audiocntl1 {
            /// CVSD_BITORDER1 — 1 bit (offset 0)
            #[inline(always)]
            pub const fn cvsd_bitorder1(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cvsd_bitorder1(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// CVSDEN1 — 1 bit (offset 7)
            #[inline(always)]
            pub const fn cvsden1(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cvsden1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// AULAW_CODE1 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn aulaw_code1(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_aulaw_code1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// AULAWEN1 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn aulawen1(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aulawen1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// SAMPLE_TYPE1 — 2 bits (offset 16)
            #[inline(always)]
            pub const fn sample_type1(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_sample_type1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// LINEAR_FORMAT1 — 2 bits (offset 20)
            #[inline(always)]
            pub const fn linear_format1(&self) -> u8 {
                ((self.0 >> 20) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_linear_format1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 20)) | ((val as u32 & 0x3) << 20);
            }
        }

        /// AUDIOCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Audiocntl2(pub u32);

        impl Audiocntl2 {
            /// CVSD_BITORDER2 — 1 bit (offset 0)
            #[inline(always)]
            pub const fn cvsd_bitorder2(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cvsd_bitorder2(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// CVSDEN2 — 1 bit (offset 7)
            #[inline(always)]
            pub const fn cvsden2(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cvsden2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// AULAW_CODE2 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn aulaw_code2(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_aulaw_code2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// AULAWEN2 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn aulawen2(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aulawen2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// SAMPLE_TYPE2 — 2 bits (offset 16)
            #[inline(always)]
            pub const fn sample_type2(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_sample_type2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// LINEAR_FORMAT2 — 2 bits (offset 20)
            #[inline(always)]
            pub const fn linear_format2(&self) -> u8 {
                ((self.0 >> 20) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_linear_format2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 20)) | ((val as u32 & 0x3) << 20);
            }
        }

        /// PCMGENCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmgencntl(pub u32);

        impl Pcmgencntl {
            /// PCMEN — 1 bit (offset 0)
            #[inline(always)]
            pub const fn pcmen(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_pcmen(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// LRSWAP — 1 bit (offset 1)
            #[inline(always)]
            pub const fn lrswap(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_lrswap(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// BYTESWAP — 1 bit (offset 2)
            #[inline(always)]
            pub const fn byteswap(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_byteswap(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// MSTSLV — 1 bit (offset 3)
            #[inline(always)]
            pub const fn mstslv(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mstslv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// MONO_STEREO — 1 bit (offset 4)
            #[inline(always)]
            pub const fn mono_stereo(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mono_stereo(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// MONO_LR_SEL — 1 bit (offset 5)
            #[inline(always)]
            pub const fn mono_lr_sel(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mono_lr_sel(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// LOOPBACK — 1 bit (offset 6)
            #[inline(always)]
            pub const fn loopback(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_loopback(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// VXCHSEL — 2 bits (offset 8)
            #[inline(always)]
            pub const fn vxchsel(&self) -> u8 {
                ((self.0 >> 8) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_vxchsel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 8)) | ((val as u32 & 0x3) << 8);
            }
        }

        /// PCMPHYSCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmphyscntl0(pub u32);

        impl Pcmphyscntl0 {
            /// FSYNCSHP — 3 bits (offset 0)
            #[inline(always)]
            pub const fn fsyncshp(&self) -> u8 {
                (self.0 & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_fsyncshp(&mut self, val: u8) {
                self.0 = (self.0 & !0x7) | (val as u32 & 0x7);
            }

            /// DOUTCFG — 2 bits (offset 4)
            #[inline(always)]
            pub const fn doutcfg(&self) -> u8 {
                ((self.0 >> 4) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_doutcfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 4)) | ((val as u32 & 0x3) << 4);
            }

            /// LRCHPOL — 1 bit (offset 8)
            #[inline(always)]
            pub const fn lrchpol(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_lrchpol(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// PCM_IOM — 1 bit (offset 9)
            #[inline(always)]
            pub const fn pcm_iom(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_pcm_iom(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// LSB1ST — 1 bit (offset 10)
            #[inline(always)]
            pub const fn lsb1st(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_lsb1st(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// SAMPSZ — 1 bit (offset 12)
            #[inline(always)]
            pub const fn sampsz(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sampsz(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// SAMPTYPE — 1 bit (offset 13)
            #[inline(always)]
            pub const fn samptype(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_samptype(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// SLOTNB — 3 bits (offset 16)
            #[inline(always)]
            pub const fn slotnb(&self) -> u8 {
                ((self.0 >> 16) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_slotnb(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 16)) | ((val as u32 & 0x7) << 16);
            }

            /// FIRSTACTSLOT — 2 bits (offset 20)
            #[inline(always)]
            pub const fn firstactslot(&self) -> u8 {
                ((self.0 >> 20) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_firstactslot(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 20)) | ((val as u32 & 0x3) << 20);
            }
        }

        /// PCMPHYSCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmphyscntl1(pub u32);

        impl Pcmphyscntl1 {
            /// PCMCLKVAL — 9 bits (offset 0)
            #[inline(always)]
            pub const fn pcmclkval(&self) -> u16 {
                (self.0 & 0x1FF) as u16
            }

            #[inline(always)]
            pub fn set_pcmclkval(&mut self, val: u16) {
                self.0 = (self.0 & !0x1FF) | (val as u32 & 0x1FF);
            }

            /// PCMCLKLIMIT — 8 bits (offset 16)
            #[inline(always)]
            pub const fn pcmclklimit(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_pcmclklimit(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// CLKINV — 1 bit (offset 31)
            #[inline(always)]
            pub const fn clkinv(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clkinv(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// PCMPADDING
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmpadding(pub u32);

        impl Pcmpadding {
            /// LSAMPPAD — 16 bits (offset 0)
            #[inline(always)]
            pub const fn lsamppad(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_lsamppad(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// RSAMPPAD — 16 bits (offset 16)
            #[inline(always)]
            pub const fn rsamppad(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_rsamppad(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// PCMPLLCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmpllcntl0(pub u32);

        impl Pcmpllcntl0 {
            /// RL — 20 bits (offset 0)
            #[inline(always)]
            pub const fn rl(&self) -> u32 {
                self.0 & 0xFFFFF
            }

            #[inline(always)]
            pub fn set_rl(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFF) | (val & 0xFFFFF);
            }
        }

        /// PCMPLLCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmpllcntl1(pub u32);

        impl Pcmpllcntl1 {
            /// A — 19 bits (offset 0)
            #[inline(always)]
            pub const fn a(&self) -> u32 {
                self.0 & 0x7FFFF
            }

            #[inline(always)]
            pub fn set_a(&mut self, val: u32) {
                self.0 = (self.0 & !0x7FFFF) | (val & 0x7FFFF);
            }

            /// OLC — 11 bits (offset 20)
            #[inline(always)]
            pub const fn olc(&self) -> u16 {
                ((self.0 >> 20) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_olc(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 20)) | ((val as u32 & 0x7FF) << 20);
            }
        }

        /// PCMPLLCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmpllcntl2(pub u32);

        impl Pcmpllcntl2 {
            /// W — 19 bits (offset 0)
            #[inline(always)]
            pub const fn w(&self) -> u32 {
                self.0 & 0x7FFFF
            }

            #[inline(always)]
            pub fn set_w(&mut self, val: u32) {
                self.0 = (self.0 & !0x7FFFF) | (val & 0x7FFFF);
            }
        }

        /// PCMSOURCEPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmsourceptr(pub u32);

        impl Pcmsourceptr {
            /// PCMSOURCEPTR0 — 14 bits (offset 0)
            #[inline(always)]
            pub const fn pcmsourceptr0(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_pcmsourceptr0(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// PCMSOURCEPTR1 — 14 bits (offset 16)
            #[inline(always)]
            pub const fn pcmsourceptr1(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_pcmsourceptr1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// PCMSINKPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Pcmsinkptr(pub u32);

        impl Pcmsinkptr {
            /// PCMSINKPTR0 — 14 bits (offset 0)
            #[inline(always)]
            pub const fn pcmsinkptr0(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_pcmsinkptr0(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// PCMSINKPTR1 — 14 bits (offset 16)
            #[inline(always)]
            pub const fn pcmsinkptr1(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_pcmsinkptr1(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }
    }
}

pub mod ble {
    #[allow(unused_imports)]
    use crate::common::{Reg, R, RW, W};

    /// BLE core block.
    #[derive(Copy, Clone, Eq, PartialEq)]
    pub struct Ble {
        ptr: *mut u8,
    }

    unsafe impl Send for Ble {}
    unsafe impl Sync for Ble {}

    impl Ble {
        /// # Safety
        /// `ptr` must point to a valid BLE core register block.
        #[inline(always)]
        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
            Self {
                ptr: ptr as *mut u8,
            }
        }

        #[inline(always)]
        pub const fn as_ptr(&self) -> *mut () {
            self.ptr as *mut ()
        }

        /// RWBLECNTL (0x00)
        #[inline(always)]
        pub const fn rwblecntl(self) -> Reg<regs::Rwblecntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x00usize) as _) }
        }

        /// VERSION (0x04)
        #[inline(always)]
        pub const fn version(self) -> Reg<regs::Version, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x04usize) as _) }
        }

        /// RWBLECONF (0x08)
        #[inline(always)]
        pub const fn rwbleconf(self) -> Reg<regs::Rwbleconf, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x08usize) as _) }
        }

        /// INTCNTL0 (0x0C)
        #[inline(always)]
        pub const fn intcntl0(self) -> Reg<regs::Intcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x0Cusize) as _) }
        }

        /// INTSTAT0 (0x10)
        #[inline(always)]
        pub const fn intstat0(self) -> Reg<regs::Intstat0, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x10usize) as _) }
        }

        /// INTACK0 (0x14)
        #[inline(always)]
        pub const fn intack0(self) -> Reg<regs::Intack0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x14usize) as _) }
        }

        /// INTCNTL1 (0x18)
        #[inline(always)]
        pub const fn intcntl1(self) -> Reg<regs::Intcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x18usize) as _) }
        }

        /// INTSTAT1 (0x1C)
        #[inline(always)]
        pub const fn intstat1(self) -> Reg<regs::Intstat1, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1Cusize) as _) }
        }

        /// INTACK1 (0x20)
        #[inline(always)]
        pub const fn intack1(self) -> Reg<regs::Intack1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x20usize) as _) }
        }

        /// ACTFIFOSTAT (0x24)
        #[inline(always)]
        pub const fn actfifostat(self) -> Reg<regs::Actfifostat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x24usize) as _) }
        }

        /// CURRENTRXDESCPTR (0x28)
        #[inline(always)]
        pub const fn currentrxdescptr(self) -> Reg<regs::Currentrxdescptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x28usize) as _) }
        }

        /// ETPTR (0x2C)
        #[inline(always)]
        pub const fn etptr(self) -> Reg<regs::Etptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x2Cusize) as _) }
        }

        /// DEEPSLCNTL (0x30)
        #[inline(always)]
        pub const fn deepslcntl(self) -> Reg<regs::Deepslcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x30usize) as _) }
        }

        /// DEEPSLWKUP (0x34)
        #[inline(always)]
        pub const fn deepslwkup(self) -> Reg<regs::Deepslwkup, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x34usize) as _) }
        }

        /// DEEPSLSTAT (0x38)
        #[inline(always)]
        pub const fn deepslstat(self) -> Reg<regs::Deepslstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x38usize) as _) }
        }

        /// ENBPRESET (0x3C)
        #[inline(always)]
        pub const fn enbpreset(self) -> Reg<regs::Enbpreset, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x3Cusize) as _) }
        }

        /// FINECNTCORR (0x40)
        #[inline(always)]
        pub const fn finecntcorr(self) -> Reg<regs::Finecntcorr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x40usize) as _) }
        }

        /// CLKNCNTCORR (0x44)
        #[inline(always)]
        pub const fn clkncntcorr(self) -> Reg<regs::Clkncntcorr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x44usize) as _) }
        }

        /// DIAGCNTL (0x50)
        #[inline(always)]
        pub const fn diagcntl(self) -> Reg<regs::Diagcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x50usize) as _) }
        }

        /// DIAGSTAT (0x54)
        #[inline(always)]
        pub const fn diagstat(self) -> Reg<regs::Diagstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x54usize) as _) }
        }

        /// DEBUGADDMAX (0x58)
        #[inline(always)]
        pub const fn debugaddmax(self) -> Reg<regs::Debugaddmax, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x58usize) as _) }
        }

        /// DEBUGADDMIN (0x5C)
        #[inline(always)]
        pub const fn debugaddmin(self) -> Reg<regs::Debugaddmin, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x5Cusize) as _) }
        }

        /// ERRORTYPESTAT (0x60)
        #[inline(always)]
        pub const fn errortypestat(self) -> Reg<regs::Errortypestat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x60usize) as _) }
        }

        /// SWPROFILING (0x64)
        #[inline(always)]
        pub const fn swprofiling(self) -> Reg<regs::Swprofiling, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x64usize) as _) }
        }

        /// RADIOCNTL0 (0x70)
        #[inline(always)]
        pub const fn radiocntl0(self) -> Reg<regs::Radiocntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x70usize) as _) }
        }

        /// RADIOCNTL1 (0x74)
        #[inline(always)]
        pub const fn radiocntl1(self) -> Reg<regs::Radiocntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x74usize) as _) }
        }

        /// RADIOCNTL2 (0x78)
        #[inline(always)]
        pub const fn radiocntl2(self) -> Reg<regs::Radiocntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x78usize) as _) }
        }

        /// RADIOCNTL3 (0x7C)
        #[inline(always)]
        pub const fn radiocntl3(self) -> Reg<regs::Radiocntl3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x7Cusize) as _) }
        }

        /// RADIOPWRUPDN0 (0x80)
        #[inline(always)]
        pub const fn radiopwrupdn0(self) -> Reg<regs::Radiopwrupdn0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x80usize) as _) }
        }

        /// RADIOPWRUPDN1 (0x84)
        #[inline(always)]
        pub const fn radiopwrupdn1(self) -> Reg<regs::Radiopwrupdn1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x84usize) as _) }
        }

        /// RADIOPWRUPDN2 (0x88)
        #[inline(always)]
        pub const fn radiopwrupdn2(self) -> Reg<regs::Radiopwrupdn2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x88usize) as _) }
        }

        /// RADIOPWRUPDN3 (0x8C)
        #[inline(always)]
        pub const fn radiopwrupdn3(self) -> Reg<regs::Radiopwrupdn3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x8Cusize) as _) }
        }

        /// RADIOTXRXTIM0 (0x90)
        #[inline(always)]
        pub const fn radiotxrxtim0(self) -> Reg<regs::Radiotxrxtim0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x90usize) as _) }
        }

        /// RADIOTXRXTIM1 (0x94)
        #[inline(always)]
        pub const fn radiotxrxtim1(self) -> Reg<regs::Radiotxrxtim1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x94usize) as _) }
        }

        /// RADIOTXRXTIM2 (0x98)
        #[inline(always)]
        pub const fn radiotxrxtim2(self) -> Reg<regs::Radiotxrxtim2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x98usize) as _) }
        }

        /// RADIOTXRXTIM3 (0x9C)
        #[inline(always)]
        pub const fn radiotxrxtim3(self) -> Reg<regs::Radiotxrxtim3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x9Cusize) as _) }
        }

        /// SPIPTRCNTL0 (0xA0)
        #[inline(always)]
        pub const fn spiptrcntl0(self) -> Reg<regs::Spiptrcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xA0usize) as _) }
        }

        /// SPIPTRCNTL1 (0xA4)
        #[inline(always)]
        pub const fn spiptrcntl1(self) -> Reg<regs::Spiptrcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xA4usize) as _) }
        }

        /// SPIPTRCNTL2 (0xA8)
        #[inline(always)]
        pub const fn spiptrcntl2(self) -> Reg<regs::Spiptrcntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xA8usize) as _) }
        }

        /// SPIPTRCNTL3 (0xAC)
        #[inline(always)]
        pub const fn spiptrcntl3(self) -> Reg<regs::Spiptrcntl3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xACusize) as _) }
        }

        /// AESCNTL (0xB0)
        #[inline(always)]
        pub const fn aescntl(self) -> Reg<regs::Aescntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB0usize) as _) }
        }

        /// AESKEY31_0 (0xB4)
        #[inline(always)]
        pub const fn aeskey31_0(self) -> Reg<regs::Aeskey310, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB4usize) as _) }
        }

        /// AESKEY63_32 (0xB8)
        #[inline(always)]
        pub const fn aeskey63_32(self) -> Reg<regs::Aeskey6332, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xB8usize) as _) }
        }

        /// AESKEY95_64 (0xBC)
        #[inline(always)]
        pub const fn aeskey95_64(self) -> Reg<regs::Aeskey9564, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xBCusize) as _) }
        }

        /// AESKEY127_96 (0xC0)
        #[inline(always)]
        pub const fn aeskey127_96(self) -> Reg<regs::Aeskey12796, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC0usize) as _) }
        }

        /// AESPTR (0xC4)
        #[inline(always)]
        pub const fn aesptr(self) -> Reg<regs::Aesptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC4usize) as _) }
        }

        /// TXMICVAL (0xC8)
        #[inline(always)]
        pub const fn txmicval(self) -> Reg<regs::Txmicval, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xC8usize) as _) }
        }

        /// RXMICVAL (0xCC)
        #[inline(always)]
        pub const fn rxmicval(self) -> Reg<regs::Rxmicval, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xCCusize) as _) }
        }

        /// RFTESTCNTL (0xD0)
        #[inline(always)]
        pub const fn rftestcntl(self) -> Reg<regs::Rftestcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xD0usize) as _) }
        }

        /// RFTESTTXSTAT (0xD4)
        #[inline(always)]
        pub const fn rftesttxstat(self) -> Reg<regs::Rftesttxstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xD4usize) as _) }
        }

        /// RFTESTRXSTAT (0xD8)
        #[inline(always)]
        pub const fn rftestrxstat(self) -> Reg<regs::Rftestrxstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0xD8usize) as _) }
        }

        /// TIMGENCNTL (0xE0)
        #[inline(always)]
        pub const fn timgencntl(self) -> Reg<regs::Timgencntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE0usize) as _) }
        }

        /// FINETIMTGT (0xE4)
        #[inline(always)]
        pub const fn finetimtgt(self) -> Reg<regs::Finetimtgt, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE4usize) as _) }
        }

        /// CLKNTGT1 (0xE8)
        #[inline(always)]
        pub const fn clkntgt1(self) -> Reg<regs::Clkntgt1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xE8usize) as _) }
        }

        /// HMICROSECTGT1 (0xEC)
        #[inline(always)]
        pub const fn hmicrosectgt1(self) -> Reg<regs::Hmicrosectgt1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xECusize) as _) }
        }

        /// CLKNTGT2 (0xF0)
        #[inline(always)]
        pub const fn clkntgt2(self) -> Reg<regs::Clkntgt2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF0usize) as _) }
        }

        /// HMICROSECTGT2 (0xF4)
        #[inline(always)]
        pub const fn hmicrosectgt2(self) -> Reg<regs::Hmicrosectgt2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF4usize) as _) }
        }

        /// CLKNTGT3 (0xF8)
        #[inline(always)]
        pub const fn clkntgt3(self) -> Reg<regs::Clkntgt3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xF8usize) as _) }
        }

        /// HMICROSECTGT3 (0xFC)
        #[inline(always)]
        pub const fn hmicrosectgt3(self) -> Reg<regs::Hmicrosectgt3, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0xFCusize) as _) }
        }

        /// SLOTCLK (0x100)
        #[inline(always)]
        pub const fn slotclk(self) -> Reg<regs::Slotclk, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x100usize) as _) }
        }

        /// FINETIMECNT (0x104)
        #[inline(always)]
        pub const fn finetimecnt(self) -> Reg<regs::Finetimecnt, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x104usize) as _) }
        }

        /// ACTSCHCNTL (0x110)
        #[inline(always)]
        pub const fn actschcntl(self) -> Reg<regs::Actschcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x110usize) as _) }
        }

        /// STARTEVTCLKNTS (0x114)
        #[inline(always)]
        pub const fn startevtclknts(self) -> Reg<regs::Startevtclknts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x114usize) as _) }
        }

        /// STARTEVTFINECNTTS (0x118)
        #[inline(always)]
        pub const fn startevtfinecntts(self) -> Reg<regs::Startevtfinecntts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x118usize) as _) }
        }

        /// ENDEVTCLKNTS (0x11C)
        #[inline(always)]
        pub const fn endevtclknts(self) -> Reg<regs::Endevtclknts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x11Cusize) as _) }
        }

        /// ENDEVTFINECNTTS (0x120)
        #[inline(always)]
        pub const fn endevtfinecntts(self) -> Reg<regs::Endevtfinecntts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x120usize) as _) }
        }

        /// SKIPEVTCLKNTS (0x124)
        #[inline(always)]
        pub const fn skipevtclknts(self) -> Reg<regs::Skipevtclknts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x124usize) as _) }
        }

        /// SKIPEVTFINECNTTS (0x128)
        #[inline(always)]
        pub const fn skipevtfinecntts(self) -> Reg<regs::Skipevtfinecntts, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x128usize) as _) }
        }

        /// ADVTIM (0x130)
        #[inline(always)]
        pub const fn advtim(self) -> Reg<regs::Advtim, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x130usize) as _) }
        }

        /// ACTSCANCNTL (0x134)
        #[inline(always)]
        pub const fn actscancntl(self) -> Reg<regs::Actscancntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x134usize) as _) }
        }

        /// WPALCNTL (0x140)
        #[inline(always)]
        pub const fn wpalcntl(self) -> Reg<regs::Wpalcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x140usize) as _) }
        }

        /// WPALCURRENTPTR (0x144)
        #[inline(always)]
        pub const fn wpalcurrentptr(self) -> Reg<regs::Wpalcurrentptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x144usize) as _) }
        }

        /// SEARCH_TIMEOUT (0x148)
        #[inline(always)]
        pub const fn search_timeout(self) -> Reg<regs::SearchTimeout, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x148usize) as _) }
        }

        /// COEXIFCNTL0 (0x150)
        #[inline(always)]
        pub const fn coexifcntl0(self) -> Reg<regs::Coexifcntl0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x150usize) as _) }
        }

        /// COEXIFCNTL1 (0x154)
        #[inline(always)]
        pub const fn coexifcntl1(self) -> Reg<regs::Coexifcntl1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x154usize) as _) }
        }

        /// COEXIFCNTL2 (0x158)
        #[inline(always)]
        pub const fn coexifcntl2(self) -> Reg<regs::Coexifcntl2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x158usize) as _) }
        }

        /// BLEMPRIO0 (0x15C)
        #[inline(always)]
        pub const fn blemprio0(self) -> Reg<regs::Blemprio0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x15Cusize) as _) }
        }

        /// BLEMPRIO1 (0x160)
        #[inline(always)]
        pub const fn blemprio1(self) -> Reg<regs::Blemprio1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x160usize) as _) }
        }

        /// BLEMPRIO2 (0x164)
        #[inline(always)]
        pub const fn blemprio2(self) -> Reg<regs::Blemprio2, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x164usize) as _) }
        }

        /// RALCNTL (0x170)
        #[inline(always)]
        pub const fn ralcntl(self) -> Reg<regs::Ralcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x170usize) as _) }
        }

        /// RALCURRENTPTR (0x174)
        #[inline(always)]
        pub const fn ralcurrentptr(self) -> Reg<regs::Ralcurrentptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x174usize) as _) }
        }

        /// RAL_LOCAL_RND (0x178)
        #[inline(always)]
        pub const fn ral_local_rnd(self) -> Reg<regs::RalLocalRnd, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x178usize) as _) }
        }

        /// RAL_PEER_RND (0x17C)
        #[inline(always)]
        pub const fn ral_peer_rnd(self) -> Reg<regs::RalPeerRnd, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x17Cusize) as _) }
        }

        /// DFCNTL0_1US (0x180)
        #[inline(always)]
        pub const fn dfcntl0_1us(self) -> Reg<regs::Dfcntl01us, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x180usize) as _) }
        }

        /// DFCNTL0_2US (0x184)
        #[inline(always)]
        pub const fn dfcntl0_2us(self) -> Reg<regs::Dfcntl02us, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x184usize) as _) }
        }

        /// DFCNTL1_1US (0x188)
        #[inline(always)]
        pub const fn dfcntl1_1us(self) -> Reg<regs::Dfcntl11us, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x188usize) as _) }
        }

        /// DFCNTL1_2US (0x18C)
        #[inline(always)]
        pub const fn dfcntl1_2us(self) -> Reg<regs::Dfcntl12us, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x18Cusize) as _) }
        }

        /// DFCURRENTPTR (0x190)
        #[inline(always)]
        pub const fn dfcurrentptr(self) -> Reg<regs::Dfcurrentptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x190usize) as _) }
        }

        /// DFANTCNTL (0x194)
        #[inline(always)]
        pub const fn dfantcntl(self) -> Reg<regs::Dfantcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x194usize) as _) }
        }

        /// DFIFCNTL (0x198)
        #[inline(always)]
        pub const fn dfifcntl(self) -> Reg<regs::Dfifcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x198usize) as _) }
        }

        /// FREQSELCNTL (0x1A0)
        #[inline(always)]
        pub const fn freqselcntl(self) -> Reg<regs::Freqselcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1A0usize) as _) }
        }

        /// FREQSELPTR (0x1A4)
        #[inline(always)]
        pub const fn freqselptr(self) -> Reg<regs::Freqselptr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1A4usize) as _) }
        }

        /// FREQSEL_CS1_SEED (0x1A8)
        #[inline(always)]
        pub const fn freqsel_cs1_seed(self) -> Reg<regs::FreqselCs1Seed, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1A8usize) as _) }
        }

        /// FREQSEL_CS2_SEED (0x1AC)
        #[inline(always)]
        pub const fn freqsel_cs2_seed(self) -> Reg<regs::FreqselCs2Seed, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1ACusize) as _) }
        }

        /// FREQSEL_LLCHMAP0 (0x1B0)
        #[inline(always)]
        pub const fn freqsel_llchmap0(self) -> Reg<regs::FreqselLlchmap0, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1B0usize) as _) }
        }

        /// FREQSEL_LLCHMAP1 (0x1B4)
        #[inline(always)]
        pub const fn freqsel_llchmap1(self) -> Reg<regs::FreqselLlchmap1, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1B4usize) as _) }
        }

        /// ISOCNTCNTL (0x1C0)
        #[inline(always)]
        pub const fn isocntcntl(self) -> Reg<regs::Isocntcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1C0usize) as _) }
        }

        /// ISOCNTSAMP (0x1C4)
        #[inline(always)]
        pub const fn isocntsamp(self) -> Reg<regs::Isocntsamp, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1C4usize) as _) }
        }

        /// ISOCNTCORR (0x1C8)
        #[inline(always)]
        pub const fn isocntcorr(self) -> Reg<regs::Isocntcorr, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1C8usize) as _) }
        }

        /// ISOCNTCORR_HUS (0x1CC)
        #[inline(always)]
        pub const fn isocntcorr_hus(self) -> Reg<regs::IsocntcorrHus, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1CCusize) as _) }
        }

        /// ISOINTCNTL (0x1D0)
        #[inline(always)]
        pub const fn isointcntl(self) -> Reg<regs::Isointcntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1D0usize) as _) }
        }

        /// ISOINTSTAT (0x1D4)
        #[inline(always)]
        pub const fn isointstat(self) -> Reg<regs::Isointstat, R> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1D4usize) as _) }
        }

        /// ISOINTACK (0x1D8)
        #[inline(always)]
        pub const fn isointack(self) -> Reg<regs::Isointack, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1D8usize) as _) }
        }

        /// ISOGPIOCNTL (0x1E0)
        #[inline(always)]
        pub const fn isogpiocntl(self) -> Reg<regs::Isogpiocntl, RW> {
            unsafe { Reg::from_ptr(self.ptr.add(0x1E0usize) as _) }
        }

        /// ISOTIMERTGT (0x1F0) — 8 entries, stride 4
        #[inline(always)]
        pub const fn isotimertgt(self, n: usize) -> Reg<regs::Isotimertgt, RW> {
            assert!(n < 8);
            unsafe { Reg::from_ptr(self.ptr.add(0x1F0usize + n * 4usize) as _) }
        }
    }

    pub mod regs {

        /// RWBLECNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rwblecntl(pub u32);

        impl Rwblecntl {
            /// RXWINSZDEF — 4 bits (offset 0)
            #[inline(always)]
            pub const fn rxwinszdef(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_rxwinszdef(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// RWBLE_EN — 1 bit (offset 8)
            #[inline(always)]
            pub const fn rwble_en(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rwble_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// ADVERTFILT_EN — 1 bit (offset 9)
            #[inline(always)]
            pub const fn advertfilt_en(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_advertfilt_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// ANONYMOUS_ADV_FILT_EN — 1 bit (offset 10)
            #[inline(always)]
            pub const fn anonymous_adv_filt_en(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_anonymous_adv_filt_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// RXCTEERR_RETX_EN — 1 bit (offset 11)
            #[inline(always)]
            pub const fn rxcteerr_retx_en(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxcteerr_retx_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// HOP_REMAP_DSB — 1 bit (offset 12)
            #[inline(always)]
            pub const fn hop_remap_dsb(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_hop_remap_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// CRC_DSB — 1 bit (offset 13)
            #[inline(always)]
            pub const fn crc_dsb(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_crc_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// WHIT_DSB — 1 bit (offset 14)
            #[inline(always)]
            pub const fn whit_dsb(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_whit_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// LRFEC_DSB — 1 bit (offset 15)
            #[inline(always)]
            pub const fn lrfec_dsb(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_lrfec_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// LRPMAP_DSB — 1 bit (offset 16)
            #[inline(always)]
            pub const fn lrpmap_dsb(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_lrpmap_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// CRYPT_DSB — 1 bit (offset 17)
            #[inline(always)]
            pub const fn crypt_dsb(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_crypt_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// NESN_DSB — 1 bit (offset 18)
            #[inline(always)]
            pub const fn nesn_dsb(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_nesn_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// SN_DSB — 1 bit (offset 19)
            #[inline(always)]
            pub const fn sn_dsb(&self) -> bool {
                (self.0 >> 19) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sn_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 19)) | ((val as u32) << 19);
            }

            /// MD_DSB — 1 bit (offset 20)
            #[inline(always)]
            pub const fn md_dsb(&self) -> bool {
                (self.0 >> 20) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_md_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 20)) | ((val as u32) << 20);
            }

            /// NPI_DSB — 1 bit (offset 21)
            #[inline(always)]
            pub const fn npi_dsb(&self) -> bool {
                (self.0 >> 21) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_npi_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 21)) | ((val as u32) << 21);
            }

            /// CIE_DSB — 1 bit (offset 22)
            #[inline(always)]
            pub const fn cie_dsb(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cie_dsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// SCAN_ABORT — 1 bit (offset 24)
            #[inline(always)]
            pub const fn scan_abort(&self) -> bool {
                (self.0 >> 24) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_scan_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 24)) | ((val as u32) << 24);
            }

            /// ADVERT_ABORT — 1 bit (offset 25)
            #[inline(always)]
            pub const fn advert_abort(&self) -> bool {
                (self.0 >> 25) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_advert_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 25)) | ((val as u32) << 25);
            }

            /// RFTEST_ABORT — 1 bit (offset 26)
            #[inline(always)]
            pub const fn rftest_abort(&self) -> bool {
                (self.0 >> 26) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rftest_abort(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 26)) | ((val as u32) << 26);
            }
        }

        /// VERSION
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Version(pub u32);

        impl Version {
            /// BUILD — 8 bits (offset 0)
            #[inline(always)]
            pub const fn build(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_build(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// UPG — 8 bits (offset 8)
            #[inline(always)]
            pub const fn upg(&self) -> u8 {
                ((self.0 >> 8) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_upg(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 8)) | ((val as u32 & 0xFF) << 8);
            }

            /// REL — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rel(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rel(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// TYP — 8 bits (offset 24)
            #[inline(always)]
            pub const fn typ(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_typ(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// RWBLECONF
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rwbleconf(pub u32);

        impl Rwbleconf {
            /// ADDR_WIDTH — 5 bits (offset 0)
            #[inline(always)]
            pub const fn addr_width(&self) -> u8 {
                (self.0 & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_addr_width(&mut self, val: u8) {
                self.0 = (self.0 & !0x1F) | (val as u32 & 0x1F);
            }

            /// BUS_TYPE — 1 bit (offset 6)
            #[inline(always)]
            pub const fn bus_type(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_bus_type(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// INTMODE — 1 bit (offset 7)
            #[inline(always)]
            pub const fn intmode(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_intmode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// CLK_SEL — 6 bits (offset 8)
            #[inline(always)]
            pub const fn clk_sel(&self) -> u8 {
                ((self.0 >> 8) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_clk_sel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 8)) | ((val as u32 & 0x3F) << 8);
            }

            /// DECIPHER — 1 bit (offset 14)
            #[inline(always)]
            pub const fn decipher(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_decipher(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// USEDBG — 1 bit (offset 15)
            #[inline(always)]
            pub const fn usedbg(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_usedbg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// RFIF — 7 bits (offset 16)
            #[inline(always)]
            pub const fn rfif(&self) -> u8 {
                ((self.0 >> 16) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rfif(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 16)) | ((val as u32 & 0x7F) << 16);
            }

            /// USEISO — 1 bit (offset 24)
            #[inline(always)]
            pub const fn useiso(&self) -> bool {
                (self.0 >> 24) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_useiso(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 24)) | ((val as u32) << 24);
            }

            /// USETXLR — 1 bit (offset 26)
            #[inline(always)]
            pub const fn usetxlr(&self) -> bool {
                (self.0 >> 26) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_usetxlr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 26)) | ((val as u32) << 26);
            }

            /// USERXLR — 1 bit (offset 27)
            #[inline(always)]
            pub const fn userxlr(&self) -> bool {
                (self.0 >> 27) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_userxlr(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 27)) | ((val as u32) << 27);
            }

            /// CORRELATOR — 1 bit (offset 28)
            #[inline(always)]
            pub const fn correlator(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_correlator(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// WLANCOEX — 1 bit (offset 29)
            #[inline(always)]
            pub const fn wlancoex(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_wlancoex(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// DF — 1 bit (offset 30)
            #[inline(always)]
            pub const fn df(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_df(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// DMMODE — 1 bit (offset 31)
            #[inline(always)]
            pub const fn dmmode(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_dmmode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// INTCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intcntl0(pub u32);

        impl Intcntl0 {
            /// STARTEVTINTMSK — 1 bit (offset 0)
            #[inline(always)]
            pub const fn startevtintmsk(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_startevtintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// ENDEVTINTMSK — 1 bit (offset 1)
            #[inline(always)]
            pub const fn endevtintmsk(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_endevtintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SKIPEVTINTMSK — 1 bit (offset 2)
            #[inline(always)]
            pub const fn skipevtintmsk(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_skipevtintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// TXINTMSK — 1 bit (offset 3)
            #[inline(always)]
            pub const fn txintmsk(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// RXINTMSK — 1 bit (offset 4)
            #[inline(always)]
            pub const fn rxintmsk(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// ISOTXINTMSK — 1 bit (offset 5)
            #[inline(always)]
            pub const fn isotxintmsk(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isotxintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// ISORXINTMSK — 1 bit (offset 6)
            #[inline(always)]
            pub const fn isorxintmsk(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isorxintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// HOPINTMSK — 1 bit (offset 7)
            #[inline(always)]
            pub const fn hopintmsk(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_hopintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// ERRORINTMSK — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintmsk(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTSTAT0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intstat0(pub u32);

        impl Intstat0 {
            /// HOPINTSTAT — 1 bit (offset 7)
            #[inline(always)]
            pub const fn hopintstat(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_hopintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// ERRORINTSTAT — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintstat(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTACK0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intack0(pub u32);

        impl Intack0 {
            /// HOPINTACK — 1 bit (offset 7)
            #[inline(always)]
            pub const fn hopintack(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_hopintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// ERRORINTACK — 1 bit (offset 16)
            #[inline(always)]
            pub const fn errorintack(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_errorintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }
        }

        /// INTCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intcntl1(pub u32);

        impl Intcntl1 {
            /// CLKNINTMSK — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintmsk(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTMSK — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintmsk(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTMSK — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintmsk(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTMSK — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintmsk(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTMSK — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintmsk(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTMSK — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intmsk(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTMSK — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intmsk(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTMSK — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intmsk(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTMSK — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointmsk(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointmsk(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// CLKNINTSRVAL — 4 bits (offset 24)
            #[inline(always)]
            pub const fn clknintsrval(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_clknintsrval(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// CLKNINTSRMSK — 3 bits (offset 28)
            #[inline(always)]
            pub const fn clknintsrmsk(&self) -> u8 {
                ((self.0 >> 28) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_clknintsrmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 28)) | ((val as u32 & 0x7) << 28);
            }
        }

        /// INTSTAT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intstat1(pub u32);

        impl Intstat1 {
            /// CLKNINTSTAT — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintstat(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintstat(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTSTAT — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintstat(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTSTAT — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintstat(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTSTAT — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintstat(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTSTAT — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintstat(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTSTAT — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intstat(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTSTAT — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intstat(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTSTAT — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intstat(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTSTAT — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointstat(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }
        }

        /// INTACK1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Intack1(pub u32);

        impl Intack1 {
            /// CLKNINTACK — 1 bit (offset 0)
            #[inline(always)]
            pub const fn clknintack(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clknintack(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SLPINTACK — 1 bit (offset 1)
            #[inline(always)]
            pub const fn slpintack(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_slpintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// CRYPTINTACK — 1 bit (offset 2)
            #[inline(always)]
            pub const fn cryptintack(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_cryptintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWINTACK — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swintack(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// FINETGTINTACK — 1 bit (offset 4)
            #[inline(always)]
            pub const fn finetgtintack(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_finetgtintack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// TIMESTAMPTGT1INTACK — 1 bit (offset 5)
            #[inline(always)]
            pub const fn timestamptgt1intack(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt1intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// TIMESTAMPTGT2INTACK — 1 bit (offset 6)
            #[inline(always)]
            pub const fn timestamptgt2intack(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt2intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// TIMESTAMPTGT3INTACK — 1 bit (offset 7)
            #[inline(always)]
            pub const fn timestamptgt3intack(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_timestamptgt3intack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// FIFOINTACK — 1 bit (offset 15)
            #[inline(always)]
            pub const fn fifointack(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointack(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }
        }

        /// ACTFIFOSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Actfifostat(pub u32);

        impl Actfifostat {
            /// STARTACTINTSTAT — 1 bit (offset 0)
            #[inline(always)]
            pub const fn startactintstat(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_startactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// ENDACTINTSTAT — 1 bit (offset 1)
            #[inline(always)]
            pub const fn endactintstat(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_endactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SKIPACTINTSTAT — 1 bit (offset 2)
            #[inline(always)]
            pub const fn skipactintstat(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_skipactintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// TXINTSTAT — 1 bit (offset 3)
            #[inline(always)]
            pub const fn txintstat(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// RXINTSTAT — 1 bit (offset 4)
            #[inline(always)]
            pub const fn rxintstat(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// ISOTXINTSTAT — 1 bit (offset 5)
            #[inline(always)]
            pub const fn isotxintstat(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isotxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// ISORXINTSTAT — 1 bit (offset 6)
            #[inline(always)]
            pub const fn isorxintstat(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isorxintstat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// ACTFLAG — 1 bit (offset 15)
            #[inline(always)]
            pub const fn actflag(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_actflag(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// CURRENT_ET_IDX — 4 bits (offset 24)
            #[inline(always)]
            pub const fn current_et_idx(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_current_et_idx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// SKIP_ET_IDX — 4 bits (offset 28)
            #[inline(always)]
            pub const fn skip_et_idx(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_skip_et_idx(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// CURRENTRXDESCPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Currentrxdescptr(pub u32);

        impl Currentrxdescptr {
            /// CURRENTRXDESCPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn currentrxdescptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_currentrxdescptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// ETPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Etptr(pub u32);

        impl Etptr {
            /// ETPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn etptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_etptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// DEEPSLCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslcntl(pub u32);

        impl Deepslcntl {
            /// OSC_SLEEP_EN — 1 bit (offset 0)
            #[inline(always)]
            pub const fn osc_sleep_en(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_osc_sleep_en(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// RADIO_SLEEP_EN — 1 bit (offset 1)
            #[inline(always)]
            pub const fn radio_sleep_en(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_radio_sleep_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// DEEP_SLEEP_ON — 1 bit (offset 2)
            #[inline(always)]
            pub const fn deep_sleep_on(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_on(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// DEEP_SLEEP_CORR_EN — 1 bit (offset 3)
            #[inline(always)]
            pub const fn deep_sleep_corr_en(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_corr_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// DEEP_SLEEP_STAT — 1 bit (offset 15)
            #[inline(always)]
            pub const fn deep_sleep_stat(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_deep_sleep_stat(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// EXTWKUPDSB — 1 bit (offset 31)
            #[inline(always)]
            pub const fn extwkupdsb(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_extwkupdsb(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DEEPSLWKUP
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslwkup(pub u32);

        impl Deepslwkup {
            /// DEEPSLTIME — 32 bits (offset 0)
            #[inline(always)]
            pub const fn deepsltime(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_deepsltime(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// DEEPSLSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Deepslstat(pub u32);

        impl Deepslstat {
            /// DEEPSLDUR — 32 bits (offset 0)
            #[inline(always)]
            pub const fn deepsldur(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_deepsldur(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// ENBPRESET
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Enbpreset(pub u32);

        impl Enbpreset {
            /// TWRM — 10 bits (offset 0)
            #[inline(always)]
            pub const fn twrm(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_twrm(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }

            /// TWOSC — 11 bits (offset 10)
            #[inline(always)]
            pub const fn twosc(&self) -> u16 {
                ((self.0 >> 10) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_twosc(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 10)) | ((val as u32 & 0x7FF) << 10);
            }

            /// TWEXT — 11 bits (offset 21)
            #[inline(always)]
            pub const fn twext(&self) -> u16 {
                ((self.0 >> 21) & 0x7FF) as u16
            }

            #[inline(always)]
            pub fn set_twext(&mut self, val: u16) {
                self.0 = (self.0 & !(0x7FF << 21)) | ((val as u32 & 0x7FF) << 21);
            }
        }

        /// FINECNTCORR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finecntcorr(pub u32);

        impl Finecntcorr {
            /// FINECNTCORR — 10 bits (offset 0)
            #[inline(always)]
            pub const fn finecntcorr(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_finecntcorr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNCNTCORR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkncntcorr(pub u32);

        impl Clkncntcorr {
            /// CLKNCNTCORR — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkncntcorr(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkncntcorr(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }

            /// ABS_DELTA — 1 bit (offset 31)
            #[inline(always)]
            pub const fn abs_delta(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_abs_delta(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DIAGCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Diagcntl(pub u32);

        impl Diagcntl {
            /// DIAG0 — 7 bits (offset 0)
            #[inline(always)]
            pub const fn diag0(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_diag0(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// DIAG0_EN — 1 bit (offset 7)
            #[inline(always)]
            pub const fn diag0_en(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag0_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// DIAG1 — 7 bits (offset 8)
            #[inline(always)]
            pub const fn diag1(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_diag1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// DIAG1_EN — 1 bit (offset 15)
            #[inline(always)]
            pub const fn diag1_en(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag1_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// DIAG2 — 7 bits (offset 16)
            #[inline(always)]
            pub const fn diag2(&self) -> u8 {
                ((self.0 >> 16) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_diag2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 16)) | ((val as u32 & 0x7F) << 16);
            }

            /// DIAG2_EN — 1 bit (offset 23)
            #[inline(always)]
            pub const fn diag2_en(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag2_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }

            /// DIAG3 — 7 bits (offset 24)
            #[inline(always)]
            pub const fn diag3(&self) -> u8 {
                ((self.0 >> 24) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_diag3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 24)) | ((val as u32 & 0x7F) << 24);
            }

            /// DIAG3_EN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn diag3_en(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_diag3_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// DIAGSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Diagstat(pub u32);

        impl Diagstat {
            /// DIAG0STAT — 8 bits (offset 0)
            #[inline(always)]
            pub const fn diag0stat(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag0stat(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// DIAG1STAT — 8 bits (offset 8)
            #[inline(always)]
            pub const fn diag1stat(&self) -> u8 {
                ((self.0 >> 8) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag1stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 8)) | ((val as u32 & 0xFF) << 8);
            }

            /// DIAG2STAT — 8 bits (offset 16)
            #[inline(always)]
            pub const fn diag2stat(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag2stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// DIAG3STAT — 8 bits (offset 24)
            #[inline(always)]
            pub const fn diag3stat(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_diag3stat(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// DEBUGADDMAX
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Debugaddmax(pub u32);

        impl Debugaddmax {
            /// EM_ADDMAX — 16 bits (offset 0)
            #[inline(always)]
            pub const fn em_addmax(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_em_addmax(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// REG_ADDMAX — 16 bits (offset 16)
            #[inline(always)]
            pub const fn reg_addmax(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_reg_addmax(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// DEBUGADDMIN
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Debugaddmin(pub u32);

        impl Debugaddmin {
            /// EM_ADDMIN — 16 bits (offset 0)
            #[inline(always)]
            pub const fn em_addmin(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_em_addmin(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// REG_ADDMIN — 16 bits (offset 16)
            #[inline(always)]
            pub const fn reg_addmin(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_reg_addmin(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// ERRORTYPESTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Errortypestat(pub u32);

        impl Errortypestat {
            /// TXCRYPT_ERROR — 1 bit (offset 0)
            #[inline(always)]
            pub const fn txcrypt_error(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txcrypt_error(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// RXCRYPT_ERROR — 1 bit (offset 1)
            #[inline(always)]
            pub const fn rxcrypt_error(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxcrypt_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// PKTCNTL_EMACC_ERROR — 1 bit (offset 2)
            #[inline(always)]
            pub const fn pktcntl_emacc_error(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_pktcntl_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// RADIO_EMACC_ERROR — 1 bit (offset 3)
            #[inline(always)]
            pub const fn radio_emacc_error(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_radio_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// ACT_SCHDL_ENTRY_ERROR — 1 bit (offset 4)
            #[inline(always)]
            pub const fn act_schdl_entry_error(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_act_schdl_entry_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// ACT_SCHDL_APFM_ERROR — 1 bit (offset 5)
            #[inline(always)]
            pub const fn act_schdl_apfm_error(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_act_schdl_apfm_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// EVT_CNTL_APFM_ERROR — 1 bit (offset 6)
            #[inline(always)]
            pub const fn evt_cntl_apfm_error(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_evt_cntl_apfm_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// LIST_ERROR — 1 bit (offset 7)
            #[inline(always)]
            pub const fn list_error(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_list_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// IFS_UNDERRUN — 1 bit (offset 8)
            #[inline(always)]
            pub const fn ifs_underrun(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_ifs_underrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// ADV_UNDERRUN — 1 bit (offset 9)
            #[inline(always)]
            pub const fn adv_underrun(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_adv_underrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// LLCHMAP_ERROR — 1 bit (offset 10)
            #[inline(always)]
            pub const fn llchmap_error(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_llchmap_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// CSFORMAT_ERROR — 1 bit (offset 11)
            #[inline(always)]
            pub const fn csformat_error(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_csformat_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// TXDESC_EMPTY_ERROR — 1 bit (offset 12)
            #[inline(always)]
            pub const fn txdesc_empty_error(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txdesc_empty_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// RXDESC_EMPTY_ERROR — 1 bit (offset 13)
            #[inline(always)]
            pub const fn rxdesc_empty_error(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxdesc_empty_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// TXDATA_PTR_ERROR — 1 bit (offset 14)
            #[inline(always)]
            pub const fn txdata_ptr_error(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txdata_ptr_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// RXDATA_PTR_ERROR — 1 bit (offset 15)
            #[inline(always)]
            pub const fn rxdata_ptr_error(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxdata_ptr_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// RAL_ERROR — 1 bit (offset 16)
            #[inline(always)]
            pub const fn ral_error(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_ral_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// RAL_UNDERRUN — 1 bit (offset 17)
            #[inline(always)]
            pub const fn ral_underrun(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_ral_underrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// TMAFS_UNDERRUN — 1 bit (offset 18)
            #[inline(always)]
            pub const fn tmafs_underrun(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_tmafs_underrun(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// TXAEHEADER_PTR_ERROR — 1 bit (offset 19)
            #[inline(always)]
            pub const fn txaeheader_ptr_error(&self) -> bool {
                (self.0 >> 19) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txaeheader_ptr_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 19)) | ((val as u32) << 19);
            }

            /// PHY_ERROR — 1 bit (offset 20)
            #[inline(always)]
            pub const fn phy_error(&self) -> bool {
                (self.0 >> 20) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_phy_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 20)) | ((val as u32) << 20);
            }

            /// FIFOINTOVF — 1 bit (offset 21)
            #[inline(always)]
            pub const fn fifointovf(&self) -> bool {
                (self.0 >> 21) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_fifointovf(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 21)) | ((val as u32) << 21);
            }

            /// DFCNTL_EMACC_ERROR — 1 bit (offset 22)
            #[inline(always)]
            pub const fn dfcntl_emacc_error(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_dfcntl_emacc_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// FREQSEL_ERROR — 1 bit (offset 23)
            #[inline(always)]
            pub const fn freqsel_error(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_freqsel_error(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }
        }

        /// SWPROFILING
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Swprofiling(pub u32);

        impl Swprofiling {
            /// SWPROF0 — 1 bit (offset 0)
            #[inline(always)]
            pub const fn swprof0(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof0(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SWPROF1 — 1 bit (offset 1)
            #[inline(always)]
            pub const fn swprof1(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof1(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SWPROF2 — 1 bit (offset 2)
            #[inline(always)]
            pub const fn swprof2(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof2(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// SWPROF3 — 1 bit (offset 3)
            #[inline(always)]
            pub const fn swprof3(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof3(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// SWPROF4 — 1 bit (offset 4)
            #[inline(always)]
            pub const fn swprof4(&self) -> bool {
                (self.0 >> 4) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof4(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 4)) | ((val as u32) << 4);
            }

            /// SWPROF5 — 1 bit (offset 5)
            #[inline(always)]
            pub const fn swprof5(&self) -> bool {
                (self.0 >> 5) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof5(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 5)) | ((val as u32) << 5);
            }

            /// SWPROF6 — 1 bit (offset 6)
            #[inline(always)]
            pub const fn swprof6(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof6(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// SWPROF7 — 1 bit (offset 7)
            #[inline(always)]
            pub const fn swprof7(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof7(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// SWPROF8 — 1 bit (offset 8)
            #[inline(always)]
            pub const fn swprof8(&self) -> bool {
                (self.0 >> 8) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof8(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 8)) | ((val as u32) << 8);
            }

            /// SWPROF9 — 1 bit (offset 9)
            #[inline(always)]
            pub const fn swprof9(&self) -> bool {
                (self.0 >> 9) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof9(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 9)) | ((val as u32) << 9);
            }

            /// SWPROF10 — 1 bit (offset 10)
            #[inline(always)]
            pub const fn swprof10(&self) -> bool {
                (self.0 >> 10) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof10(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 10)) | ((val as u32) << 10);
            }

            /// SWPROF11 — 1 bit (offset 11)
            #[inline(always)]
            pub const fn swprof11(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof11(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// SWPROF12 — 1 bit (offset 12)
            #[inline(always)]
            pub const fn swprof12(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof12(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// SWPROF13 — 1 bit (offset 13)
            #[inline(always)]
            pub const fn swprof13(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof13(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// SWPROF14 — 1 bit (offset 14)
            #[inline(always)]
            pub const fn swprof14(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof14(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// SWPROF15 — 1 bit (offset 15)
            #[inline(always)]
            pub const fn swprof15(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof15(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// SWPROF16 — 1 bit (offset 16)
            #[inline(always)]
            pub const fn swprof16(&self) -> bool {
                (self.0 >> 16) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof16(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 16)) | ((val as u32) << 16);
            }

            /// SWPROF17 — 1 bit (offset 17)
            #[inline(always)]
            pub const fn swprof17(&self) -> bool {
                (self.0 >> 17) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof17(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 17)) | ((val as u32) << 17);
            }

            /// SWPROF18 — 1 bit (offset 18)
            #[inline(always)]
            pub const fn swprof18(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof18(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// SWPROF19 — 1 bit (offset 19)
            #[inline(always)]
            pub const fn swprof19(&self) -> bool {
                (self.0 >> 19) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof19(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 19)) | ((val as u32) << 19);
            }

            /// SWPROF20 — 1 bit (offset 20)
            #[inline(always)]
            pub const fn swprof20(&self) -> bool {
                (self.0 >> 20) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof20(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 20)) | ((val as u32) << 20);
            }

            /// SWPROF21 — 1 bit (offset 21)
            #[inline(always)]
            pub const fn swprof21(&self) -> bool {
                (self.0 >> 21) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof21(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 21)) | ((val as u32) << 21);
            }

            /// SWPROF22 — 1 bit (offset 22)
            #[inline(always)]
            pub const fn swprof22(&self) -> bool {
                (self.0 >> 22) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof22(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 22)) | ((val as u32) << 22);
            }

            /// SWPROF23 — 1 bit (offset 23)
            #[inline(always)]
            pub const fn swprof23(&self) -> bool {
                (self.0 >> 23) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof23(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 23)) | ((val as u32) << 23);
            }

            /// SWPROF24 — 1 bit (offset 24)
            #[inline(always)]
            pub const fn swprof24(&self) -> bool {
                (self.0 >> 24) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof24(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 24)) | ((val as u32) << 24);
            }

            /// SWPROF25 — 1 bit (offset 25)
            #[inline(always)]
            pub const fn swprof25(&self) -> bool {
                (self.0 >> 25) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof25(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 25)) | ((val as u32) << 25);
            }

            /// SWPROF26 — 1 bit (offset 26)
            #[inline(always)]
            pub const fn swprof26(&self) -> bool {
                (self.0 >> 26) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof26(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 26)) | ((val as u32) << 26);
            }

            /// SWPROF27 — 1 bit (offset 27)
            #[inline(always)]
            pub const fn swprof27(&self) -> bool {
                (self.0 >> 27) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof27(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 27)) | ((val as u32) << 27);
            }

            /// SWPROF28 — 1 bit (offset 28)
            #[inline(always)]
            pub const fn swprof28(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof28(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// SWPROF29 — 1 bit (offset 29)
            #[inline(always)]
            pub const fn swprof29(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof29(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// SWPROF30 — 1 bit (offset 30)
            #[inline(always)]
            pub const fn swprof30(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof30(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// SWPROF31 — 1 bit (offset 31)
            #[inline(always)]
            pub const fn swprof31(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_swprof31(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// RADIOCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl0(pub u32);

        impl Radiocntl0 {
            /// SPICOMP — 1 bit (offset 1)
            #[inline(always)]
            pub const fn spicomp(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_spicomp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// SPIFREQ — 2 bits (offset 4)
            #[inline(always)]
            pub const fn spifreq(&self) -> u8 {
                ((self.0 >> 4) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_spifreq(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 4)) | ((val as u32 & 0x3) << 4);
            }

            /// SPICFG — 1 bit (offset 7)
            #[inline(always)]
            pub const fn spicfg(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_spicfg(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// SPIPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn spiptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_spiptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// RADIOCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl1(pub u32);

        impl Radiocntl1 {
            /// SUBVERSION — 4 bits (offset 0)
            #[inline(always)]
            pub const fn subversion(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_subversion(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// XRFSEL — 6 bits (offset 4)
            #[inline(always)]
            pub const fn xrfsel(&self) -> u8 {
                ((self.0 >> 4) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_xrfsel(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 4)) | ((val as u32 & 0x3F) << 4);
            }

            /// JEF_SELECT — 1 bit (offset 12)
            #[inline(always)]
            pub const fn jef_select(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_jef_select(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// DPCORR_EN — 1 bit (offset 13)
            #[inline(always)]
            pub const fn dpcorr_en(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_dpcorr_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// SYNC_PULSE_SRC — 1 bit (offset 14)
            #[inline(always)]
            pub const fn sync_pulse_src(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sync_pulse_src(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// SYNC_PULSE_MODE — 1 bit (offset 15)
            #[inline(always)]
            pub const fn sync_pulse_mode(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sync_pulse_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// FORCEAGC_LENGTH — 12 bits (offset 16)
            #[inline(always)]
            pub const fn forceagc_length(&self) -> u16 {
                ((self.0 >> 16) & 0xFFF) as u16
            }

            #[inline(always)]
            pub fn set_forceagc_length(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFF << 16)) | ((val as u32 & 0xFFF) << 16);
            }

            /// TXDNSL — 1 bit (offset 28)
            #[inline(always)]
            pub const fn txdnsl(&self) -> bool {
                (self.0 >> 28) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txdnsl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 28)) | ((val as u32) << 28);
            }

            /// RXDNSL — 1 bit (offset 29)
            #[inline(always)]
            pub const fn rxdnsl(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxdnsl(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// FORCEIQ — 1 bit (offset 30)
            #[inline(always)]
            pub const fn forceiq(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_forceiq(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// FORCEAGC_EN — 1 bit (offset 31)
            #[inline(always)]
            pub const fn forceagc_en(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_forceagc_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// RADIOCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl2(pub u32);

        impl Radiocntl2 {
            /// FREQTABLE_PTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn freqtable_ptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_freqtable_ptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// SYNCERR — 3 bits (offset 16)
            #[inline(always)]
            pub const fn syncerr(&self) -> u8 {
                ((self.0 >> 16) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_syncerr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 16)) | ((val as u32 & 0x7) << 16);
            }

            /// LRSYNCERR — 2 bits (offset 20)
            #[inline(always)]
            pub const fn lrsyncerr(&self) -> u8 {
                ((self.0 >> 20) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_lrsyncerr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 20)) | ((val as u32 & 0x3) << 20);
            }

            /// PHYMSK — 2 bits (offset 22)
            #[inline(always)]
            pub const fn phymsk(&self) -> u8 {
                ((self.0 >> 22) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_phymsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 22)) | ((val as u32 & 0x3) << 22);
            }

            /// LRVTBFLUSH — 5 bits (offset 24)
            #[inline(always)]
            pub const fn lrvtbflush(&self) -> u8 {
                ((self.0 >> 24) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_lrvtbflush(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 24)) | ((val as u32 & 0x1F) << 24);
            }

            /// RXCITERMBYPASS — 1 bit (offset 29)
            #[inline(always)]
            pub const fn rxcitermbypass(&self) -> bool {
                (self.0 >> 29) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxcitermbypass(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 29)) | ((val as u32) << 29);
            }

            /// LRSYNCCOMPMODE — 2 bits (offset 30)
            #[inline(always)]
            pub const fn lrsynccompmode(&self) -> u8 {
                ((self.0 >> 30) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_lrsynccompmode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 30)) | ((val as u32 & 0x3) << 30);
            }
        }

        /// RADIOCNTL3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiocntl3(pub u32);

        impl Radiocntl3 {
            /// TXVALID_BEH — 2 bits (offset 0)
            #[inline(always)]
            pub const fn txvalid_beh(&self) -> u8 {
                (self.0 & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txvalid_beh(&mut self, val: u8) {
                self.0 = (self.0 & !0x3) | (val as u32 & 0x3);
            }

            /// TXRATE0CFG — 2 bits (offset 8)
            #[inline(always)]
            pub const fn txrate0cfg(&self) -> u8 {
                ((self.0 >> 8) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txrate0cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 8)) | ((val as u32 & 0x3) << 8);
            }

            /// TXRATE1CFG — 2 bits (offset 10)
            #[inline(always)]
            pub const fn txrate1cfg(&self) -> u8 {
                ((self.0 >> 10) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txrate1cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 10)) | ((val as u32 & 0x3) << 10);
            }

            /// TXRATE2CFG — 2 bits (offset 12)
            #[inline(always)]
            pub const fn txrate2cfg(&self) -> u8 {
                ((self.0 >> 12) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txrate2cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 12)) | ((val as u32 & 0x3) << 12);
            }

            /// TXRATE3CFG — 2 bits (offset 14)
            #[inline(always)]
            pub const fn txrate3cfg(&self) -> u8 {
                ((self.0 >> 14) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_txrate3cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 14)) | ((val as u32 & 0x3) << 14);
            }

            /// RXVALID_BEH — 2 bits (offset 16)
            #[inline(always)]
            pub const fn rxvalid_beh(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxvalid_beh(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// RXSYNC_ROUTING — 1 bit (offset 18)
            #[inline(always)]
            pub const fn rxsync_routing(&self) -> bool {
                (self.0 >> 18) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxsync_routing(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 18)) | ((val as u32) << 18);
            }

            /// GETRSSIDELAY — 3 bits (offset 20)
            #[inline(always)]
            pub const fn getrssidelay(&self) -> u8 {
                ((self.0 >> 20) & 0x7) as u8
            }

            #[inline(always)]
            pub fn set_getrssidelay(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7 << 20)) | ((val as u32 & 0x7) << 20);
            }

            /// RXRATE0CFG — 2 bits (offset 24)
            #[inline(always)]
            pub const fn rxrate0cfg(&self) -> u8 {
                ((self.0 >> 24) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxrate0cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 24)) | ((val as u32 & 0x3) << 24);
            }

            /// RXRATE1CFG — 2 bits (offset 26)
            #[inline(always)]
            pub const fn rxrate1cfg(&self) -> u8 {
                ((self.0 >> 26) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxrate1cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 26)) | ((val as u32 & 0x3) << 26);
            }

            /// RXRATE2CFG — 2 bits (offset 28)
            #[inline(always)]
            pub const fn rxrate2cfg(&self) -> u8 {
                ((self.0 >> 28) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxrate2cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 28)) | ((val as u32 & 0x3) << 28);
            }

            /// RXRATE3CFG — 2 bits (offset 30)
            #[inline(always)]
            pub const fn rxrate3cfg(&self) -> u8 {
                ((self.0 >> 30) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_rxrate3cfg(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 30)) | ((val as u32 & 0x3) << 30);
            }
        }

        /// RADIOPWRUPDN0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiopwrupdn0(pub u32);

        impl Radiopwrupdn0 {
            /// TXPWRUP0 — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txpwrup0(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txpwrup0(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// TXPWRDN0 — 7 bits (offset 8)
            #[inline(always)]
            pub const fn txpwrdn0(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpwrdn0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// RXPWRUP0 — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rxpwrup0(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxpwrup0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// SYNC_POSITION0 — 8 bits (offset 24)
            #[inline(always)]
            pub const fn sync_position0(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_sync_position0(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// RADIOPWRUPDN1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiopwrupdn1(pub u32);

        impl Radiopwrupdn1 {
            /// TXPWRUP1 — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txpwrup1(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txpwrup1(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// TXPWRDN1 — 7 bits (offset 8)
            #[inline(always)]
            pub const fn txpwrdn1(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpwrdn1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// RXPWRUP1 — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rxpwrup1(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxpwrup1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// SYNC_POSITION1 — 8 bits (offset 24)
            #[inline(always)]
            pub const fn sync_position1(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_sync_position1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// RADIOPWRUPDN2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiopwrupdn2(pub u32);

        impl Radiopwrupdn2 {
            /// TXPWRUP2 — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txpwrup2(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txpwrup2(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// TXPWRDN2 — 7 bits (offset 8)
            #[inline(always)]
            pub const fn txpwrdn2(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpwrdn2(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// RXPWRUP2 — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rxpwrup2(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxpwrup2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// SYNC_POSITION2 — 8 bits (offset 24)
            #[inline(always)]
            pub const fn sync_position2(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_sync_position2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// RADIOPWRUPDN3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiopwrupdn3(pub u32);

        impl Radiopwrupdn3 {
            /// TXPWRUP3 — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txpwrup3(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txpwrup3(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// TXPWRDN3 — 7 bits (offset 8)
            #[inline(always)]
            pub const fn txpwrdn3(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpwrdn3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }
        }

        /// RADIOTXRXTIM0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiotxrxtim0(pub u32);

        impl Radiotxrxtim0 {
            /// TXPATHDLY0 — 7 bits (offset 0)
            #[inline(always)]
            pub const fn txpathdly0(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpathdly0(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// RXPATHDLY0 — 7 bits (offset 8)
            #[inline(always)]
            pub const fn rxpathdly0(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rxpathdly0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// RFRXTMDA0 — 7 bits (offset 16)
            #[inline(always)]
            pub const fn rfrxtmda0(&self) -> u8 {
                ((self.0 >> 16) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rfrxtmda0(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 16)) | ((val as u32 & 0x7F) << 16);
            }
        }

        /// RADIOTXRXTIM1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiotxrxtim1(pub u32);

        impl Radiotxrxtim1 {
            /// TXPATHDLY1 — 7 bits (offset 0)
            #[inline(always)]
            pub const fn txpathdly1(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpathdly1(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// RXPATHDLY1 — 7 bits (offset 8)
            #[inline(always)]
            pub const fn rxpathdly1(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rxpathdly1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// RFRXTMDA1 — 7 bits (offset 16)
            #[inline(always)]
            pub const fn rfrxtmda1(&self) -> u8 {
                ((self.0 >> 16) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rfrxtmda1(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 16)) | ((val as u32 & 0x7F) << 16);
            }
        }

        /// RADIOTXRXTIM2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiotxrxtim2(pub u32);

        impl Radiotxrxtim2 {
            /// TXPATHDLY2 — 7 bits (offset 0)
            #[inline(always)]
            pub const fn txpathdly2(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpathdly2(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// RXPATHDLY2 — 8 bits (offset 8)
            #[inline(always)]
            pub const fn rxpathdly2(&self) -> u8 {
                ((self.0 >> 8) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxpathdly2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 8)) | ((val as u32 & 0xFF) << 8);
            }

            /// RFRXTMDA2 — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rfrxtmda2(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rfrxtmda2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// RXFLUSHPATHDLY2 — 8 bits (offset 24)
            #[inline(always)]
            pub const fn rxflushpathdly2(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxflushpathdly2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// RADIOTXRXTIM3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Radiotxrxtim3(pub u32);

        impl Radiotxrxtim3 {
            /// TXPATHDLY3 — 7 bits (offset 0)
            #[inline(always)]
            pub const fn txpathdly3(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txpathdly3(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// RFRXTMDA3 — 7 bits (offset 16)
            #[inline(always)]
            pub const fn rfrxtmda3(&self) -> u8 {
                ((self.0 >> 16) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rfrxtmda3(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 16)) | ((val as u32 & 0x7F) << 16);
            }

            /// RXFLUSHPATHDLY3 — 8 bits (offset 24)
            #[inline(always)]
            pub const fn rxflushpathdly3(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxflushpathdly3(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// SPIPTRCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Spiptrcntl0(pub u32);

        impl Spiptrcntl0 {
            /// TXONPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn txonptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_txonptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// TXOFFPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn txoffptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_txoffptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// SPIPTRCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Spiptrcntl1(pub u32);

        impl Spiptrcntl1 {
            /// RXONPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn rxonptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rxonptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// RXOFFPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn rxoffptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rxoffptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// SPIPTRCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Spiptrcntl2(pub u32);

        impl Spiptrcntl2 {
            /// RSSIPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn rssiptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rssiptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// RXLENGTHPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn rxlengthptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rxlengthptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// SPIPTRCNTL3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Spiptrcntl3(pub u32);

        impl Spiptrcntl3 {
            /// RXPKTTYPPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn rxpkttypptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_rxpkttypptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// CTESAMPPTR — 14 bits (offset 16)
            #[inline(always)]
            pub const fn ctesampptr(&self) -> u16 {
                ((self.0 >> 16) & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_ctesampptr(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FFF << 16)) | ((val as u32 & 0x3FFF) << 16);
            }
        }

        /// AESCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aescntl(pub u32);

        impl Aescntl {
            /// AES_START — 1 bit (offset 0)
            #[inline(always)]
            pub const fn aes_start(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aes_start(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// AES_MODE — 1 bit (offset 1)
            #[inline(always)]
            pub const fn aes_mode(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_aes_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }
        }

        /// AESKEY31_0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey310(pub u32);

        impl Aeskey310 {
            /// AESKEY31_0 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey31_0(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey31_0(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY63_32
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey6332(pub u32);

        impl Aeskey6332 {
            /// AESKEY63_32 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey63_32(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey63_32(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY95_64
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey9564(pub u32);

        impl Aeskey9564 {
            /// AESKEY95_64 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey95_64(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey95_64(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESKEY127_96
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aeskey12796(pub u32);

        impl Aeskey12796 {
            /// AESKEY127_96 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn aeskey127_96(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_aeskey127_96(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// AESPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Aesptr(pub u32);

        impl Aesptr {
            /// AESPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn aesptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_aesptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// TXMICVAL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Txmicval(pub u32);

        impl Txmicval {
            /// TXMICVAL — 32 bits (offset 0)
            #[inline(always)]
            pub const fn txmicval(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_txmicval(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// RXMICVAL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rxmicval(pub u32);

        impl Rxmicval {
            /// RXMICVAL — 32 bits (offset 0)
            #[inline(always)]
            pub const fn rxmicval(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_rxmicval(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// RFTESTCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rftestcntl(pub u32);

        impl Rftestcntl {
            /// TXLENGTH — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txlength(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txlength(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// TXPKTCNTEN — 1 bit (offset 11)
            #[inline(always)]
            pub const fn txpktcnten(&self) -> bool {
                (self.0 >> 11) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txpktcnten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 11)) | ((val as u32) << 11);
            }

            /// TXPLDSRC — 1 bit (offset 12)
            #[inline(always)]
            pub const fn txpldsrc(&self) -> bool {
                (self.0 >> 12) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txpldsrc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 12)) | ((val as u32) << 12);
            }

            /// PRBSTYPE — 1 bit (offset 13)
            #[inline(always)]
            pub const fn prbstype(&self) -> bool {
                (self.0 >> 13) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_prbstype(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 13)) | ((val as u32) << 13);
            }

            /// TXLENGTHSRC — 1 bit (offset 14)
            #[inline(always)]
            pub const fn txlengthsrc(&self) -> bool {
                (self.0 >> 14) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txlengthsrc(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 14)) | ((val as u32) << 14);
            }

            /// INFINITETX — 1 bit (offset 15)
            #[inline(always)]
            pub const fn infinitetx(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_infinitetx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }

            /// PERCOUNT_MODE — 2 bits (offset 24)
            #[inline(always)]
            pub const fn percount_mode(&self) -> u8 {
                ((self.0 >> 24) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_percount_mode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 24)) | ((val as u32 & 0x3) << 24);
            }

            /// RXPKTCNTEN — 1 bit (offset 27)
            #[inline(always)]
            pub const fn rxpktcnten(&self) -> bool {
                (self.0 >> 27) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxpktcnten(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 27)) | ((val as u32) << 27);
            }

            /// INFINITERX — 1 bit (offset 31)
            #[inline(always)]
            pub const fn infiniterx(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_infiniterx(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// RFTESTTXSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rftesttxstat(pub u32);

        impl Rftesttxstat {
            /// TXPKTCNT — 32 bits (offset 0)
            #[inline(always)]
            pub const fn txpktcnt(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_txpktcnt(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// RFTESTRXSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Rftestrxstat(pub u32);

        impl Rftestrxstat {
            /// RXPKTCNT — 32 bits (offset 0)
            #[inline(always)]
            pub const fn rxpktcnt(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_rxpktcnt(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// TIMGENCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Timgencntl(pub u32);

        impl Timgencntl {
            /// PREFETCH_TIME — 9 bits (offset 0)
            #[inline(always)]
            pub const fn prefetch_time(&self) -> u16 {
                (self.0 & 0x1FF) as u16
            }

            #[inline(always)]
            pub fn set_prefetch_time(&mut self, val: u16) {
                self.0 = (self.0 & !0x1FF) | (val as u32 & 0x1FF);
            }

            /// PREFETCHABORT_TIME — 10 bits (offset 16)
            #[inline(always)]
            pub const fn prefetchabort_time(&self) -> u16 {
                ((self.0 >> 16) & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_prefetchabort_time(&mut self, val: u16) {
                self.0 = (self.0 & !(0x3FF << 16)) | ((val as u32 & 0x3FF) << 16);
            }
        }

        /// FINETIMTGT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finetimtgt(pub u32);

        impl Finetimtgt {
            /// FINETARGET — 28 bits (offset 0)
            #[inline(always)]
            pub const fn finetarget(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_finetarget(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// CLKNTGT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt1(pub u32);

        impl Clkntgt1 {
            /// CLKNTGT1 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt1(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt1(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt1(pub u32);

        impl Hmicrosectgt1 {
            /// HMICROSECTGT1 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt1(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt1(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNTGT2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt2(pub u32);

        impl Clkntgt2 {
            /// CLKNTGT2 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt2(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt2(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt2(pub u32);

        impl Hmicrosectgt2 {
            /// HMICROSECTGT2 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt2(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt2(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// CLKNTGT3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Clkntgt3(pub u32);

        impl Clkntgt3 {
            /// CLKNTGT3 — 28 bits (offset 0)
            #[inline(always)]
            pub const fn clkntgt3(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_clkntgt3(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// HMICROSECTGT3
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Hmicrosectgt3(pub u32);

        impl Hmicrosectgt3 {
            /// HMICROSECTGT3 — 10 bits (offset 0)
            #[inline(always)]
            pub const fn hmicrosectgt3(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_hmicrosectgt3(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// SLOTCLK
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Slotclk(pub u32);

        impl Slotclk {
            /// SCLK — 28 bits (offset 0)
            #[inline(always)]
            pub const fn sclk(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_sclk(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }

            /// CLKN_UPD — 1 bit (offset 30)
            #[inline(always)]
            pub const fn clkn_upd(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_clkn_upd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// SAMP — 1 bit (offset 31)
            #[inline(always)]
            pub const fn samp(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_samp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// FINETIMECNT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Finetimecnt(pub u32);

        impl Finetimecnt {
            /// FINECNT — 10 bits (offset 0)
            #[inline(always)]
            pub const fn finecnt(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_finecnt(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// ACTSCHCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Actschcntl(pub u32);

        impl Actschcntl {
            /// ENTRY_IDX — 4 bits (offset 0)
            #[inline(always)]
            pub const fn entry_idx(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_entry_idx(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// START_ACT — 1 bit (offset 31)
            #[inline(always)]
            pub const fn start_act(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_start_act(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// STARTEVTCLKNTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Startevtclknts(pub u32);

        impl Startevtclknts {
            /// STARTEVTCLKNTS — 28 bits (offset 0)
            #[inline(always)]
            pub const fn startevtclknts(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_startevtclknts(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// STARTEVTFINECNTTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Startevtfinecntts(pub u32);

        impl Startevtfinecntts {
            /// STARTEVTFINECNTTS — 10 bits (offset 0)
            #[inline(always)]
            pub const fn startevtfinecntts(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_startevtfinecntts(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// ENDEVTCLKNTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Endevtclknts(pub u32);

        impl Endevtclknts {
            /// ENDEVTCLKNTS — 28 bits (offset 0)
            #[inline(always)]
            pub const fn endevtclknts(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_endevtclknts(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// ENDEVTFINECNTTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Endevtfinecntts(pub u32);

        impl Endevtfinecntts {
            /// ENDEVTFINECNTTS — 10 bits (offset 0)
            #[inline(always)]
            pub const fn endevtfinecntts(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_endevtfinecntts(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// SKIPEVTCLKNTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Skipevtclknts(pub u32);

        impl Skipevtclknts {
            /// SKIPEVTCLKNTS — 28 bits (offset 0)
            #[inline(always)]
            pub const fn skipevtclknts(&self) -> u32 {
                self.0 & 0xFFFFFFF
            }

            #[inline(always)]
            pub fn set_skipevtclknts(&mut self, val: u32) {
                self.0 = (self.0 & !0xFFFFFFF) | (val & 0xFFFFFFF);
            }
        }

        /// SKIPEVTFINECNTTS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Skipevtfinecntts(pub u32);

        impl Skipevtfinecntts {
            /// SKIPEVTFINECNTTS — 10 bits (offset 0)
            #[inline(always)]
            pub const fn skipevtfinecntts(&self) -> u16 {
                (self.0 & 0x3FF) as u16
            }

            #[inline(always)]
            pub fn set_skipevtfinecntts(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FF) | (val as u32 & 0x3FF);
            }
        }

        /// ADVTIM
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Advtim(pub u32);

        impl Advtim {
            /// ADVINT — 14 bits (offset 0)
            #[inline(always)]
            pub const fn advint(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_advint(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// RX_AUXPTR_THR — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rx_auxptr_thr(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rx_auxptr_thr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// TX_AUXPTR_THR — 8 bits (offset 24)
            #[inline(always)]
            pub const fn tx_auxptr_thr(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_tx_auxptr_thr(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// ACTSCANCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Actscancntl(pub u32);

        impl Actscancntl {
            /// UPPERLIMIT — 9 bits (offset 0)
            #[inline(always)]
            pub const fn upperlimit(&self) -> u16 {
                (self.0 & 0x1FF) as u16
            }

            #[inline(always)]
            pub fn set_upperlimit(&mut self, val: u16) {
                self.0 = (self.0 & !0x1FF) | (val as u32 & 0x1FF);
            }

            /// BACKOFF — 9 bits (offset 16)
            #[inline(always)]
            pub const fn backoff(&self) -> u16 {
                ((self.0 >> 16) & 0x1FF) as u16
            }

            #[inline(always)]
            pub fn set_backoff(&mut self, val: u16) {
                self.0 = (self.0 & !(0x1FF << 16)) | ((val as u32 & 0x1FF) << 16);
            }
        }

        /// WPALCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Wpalcntl(pub u32);

        impl Wpalcntl {
            /// WPALBASEPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn wpalbaseptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_wpalbaseptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// WPALNBDEV — 8 bits (offset 16)
            #[inline(always)]
            pub const fn wpalnbdev(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_wpalnbdev(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }
        }

        /// WPALCURRENTPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Wpalcurrentptr(pub u32);

        impl Wpalcurrentptr {
            /// WPALCURRENTPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn wpalcurrentptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_wpalcurrentptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// SEARCH_TIMEOUT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct SearchTimeout(pub u32);

        impl SearchTimeout {
            /// SEARCH_TIMEOUT — 6 bits (offset 0)
            #[inline(always)]
            pub const fn search_timeout(&self) -> u8 {
                (self.0 & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_search_timeout(&mut self, val: u8) {
                self.0 = (self.0 & !0x3F) | (val as u32 & 0x3F);
            }
        }

        /// COEXIFCNTL0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Coexifcntl0(pub u32);

        impl Coexifcntl0 {
            /// WLANCOEX_EN — 1 bit (offset 0)
            #[inline(always)]
            pub const fn wlancoex_en(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_wlancoex_en(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// SYNCGEN_EN — 1 bit (offset 1)
            #[inline(always)]
            pub const fn syncgen_en(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_syncgen_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// MWSCOEX_EN — 1 bit (offset 2)
            #[inline(always)]
            pub const fn mwscoex_en(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwscoex_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// MWSWCI_EN — 1 bit (offset 3)
            #[inline(always)]
            pub const fn mwswci_en(&self) -> bool {
                (self.0 >> 3) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_mwswci_en(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 3)) | ((val as u32) << 3);
            }

            /// WLANRXMSK — 2 bits (offset 4)
            #[inline(always)]
            pub const fn wlanrxmsk(&self) -> u8 {
                ((self.0 >> 4) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_wlanrxmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 4)) | ((val as u32 & 0x3) << 4);
            }

            /// WLANTXMSK — 2 bits (offset 6)
            #[inline(always)]
            pub const fn wlantxmsk(&self) -> u8 {
                ((self.0 >> 6) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_wlantxmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 6)) | ((val as u32 & 0x3) << 6);
            }

            /// MWSRXMSK — 2 bits (offset 8)
            #[inline(always)]
            pub const fn mwsrxmsk(&self) -> u8 {
                ((self.0 >> 8) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwsrxmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 8)) | ((val as u32 & 0x3) << 8);
            }

            /// MWSTXMSK — 2 bits (offset 10)
            #[inline(always)]
            pub const fn mwstxmsk(&self) -> u8 {
                ((self.0 >> 10) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwstxmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 10)) | ((val as u32 & 0x3) << 10);
            }

            /// MWSRXFRQMSK — 2 bits (offset 12)
            #[inline(always)]
            pub const fn mwsrxfrqmsk(&self) -> u8 {
                ((self.0 >> 12) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwsrxfrqmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 12)) | ((val as u32 & 0x3) << 12);
            }

            /// MWSTXFRQMSK — 2 bits (offset 14)
            #[inline(always)]
            pub const fn mwstxfrqmsk(&self) -> u8 {
                ((self.0 >> 14) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwstxfrqmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 14)) | ((val as u32 & 0x3) << 14);
            }

            /// WLCTXPRIOMODE — 2 bits (offset 16)
            #[inline(always)]
            pub const fn wlctxpriomode(&self) -> u8 {
                ((self.0 >> 16) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_wlctxpriomode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 16)) | ((val as u32 & 0x3) << 16);
            }

            /// WLCRXPRIOMODE — 2 bits (offset 18)
            #[inline(always)]
            pub const fn wlcrxpriomode(&self) -> u8 {
                ((self.0 >> 18) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_wlcrxpriomode(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 18)) | ((val as u32 & 0x3) << 18);
            }

            /// MWSSCANFREQMSK — 2 bits (offset 20)
            #[inline(always)]
            pub const fn mwsscanfreqmsk(&self) -> u8 {
                ((self.0 >> 20) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_mwsscanfreqmsk(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 20)) | ((val as u32 & 0x3) << 20);
            }
        }

        /// COEXIFCNTL1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Coexifcntl1(pub u32);

        impl Coexifcntl1 {
            /// WLCPDELAY — 7 bits (offset 0)
            #[inline(always)]
            pub const fn wlcpdelay(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_wlcpdelay(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// WLCPDURATION — 7 bits (offset 8)
            #[inline(always)]
            pub const fn wlcpduration(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_wlcpduration(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// WLCPTXTHR — 5 bits (offset 16)
            #[inline(always)]
            pub const fn wlcptxthr(&self) -> u8 {
                ((self.0 >> 16) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_wlcptxthr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 16)) | ((val as u32 & 0x1F) << 16);
            }

            /// WLCPRXTHR — 5 bits (offset 24)
            #[inline(always)]
            pub const fn wlcprxthr(&self) -> u8 {
                ((self.0 >> 24) & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_wlcprxthr(&mut self, val: u8) {
                self.0 = (self.0 & !(0x1F << 24)) | ((val as u32 & 0x1F) << 24);
            }
        }

        /// COEXIFCNTL2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Coexifcntl2(pub u32);

        impl Coexifcntl2 {
            /// TX_ANT_DELAY — 4 bits (offset 0)
            #[inline(always)]
            pub const fn tx_ant_delay(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_tx_ant_delay(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// RX_ANT_DELAY — 4 bits (offset 8)
            #[inline(always)]
            pub const fn rx_ant_delay(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_rx_ant_delay(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }
        }

        /// BLEMPRIO0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Blemprio0(pub u32);

        impl Blemprio0 {
            /// BLEM0 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn blem0(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem0(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// BLEM1 — 4 bits (offset 4)
            #[inline(always)]
            pub const fn blem1(&self) -> u8 {
                ((self.0 >> 4) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem1(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 4)) | ((val as u32 & 0xF) << 4);
            }

            /// BLEM2 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn blem2(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem2(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// BLEM3 — 4 bits (offset 12)
            #[inline(always)]
            pub const fn blem3(&self) -> u8 {
                ((self.0 >> 12) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem3(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 12)) | ((val as u32 & 0xF) << 12);
            }

            /// BLEM4 — 4 bits (offset 16)
            #[inline(always)]
            pub const fn blem4(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem4(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// BLEM5 — 4 bits (offset 20)
            #[inline(always)]
            pub const fn blem5(&self) -> u8 {
                ((self.0 >> 20) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem5(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 20)) | ((val as u32 & 0xF) << 20);
            }

            /// BLEM6 — 4 bits (offset 24)
            #[inline(always)]
            pub const fn blem6(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem6(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// BLEM7 — 4 bits (offset 28)
            #[inline(always)]
            pub const fn blem7(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem7(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// BLEMPRIO1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Blemprio1(pub u32);

        impl Blemprio1 {
            /// BLEM8 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn blem8(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem8(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// BLEM9 — 4 bits (offset 4)
            #[inline(always)]
            pub const fn blem9(&self) -> u8 {
                ((self.0 >> 4) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem9(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 4)) | ((val as u32 & 0xF) << 4);
            }

            /// BLEM10 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn blem10(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem10(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// BLEM11 — 4 bits (offset 12)
            #[inline(always)]
            pub const fn blem11(&self) -> u8 {
                ((self.0 >> 12) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem11(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 12)) | ((val as u32 & 0xF) << 12);
            }

            /// BLEM12 — 4 bits (offset 16)
            #[inline(always)]
            pub const fn blem12(&self) -> u8 {
                ((self.0 >> 16) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem12(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 16)) | ((val as u32 & 0xF) << 16);
            }

            /// BLEM13 — 4 bits (offset 20)
            #[inline(always)]
            pub const fn blem13(&self) -> u8 {
                ((self.0 >> 20) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem13(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 20)) | ((val as u32 & 0xF) << 20);
            }

            /// BLEM14 — 4 bits (offset 24)
            #[inline(always)]
            pub const fn blem14(&self) -> u8 {
                ((self.0 >> 24) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem14(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 24)) | ((val as u32 & 0xF) << 24);
            }

            /// BLEM15 — 4 bits (offset 28)
            #[inline(always)]
            pub const fn blem15(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem15(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// BLEMPRIO2
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Blemprio2(pub u32);

        impl Blemprio2 {
            /// BLEM16 — 4 bits (offset 0)
            #[inline(always)]
            pub const fn blem16(&self) -> u8 {
                (self.0 & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem16(&mut self, val: u8) {
                self.0 = (self.0 & !0xF) | (val as u32 & 0xF);
            }

            /// BLEM17 — 4 bits (offset 4)
            #[inline(always)]
            pub const fn blem17(&self) -> u8 {
                ((self.0 >> 4) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem17(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 4)) | ((val as u32 & 0xF) << 4);
            }

            /// BLEM18 — 4 bits (offset 8)
            #[inline(always)]
            pub const fn blem18(&self) -> u8 {
                ((self.0 >> 8) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blem18(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 8)) | ((val as u32 & 0xF) << 8);
            }

            /// BLEMDEFAULT — 4 bits (offset 28)
            #[inline(always)]
            pub const fn blemdefault(&self) -> u8 {
                ((self.0 >> 28) & 0xF) as u8
            }

            #[inline(always)]
            pub fn set_blemdefault(&mut self, val: u8) {
                self.0 = (self.0 & !(0xF << 28)) | ((val as u32 & 0xF) << 28);
            }
        }

        /// RALCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Ralcntl(pub u32);

        impl Ralcntl {
            /// RALBASEPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn ralbaseptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_ralbaseptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }

            /// RALNBDEV — 8 bits (offset 16)
            #[inline(always)]
            pub const fn ralnbdev(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_ralnbdev(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }
        }

        /// RALCURRENTPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Ralcurrentptr(pub u32);

        impl Ralcurrentptr {
            /// RALCURRENTPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn ralcurrentptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_ralcurrentptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// RAL_LOCAL_RND
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct RalLocalRnd(pub u32);

        impl RalLocalRnd {
            /// LRND_VAL — 22 bits (offset 0)
            #[inline(always)]
            pub const fn lrnd_val(&self) -> u32 {
                self.0 & 0x3FFFFF
            }

            #[inline(always)]
            pub fn set_lrnd_val(&mut self, val: u32) {
                self.0 = (self.0 & !0x3FFFFF) | (val & 0x3FFFFF);
            }
        }

        /// RAL_PEER_RND
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct RalPeerRnd(pub u32);

        impl RalPeerRnd {
            /// PRND_VAL — 22 bits (offset 0)
            #[inline(always)]
            pub const fn prnd_val(&self) -> u32 {
                self.0 & 0x3FFFFF
            }

            #[inline(always)]
            pub fn set_prnd_val(&mut self, val: u32) {
                self.0 = (self.0 & !0x3FFFFF) | (val & 0x3FFFFF);
            }
        }

        /// DFCNTL0_1US
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Dfcntl01us(pub u32);

        impl Dfcntl01us {
            /// TXSWSTINST0_1US — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txswstinst0_1us(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txswstinst0_1us(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// RXSWSTINST0_1US — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rxswstinst0_1us(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxswstinst0_1us(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// RXSAMPSTINST0_1US — 8 bits (offset 24)
            #[inline(always)]
            pub const fn rxsampstinst0_1us(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxsampstinst0_1us(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// DFCNTL0_2US
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Dfcntl02us(pub u32);

        impl Dfcntl02us {
            /// TXSWSTINST0_2US — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txswstinst0_2us(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txswstinst0_2us(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// RXSWSTINST0_2US — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rxswstinst0_2us(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxswstinst0_2us(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// RXSAMPSTINST0_2US — 8 bits (offset 24)
            #[inline(always)]
            pub const fn rxsampstinst0_2us(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxsampstinst0_2us(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// DFCNTL1_1US
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Dfcntl11us(pub u32);

        impl Dfcntl11us {
            /// TXSWSTINST1_1US — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txswstinst1_1us(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txswstinst1_1us(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// RXSWSTINST1_1US — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rxswstinst1_1us(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxswstinst1_1us(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// RXSAMPSTINST1_1US — 8 bits (offset 24)
            #[inline(always)]
            pub const fn rxsampstinst1_1us(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxsampstinst1_1us(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// DFCNTL1_2US
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Dfcntl12us(pub u32);

        impl Dfcntl12us {
            /// TXSWSTINST1_2US — 8 bits (offset 0)
            #[inline(always)]
            pub const fn txswstinst1_2us(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_txswstinst1_2us(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// RXSWSTINST1_2US — 8 bits (offset 16)
            #[inline(always)]
            pub const fn rxswstinst1_2us(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxswstinst1_2us(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }

            /// RXSAMPSTINST1_2US — 8 bits (offset 24)
            #[inline(always)]
            pub const fn rxsampstinst1_2us(&self) -> u8 {
                ((self.0 >> 24) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_rxsampstinst1_2us(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 24)) | ((val as u32 & 0xFF) << 24);
            }
        }

        /// DFCURRENTPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Dfcurrentptr(pub u32);

        impl Dfcurrentptr {
            /// DFCURRENTPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn dfcurrentptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_dfcurrentptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// DFANTCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Dfantcntl(pub u32);

        impl Dfantcntl {
            /// TXPRIMANTID — 7 bits (offset 0)
            #[inline(always)]
            pub const fn txprimantid(&self) -> u8 {
                (self.0 & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_txprimantid(&mut self, val: u8) {
                self.0 = (self.0 & !0x7F) | (val as u32 & 0x7F);
            }

            /// TXPRIMIDCNTLEN — 1 bit (offset 7)
            #[inline(always)]
            pub const fn txprimidcntlen(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_txprimidcntlen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }

            /// RXPRIMANTID — 7 bits (offset 8)
            #[inline(always)]
            pub const fn rxprimantid(&self) -> u8 {
                ((self.0 >> 8) & 0x7F) as u8
            }

            #[inline(always)]
            pub fn set_rxprimantid(&mut self, val: u8) {
                self.0 = (self.0 & !(0x7F << 8)) | ((val as u32 & 0x7F) << 8);
            }

            /// RXPRIMIDCNTLEN — 1 bit (offset 15)
            #[inline(always)]
            pub const fn rxprimidcntlen(&self) -> bool {
                (self.0 >> 15) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_rxprimidcntlen(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 15)) | ((val as u32) << 15);
            }
        }

        /// DFIFCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Dfifcntl(pub u32);

        impl Dfifcntl {
            /// SYMBOL_ORDER — 1 bit (offset 0)
            #[inline(always)]
            pub const fn symbol_order(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_symbol_order(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// MSB_LSB_ORDER — 1 bit (offset 1)
            #[inline(always)]
            pub const fn msb_lsb_order(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_msb_lsb_order(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// IF_WIDTH — 2 bits (offset 2)
            #[inline(always)]
            pub const fn if_width(&self) -> u8 {
                ((self.0 >> 2) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_if_width(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 2)) | ((val as u32 & 0x3) << 2);
            }

            /// SAMPVALID_BEH — 2 bits (offset 4)
            #[inline(always)]
            pub const fn sampvalid_beh(&self) -> u8 {
                ((self.0 >> 4) & 0x3) as u8
            }

            #[inline(always)]
            pub fn set_sampvalid_beh(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3 << 4)) | ((val as u32 & 0x3) << 4);
            }

            /// SAMPREQ_BEH — 1 bit (offset 6)
            #[inline(always)]
            pub const fn sampreq_beh(&self) -> bool {
                (self.0 >> 6) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_sampreq_beh(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 6)) | ((val as u32) << 6);
            }

            /// ANTSWITCH_BEH — 1 bit (offset 7)
            #[inline(always)]
            pub const fn antswitch_beh(&self) -> bool {
                (self.0 >> 7) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_antswitch_beh(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 7)) | ((val as u32) << 7);
            }
        }

        /// FREQSELCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Freqselcntl(pub u32);

        impl Freqselcntl {
            /// FREQSEL_START — 1 bit (offset 0)
            #[inline(always)]
            pub const fn freqsel_start(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_freqsel_start(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// FREQSEL_MODE — 1 bit (offset 1)
            #[inline(always)]
            pub const fn freqsel_mode(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_freqsel_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// NBLOOPS — 8 bits (offset 16)
            #[inline(always)]
            pub const fn nbloops(&self) -> u8 {
                ((self.0 >> 16) & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_nbloops(&mut self, val: u8) {
                self.0 = (self.0 & !(0xFF << 16)) | ((val as u32 & 0xFF) << 16);
            }
        }

        /// FREQSELPTR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Freqselptr(pub u32);

        impl Freqselptr {
            /// FREQSELPTR — 14 bits (offset 0)
            #[inline(always)]
            pub const fn freqselptr(&self) -> u16 {
                (self.0 & 0x3FFF) as u16
            }

            #[inline(always)]
            pub fn set_freqselptr(&mut self, val: u16) {
                self.0 = (self.0 & !0x3FFF) | (val as u32 & 0x3FFF);
            }
        }

        /// FREQSEL_CS1_SEED
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct FreqselCs1Seed(pub u32);

        impl FreqselCs1Seed {
            /// FREQSEL_HOPINT — 5 bits (offset 0)
            #[inline(always)]
            pub const fn freqsel_hopint(&self) -> u8 {
                (self.0 & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_freqsel_hopint(&mut self, val: u8) {
                self.0 = (self.0 & !0x1F) | (val as u32 & 0x1F);
            }

            /// FREQSEL_LAST_CHIDX — 6 bits (offset 16)
            #[inline(always)]
            pub const fn freqsel_last_chidx(&self) -> u8 {
                ((self.0 >> 16) & 0x3F) as u8
            }

            #[inline(always)]
            pub fn set_freqsel_last_chidx(&mut self, val: u8) {
                self.0 = (self.0 & !(0x3F << 16)) | ((val as u32 & 0x3F) << 16);
            }
        }

        /// FREQSEL_CS2_SEED
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct FreqselCs2Seed(pub u32);

        impl FreqselCs2Seed {
            /// FREQSEL_EVTCNT — 16 bits (offset 0)
            #[inline(always)]
            pub const fn freqsel_evtcnt(&self) -> u16 {
                (self.0 & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_freqsel_evtcnt(&mut self, val: u16) {
                self.0 = (self.0 & !0xFFFF) | (val as u32 & 0xFFFF);
            }

            /// CHANNEL_IDENTIFIER — 16 bits (offset 16)
            #[inline(always)]
            pub const fn channel_identifier(&self) -> u16 {
                ((self.0 >> 16) & 0xFFFF) as u16
            }

            #[inline(always)]
            pub fn set_channel_identifier(&mut self, val: u16) {
                self.0 = (self.0 & !(0xFFFF << 16)) | ((val as u32 & 0xFFFF) << 16);
            }
        }

        /// FREQSEL_LLCHMAP0
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct FreqselLlchmap0(pub u32);

        impl FreqselLlchmap0 {
            /// FREQSEL_LLCHMAP0 — 32 bits (offset 0)
            #[inline(always)]
            pub const fn freqsel_llchmap0(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_freqsel_llchmap0(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// FREQSEL_LLCHMAP1
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct FreqselLlchmap1(pub u32);

        impl FreqselLlchmap1 {
            /// FREQSEL_LLCHMAP1 — 5 bits (offset 0)
            #[inline(always)]
            pub const fn freqsel_llchmap1(&self) -> u8 {
                (self.0 & 0x1F) as u8
            }

            #[inline(always)]
            pub fn set_freqsel_llchmap1(&mut self, val: u8) {
                self.0 = (self.0 & !0x1F) | (val as u32 & 0x1F);
            }
        }

        /// ISOCNTCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Isocntcntl(pub u32);

        impl Isocntcntl {
            /// ISOCORRMODE — 1 bit (offset 0)
            #[inline(always)]
            pub const fn isocorrmode(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isocorrmode(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }

            /// ISO_PHASE_SHIFT_MODE — 1 bit (offset 1)
            #[inline(always)]
            pub const fn iso_phase_shift_mode(&self) -> bool {
                (self.0 >> 1) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_iso_phase_shift_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 1)) | ((val as u32) << 1);
            }

            /// ISO_CLKSHIFT_MODE — 1 bit (offset 2)
            #[inline(always)]
            pub const fn iso_clkshift_mode(&self) -> bool {
                (self.0 >> 2) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_iso_clkshift_mode(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 2)) | ((val as u32) << 2);
            }

            /// ISO_UPD — 1 bit (offset 30)
            #[inline(always)]
            pub const fn iso_upd(&self) -> bool {
                (self.0 >> 30) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_iso_upd(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 30)) | ((val as u32) << 30);
            }

            /// ISOSAMP — 1 bit (offset 31)
            #[inline(always)]
            pub const fn isosamp(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isosamp(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// ISOCNTSAMP
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Isocntsamp(pub u32);

        impl Isocntsamp {
            /// ISOCNTSAMP — 32 bits (offset 0)
            #[inline(always)]
            pub const fn isocntsamp(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_isocntsamp(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// ISOCNTCORR
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Isocntcorr(pub u32);

        impl Isocntcorr {
            /// ISOCNTCORR — 32 bits (offset 0)
            #[inline(always)]
            pub const fn isocntcorr(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_isocntcorr(&mut self, val: u32) {
                self.0 = val;
            }
        }

        /// ISOCNTCORR_HUS
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct IsocntcorrHus(pub u32);

        impl IsocntcorrHus {
            /// ISOCNTCORR_HUS — 1 bit (offset 0)
            #[inline(always)]
            pub const fn isocntcorr_hus(&self) -> bool {
                self.0 & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isocntcorr_hus(&mut self, val: bool) {
                self.0 = (self.0 & !0x1) | (val as u32);
            }
        }

        /// ISOINTCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Isointcntl(pub u32);

        impl Isointcntl {
            /// ISOINTMSK — 8 bits (offset 0)
            #[inline(always)]
            pub const fn isointmsk(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_isointmsk(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }
        }

        /// ISOINTSTAT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Isointstat(pub u32);

        impl Isointstat {
            /// ISOINTSTAT — 8 bits (offset 0)
            #[inline(always)]
            pub const fn isointstat(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_isointstat(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }
        }

        /// ISOINTACK
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Isointack(pub u32);

        impl Isointack {
            /// ISOINTACK — 8 bits (offset 0)
            #[inline(always)]
            pub const fn isointack(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_isointack(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }
        }

        /// ISOGPIOCNTL
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Isogpiocntl(pub u32);

        impl Isogpiocntl {
            /// ISOGPIOMSK — 8 bits (offset 0)
            #[inline(always)]
            pub const fn isogpiomsk(&self) -> u8 {
                (self.0 & 0xFF) as u8
            }

            #[inline(always)]
            pub fn set_isogpiomsk(&mut self, val: u8) {
                self.0 = (self.0 & !0xFF) | (val as u32 & 0xFF);
            }

            /// ISOGPIOBEH — 1 bit (offset 31)
            #[inline(always)]
            pub const fn isogpiobeh(&self) -> bool {
                (self.0 >> 31) & 0x1 != 0
            }

            #[inline(always)]
            pub fn set_isogpiobeh(&mut self, val: bool) {
                self.0 = (self.0 & !(0x1 << 31)) | ((val as u32) << 31);
            }
        }

        /// ISOTIMERTGT
        #[repr(transparent)]
        #[derive(Copy, Clone, Default, Eq, PartialEq)]
        pub struct Isotimertgt(pub u32);

        impl Isotimertgt {
            /// ISOTIMERTGT — 32 bits (offset 0)
            #[inline(always)]
            pub const fn isotimertgt(&self) -> u32 {
                self.0
            }

            #[inline(always)]
            pub fn set_isotimertgt(&mut self, val: u32) {
                self.0 = val;
            }
        }
    }
}
