#[doc = "Register `CONTINUE_OP` writer"]
pub type W = crate::W<CONTINUE_OP_SPEC>;
#[doc = "Field `CONTINUE_OP` writer - Set this bit to 1 to continue AES operation."]
pub type CONTINUE_OP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONTINUE_OP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to continue AES operation."]
    #[inline(always)]
    #[must_use]
    pub fn continue_op(&mut self) -> CONTINUE_OP_W<CONTINUE_OP_SPEC> {
        CONTINUE_OP_W::new(self, 0)
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
#[doc = "Operation continue controlling register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`continue_op::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONTINUE_OP_SPEC;
impl crate::RegisterSpec for CONTINUE_OP_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`continue_op::W`](W) writer structure"]
impl crate::Writable for CONTINUE_OP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTINUE_OP to value 0"]
impl crate::Resettable for CONTINUE_OP_SPEC {
    const RESET_VALUE: u32 = 0;
}
