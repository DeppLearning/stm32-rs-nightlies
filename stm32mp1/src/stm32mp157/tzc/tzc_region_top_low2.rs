#[doc = "Register `TZC_REGION_TOP_LOW2` reader"]
pub type R = crate::R<TZC_REGION_TOP_LOW2rs>;
#[doc = "Register `TZC_REGION_TOP_LOW2` writer"]
pub type W = crate::W<TZC_REGION_TOP_LOW2rs>;
#[doc = "Field `TOP_ADDRESS_LOW` reader - TOP_ADDRESS_LOW"]
pub type TOP_ADDRESS_LOW_R = crate::FieldReader<u32>;
#[doc = "Field `TOP_ADDRESS_LOW` writer - TOP_ADDRESS_LOW"]
pub type TOP_ADDRESS_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 12:31 - TOP_ADDRESS_LOW"]
    #[inline(always)]
    pub fn top_address_low(&self) -> TOP_ADDRESS_LOW_R {
        TOP_ADDRESS_LOW_R::new((self.bits >> 12) & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 12:31 - TOP_ADDRESS_LOW"]
    #[inline(always)]
    #[must_use]
    pub fn top_address_low(&mut self) -> TOP_ADDRESS_LOW_W<TZC_REGION_TOP_LOW2rs> {
        TOP_ADDRESS_LOW_W::new(self, 12)
    }
}
#[doc = "Top address bits \\[31:12\\]
for region x.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tzc_region_top_low2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tzc_region_top_low2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TZC_REGION_TOP_LOW2rs;
impl crate::RegisterSpec for TZC_REGION_TOP_LOW2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tzc_region_top_low2::R`](R) reader structure"]
impl crate::Readable for TZC_REGION_TOP_LOW2rs {}
#[doc = "`write(|w| ..)` method takes [`tzc_region_top_low2::W`](W) writer structure"]
impl crate::Writable for TZC_REGION_TOP_LOW2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TZC_REGION_TOP_LOW2 to value 0x0fff"]
impl crate::Resettable for TZC_REGION_TOP_LOW2rs {
    const RESET_VALUE: u32 = 0x0fff;
}
