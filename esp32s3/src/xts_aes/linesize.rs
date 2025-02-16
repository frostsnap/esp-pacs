#[doc = "Register `LINESIZE` reader"]
pub type R = crate::R<LINESIZE_SPEC>;
#[doc = "Register `LINESIZE` writer"]
pub type W = crate::W<LINESIZE_SPEC>;
#[doc = "Field `LINESIZE` reader - Configures the data size of one encryption."]
pub type LINESIZE_R = crate::BitReader;
#[doc = "Field `LINESIZE` writer - Configures the data size of one encryption."]
pub type LINESIZE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures the data size of one encryption."]
    #[inline(always)]
    pub fn linesize(&self) -> LINESIZE_R {
        LINESIZE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LINESIZE")
            .field("linesize", &format_args!("{}", self.linesize().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LINESIZE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures the data size of one encryption."]
    #[inline(always)]
    #[must_use]
    pub fn linesize(&mut self) -> LINESIZE_W<LINESIZE_SPEC> {
        LINESIZE_W::new(self, 0)
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
#[doc = "XTS-AES line-size register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`linesize::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`linesize::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LINESIZE_SPEC;
impl crate::RegisterSpec for LINESIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`linesize::R`](R) reader structure"]
impl crate::Readable for LINESIZE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`linesize::W`](W) writer structure"]
impl crate::Writable for LINESIZE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINESIZE to value 0"]
impl crate::Resettable for LINESIZE_SPEC {
    const RESET_VALUE: u32 = 0;
}
