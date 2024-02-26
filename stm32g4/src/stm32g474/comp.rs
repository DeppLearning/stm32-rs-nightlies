#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    c1csr: C1CSR,
    c2csr: C2CSR,
    c3csr: C3CSR,
    c4csr: C4CSR,
    c5csr: C5CSR,
    c6csr: C6CSR,
    c7csr: C7CSR,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c1csr(&self) -> &C1CSR {
        &self.c1csr
    }
    #[doc = "0x04 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c2csr(&self) -> &C2CSR {
        &self.c2csr
    }
    #[doc = "0x08 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c3csr(&self) -> &C3CSR {
        &self.c3csr
    }
    #[doc = "0x0c - Comparator control/status register"]
    #[inline(always)]
    pub const fn c4csr(&self) -> &C4CSR {
        &self.c4csr
    }
    #[doc = "0x10 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c5csr(&self) -> &C5CSR {
        &self.c5csr
    }
    #[doc = "0x14 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c6csr(&self) -> &C6CSR {
        &self.c6csr
    }
    #[doc = "0x18 - Comparator control/status register"]
    #[inline(always)]
    pub const fn c7csr(&self) -> &C7CSR {
        &self.c7csr
    }
}
#[doc = "C1CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c1csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c1csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1csr`]
module"]
pub type C1CSR = crate::Reg<c1csr::C1CSRrs>;
#[doc = "Comparator control/status register"]
pub mod c1csr;
#[doc = "C2CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c2csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c2csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c2csr`]
module"]
pub type C2CSR = crate::Reg<c2csr::C2CSRrs>;
#[doc = "Comparator control/status register"]
pub mod c2csr;
#[doc = "C3CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c3csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c3csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c3csr`]
module"]
pub type C3CSR = crate::Reg<c3csr::C3CSRrs>;
#[doc = "Comparator control/status register"]
pub mod c3csr;
#[doc = "C4CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c4csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c4csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c4csr`]
module"]
pub type C4CSR = crate::Reg<c4csr::C4CSRrs>;
#[doc = "Comparator control/status register"]
pub mod c4csr;
#[doc = "C5CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c5csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c5csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c5csr`]
module"]
pub type C5CSR = crate::Reg<c5csr::C5CSRrs>;
#[doc = "Comparator control/status register"]
pub mod c5csr;
#[doc = "C6CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c6csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c6csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c6csr`]
module"]
pub type C6CSR = crate::Reg<c6csr::C6CSRrs>;
#[doc = "Comparator control/status register"]
pub mod c6csr;
#[doc = "C7CSR (rw) register accessor: Comparator control/status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`c7csr::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`c7csr::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c7csr`]
module"]
pub type C7CSR = crate::Reg<c7csr::C7CSRrs>;
#[doc = "Comparator control/status register"]
pub mod c7csr;
