#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tr: TR,
    dr: DR,
    ssr: SSR,
    icsr: ICSR,
    prer: PRER,
    wutr: WUTR,
    cr: CR,
    _reserved7: [u8; 0x04],
    smcr: SMCR,
    wpr: WPR,
    calr: CALR,
    shiftr: SHIFTR,
    tstr: TSTR,
    tsdr: TSDR,
    tsssr: TSSSR,
    _reserved14: [u8; 0x04],
    alrmr: (),
    _reserved15: [u8; 0x04],
    alrmssr: (),
    _reserved16: [u8; 0x0c],
    sr: SR,
    misr: MISR,
    smisr: SMISR,
    scr: SCR,
    cfgr: CFGR,
    _reserved21: [u8; 0x038c],
    hwcfgr: HWCFGR,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn tr(&self) -> &TR {
        &self.tr
    }
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn dr(&self) -> &DR {
        &self.dr
    }
    #[doc = "0x08 - RTC sub second register"]
    #[inline(always)]
    pub const fn ssr(&self) -> &SSR {
        &self.ssr
    }
    #[doc = "0x0c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn icsr(&self) -> &ICSR {
        &self.icsr
    }
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn prer(&self) -> &PRER {
        &self.prer
    }
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn wutr(&self) -> &WUTR {
        &self.wutr
    }
    #[doc = "0x18 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x20 - This register can be written only when the APB access is secure."]
    #[inline(always)]
    pub const fn smcr(&self) -> &SMCR {
        &self.smcr
    }
    #[doc = "0x24 - RTC write protection register"]
    #[inline(always)]
    pub const fn wpr(&self) -> &WPR {
        &self.wpr
    }
    #[doc = "0x28 - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn calr(&self) -> &CALR {
        &self.calr
    }
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn shiftr(&self) -> &SHIFTR {
        &self.shiftr
    }
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn tstr(&self) -> &TSTR {
        &self.tstr
    }
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn tsdr(&self) -> &TSDR {
        &self.tsdr
    }
    #[doc = "0x38 - The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when the TSF bit is reset. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn tsssr(&self) -> &TSSSR {
        &self.tsssr
    }
    #[doc = "0x40..0x48 - Alarm %s register"]
    #[inline(always)]
    pub const fn alrmr(&self, n: usize) -> &ALRMR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(64).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x48 - Alarm %s register"]
    #[inline(always)]
    pub fn alrmr_iter(&self) -> impl Iterator<Item = &ALRMR> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(64).add(8 * n).cast() })
    }
    #[doc = "0x40 - Alarm A register"]
    #[inline(always)]
    pub const fn alrmar(&self) -> &ALRMR {
        self.alrmr(0)
    }
    #[doc = "0x48 - Alarm B register"]
    #[inline(always)]
    pub const fn alrmbr(&self) -> &ALRMR {
        self.alrmr(1)
    }
    #[doc = "0x44..0x4c - Alarm %s sub-second register"]
    #[inline(always)]
    pub const fn alrmssr(&self, n: usize) -> &ALRMSSR {
        #[allow(clippy::no_effect)]
        [(); 2][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(68).add(8 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x44..0x4c - Alarm %s sub-second register"]
    #[inline(always)]
    pub fn alrmssr_iter(&self) -> impl Iterator<Item = &ALRMSSR> {
        (0..2)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(68).add(8 * n).cast() })
    }
    #[doc = "0x44 - Alarm A sub-second register"]
    #[inline(always)]
    pub const fn alrmassr(&self) -> &ALRMSSR {
        self.alrmssr(0)
    }
    #[doc = "0x4c - Alarm B sub-second register"]
    #[inline(always)]
    pub const fn alrmbssr(&self) -> &ALRMSSR {
        self.alrmssr(1)
    }
    #[doc = "0x50 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn sr(&self) -> &SR {
        &self.sr
    }
    #[doc = "0x54 - RTC non-secure masked interrupt status register"]
    #[inline(always)]
    pub const fn misr(&self) -> &MISR {
        &self.misr
    }
    #[doc = "0x58 - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn smisr(&self) -> &SMISR {
        &self.smisr
    }
    #[doc = "0x5c - This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
    #[inline(always)]
    pub const fn scr(&self) -> &SCR {
        &self.scr
    }
    #[doc = "0x60 - RTC configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &CFGR {
        &self.cfgr
    }
    #[doc = "0x3f0 - RTC hardware configuration register"]
    #[inline(always)]
    pub const fn hwcfgr(&self) -> &HWCFGR {
        &self.hwcfgr
    }
    #[doc = "0x3f4 - RTC version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - RTC identification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x3fc - RTC size identification register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "TR (rw) register accessor: The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tr`]
module"]
pub type TR = crate::Reg<tr::TRrs>;
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod tr;
#[doc = "DR (rw) register accessor: The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`]
module"]
pub type DR = crate::Reg<dr::DRrs>;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page1830 and Reading the calendar on page1831. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod dr;
#[doc = "SSR (r) register accessor: RTC sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ssr`]
module"]
pub type SSR = crate::Reg<ssr::SSRrs>;
#[doc = "RTC sub second register"]
pub mod ssr;
#[doc = "ICSR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`icsr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`icsr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icsr`]
module"]
pub type ICSR = crate::Reg<icsr::ICSRrs>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod icsr;
#[doc = "PRER (rw) register accessor: This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prer::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prer::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prer`]
module"]
pub type PRER = crate::Reg<prer::PRERrs>;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page1830. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod prer;
#[doc = "WUTR (rw) register accessor: This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wutr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wutr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wutr`]
module"]
pub type WUTR = crate::Reg<wutr::WUTRrs>;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ICSR. This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod wutr;
#[doc = "CR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be globally protected, or each bit of this register can be individually protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod cr;
#[doc = "SMCR (rw) register accessor: This register can be written only when the APB access is secure.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smcr`]
module"]
pub type SMCR = crate::Reg<smcr::SMCRrs>;
#[doc = "This register can be written only when the APB access is secure."]
pub mod smcr;
#[doc = "WPR (w) register accessor: RTC write protection register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpr`]
module"]
pub type WPR = crate::Reg<wpr::WPRrs>;
#[doc = "RTC write protection register"]
pub mod wpr;
#[doc = "CALR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`calr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`calr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calr`]
module"]
pub type CALR = crate::Reg<calr::CALRrs>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be write-protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod calr;
#[doc = "SHIFTR (w) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shiftr`]
module"]
pub type SHIFTR = crate::Reg<shiftr::SHIFTRrs>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page1830. This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod shiftr;
pub use dr as tsdr;
pub use ssr as tsssr;
pub use tr as tstr;
pub use DR as TSDR;
pub use SSR as TSSSR;
pub use TR as TSTR;
#[doc = "ALRMR (rw) register accessor: Alarm %s register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmr`]
module"]
pub type ALRMR = crate::Reg<alrmr::ALRMRrs>;
#[doc = "Alarm %s register"]
pub mod alrmr;
#[doc = "ALRMSSR (rw) register accessor: Alarm %s sub-second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmssr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmssr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@alrmssr`]
module"]
pub type ALRMSSR = crate::Reg<alrmssr::ALRMSSRrs>;
#[doc = "Alarm %s sub-second register"]
pub mod alrmssr;
#[doc = "SR (r) register accessor: This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
pub type SR = crate::Reg<sr::SRrs>;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod sr;
#[doc = "MISR (r) register accessor: RTC non-secure masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@misr`]
module"]
pub type MISR = crate::Reg<misr::MISRrs>;
#[doc = "RTC non-secure masked interrupt status register"]
pub mod misr;
#[doc = "SMISR (r) register accessor: This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smisr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smisr`]
module"]
pub type SMISR = crate::Reg<smisr::SMISRrs>;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod smisr;
#[doc = "SCR (w) register accessor: This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`scr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scr`]
module"]
pub type SCR = crate::Reg<scr::SCRrs>;
#[doc = "This register can be protected against non-secure access. Refer to Section50.3.4: RTC secure protection modes."]
pub mod scr;
#[doc = "CFGR (rw) register accessor: RTC configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cfgr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`]
module"]
pub type CFGR = crate::Reg<cfgr::CFGRrs>;
#[doc = "RTC configuration register"]
pub mod cfgr;
#[doc = "HWCFGR (r) register accessor: RTC hardware configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr`]
module"]
pub type HWCFGR = crate::Reg<hwcfgr::HWCFGRrs>;
#[doc = "RTC hardware configuration register"]
pub mod hwcfgr;
#[doc = "VERR (r) register accessor: RTC version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "RTC version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: RTC identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "RTC identification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: RTC size identification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "RTC size identification register"]
pub mod sidr;
