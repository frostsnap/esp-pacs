#[doc = "Register `CLK_EN0` reader"]
pub type R = crate::R<CLK_EN0_SPEC>;
#[doc = "Register `CLK_EN0` writer"]
pub type W = crate::W<CLK_EN0_SPEC>;
#[doc = "Field `CLK_EN` reader - Reserved"]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - Reserved"]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_EN0")
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_EN0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<CLK_EN0_SPEC> {
        CLK_EN_W::new(self, 0)
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
#[doc = "Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_en0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_en0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_EN0_SPEC;
impl crate::RegisterSpec for CLK_EN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk_en0::R`](R) reader structure"]
impl crate::Readable for CLK_EN0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk_en0::W`](W) writer structure"]
impl crate::Writable for CLK_EN0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK_EN0 to value 0x01"]
impl crate::Resettable for CLK_EN0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
