#[doc = "Register `SAR_DEBUG_CONF` reader"]
pub type R = crate::R<SAR_DEBUG_CONF_SPEC>;
#[doc = "Register `SAR_DEBUG_CONF` writer"]
pub type W = crate::W<SAR_DEBUG_CONF_SPEC>;
#[doc = "Field `SAR_DEBUG_BIT_SEL` reader - no public"]
pub type SAR_DEBUG_BIT_SEL_R = crate::FieldReader;
#[doc = "Field `SAR_DEBUG_BIT_SEL` writer - no public"]
pub type SAR_DEBUG_BIT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - no public"]
    #[inline(always)]
    pub fn sar_debug_bit_sel(&self) -> SAR_DEBUG_BIT_SEL_R {
        SAR_DEBUG_BIT_SEL_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_DEBUG_CONF")
            .field(
                "sar_debug_bit_sel",
                &format_args!("{}", self.sar_debug_bit_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_DEBUG_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - no public"]
    #[inline(always)]
    #[must_use]
    pub fn sar_debug_bit_sel(&mut self) -> SAR_DEBUG_BIT_SEL_W<SAR_DEBUG_CONF_SPEC> {
        SAR_DEBUG_BIT_SEL_W::new(self, 0)
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
#[doc = "rtc peri debug configure\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_debug_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sar_debug_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_DEBUG_CONF_SPEC;
impl crate::RegisterSpec for SAR_DEBUG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_debug_conf::R`](R) reader structure"]
impl crate::Readable for SAR_DEBUG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar_debug_conf::W`](W) writer structure"]
impl crate::Writable for SAR_DEBUG_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SAR_DEBUG_CONF to value 0"]
impl crate::Resettable for SAR_DEBUG_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
