#[doc = "Register `CH1_DAR0` reader"]
pub type R = crate::R<CH1_DAR0_SPEC>;
#[doc = "Register `CH1_DAR0` writer"]
pub type W = crate::W<CH1_DAR0_SPEC>;
#[doc = "Field `CH1_DAR0` reader - NA"]
pub type CH1_DAR0_R = crate::FieldReader<u32>;
#[doc = "Field `CH1_DAR0` writer - NA"]
pub type CH1_DAR0_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn ch1_dar0(&self) -> CH1_DAR0_R {
        CH1_DAR0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH1_DAR0")
            .field("ch1_dar0", &format_args!("{}", self.ch1_dar0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH1_DAR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    #[must_use]
    pub fn ch1_dar0(&mut self) -> CH1_DAR0_W<CH1_DAR0_SPEC> {
        CH1_DAR0_W::new(self, 0)
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
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch1_dar0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch1_dar0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH1_DAR0_SPEC;
impl crate::RegisterSpec for CH1_DAR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch1_dar0::R`](R) reader structure"]
impl crate::Readable for CH1_DAR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch1_dar0::W`](W) writer structure"]
impl crate::Writable for CH1_DAR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH1_DAR0 to value 0"]
impl crate::Resettable for CH1_DAR0_SPEC {
    const RESET_VALUE: u32 = 0;
}
