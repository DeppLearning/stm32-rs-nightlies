#[doc = "Register `TSCV` reader"]
pub type R = crate::R<TSCVrs>;
#[doc = "Register `TSCV` writer"]
pub type W = crate::W<TSCVrs>;
#[doc = "Field `TSC` reader - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\\[TSS\\]
= 01, the timestamp counter is incremented in multiples of CAN bit times \\[1 â\u{80}¦ 16\\]
depending on the configuration of TSCC\\[TCP\\]. A wrap around sets interrupt flag IR\\[TSW\\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact."]
pub type TSC_R = crate::FieldReader<u16>;
#[doc = "Field `TSC` writer - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\\[TSS\\]
= 01, the timestamp counter is incremented in multiples of CAN bit times \\[1 â\u{80}¦ 16\\]
depending on the configuration of TSCC\\[TCP\\]. A wrap around sets interrupt flag IR\\[TSW\\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact."]
pub type TSC_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\\[TSS\\]
= 01, the timestamp counter is incremented in multiples of CAN bit times \\[1 â\u{80}¦ 16\\]
depending on the configuration of TSCC\\[TCP\\]. A wrap around sets interrupt flag IR\\[TSW\\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact."]
    #[inline(always)]
    pub fn tsc(&self) -> TSC_R {
        TSC_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timestamp counter The internal/external timestamp counter value is captured on start of frame (both Rx and Tx). When TSCC\\[TSS\\]
= 01, the timestamp counter is incremented in multiples of CAN bit times \\[1 â\u{80}¦ 16\\]
depending on the configuration of TSCC\\[TCP\\]. A wrap around sets interrupt flag IR\\[TSW\\]. Write access resets the counter to 0. When TSCC.TSS = 10, TSC reflects the external timestamp counter value. A write access has no impact."]
    #[inline(always)]
    #[must_use]
    pub fn tsc(&mut self) -> TSC_W<TSCVrs> {
        TSC_W::new(self, 0)
    }
}
#[doc = "FDCAN timestamp counter value register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tscv::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tscv::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSCVrs;
impl crate::RegisterSpec for TSCVrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tscv::R`](R) reader structure"]
impl crate::Readable for TSCVrs {}
#[doc = "`write(|w| ..)` method takes [`tscv::W`](W) writer structure"]
impl crate::Writable for TSCVrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TSCV to value 0"]
impl crate::Resettable for TSCVrs {
    const RESET_VALUE: u32 = 0;
}
