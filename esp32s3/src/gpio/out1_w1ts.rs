#[doc = "Register `OUT1_W1TS` writer"]
pub type W = crate::W<OUT1_W1TS_SPEC>;
#[doc = "Field `OUT1_W1TS` writer - GPIO output set register for GPIO32-53"]
pub type OUT1_W1TS_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT1_W1TS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:21 - GPIO output set register for GPIO32-53"]
    #[inline(always)]
    #[must_use]
    pub fn out1_w1ts(&mut self) -> OUT1_W1TS_W<OUT1_W1TS_SPEC> {
        OUT1_W1TS_W::new(self, 0)
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
#[doc = "GPIO output set register for GPIO32-53\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out1_w1ts::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT1_W1TS_SPEC;
impl crate::RegisterSpec for OUT1_W1TS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`out1_w1ts::W`](W) writer structure"]
impl crate::Writable for OUT1_W1TS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT1_W1TS to value 0"]
impl crate::Resettable for OUT1_W1TS_SPEC {
    const RESET_VALUE: u32 = 0;
}
