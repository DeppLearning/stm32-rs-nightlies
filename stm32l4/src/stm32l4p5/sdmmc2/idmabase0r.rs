#[doc = "Register `IDMABASE0R` reader"]
pub type R = crate::R<IDMABASE0Rrs>;
#[doc = "Register `IDMABASE0R` writer"]
pub type W = crate::W<IDMABASE0Rrs>;
#[doc = "Field `IDMABASE0` reader - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
pub type IDMABASE0_R = crate::FieldReader<u32>;
#[doc = "Field `IDMABASE0` writer - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
pub type IDMABASE0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
    #[inline(always)]
    pub fn idmabase0(&self) -> IDMABASE0_R {
        IDMABASE0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Buffer 0 memory base address bits \\[31:2\\], shall be word aligned (bit \\[1:0\\]
are always 0 and read only)"]
    #[inline(always)]
    #[must_use]
    pub fn idmabase0(&mut self) -> IDMABASE0_W<IDMABASE0Rrs> {
        IDMABASE0_W::new(self, 0)
    }
}
#[doc = "IDMA buffer 0 base address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`idmabase0r::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`idmabase0r::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IDMABASE0Rrs;
impl crate::RegisterSpec for IDMABASE0Rrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`idmabase0r::R`](R) reader structure"]
impl crate::Readable for IDMABASE0Rrs {}
#[doc = "`write(|w| ..)` method takes [`idmabase0r::W`](W) writer structure"]
impl crate::Writable for IDMABASE0Rrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IDMABASE0R to value 0"]
impl crate::Resettable for IDMABASE0Rrs {
    const RESET_VALUE: u32 = 0;
}
