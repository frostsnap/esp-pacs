#[doc = "Register `DLL_CONF` reader"]
pub type R = crate::R<DLL_CONF_SPEC>;
#[doc = "Register `DLL_CONF` writer"]
pub type W = crate::W<DLL_CONF_SPEC>;
#[doc = "Field `DLL_CAL_STOP` reader - Set 1 to stop calibration."]
pub type DLL_CAL_STOP_R = crate::BitReader;
#[doc = "Field `DLL_CAL_STOP` writer - Set 1 to stop calibration."]
pub type DLL_CAL_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLL_CAL_END` reader - 1 means calibration finished."]
pub type DLL_CAL_END_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to stop calibration."]
    #[inline(always)]
    pub fn dll_cal_stop(&self) -> DLL_CAL_STOP_R {
        DLL_CAL_STOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1 means calibration finished."]
    #[inline(always)]
    pub fn dll_cal_end(&self) -> DLL_CAL_END_R {
        DLL_CAL_END_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DLL_CONF")
            .field(
                "dll_cal_stop",
                &format_args!("{}", self.dll_cal_stop().bit()),
            )
            .field("dll_cal_end", &format_args!("{}", self.dll_cal_end().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DLL_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to stop calibration."]
    #[inline(always)]
    #[must_use]
    pub fn dll_cal_stop(&mut self) -> DLL_CAL_STOP_W<DLL_CONF_SPEC> {
        DLL_CAL_STOP_W::new(self, 0)
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
#[doc = "SDIO DLL configuration register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dll_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dll_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DLL_CONF_SPEC;
impl crate::RegisterSpec for DLL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dll_conf::R`](R) reader structure"]
impl crate::Readable for DLL_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dll_conf::W`](W) writer structure"]
impl crate::Writable for DLL_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DLL_CONF to value 0"]
impl crate::Resettable for DLL_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
