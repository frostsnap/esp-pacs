#[doc = "Register `BLK3_RDATA3` reader"]
pub type R = crate::R<BLK3_RDATA3_SPEC>;
#[doc = "Register `BLK3_RDATA3` writer"]
pub type W = crate::W<BLK3_RDATA3_SPEC>;
#[doc = "Field `RD_ADC1_TP_LOW` reader - "]
pub type RD_ADC1_TP_LOW_R = crate::FieldReader;
#[doc = "Field `RD_ADC1_TP_LOW` writer - "]
pub type RD_ADC1_TP_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RD_ADC1_TP_HIGH` reader - "]
pub type RD_ADC1_TP_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `RD_ADC1_TP_HIGH` writer - "]
pub type RD_ADC1_TP_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RD_ADC2_TP_LOW` reader - "]
pub type RD_ADC2_TP_LOW_R = crate::FieldReader;
#[doc = "Field `RD_ADC2_TP_LOW` writer - "]
pub type RD_ADC2_TP_LOW_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `RD_ADC2_TP_HIGH` reader - "]
pub type RD_ADC2_TP_HIGH_R = crate::FieldReader<u16>;
#[doc = "Field `RD_ADC2_TP_HIGH` writer - "]
pub type RD_ADC2_TP_HIGH_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
impl R {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    pub fn rd_adc1_tp_low(&self) -> RD_ADC1_TP_LOW_R {
        RD_ADC1_TP_LOW_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    pub fn rd_adc1_tp_high(&self) -> RD_ADC1_TP_HIGH_R {
        RD_ADC1_TP_HIGH_R::new(((self.bits >> 7) & 0x01ff) as u16)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    pub fn rd_adc2_tp_low(&self) -> RD_ADC2_TP_LOW_R {
        RD_ADC2_TP_LOW_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    pub fn rd_adc2_tp_high(&self) -> RD_ADC2_TP_HIGH_R {
        RD_ADC2_TP_HIGH_R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK3_RDATA3")
            .field(
                "rd_adc1_tp_low",
                &format_args!("{}", self.rd_adc1_tp_low().bits()),
            )
            .field(
                "rd_adc1_tp_high",
                &format_args!("{}", self.rd_adc1_tp_high().bits()),
            )
            .field(
                "rd_adc2_tp_low",
                &format_args!("{}", self.rd_adc2_tp_low().bits()),
            )
            .field(
                "rd_adc2_tp_high",
                &format_args!("{}", self.rd_adc2_tp_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK3_RDATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:6"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc1_tp_low(&mut self) -> RD_ADC1_TP_LOW_W<BLK3_RDATA3_SPEC> {
        RD_ADC1_TP_LOW_W::new(self, 0)
    }
    #[doc = "Bits 7:15"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc1_tp_high(&mut self) -> RD_ADC1_TP_HIGH_W<BLK3_RDATA3_SPEC> {
        RD_ADC1_TP_HIGH_W::new(self, 7)
    }
    #[doc = "Bits 16:22"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc2_tp_low(&mut self) -> RD_ADC2_TP_LOW_W<BLK3_RDATA3_SPEC> {
        RD_ADC2_TP_LOW_W::new(self, 16)
    }
    #[doc = "Bits 23:31"]
    #[inline(always)]
    #[must_use]
    pub fn rd_adc2_tp_high(&mut self) -> RD_ADC2_TP_HIGH_W<BLK3_RDATA3_SPEC> {
        RD_ADC2_TP_HIGH_W::new(self, 23)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk3_rdata3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk3_rdata3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK3_RDATA3_SPEC;
impl crate::RegisterSpec for BLK3_RDATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk3_rdata3::R`](R) reader structure"]
impl crate::Readable for BLK3_RDATA3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk3_rdata3::W`](W) writer structure"]
impl crate::Writable for BLK3_RDATA3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK3_RDATA3 to value 0"]
impl crate::Resettable for BLK3_RDATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
