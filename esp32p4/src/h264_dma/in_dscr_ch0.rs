#[doc = "Register `IN_DSCR_CH0` reader"]
pub type R = crate::R<IN_DSCR_CH0_SPEC>;
#[doc = "Field `INLINK_DSCR_CH0` reader - The address of the next inlink descriptor address x."]
pub type INLINK_DSCR_CH0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The address of the next inlink descriptor address x."]
    #[inline(always)]
    pub fn inlink_dscr_ch0(&self) -> INLINK_DSCR_CH0_R {
        INLINK_DSCR_CH0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_DSCR_CH0")
            .field(
                "inlink_dscr_ch0",
                &format_args!("{}", self.inlink_dscr_ch0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IN_DSCR_CH0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "RX CH0 next dscr addr register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`in_dscr_ch0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_DSCR_CH0_SPEC;
impl crate::RegisterSpec for IN_DSCR_CH0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_dscr_ch0::R`](R) reader structure"]
impl crate::Readable for IN_DSCR_CH0_SPEC {}
#[doc = "`reset()` method sets IN_DSCR_CH0 to value 0"]
impl crate::Resettable for IN_DSCR_CH0_SPEC {
    const RESET_VALUE: u32 = 0;
}
