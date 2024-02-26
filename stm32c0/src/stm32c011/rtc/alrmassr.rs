#[doc = "Register `ALRMASSR` reader"]
pub type R = crate::R<ALRMASSRrs>;
#[doc = "Register `ALRMASSR` writer"]
pub type W = crate::W<ALRMASSRrs>;
#[doc = "Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared."]
pub type SS_R = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared."]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit 2: SS\\[14:2\\]
are don’t care in alarm A comparison. Only SS\\[1:0\\]
are compared. 3: SS\\[14:3\\]
are don’t care in alarm A comparison. Only SS\\[2:0\\]
are compared. ... 12: SS\\[14:12\\]
are don’t care in alarm A comparison. SS\\[11:0\\]
are compared. 13: SS\\[14:13\\]
are don’t care in alarm A comparison. SS\\[12:0\\]
are compared. 14: SS\\[14\\]
is don’t care in alarm A comparison. SS\\[13:0\\]
are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MASKSS_R = crate::FieldReader;
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit 2: SS\\[14:2\\]
are don’t care in alarm A comparison. Only SS\\[1:0\\]
are compared. 3: SS\\[14:3\\]
are don’t care in alarm A comparison. Only SS\\[2:0\\]
are compared. ... 12: SS\\[14:12\\]
are don’t care in alarm A comparison. SS\\[11:0\\]
are compared. 13: SS\\[14:13\\]
are don’t care in alarm A comparison. SS\\[12:0\\]
are compared. 14: SS\\[14\\]
is don’t care in alarm A comparison. SS\\[13:0\\]
are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared."]
    #[inline(always)]
    pub fn ss(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit 2: SS\\[14:2\\]
are don’t care in alarm A comparison. Only SS\\[1:0\\]
are compared. 3: SS\\[14:3\\]
are don’t care in alarm A comparison. Only SS\\[2:0\\]
are compared. ... 12: SS\\[14:12\\]
are don’t care in alarm A comparison. SS\\[11:0\\]
are compared. 13: SS\\[14:13\\]
are don’t care in alarm A comparison. SS\\[12:0\\]
are compared. 14: SS\\[14\\]
is don’t care in alarm A comparison. SS\\[13:0\\]
are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn maskss(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared."]
    #[inline(always)]
    #[must_use]
    pub fn ss(&mut self) -> SS_W<ALRMASSRrs> {
        SS_W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit 2: SS\\[14:2\\]
are don’t care in alarm A comparison. Only SS\\[1:0\\]
are compared. 3: SS\\[14:3\\]
are don’t care in alarm A comparison. Only SS\\[2:0\\]
are compared. ... 12: SS\\[14:12\\]
are don’t care in alarm A comparison. SS\\[11:0\\]
are compared. 13: SS\\[14:13\\]
are don’t care in alarm A comparison. SS\\[12:0\\]
are compared. 14: SS\\[14\\]
is don’t care in alarm A comparison. SS\\[13:0\\]
are compared. 15: All 15 SS bits are compared and must match to activate alarm. The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation. Note: The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    #[must_use]
    pub fn maskss(&mut self) -> MASKSS_W<ALRMASSRrs> {
        MASKSS_W::new(self, 24)
    }
}
#[doc = "RTC alarm A sub second register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`alrmassr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`alrmassr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMASSRrs;
impl crate::RegisterSpec for ALRMASSRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmassr::R`](R) reader structure"]
impl crate::Readable for ALRMASSRrs {}
#[doc = "`write(|w| ..)` method takes [`alrmassr::W`](W) writer structure"]
impl crate::Writable for ALRMASSRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ALRMASSR to value 0"]
impl crate::Resettable for ALRMASSRrs {
    const RESET_VALUE: u32 = 0;
}
