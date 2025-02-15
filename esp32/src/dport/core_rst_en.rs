#[doc = "Register `CORE_RST_EN` reader"]
pub type R = crate::R<CORE_RST_EN_SPEC>;
#[doc = "Register `CORE_RST_EN` writer"]
pub type W = crate::W<CORE_RST_EN_SPEC>;
#[doc = "Field `CORE_RST` reader - "]
pub type CORE_RST_R = crate::FieldReader;
#[doc = "Field `CORE_RST` writer - "]
pub type CORE_RST_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn core_rst(&self) -> CORE_RST_R {
        CORE_RST_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_RST_EN")
            .field("core_rst", &format_args!("{}", self.core_rst().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_RST_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn core_rst(&mut self) -> CORE_RST_W<CORE_RST_EN_SPEC> {
        CORE_RST_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`core_rst_en::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`core_rst_en::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_RST_EN_SPEC;
impl crate::RegisterSpec for CORE_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_rst_en::R`](R) reader structure"]
impl crate::Readable for CORE_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`core_rst_en::W`](W) writer structure"]
impl crate::Writable for CORE_RST_EN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CORE_RST_EN to value 0"]
impl crate::Resettable for CORE_RST_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
