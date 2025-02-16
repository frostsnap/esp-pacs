#[doc = "Register `RSA_INT_MAP` reader"]
pub type R = crate::R<RSA_INT_MAP_SPEC>;
#[doc = "Register `RSA_INT_MAP` writer"]
pub type W = crate::W<RSA_INT_MAP_SPEC>;
#[doc = "Field `RSA_INT_MAP` reader - reg_core0_rsa_int_map"]
pub type RSA_INT_MAP_R = crate::FieldReader;
#[doc = "Field `RSA_INT_MAP` writer - reg_core0_rsa_int_map"]
pub type RSA_INT_MAP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - reg_core0_rsa_int_map"]
    #[inline(always)]
    pub fn rsa_int_map(&self) -> RSA_INT_MAP_R {
        RSA_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSA_INT_MAP")
            .field(
                "rsa_int_map",
                &format_args!("{}", self.rsa_int_map().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RSA_INT_MAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - reg_core0_rsa_int_map"]
    #[inline(always)]
    #[must_use]
    pub fn rsa_int_map(&mut self) -> RSA_INT_MAP_W<RSA_INT_MAP_SPEC> {
        RSA_INT_MAP_W::new(self, 0)
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
#[doc = "rsa intr map register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsa_int_map::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsa_int_map::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSA_INT_MAP_SPEC;
impl crate::RegisterSpec for RSA_INT_MAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rsa_int_map::R`](R) reader structure"]
impl crate::Readable for RSA_INT_MAP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsa_int_map::W`](W) writer structure"]
impl crate::Writable for RSA_INT_MAP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RSA_INT_MAP to value 0"]
impl crate::Resettable for RSA_INT_MAP_SPEC {
    const RESET_VALUE: u32 = 0;
}
