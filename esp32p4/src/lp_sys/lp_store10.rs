#[doc = "Register `LP_STORE10` reader"]
pub type R = crate::R<LP_STORE10_SPEC>;
#[doc = "Register `LP_STORE10` writer"]
pub type W = crate::W<LP_STORE10_SPEC>;
#[doc = "Field `LP_SCRATCH10` reader - need_des"]
pub type LP_SCRATCH10_R = crate::FieldReader<u32>;
#[doc = "Field `LP_SCRATCH10` writer - need_des"]
pub type LP_SCRATCH10_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_scratch10(&self) -> LP_SCRATCH10_R {
        LP_SCRATCH10_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_STORE10")
            .field(
                "lp_scratch10",
                &format_args!("{}", self.lp_scratch10().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_STORE10_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_scratch10(&mut self) -> LP_SCRATCH10_W<LP_STORE10_SPEC> {
        LP_SCRATCH10_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_store10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_store10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_STORE10_SPEC;
impl crate::RegisterSpec for LP_STORE10_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_store10::R`](R) reader structure"]
impl crate::Readable for LP_STORE10_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_store10::W`](W) writer structure"]
impl crate::Writable for LP_STORE10_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_STORE10 to value 0"]
impl crate::Resettable for LP_STORE10_SPEC {
    const RESET_VALUE: u32 = 0;
}
