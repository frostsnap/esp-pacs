#[doc = "Register `GPIO_HOLD1` reader"]
pub type R = crate::R<GPIO_HOLD1_SPEC>;
#[doc = "Register `GPIO_HOLD1` writer"]
pub type W = crate::W<GPIO_HOLD1_SPEC>;
#[doc = "Field `GPIO_HOLD1` reader - need_des"]
pub type GPIO_HOLD1_R = crate::FieldReader<u32>;
#[doc = "Field `GPIO_HOLD1` writer - need_des"]
pub type GPIO_HOLD1_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn gpio_hold1(&self) -> GPIO_HOLD1_R {
        GPIO_HOLD1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO_HOLD1")
            .field("gpio_hold1", &format_args!("{}", self.gpio_hold1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<GPIO_HOLD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn gpio_hold1(&mut self) -> GPIO_HOLD1_W<GPIO_HOLD1_SPEC> {
        GPIO_HOLD1_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio_hold1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio_hold1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GPIO_HOLD1_SPEC;
impl crate::RegisterSpec for GPIO_HOLD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio_hold1::R`](R) reader structure"]
impl crate::Readable for GPIO_HOLD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gpio_hold1::W`](W) writer structure"]
impl crate::Writable for GPIO_HOLD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO_HOLD1 to value 0"]
impl crate::Resettable for GPIO_HOLD1_SPEC {
    const RESET_VALUE: u32 = 0;
}
