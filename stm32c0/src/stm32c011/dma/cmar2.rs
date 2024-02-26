#[doc = "Register `CMAR2` reader"]
pub type R = crate::R<CMAR2rs>;
#[doc = "Register `CMAR2` writer"]
pub type W = crate::W<CMAR2rs>;
#[doc = "Field `MA` reader - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type MA_R = crate::FieldReader<u32>;
#[doc = "Field `MA` writer - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
pub type MA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    pub fn ma(&self) -> MA_R {
        MA_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - peripheral address It contains the base address of the memory from/to which the data will be read/written. When MSIZE\\[1:0\\]
= 01 (16 bits), bit 0 of MA\\[31:0\\]
is ignored. Access is automatically aligned to a half-word address. When MSIZE = 10 (32 bits), bits 1 and 0 of MA\\[31:0\\]
are ignored. Access is automatically aligned to a word address. In memory-to-memory mode, this register identifies the memory source address if DIR = 1 and the memory destination address if DIR = 0. In peripheral-to-peripheral mode, this register identifies the peripheral source address DIR = 1 and the peripheral destination address if DIR = 0. Note: this register is set and cleared by software. It must not be written when the channel is enabled (EN = 1). It is not read-only when the channel is enabled (EN = 1)."]
    #[inline(always)]
    #[must_use]
    pub fn ma(&mut self) -> MA_W<CMAR2rs> {
        MA_W::new(self, 0)
    }
}
#[doc = "DMA channel 2 memory address register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cmar2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmar2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CMAR2rs;
impl crate::RegisterSpec for CMAR2rs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmar2::R`](R) reader structure"]
impl crate::Readable for CMAR2rs {}
#[doc = "`write(|w| ..)` method takes [`cmar2::W`](W) writer structure"]
impl crate::Writable for CMAR2rs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CMAR2 to value 0"]
impl crate::Resettable for CMAR2rs {
    const RESET_VALUE: u32 = 0;
}
