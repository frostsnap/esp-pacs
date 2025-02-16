#[doc = "Register `ENDIAN_MODE` reader"]
pub type R = crate::R<ENDIAN_MODE_SPEC>;
#[doc = "Register `ENDIAN_MODE` writer"]
pub type W = crate::W<ENDIAN_MODE_SPEC>;
#[doc = "Field `BYTE_ENDIAN_ORDER` reader - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed."]
pub type BYTE_ENDIAN_ORDER_R = crate::BitReader;
#[doc = "Field `BYTE_ENDIAN_ORDER` writer - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed."]
pub type BYTE_ENDIAN_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BIT_ENDIAN_ORDER` reader - N/A"]
pub type BIT_ENDIAN_ORDER_R = crate::BitReader;
#[doc = "Field `BIT_ENDIAN_ORDER` writer - N/A"]
pub type BIT_ENDIAN_ORDER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed."]
    #[inline(always)]
    pub fn byte_endian_order(&self) -> BYTE_ENDIAN_ORDER_R {
        BYTE_ENDIAN_ORDER_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    pub fn bit_endian_order(&self) -> BIT_ENDIAN_ORDER_R {
        BIT_ENDIAN_ORDER_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ENDIAN_MODE")
            .field(
                "byte_endian_order",
                &format_args!("{}", self.byte_endian_order().bit()),
            )
            .field(
                "bit_endian_order",
                &format_args!("{}", self.bit_endian_order().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ENDIAN_MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - endianness order in bytes. 2'h0 is normal mode and 2'h3 is useful to YUV420(Legacy) when isp is bapassed."]
    #[inline(always)]
    #[must_use]
    pub fn byte_endian_order(&mut self) -> BYTE_ENDIAN_ORDER_W<ENDIAN_MODE_SPEC> {
        BYTE_ENDIAN_ORDER_W::new(self, 0)
    }
    #[doc = "Bit 1 - N/A"]
    #[inline(always)]
    #[must_use]
    pub fn bit_endian_order(&mut self) -> BIT_ENDIAN_ORDER_W<ENDIAN_MODE_SPEC> {
        BIT_ENDIAN_ORDER_W::new(self, 1)
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
#[doc = "data endianness order configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`endian_mode::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`endian_mode::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ENDIAN_MODE_SPEC;
impl crate::RegisterSpec for ENDIAN_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`endian_mode::R`](R) reader structure"]
impl crate::Readable for ENDIAN_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`endian_mode::W`](W) writer structure"]
impl crate::Writable for ENDIAN_MODE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENDIAN_MODE to value 0"]
impl crate::Resettable for ENDIAN_MODE_SPEC {
    const RESET_VALUE: u32 = 0;
}
