#[doc = "Register `GNPTXFSIZ_Device` reader"]
pub type R = crate::R<GNPTXFSIZ_DEVICErs>;
#[doc = "Register `GNPTXFSIZ_Device` writer"]
pub type W = crate::W<GNPTXFSIZ_DEVICErs>;
#[doc = "Field `TX0FSA` reader - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FSA` writer - Endpoint 0 transmit RAM start address"]
pub type TX0FSA_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TX0FD` reader - Endpoint 0 TxFIFO depth"]
pub type TX0FD_R = crate::FieldReader<u16>;
#[doc = "Field `TX0FD` writer - Endpoint 0 TxFIFO depth"]
pub type TX0FD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    pub fn tx0fsa(&self) -> TX0FSA_R {
        TX0FSA_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    pub fn tx0fd(&self) -> TX0FD_R {
        TX0FD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Endpoint 0 transmit RAM start address"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fsa(&mut self) -> TX0FSA_W<GNPTXFSIZ_DEVICErs> {
        TX0FSA_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Endpoint 0 TxFIFO depth"]
    #[inline(always)]
    #[must_use]
    pub fn tx0fd(&mut self) -> TX0FD_W<GNPTXFSIZ_DEVICErs> {
        TX0FD_W::new(self, 16)
    }
}
#[doc = "OTG_FS non-periodic transmit FIFO size register (Device mode)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gnptxfsiz_device::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gnptxfsiz_device::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GNPTXFSIZ_DEVICErs;
impl crate::RegisterSpec for GNPTXFSIZ_DEVICErs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gnptxfsiz_device::R`](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_DEVICErs {}
#[doc = "`write(|w| ..)` method takes [`gnptxfsiz_device::W`](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_DEVICErs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GNPTXFSIZ_Device to value 0x0200"]
impl crate::Resettable for GNPTXFSIZ_DEVICErs {
    const RESET_VALUE: u32 = 0x0200;
}
