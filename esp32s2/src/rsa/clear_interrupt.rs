#[doc = "Register `CLEAR_INTERRUPT` writer"]
pub type W = crate::W<CLEAR_INTERRUPT_SPEC>;
#[doc = "Field `CLEAR_INTERRUPT` writer - Set this bit to 1 to clear the RSA interrupts."]
pub type CLEAR_INTERRUPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLEAR_INTERRUPT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to clear the RSA interrupts."]
    #[inline(always)]
    #[must_use]
    pub fn clear_interrupt(&mut self) -> CLEAR_INTERRUPT_W<CLEAR_INTERRUPT_SPEC> {
        CLEAR_INTERRUPT_W::new(self, 0)
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
#[doc = "RSA clear interrupt register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clear_interrupt::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLEAR_INTERRUPT_SPEC;
impl crate::RegisterSpec for CLEAR_INTERRUPT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clear_interrupt::W`](W) writer structure"]
impl crate::Writable for CLEAR_INTERRUPT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLEAR_INTERRUPT to value 0"]
impl crate::Resettable for CLEAR_INTERRUPT_SPEC {
    const RESET_VALUE: u32 = 0;
}
