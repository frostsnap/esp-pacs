#[doc = "Register `I2S_CONF` reader"]
pub type R = crate::R<I2S_CONF_SPEC>;
#[doc = "Register `I2S_CONF` writer"]
pub type W = crate::W<I2S_CONF_SPEC>;
#[doc = "Field `I2S_CLK_EN` reader - Set 1 to enable i2s apb clock"]
pub type I2S_CLK_EN_R = crate::BitReader;
#[doc = "Field `I2S_CLK_EN` writer - Set 1 to enable i2s apb clock"]
pub type I2S_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S_RST_EN` reader - Set 0 to reset i2s module"]
pub type I2S_RST_EN_R = crate::BitReader;
#[doc = "Field `I2S_RST_EN` writer - Set 0 to reset i2s module"]
pub type I2S_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable i2s apb clock"]
    #[inline(always)]
    pub fn i2s_clk_en(&self) -> I2S_CLK_EN_R {
        I2S_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset i2s module"]
    #[inline(always)]
    pub fn i2s_rst_en(&self) -> I2S_RST_EN_R {
        I2S_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S_CONF")
            .field("i2s_clk_en", &format_args!("{}", self.i2s_clk_en().bit()))
            .field("i2s_rst_en", &format_args!("{}", self.i2s_rst_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<I2S_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable i2s apb clock"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_clk_en(&mut self) -> I2S_CLK_EN_W<I2S_CONF_SPEC> {
        I2S_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset i2s module"]
    #[inline(always)]
    #[must_use]
    pub fn i2s_rst_en(&mut self) -> I2S_RST_EN_W<I2S_CONF_SPEC> {
        I2S_RST_EN_W::new(self, 1)
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
#[doc = "I2S configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`i2s_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`i2s_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I2S_CONF_SPEC;
impl crate::RegisterSpec for I2S_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i2s_conf::R`](R) reader structure"]
impl crate::Readable for I2S_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`i2s_conf::W`](W) writer structure"]
impl crate::Writable for I2S_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets I2S_CONF to value 0x01"]
impl crate::Resettable for I2S_CONF_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
