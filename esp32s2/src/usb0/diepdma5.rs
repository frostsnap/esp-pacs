#[doc = "Register `DIEPDMA5` reader"]
pub type R = crate::R<DIEPDMA5_SPEC>;
#[doc = "Register `DIEPDMA5` writer"]
pub type W = crate::W<DIEPDMA5_SPEC>;
#[doc = "Field `D_DMAADDR5` reader - "]
pub type D_DMAADDR5_R = crate::FieldReader<u32>;
#[doc = "Field `D_DMAADDR5` writer - "]
pub type D_DMAADDR5_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn d_dmaaddr5(&self) -> D_DMAADDR5_R {
        D_DMAADDR5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMA5")
            .field("d_dmaaddr5", &format_args!("{}", self.d_dmaaddr5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DIEPDMA5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn d_dmaaddr5(&mut self) -> D_DMAADDR5_W<DIEPDMA5_SPEC> {
        D_DMAADDR5_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`diepdma5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`diepdma5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMA5_SPEC;
impl crate::RegisterSpec for DIEPDMA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdma5::R`](R) reader structure"]
impl crate::Readable for DIEPDMA5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`diepdma5::W`](W) writer structure"]
impl crate::Writable for DIEPDMA5_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DIEPDMA5 to value 0"]
impl crate::Resettable for DIEPDMA5_SPEC {
    const RESET_VALUE: u32 = 0;
}
