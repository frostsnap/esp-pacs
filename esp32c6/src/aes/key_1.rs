#[doc = "Register `KEY_1` reader"]
pub type R = crate::R<KEY_1_SPEC>;
#[doc = "Register `KEY_1` writer"]
pub type W = crate::W<KEY_1_SPEC>;
#[doc = "Field `KEY_1` reader - This bits stores key_1 that is a part of key material."]
pub type KEY_1_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_1` writer - This bits stores key_1 that is a part of key material."]
pub type KEY_1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_1 that is a part of key material."]
    #[inline(always)]
    pub fn key_1(&self) -> KEY_1_R {
        KEY_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_1")
            .field("key_1", &format_args!("{}", self.key_1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_1 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_1(&mut self) -> KEY_1_W<KEY_1_SPEC> {
        KEY_1_W::new(self, 0)
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
#[doc = "Key material key_1 configure register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`key_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`key_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_1_SPEC;
impl crate::RegisterSpec for KEY_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_1::R`](R) reader structure"]
impl crate::Readable for KEY_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`key_1::W`](W) writer structure"]
impl crate::Writable for KEY_1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets KEY_1 to value 0"]
impl crate::Resettable for KEY_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
