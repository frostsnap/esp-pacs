#[doc = "Register `CORE_ERR_RESP_DIS` reader"]
pub type R = crate::R<CORE_ERR_RESP_DIS_SPEC>;
#[doc = "Register `CORE_ERR_RESP_DIS` writer"]
pub type W = crate::W<CORE_ERR_RESP_DIS_SPEC>;
#[doc = "Field `CORE_ERR_RESP_DIS` reader - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
pub type CORE_ERR_RESP_DIS_R = crate::FieldReader;
#[doc = "Field `CORE_ERR_RESP_DIS` writer - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
pub type CORE_ERR_RESP_DIS_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
    #[inline(always)]
    pub fn core_err_resp_dis(&self) -> CORE_ERR_RESP_DIS_R {
        CORE_ERR_RESP_DIS_R::new((self.bits & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_ERR_RESP_DIS")
            .field(
                "core_err_resp_dis",
                &format_args!("{}", self.core_err_resp_dis().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_ERR_RESP_DIS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:2 - Set bit0 to disable ibus err resp. Set bit1 to disable dbus err resp. Set bit 2 to disable ahb err resp."]
    #[inline(always)]
    #[must_use]
    pub fn core_err_resp_dis(&mut self) -> CORE_ERR_RESP_DIS_W<CORE_ERR_RESP_DIS_SPEC> {
        CORE_ERR_RESP_DIS_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_err_resp_dis::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_err_resp_dis::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_ERR_RESP_DIS_SPEC;
impl crate::RegisterSpec for CORE_ERR_RESP_DIS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_err_resp_dis::R`](R) reader structure"]
impl crate::Readable for CORE_ERR_RESP_DIS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_err_resp_dis::W`](W) writer structure"]
impl crate::Writable for CORE_ERR_RESP_DIS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_ERR_RESP_DIS to value 0"]
impl crate::Resettable for CORE_ERR_RESP_DIS_SPEC {
    const RESET_VALUE: u32 = 0;
}
