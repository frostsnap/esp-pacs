#[doc = "Register `SLICE_HEADER_BYTE0` reader"]
pub type R = crate::R<SLICE_HEADER_BYTE0_SPEC>;
#[doc = "Register `SLICE_HEADER_BYTE0` writer"]
pub type W = crate::W<SLICE_HEADER_BYTE0_SPEC>;
#[doc = "Field `SLICE_BYTE_LSB` reader - Configures Slice Header low 32 bit"]
pub type SLICE_BYTE_LSB_R = crate::FieldReader<u32>;
#[doc = "Field `SLICE_BYTE_LSB` writer - Configures Slice Header low 32 bit"]
pub type SLICE_BYTE_LSB_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures Slice Header low 32 bit"]
    #[inline(always)]
    pub fn slice_byte_lsb(&self) -> SLICE_BYTE_LSB_R {
        SLICE_BYTE_LSB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLICE_HEADER_BYTE0")
            .field(
                "slice_byte_lsb",
                &format_args!("{}", self.slice_byte_lsb().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLICE_HEADER_BYTE0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures Slice Header low 32 bit"]
    #[inline(always)]
    #[must_use]
    pub fn slice_byte_lsb(&mut self) -> SLICE_BYTE_LSB_W<SLICE_HEADER_BYTE0_SPEC> {
        SLICE_BYTE_LSB_W::new(self, 0)
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
#[doc = "Frame Slice Header byte low 32 bit register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`slice_header_byte0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slice_header_byte0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLICE_HEADER_BYTE0_SPEC;
impl crate::RegisterSpec for SLICE_HEADER_BYTE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slice_header_byte0::R`](R) reader structure"]
impl crate::Readable for SLICE_HEADER_BYTE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slice_header_byte0::W`](W) writer structure"]
impl crate::Writable for SLICE_HEADER_BYTE0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SLICE_HEADER_BYTE0 to value 0"]
impl crate::Resettable for SLICE_HEADER_BYTE0_SPEC {
    const RESET_VALUE: u32 = 0;
}
