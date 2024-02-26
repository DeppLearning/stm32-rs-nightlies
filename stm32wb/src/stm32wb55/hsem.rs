#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    r: [R; 32],
    rlr: [RLR; 32],
    c1ier: C1IER,
    c1icr: C1ICR,
    c1isr: C1ISR,
    c1misr: C1MISR,
    c2ier: C2IER,
    c2icr: C2ICR,
    c2isr: C2ISR,
    c2misr: C2MISR,
    _reserved10: [u8; 0x20],
    cr: CR,
    keyr: KEYR,
    _reserved12: [u8; 0x02a4],
    hwcfgr2: HWCFGR2,
    hwcfgr1: HWCFGR1,
    verr: VERR,
    ipidr: IPIDR,
    sidr: SIDR,
}
impl RegisterBlock {
    #[doc = "0x00..0x80 - HSEM register HSEM_R%s"]
    #[inline(always)]
    pub const fn r(&self, n: usize) -> &R {
        &self.r[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x80 - HSEM register HSEM_R%s"]
    #[inline(always)]
    pub fn r_iter(&self) -> impl Iterator<Item = &R> {
        self.r.iter()
    }
    #[doc = "0x80..0x100 - Semaphore %s read lock register"]
    #[inline(always)]
    pub const fn rlr(&self, n: usize) -> &RLR {
        &self.rlr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x80..0x100 - Semaphore %s read lock register"]
    #[inline(always)]
    pub fn rlr_iter(&self) -> impl Iterator<Item = &RLR> {
        self.rlr.iter()
    }
    #[doc = "0x100 - HSEM Interrupt enable register"]
    #[inline(always)]
    pub const fn c1ier(&self) -> &C1IER {
        &self.c1ier
    }
    #[doc = "0x104 - HSEM Interrupt clear register"]
    #[inline(always)]
    pub const fn c1icr(&self) -> &C1ICR {
        &self.c1icr
    }
    #[doc = "0x108 - HSEM Interrupt status register"]
    #[inline(always)]
    pub const fn c1isr(&self) -> &C1ISR {
        &self.c1isr
    }
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    #[inline(always)]
    pub const fn c1misr(&self) -> &C1MISR {
        &self.c1misr
    }
    #[doc = "0x110 - HSEM Interrupt enable register"]
    #[inline(always)]
    pub const fn c2ier(&self) -> &C2IER {
        &self.c2ier
    }
    #[doc = "0x114 - HSEM Interrupt clear register"]
    #[inline(always)]
    pub const fn c2icr(&self) -> &C2ICR {
        &self.c2icr
    }
    #[doc = "0x118 - HSEM Interrupt status register"]
    #[inline(always)]
    pub const fn c2isr(&self) -> &C2ISR {
        &self.c2isr
    }
    #[doc = "0x11c - HSEM Masked interrupt status register"]
    #[inline(always)]
    pub const fn c2misr(&self) -> &C2MISR {
        &self.c2misr
    }
    #[doc = "0x140 - Semaphore Clear register"]
    #[inline(always)]
    pub const fn cr(&self) -> &CR {
        &self.cr
    }
    #[doc = "0x144 - Interrupt clear register"]
    #[inline(always)]
    pub const fn keyr(&self) -> &KEYR {
        &self.keyr
    }
    #[doc = "0x3ec - Semaphore hardware configuration register 2"]
    #[inline(always)]
    pub const fn hwcfgr2(&self) -> &HWCFGR2 {
        &self.hwcfgr2
    }
    #[doc = "0x3f0 - Semaphore hardware configuration register 1"]
    #[inline(always)]
    pub const fn hwcfgr1(&self) -> &HWCFGR1 {
        &self.hwcfgr1
    }
    #[doc = "0x3f4 - HSEM version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &VERR {
        &self.verr
    }
    #[doc = "0x3f8 - HSEM indentification register"]
    #[inline(always)]
    pub const fn ipidr(&self) -> &IPIDR {
        &self.ipidr
    }
    #[doc = "0x3fc - HSEM size indentification register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &SIDR {
        &self.sidr
    }
}
#[doc = "R (rw) register accessor: HSEM register HSEM_R%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`r::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`r::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@r`]
module"]
pub type R = crate::Reg<r::Rrs>;
#[doc = "HSEM register HSEM_R%s"]
pub mod r;
#[doc = "RLR (r) register accessor: Semaphore %s read lock register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rlr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rlr`]
module"]
pub type RLR = crate::Reg<rlr::RLRrs>;
#[doc = "Semaphore %s read lock register"]
pub mod rlr;
#[doc = "CR (rw) register accessor: Semaphore Clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`]
module"]
pub type CR = crate::Reg<cr::CRrs>;
#[doc = "Semaphore Clear register"]
pub mod cr;
#[doc = "KEYR (rw) register accessor: Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`keyr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`keyr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr`]
module"]
pub type KEYR = crate::Reg<keyr::KEYRrs>;
#[doc = "Interrupt clear register"]
pub mod keyr;
#[doc = "HWCFGR2 (r) register accessor: Semaphore hardware configuration register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr2`]
module"]
pub type HWCFGR2 = crate::Reg<hwcfgr2::HWCFGR2rs>;
#[doc = "Semaphore hardware configuration register 2"]
pub mod hwcfgr2;
#[doc = "HWCFGR1 (r) register accessor: Semaphore hardware configuration register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwcfgr1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hwcfgr1`]
module"]
pub type HWCFGR1 = crate::Reg<hwcfgr1::HWCFGR1rs>;
#[doc = "Semaphore hardware configuration register 1"]
pub mod hwcfgr1;
#[doc = "VERR (r) register accessor: HSEM version register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`verr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`]
module"]
pub type VERR = crate::Reg<verr::VERRrs>;
#[doc = "HSEM version register"]
pub mod verr;
#[doc = "IPIDR (r) register accessor: HSEM indentification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ipidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ipidr`]
module"]
pub type IPIDR = crate::Reg<ipidr::IPIDRrs>;
#[doc = "HSEM indentification register"]
pub mod ipidr;
#[doc = "SIDR (r) register accessor: HSEM size indentification register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sidr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`]
module"]
pub type SIDR = crate::Reg<sidr::SIDRrs>;
#[doc = "HSEM size indentification register"]
pub mod sidr;
#[doc = "C1IER (rw) register accessor: HSEM Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1ier`]
module"]
pub type C1IER = crate::Reg<c1ier::C1IERrs>;
#[doc = "HSEM Interrupt enable register"]
pub mod c1ier;
#[doc = "C1ICR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1icr`]
module"]
pub type C1ICR = crate::Reg<c1icr::C1ICRrs>;
#[doc = "HSEM Interrupt clear register"]
pub mod c1icr;
#[doc = "C1ISR (r) register accessor: HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1isr`]
module"]
pub type C1ISR = crate::Reg<c1isr::C1ISRrs>;
#[doc = "HSEM Interrupt status register"]
pub mod c1isr;
#[doc = "C1MISR (r) register accessor: HSEM Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1misr`]
module"]
pub type C1MISR = crate::Reg<c1misr::C1MISRrs>;
#[doc = "HSEM Masked interrupt status register"]
pub mod c1misr;
#[doc = "C2IER (rw) register accessor: HSEM Interrupt enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2ier::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2ier::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2ier`]
module"]
pub type C2IER = crate::Reg<c2ier::C2IERrs>;
#[doc = "HSEM Interrupt enable register"]
pub mod c2ier;
#[doc = "C2ICR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2icr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2icr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2icr`]
module"]
pub type C2ICR = crate::Reg<c2icr::C2ICRrs>;
#[doc = "HSEM Interrupt clear register"]
pub mod c2icr;
#[doc = "C2ISR (r) register accessor: HSEM Interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2isr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2isr`]
module"]
pub type C2ISR = crate::Reg<c2isr::C2ISRrs>;
#[doc = "HSEM Interrupt status register"]
pub mod c2isr;
#[doc = "C2MISR (r) register accessor: HSEM Masked interrupt status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2misr::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2misr`]
module"]
pub type C2MISR = crate::Reg<c2misr::C2MISRrs>;
#[doc = "HSEM Masked interrupt status register"]
pub mod c2misr;
