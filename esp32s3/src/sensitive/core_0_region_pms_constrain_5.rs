#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_5` reader"]
pub type R = crate::R<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Register `CORE_0_REGION_PMS_CONSTRAIN_5` writer"]
pub type W = crate::W<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_2` reader - Region 1 end address and Region 2 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_R = crate::FieldReader<u32>;
#[doc = "Field `CORE_0_REGION_PMS_CONSTRAIN_ADDR_2` writer - Region 1 end address and Region 2 start address for core0."]
pub type CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Region 1 end address and Region 2 start address for core0."]
    #[inline(always)]
    pub fn core_0_region_pms_constrain_addr_2(&self) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_R {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_R::new(self.bits & 0x3fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_REGION_PMS_CONSTRAIN_5")
            .field(
                "core_0_region_pms_constrain_addr_2",
                &format_args!("{}", self.core_0_region_pms_constrain_addr_2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:29 - Region 1 end address and Region 2 start address for core0."]
    #[inline(always)]
    #[must_use]
    pub fn core_0_region_pms_constrain_addr_2(
        &mut self,
    ) -> CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_W<CORE_0_REGION_PMS_CONSTRAIN_5_SPEC> {
        CORE_0_REGION_PMS_CONSTRAIN_ADDR_2_W::new(self, 0)
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
#[doc = "Core0 region permission register 5.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_0_region_pms_constrain_5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_0_region_pms_constrain_5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_REGION_PMS_CONSTRAIN_5_SPEC;
impl crate::RegisterSpec for CORE_0_REGION_PMS_CONSTRAIN_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_region_pms_constrain_5::R`](R) reader structure"]
impl crate::Readable for CORE_0_REGION_PMS_CONSTRAIN_5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_0_region_pms_constrain_5::W`](W) writer structure"]
impl crate::Writable for CORE_0_REGION_PMS_CONSTRAIN_5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_0_REGION_PMS_CONSTRAIN_5 to value 0"]
impl crate::Resettable for CORE_0_REGION_PMS_CONSTRAIN_5_SPEC {
    const RESET_VALUE: u32 = 0;
}
