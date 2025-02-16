#[doc = "Register `PIN_CTRL` reader"]
pub type R = crate::R<PIN_CTRL_SPEC>;
#[doc = "Register `PIN_CTRL` writer"]
pub type W = crate::W<PIN_CTRL_SPEC>;
#[doc = "Field `CLK1` reader - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
pub type CLK1_R = crate::FieldReader;
#[doc = "Field `CLK1` writer - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
pub type CLK1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK2` reader - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
pub type CLK2_R = crate::FieldReader;
#[doc = "Field `CLK2` writer - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
pub type CLK2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CLK3` reader - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
pub type CLK3_R = crate::FieldReader;
#[doc = "Field `CLK3` writer - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
pub type CLK3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
    #[inline(always)]
    pub fn clk1(&self) -> CLK1_R {
        CLK1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
    #[inline(always)]
    pub fn clk2(&self) -> CLK2_R {
        CLK2_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
    #[inline(always)]
    pub fn clk3(&self) -> CLK3_R {
        CLK3_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIN_CTRL")
            .field("clk1", &format_args!("{}", self.clk1().bits()))
            .field("clk2", &format_args!("{}", self.clk2().bits()))
            .field("clk3", &format_args!("{}", self.clk3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIN_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:3 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn clk1(&mut self) -> CLK1_W<PIN_CTRL_SPEC> {
        CLK1_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn clk2(&mut self) -> CLK2_W<PIN_CTRL_SPEC> {
        CLK2_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - If you want to output clock for I2S0 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0x0; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0x0 and PIN_CTRL\\[11:8\\] = 0x0. If you want to output clock for I2S1 to: CLK_OUT1, then set PIN_CTRL\\[3:0\\] = 0xF; CLK_OUT2, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[7:4\\] = 0x0; CLK_OUT3, then set PIN_CTRL\\[3:0\\] = 0xF and PIN_CTRL\\[11:8\\] = 0x0."]
    #[inline(always)]
    #[must_use]
    pub fn clk3(&mut self) -> CLK3_W<PIN_CTRL_SPEC> {
        CLK3_W::new(self, 8)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pin_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pin_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIN_CTRL_SPEC;
impl crate::RegisterSpec for PIN_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pin_ctrl::R`](R) reader structure"]
impl crate::Readable for PIN_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pin_ctrl::W`](W) writer structure"]
impl crate::Writable for PIN_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PIN_CTRL to value 0"]
impl crate::Resettable for PIN_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
