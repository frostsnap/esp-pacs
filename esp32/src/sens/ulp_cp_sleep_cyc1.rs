#[doc = "Register `ULP_CP_SLEEP_CYC1` reader"]
pub type R = crate::R<ULP_CP_SLEEP_CYC1_SPEC>;
#[doc = "Register `ULP_CP_SLEEP_CYC1` writer"]
pub type W = crate::W<ULP_CP_SLEEP_CYC1_SPEC>;
#[doc = "Field `SLEEP_CYCLES_S1` reader - "]
pub type SLEEP_CYCLES_S1_R = crate::FieldReader<u32>;
#[doc = "Field `SLEEP_CYCLES_S1` writer - "]
pub type SLEEP_CYCLES_S1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn sleep_cycles_s1(&self) -> SLEEP_CYCLES_S1_R {
        SLEEP_CYCLES_S1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ULP_CP_SLEEP_CYC1")
            .field(
                "sleep_cycles_s1",
                &format_args!("{}", self.sleep_cycles_s1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ULP_CP_SLEEP_CYC1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_cycles_s1(&mut self) -> SLEEP_CYCLES_S1_W<ULP_CP_SLEEP_CYC1_SPEC> {
        SLEEP_CYCLES_S1_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ulp_cp_sleep_cyc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ulp_cp_sleep_cyc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ULP_CP_SLEEP_CYC1_SPEC;
impl crate::RegisterSpec for ULP_CP_SLEEP_CYC1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ulp_cp_sleep_cyc1::R`](R) reader structure"]
impl crate::Readable for ULP_CP_SLEEP_CYC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ulp_cp_sleep_cyc1::W`](W) writer structure"]
impl crate::Writable for ULP_CP_SLEEP_CYC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ULP_CP_SLEEP_CYC1 to value 0x64"]
impl crate::Resettable for ULP_CP_SLEEP_CYC1_SPEC {
    const RESET_VALUE: u32 = 0x64;
}
