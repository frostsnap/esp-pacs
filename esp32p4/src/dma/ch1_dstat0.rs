#[doc = "Register `CH1_DSTAT0` reader"]
pub type R = crate::R<CH1_DSTAT0_SPEC>;
#[doc = "Field `CH1_DSTAT` reader - NA"]
pub type CH1_DSTAT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_dstat(&self) -> CH1_DSTAT_R {
        CH1_DSTAT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1_DSTAT0")
            .field("ch1_dstat", &format_args!("{}", self.ch1_dstat().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH1_DSTAT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dstat0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1_DSTAT0_SPEC;
impl crate::RegisterSpec for CH1_DSTAT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_dstat0::R`](R) reader structure"]
impl crate::Readable for CH1_DSTAT0_SPEC {}
#[doc = "`reset()` method sets CH1_DSTAT0 to value 0"]
impl crate::Resettable for CH1_DSTAT0_SPEC {
    const RESET_VALUE: u32 = 0;
}
