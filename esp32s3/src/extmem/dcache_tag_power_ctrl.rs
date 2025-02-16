#[doc = "Register `DCACHE_TAG_POWER_CTRL` reader"]
pub type R = crate::R<DCACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "Register `DCACHE_TAG_POWER_CTRL` writer"]
pub type W = crate::W<DCACHE_TAG_POWER_CTRL_SPEC>;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_ON` reader - The bit is used to close clock gating of dcache tag memory. 1: close gating, 0: open clock gating."]
pub type DCACHE_TAG_MEM_FORCE_ON_R = crate::BitReader;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_ON` writer - The bit is used to close clock gating of dcache tag memory. 1: close gating, 0: open clock gating."]
pub type DCACHE_TAG_MEM_FORCE_ON_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_PD` reader - The bit is used to power dcache tag memory down, 0: follow rtc_lslp_pd, 1: power down"]
pub type DCACHE_TAG_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_PD` writer - The bit is used to power dcache tag memory down, 0: follow rtc_lslp_pd, 1: power down"]
pub type DCACHE_TAG_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_PU` reader - The bit is used to power dcache tag memory up, 0: follow rtc_lslp_pd, 1: power up"]
pub type DCACHE_TAG_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `DCACHE_TAG_MEM_FORCE_PU` writer - The bit is used to power dcache tag memory up, 0: follow rtc_lslp_pd, 1: power up"]
pub type DCACHE_TAG_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The bit is used to close clock gating of dcache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    pub fn dcache_tag_mem_force_on(&self) -> DCACHE_TAG_MEM_FORCE_ON_R {
        DCACHE_TAG_MEM_FORCE_ON_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The bit is used to power dcache tag memory down, 0: follow rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    pub fn dcache_tag_mem_force_pd(&self) -> DCACHE_TAG_MEM_FORCE_PD_R {
        DCACHE_TAG_MEM_FORCE_PD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The bit is used to power dcache tag memory up, 0: follow rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    pub fn dcache_tag_mem_force_pu(&self) -> DCACHE_TAG_MEM_FORCE_PU_R {
        DCACHE_TAG_MEM_FORCE_PU_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_TAG_POWER_CTRL")
            .field(
                "dcache_tag_mem_force_on",
                &format_args!("{}", self.dcache_tag_mem_force_on().bit()),
            )
            .field(
                "dcache_tag_mem_force_pd",
                &format_args!("{}", self.dcache_tag_mem_force_pd().bit()),
            )
            .field(
                "dcache_tag_mem_force_pu",
                &format_args!("{}", self.dcache_tag_mem_force_pu().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_TAG_POWER_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to close clock gating of dcache tag memory. 1: close gating, 0: open clock gating."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_tag_mem_force_on(
        &mut self,
    ) -> DCACHE_TAG_MEM_FORCE_ON_W<DCACHE_TAG_POWER_CTRL_SPEC> {
        DCACHE_TAG_MEM_FORCE_ON_W::new(self, 0)
    }
    #[doc = "Bit 1 - The bit is used to power dcache tag memory down, 0: follow rtc_lslp_pd, 1: power down"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_tag_mem_force_pd(
        &mut self,
    ) -> DCACHE_TAG_MEM_FORCE_PD_W<DCACHE_TAG_POWER_CTRL_SPEC> {
        DCACHE_TAG_MEM_FORCE_PD_W::new(self, 1)
    }
    #[doc = "Bit 2 - The bit is used to power dcache tag memory up, 0: follow rtc_lslp_pd, 1: power up"]
    #[inline(always)]
    #[must_use]
    pub fn dcache_tag_mem_force_pu(
        &mut self,
    ) -> DCACHE_TAG_MEM_FORCE_PU_W<DCACHE_TAG_POWER_CTRL_SPEC> {
        DCACHE_TAG_MEM_FORCE_PU_W::new(self, 2)
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
#[doc = "******* Description ***********\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dcache_tag_power_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dcache_tag_power_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DCACHE_TAG_POWER_CTRL_SPEC;
impl crate::RegisterSpec for DCACHE_TAG_POWER_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcache_tag_power_ctrl::R`](R) reader structure"]
impl crate::Readable for DCACHE_TAG_POWER_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dcache_tag_power_ctrl::W`](W) writer structure"]
impl crate::Writable for DCACHE_TAG_POWER_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCACHE_TAG_POWER_CTRL to value 0x05"]
impl crate::Resettable for DCACHE_TAG_POWER_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
