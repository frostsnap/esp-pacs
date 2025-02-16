#[doc = "Register `AF_THRESHOLD` reader"]
pub type R = crate::R<AF_THRESHOLD_SPEC>;
#[doc = "Register `AF_THRESHOLD` writer"]
pub type W = crate::W<AF_THRESHOLD_SPEC>;
#[doc = "Field `AF_THRESHOLD` reader - this field configures user threshold. When set to non-zero, autofocus will use this threshold"]
pub type AF_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `AF_THRESHOLD` writer - this field configures user threshold. When set to non-zero, autofocus will use this threshold"]
pub type AF_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `AF_GEN_THRESHOLD` reader - this field represents the last calculated threshold"]
pub type AF_GEN_THRESHOLD_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - this field configures user threshold. When set to non-zero, autofocus will use this threshold"]
    #[inline(always)]
    pub fn af_threshold(&self) -> AF_THRESHOLD_R {
        AF_THRESHOLD_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - this field represents the last calculated threshold"]
    #[inline(always)]
    pub fn af_gen_threshold(&self) -> AF_GEN_THRESHOLD_R {
        AF_GEN_THRESHOLD_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AF_THRESHOLD")
            .field(
                "af_threshold",
                &format_args!("{}", self.af_threshold().bits()),
            )
            .field(
                "af_gen_threshold",
                &format_args!("{}", self.af_gen_threshold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<AF_THRESHOLD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - this field configures user threshold. When set to non-zero, autofocus will use this threshold"]
    #[inline(always)]
    #[must_use]
    pub fn af_threshold(&mut self) -> AF_THRESHOLD_W<AF_THRESHOLD_SPEC> {
        AF_THRESHOLD_W::new(self, 0)
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
#[doc = "af threshold register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`af_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`af_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AF_THRESHOLD_SPEC;
impl crate::RegisterSpec for AF_THRESHOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af_threshold::R`](R) reader structure"]
impl crate::Readable for AF_THRESHOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`af_threshold::W`](W) writer structure"]
impl crate::Writable for AF_THRESHOLD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AF_THRESHOLD to value 0x0100"]
impl crate::Resettable for AF_THRESHOLD_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
