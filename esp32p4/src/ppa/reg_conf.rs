#[doc = "Register `REG_CONF` reader"]
pub type R = crate::R<REG_CONF_SPEC>;
#[doc = "Register `REG_CONF` writer"]
pub type W = crate::W<REG_CONF_SPEC>;
#[doc = "Field `CLK_EN` reader - PPA register clock gate enable signal."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - PPA register clock gate enable signal."]
pub type CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PPA register clock gate enable signal."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REG_CONF")
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REG_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - PPA register clock gate enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<REG_CONF_SPEC> {
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
#[doc = "Register clock enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`reg_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`reg_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REG_CONF_SPEC;
impl crate::RegisterSpec for REG_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reg_conf::R`](R) reader structure"]
impl crate::Readable for REG_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reg_conf::W`](W) writer structure"]
impl crate::Writable for REG_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REG_CONF to value 0"]
impl crate::Resettable for REG_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
