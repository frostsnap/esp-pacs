#[doc = "Register `CNTL_DATE` reader"]
pub type R = crate::R<CNTL_DATE_SPEC>;
#[doc = "Register `CNTL_DATE` writer"]
pub type W = crate::W<CNTL_DATE_SPEC>;
#[doc = "Field `CNTL_DATE` reader - Need add desc"]
pub type CNTL_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `CNTL_DATE` writer - Need add desc"]
pub type CNTL_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:27 - Need add desc"]
    #[inline(always)]
    pub fn cntl_date(&self) -> CNTL_DATE_R {
        CNTL_DATE_R::new(self.bits & 0x0fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CNTL_DATE")
            .field("cntl_date", &format_args!("{}", self.cntl_date().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CNTL_DATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:27 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn cntl_date(&mut self) -> CNTL_DATE_W<CNTL_DATE_SPEC> {
        CNTL_DATE_W::new(self, 0)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cntl_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cntl_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CNTL_DATE_SPEC;
impl crate::RegisterSpec for CNTL_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cntl_date::R`](R) reader structure"]
impl crate::Readable for CNTL_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cntl_date::W`](W) writer structure"]
impl crate::Writable for CNTL_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CNTL_DATE to value 0x0210_7190"]
impl crate::Resettable for CNTL_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0210_7190;
}
