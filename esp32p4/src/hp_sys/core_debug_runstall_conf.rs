#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` reader"]
pub type R = crate::R<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Register `CORE_DEBUG_RUNSTALL_CONF` writer"]
pub type W = crate::W<CORE_DEBUG_RUNSTALL_CONF_SPEC>;
#[doc = "Field `CORE_DEBUG_RUNSTALL_ENABLE` reader - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
pub type CORE_DEBUG_RUNSTALL_ENABLE_R = crate::BitReader;
#[doc = "Field `CORE_DEBUG_RUNSTALL_ENABLE` writer - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
pub type CORE_DEBUG_RUNSTALL_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
    #[inline(always)]
    pub fn core_debug_runstall_enable(&self) -> CORE_DEBUG_RUNSTALL_ENABLE_R {
        CORE_DEBUG_RUNSTALL_ENABLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_DEBUG_RUNSTALL_CONF")
            .field(
                "core_debug_runstall_enable",
                &format_args!("{}", self.core_debug_runstall_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_DEBUG_RUNSTALL_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this field to 1 to enable debug runstall feature between HP-core and LP-core."]
    #[inline(always)]
    #[must_use]
    pub fn core_debug_runstall_enable(
        &mut self,
    ) -> CORE_DEBUG_RUNSTALL_ENABLE_W<CORE_DEBUG_RUNSTALL_CONF_SPEC> {
        CORE_DEBUG_RUNSTALL_ENABLE_W::new(self, 0)
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
#[doc = "Core Debug runstall configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_debug_runstall_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_debug_runstall_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_DEBUG_RUNSTALL_CONF_SPEC;
impl crate::RegisterSpec for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_debug_runstall_conf::R`](R) reader structure"]
impl crate::Readable for CORE_DEBUG_RUNSTALL_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_debug_runstall_conf::W`](W) writer structure"]
impl crate::Writable for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_DEBUG_RUNSTALL_CONF to value 0"]
impl crate::Resettable for CORE_DEBUG_RUNSTALL_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
