#[doc = "Register `W3` reader"]
pub type R = crate::R<W3_SPEC>;
#[doc = "Register `W3` writer"]
pub type W = crate::W<W3_SPEC>;
#[doc = "Field `BUF3` reader - data buffer"]
pub type BUF3_R = crate::FieldReader<u32>;
#[doc = "Field `BUF3` writer - data buffer"]
pub type BUF3_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn buf3(&self) -> BUF3_R {
        BUF3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W3")
            .field("buf3", &format_args!("{}", self.buf3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn buf3(&mut self) -> BUF3_W<W3_SPEC> {
        BUF3_W::new(self, 0)
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
#[doc = "SPI CPU-controlled buffer3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`w3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`w3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct W3_SPEC;
impl crate::RegisterSpec for W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`w3::R`](R) reader structure"]
impl crate::Readable for W3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`w3::W`](W) writer structure"]
impl crate::Writable for W3_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets W3 to value 0"]
impl crate::Resettable for W3_SPEC {
    const RESET_VALUE: u32 = 0;
}
