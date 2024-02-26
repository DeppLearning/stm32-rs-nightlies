#[doc = "Register `TARG7_FN_MOD` reader"]
pub type R = crate::R<TARG7_FN_MODrs>;
#[doc = "Register `TARG7_FN_MOD` writer"]
pub type W = crate::W<TARG7_FN_MODrs>;
#[doc = "Field `READ_ISS_OVERRIDE` reader - Override AMIB read issuing capability"]
pub type READ_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `READ_ISS_OVERRIDE` writer - Override AMIB read issuing capability"]
pub type READ_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_ISS_OVERRIDE` reader - Override AMIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_R = crate::BitReader;
#[doc = "Field `WRITE_ISS_OVERRIDE` writer - Override AMIB write issuing capability"]
pub type WRITE_ISS_OVERRIDE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Override AMIB read issuing capability"]
    #[inline(always)]
    pub fn read_iss_override(&self) -> READ_ISS_OVERRIDE_R {
        READ_ISS_OVERRIDE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Override AMIB write issuing capability"]
    #[inline(always)]
    pub fn write_iss_override(&self) -> WRITE_ISS_OVERRIDE_R {
        WRITE_ISS_OVERRIDE_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Override AMIB read issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn read_iss_override(&mut self) -> READ_ISS_OVERRIDE_W<TARG7_FN_MODrs> {
        READ_ISS_OVERRIDE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Override AMIB write issuing capability"]
    #[inline(always)]
    #[must_use]
    pub fn write_iss_override(&mut self) -> WRITE_ISS_OVERRIDE_W<TARG7_FN_MODrs> {
        WRITE_ISS_OVERRIDE_W::new(self, 1)
    }
}
#[doc = "AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`targ7_fn_mod::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`targ7_fn_mod::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TARG7_FN_MODrs;
impl crate::RegisterSpec for TARG7_FN_MODrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`targ7_fn_mod::R`](R) reader structure"]
impl crate::Readable for TARG7_FN_MODrs {}
#[doc = "`write(|w| ..)` method takes [`targ7_fn_mod::W`](W) writer structure"]
impl crate::Writable for TARG7_FN_MODrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TARG7_FN_MOD to value 0x04"]
impl crate::Resettable for TARG7_FN_MODrs {
    const RESET_VALUE: u32 = 0x04;
}
