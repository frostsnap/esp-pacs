#[doc = "Register `HW_CFG` reader"]
pub type R = crate::R<HW_CFG_SPEC>;
#[doc = "Register `HW_CFG` writer"]
pub type W = crate::W<HW_CFG_SPEC>;
#[doc = "Field `HW_STANDBY_EN` reader - Enable function that hardware control standby pin."]
pub type HW_STANDBY_EN_R = crate::BitReader;
#[doc = "Field `HW_STANDBY_EN` writer - Enable function that hardware control standby pin."]
pub type HW_STANDBY_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enable function that hardware control standby pin."]
    #[inline(always)]
    pub fn hw_standby_en(&self) -> HW_STANDBY_EN_R {
        HW_STANDBY_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HW_CFG")
            .field(
                "hw_standby_en",
                &format_args!("{}", self.hw_standby_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HW_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Enable function that hardware control standby pin."]
    #[inline(always)]
    #[must_use]
    pub fn hw_standby_en(&mut self) -> HW_STANDBY_EN_W<HW_CFG_SPEC> {
        HW_STANDBY_EN_W::new(self, 0)
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
#[doc = "Hardware configure standby pin.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hw_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hw_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HW_CFG_SPEC;
impl crate::RegisterSpec for HW_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hw_cfg::R`](R) reader structure"]
impl crate::Readable for HW_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hw_cfg::W`](W) writer structure"]
impl crate::Writable for HW_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HW_CFG to value 0"]
impl crate::Resettable for HW_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
