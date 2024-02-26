#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtsr1: RTSR1,
    ftsr1: FTSR1,
    swier1: SWIER1,
    pr1: PR1,
    _reserved4: [u8; 0x10],
    rtsr2: RTSR2,
    ftsr2: FTSR2,
    swier2: SWIER2,
    pr2: PR2,
    _reserved8: [u8; 0x50],
    c1imr1: C1IMR1,
    c1emr1: C1EMR1,
    _reserved10: [u8; 0x08],
    c1imr2: C1IMR2,
    c1emr2: C1EMR2,
    _reserved12: [u8; 0x28],
    c2imr1: C2IMR1,
    c2emr1: C2EMR1,
    _reserved14: [u8; 0x08],
    c2imr2: C2IMR2,
    c2emr2: C2EMR2,
}
impl RegisterBlock {
    #[doc = "0x00 - rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr1(&self) -> &RTSR1 {
        &self.rtsr1
    }
    #[doc = "0x04 - falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr1(&self) -> &FTSR1 {
        &self.ftsr1
    }
    #[doc = "0x08 - software interrupt event register"]
    #[inline(always)]
    pub const fn swier1(&self) -> &SWIER1 {
        &self.swier1
    }
    #[doc = "0x0c - EXTI pending register"]
    #[inline(always)]
    pub const fn pr1(&self) -> &PR1 {
        &self.pr1
    }
    #[doc = "0x20 - rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr2(&self) -> &RTSR2 {
        &self.rtsr2
    }
    #[doc = "0x24 - falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr2(&self) -> &FTSR2 {
        &self.ftsr2
    }
    #[doc = "0x28 - software interrupt event register"]
    #[inline(always)]
    pub const fn swier2(&self) -> &SWIER2 {
        &self.swier2
    }
    #[doc = "0x2c - pending register"]
    #[inline(always)]
    pub const fn pr2(&self) -> &PR2 {
        &self.pr2
    }
    #[doc = "0x80 - interrupt mask register"]
    #[inline(always)]
    pub const fn c1imr1(&self) -> &C1IMR1 {
        &self.c1imr1
    }
    #[doc = "0x84 - event mask register"]
    #[inline(always)]
    pub const fn c1emr1(&self) -> &C1EMR1 {
        &self.c1emr1
    }
    #[doc = "0x90 - wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn c1imr2(&self) -> &C1IMR2 {
        &self.c1imr2
    }
    #[doc = "0x94 - wakeup with event mask register"]
    #[inline(always)]
    pub const fn c1emr2(&self) -> &C1EMR2 {
        &self.c1emr2
    }
    #[doc = "0xc0 - interrupt mask register"]
    #[inline(always)]
    pub const fn c2imr1(&self) -> &C2IMR1 {
        &self.c2imr1
    }
    #[doc = "0xc4 - event mask register"]
    #[inline(always)]
    pub const fn c2emr1(&self) -> &C2EMR1 {
        &self.c2emr1
    }
    #[doc = "0xd0 - wakeup with interrupt mask register"]
    #[inline(always)]
    pub const fn c2imr2(&self) -> &C2IMR2 {
        &self.c2imr2
    }
    #[doc = "0xd4 - wakeup with event mask register"]
    #[inline(always)]
    pub const fn c2emr2(&self) -> &C2EMR2 {
        &self.c2emr2
    }
}
#[doc = "RTSR1 (rw) register accessor: rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr1`]
module"]
pub type RTSR1 = crate::Reg<rtsr1::RTSR1rs>;
#[doc = "rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr1`]
module"]
pub type FTSR1 = crate::Reg<ftsr1::FTSR1rs>;
#[doc = "falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier1`]
module"]
pub type SWIER1 = crate::Reg<swier1::SWIER1rs>;
#[doc = "software interrupt event register"]
pub mod swier1;
#[doc = "PR1 (rw) register accessor: EXTI pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr1`]
module"]
pub type PR1 = crate::Reg<pr1::PR1rs>;
#[doc = "EXTI pending register"]
pub mod pr1;
#[doc = "RTSR2 (rw) register accessor: rising trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rtsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr2`]
module"]
pub type RTSR2 = crate::Reg<rtsr2::RTSR2rs>;
#[doc = "rising trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 (rw) register accessor: falling trigger selection register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ftsr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr2`]
module"]
pub type FTSR2 = crate::Reg<ftsr2::FTSR2rs>;
#[doc = "falling trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 (rw) register accessor: software interrupt event register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swier2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier2`]
module"]
pub type SWIER2 = crate::Reg<swier2::SWIER2rs>;
#[doc = "software interrupt event register"]
pub mod swier2;
#[doc = "PR2 (rw) register accessor: pending register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pr2`]
module"]
pub type PR2 = crate::Reg<pr2::PR2rs>;
#[doc = "pending register"]
pub mod pr2;
#[doc = "C1IMR1 (rw) register accessor: interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1imr1`]
module"]
pub type C1IMR1 = crate::Reg<c1imr1::C1IMR1rs>;
#[doc = "interrupt mask register"]
pub mod c1imr1;
#[doc = "C1EMR1 (rw) register accessor: event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1emr1`]
module"]
pub type C1EMR1 = crate::Reg<c1emr1::C1EMR1rs>;
#[doc = "event mask register"]
pub mod c1emr1;
#[doc = "C1IMR2 (rw) register accessor: wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1imr2`]
module"]
pub type C1IMR2 = crate::Reg<c1imr2::C1IMR2rs>;
#[doc = "wakeup with interrupt mask register"]
pub mod c1imr2;
#[doc = "C1EMR2 (rw) register accessor: wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1emr2`]
module"]
pub type C1EMR2 = crate::Reg<c1emr2::C1EMR2rs>;
#[doc = "wakeup with event mask register"]
pub mod c1emr2;
#[doc = "C2IMR1 (rw) register accessor: interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr1`]
module"]
pub type C2IMR1 = crate::Reg<c2imr1::C2IMR1rs>;
#[doc = "interrupt mask register"]
pub mod c2imr1;
#[doc = "C2EMR1 (rw) register accessor: event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2emr1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2emr1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2emr1`]
module"]
pub type C2EMR1 = crate::Reg<c2emr1::C2EMR1rs>;
#[doc = "event mask register"]
pub mod c2emr1;
#[doc = "C2IMR2 (rw) register accessor: wakeup with interrupt mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2imr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2imr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2imr2`]
module"]
pub type C2IMR2 = crate::Reg<c2imr2::C2IMR2rs>;
#[doc = "wakeup with interrupt mask register"]
pub mod c2imr2;
#[doc = "C2EMR2 (rw) register accessor: wakeup with event mask register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2emr2::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2emr2::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2emr2`]
module"]
pub type C2EMR2 = crate::Reg<c2emr2::C2EMR2rs>;
#[doc = "wakeup with event mask register"]
pub mod c2emr2;
