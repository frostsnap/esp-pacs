#[doc = "Register `UNIT0_OP` reader"]
pub type R = crate::R<UNIT0_OP_SPEC>;
#[doc = "Register `UNIT0_OP` writer"]
pub type W = crate::W<UNIT0_OP_SPEC>;
#[doc = "Field `TIMER_UNIT0_VALUE_VALID` reader - timer value is sync and valid"]
pub type TIMER_UNIT0_VALUE_VALID_R = crate::BitReader;
#[doc = "Field `TIMER_UNIT0_UPDATE` writer - update timer_unit0"]
pub type TIMER_UNIT0_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - timer value is sync and valid"]
    #[inline(always)]
    pub fn timer_unit0_value_valid(&self) -> TIMER_UNIT0_VALUE_VALID_R {
        TIMER_UNIT0_VALUE_VALID_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT0_OP")
            .field(
                "timer_unit0_value_valid",
                &format_args!("{}", self.timer_unit0_value_valid().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT0_OP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 30 - update timer_unit0"]
    #[inline(always)]
    #[must_use]
    pub fn timer_unit0_update(&mut self) -> TIMER_UNIT0_UPDATE_W<UNIT0_OP_SPEC> {
        TIMER_UNIT0_UPDATE_W::new(self, 30)
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
#[doc = "system timer unit0 value update register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`unit0_op::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`unit0_op::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UNIT0_OP_SPEC;
impl crate::RegisterSpec for UNIT0_OP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`unit0_op::R`](R) reader structure"]
impl crate::Readable for UNIT0_OP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`unit0_op::W`](W) writer structure"]
impl crate::Writable for UNIT0_OP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNIT0_OP to value 0"]
impl crate::Resettable for UNIT0_OP_SPEC {
    const RESET_VALUE: u32 = 0;
}
