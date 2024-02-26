#[doc = "Register `SMCR` reader"]
pub type R = crate::R<SMCRrs>;
#[doc = "Register `SMCR` writer"]
pub type W = crate::W<SMCRrs>;
#[doc = "Field `SMS1` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS1_R = crate::FieldReader;
#[doc = "Field `SMS1` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `OCCS` reader - OCREF clear selection This bit is used to select the OCREF clear source"]
pub type OCCS_R = crate::BitReader;
#[doc = "Field `OCCS` writer - OCREF clear selection This bit is used to select the OCREF clear source"]
pub type OCCS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS1` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS1_R = crate::FieldReader;
#[doc = "Field `TS1` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MSM` reader - Master/Slave mode"]
pub type MSM_R = crate::BitReader;
#[doc = "Field `MSM` writer - Master/Slave mode"]
pub type MSM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETF` reader - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_R = crate::FieldReader;
#[doc = "Field `ETF` writer - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
pub type ETF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ETPS` reader - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
pub type ETPS_R = crate::FieldReader;
#[doc = "Field `ETPS` writer - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
pub type ETPS_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ECE` reader - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
pub type ECE_R = crate::BitReader;
#[doc = "Field `ECE` writer - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
pub type ECE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETP` reader - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
pub type ETP_R = crate::BitReader;
#[doc = "Field `ETP` writer - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
pub type ETP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMS2` reader - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS2_R = crate::BitReader;
#[doc = "Field `SMS2` writer - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
pub type SMS2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TS2` reader - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS2_R = crate::FieldReader;
#[doc = "Field `TS2` writer - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
pub type TS2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms1(&self) -> SMS1_R {
        SMS1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source"]
    #[inline(always)]
    pub fn occs(&self) -> OCCS_R {
        OCCS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts1(&self) -> TS1_R {
        TS1_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    pub fn msm(&self) -> MSM_R {
        MSM_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    pub fn etf(&self) -> ETF_R {
        ETF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
    #[inline(always)]
    pub fn etps(&self) -> ETPS_R {
        ETPS_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    pub fn ece(&self) -> ECE_R {
        ECE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
    #[inline(always)]
    pub fn etp(&self) -> ETP_R {
        ETP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    pub fn sms2(&self) -> SMS2_R {
        SMS2_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    pub fn ts2(&self) -> TS2_R {
        TS2_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms1(&mut self) -> SMS1_W<SMCRrs> {
        SMS1_W::new(self, 0)
    }
    #[doc = "Bit 3 - OCREF clear selection This bit is used to select the OCREF clear source"]
    #[inline(always)]
    #[must_use]
    pub fn occs(&mut self) -> OCCS_W<SMCRrs> {
        OCCS_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts1(&mut self) -> TS1_W<SMCRrs> {
        TS1_W::new(self, 4)
    }
    #[doc = "Bit 7 - Master/Slave mode"]
    #[inline(always)]
    #[must_use]
    pub fn msm(&mut self) -> MSM_W<SMCRrs> {
        MSM_W::new(self, 7)
    }
    #[doc = "Bits 8:11 - External trigger filter This bit-field then defines the frequency used to sample ETRP signal and the length of the digital filter applied to ETRP. The digital filter is made of an event counter in which N consecutive events are needed to validate a transition on the output:"]
    #[inline(always)]
    #[must_use]
    pub fn etf(&mut self) -> ETF_W<SMCRrs> {
        ETF_W::new(self, 8)
    }
    #[doc = "Bits 12:13 - External trigger prescaler External trigger signal ETRP frequency must be at most 1/4 of CK_INT frequency. A prescaler can be enabled to reduce ETRP frequency. It is useful when inputting fast external clocks."]
    #[inline(always)]
    #[must_use]
    pub fn etps(&mut self) -> ETPS_W<SMCRrs> {
        ETPS_W::new(self, 12)
    }
    #[doc = "Bit 14 - External clock enable This bit enables External clock mode 2. Note: Setting the ECE bit has the same effect as selecting external clock mode 1 with TRGI connected to ETRF (SMS=111 and TS=00111). It is possible to simultaneously use external clock mode 2 with the following slave modes: reset mode, gated mode and trigger mode. Nevertheless, TRGI must not be connected to ETRF in this case (TS bits must not be 00111). If external clock mode 1 and external clock mode 2 are enabled at the same time, the external clock input is ETRF."]
    #[inline(always)]
    #[must_use]
    pub fn ece(&mut self) -> ECE_W<SMCRrs> {
        ECE_W::new(self, 14)
    }
    #[doc = "Bit 15 - External trigger polarity This bit selects whether ETR or ETR is used for trigger operations"]
    #[inline(always)]
    #[must_use]
    pub fn etp(&mut self) -> ETP_W<SMCRrs> {
        ETP_W::new(self, 15)
    }
    #[doc = "Bit 16 - Slave mode selection When external signals are selected the active edge of the trigger signal (TRGI) is linked to the polarity selected on the external input (see Input Control register and Control Register description. reinitializes the counter, generates an update of the registers and starts the counter. Note: The gated mode must not be used if TI1F_ED is selected as the trigger input (TS=00100). Indeed, TI1F_ED outputs 1 pulse for each transition on TI1F, whereas the gated mode checks the level of the trigger signal. Note: The clock of the slave peripherals (timer, ADC, ...) receiving the TRGO or the TRGO2 signals must be enabled prior to receive events from the master timer, and the clock frequency (prescaler) must not be changed on-the-fly while triggers are received from the master timer."]
    #[inline(always)]
    #[must_use]
    pub fn sms2(&mut self) -> SMS2_W<SMCRrs> {
        SMS2_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Trigger selection This bit-field selects the trigger input to be used to synchronize the counter. Others: Reserved See for more details on ITRx meaning for each Timer. Note: These bits must be changed only when they are not used (e.g. when SMS=000) to avoid wrong edge detections at the transition."]
    #[inline(always)]
    #[must_use]
    pub fn ts2(&mut self) -> TS2_W<SMCRrs> {
        TS2_W::new(self, 20)
    }
}
#[doc = "TIM3 slave mode control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`smcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`smcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMCRrs;
impl crate::RegisterSpec for SMCRrs {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smcr::R`](R) reader structure"]
impl crate::Readable for SMCRrs {}
#[doc = "`write(|w| ..)` method takes [`smcr::W`](W) writer structure"]
impl crate::Writable for SMCRrs {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SMCR to value 0"]
impl crate::Resettable for SMCRrs {
    const RESET_VALUE: u32 = 0;
}
