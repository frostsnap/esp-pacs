#[doc = "Register `BOOT_ADDR_HP_LP` reader"]
pub type R = crate::R<BOOT_ADDR_HP_LP_SPEC>;
#[doc = "Register `BOOT_ADDR_HP_LP` writer"]
pub type W = crate::W<BOOT_ADDR_HP_LP_SPEC>;
#[doc = "Field `BOOT_ADDR_HP_LP` reader - need_des"]
pub type BOOT_ADDR_HP_LP_R = crate::FieldReader<u32>;
#[doc = "Field `BOOT_ADDR_HP_LP` writer - need_des"]
pub type BOOT_ADDR_HP_LP_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn boot_addr_hp_lp(&self) -> BOOT_ADDR_HP_LP_R {
        BOOT_ADDR_HP_LP_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BOOT_ADDR_HP_LP")
            .field(
                "boot_addr_hp_lp",
                &format_args!("{}", self.boot_addr_hp_lp().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BOOT_ADDR_HP_LP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn boot_addr_hp_lp(&mut self) -> BOOT_ADDR_HP_LP_W<BOOT_ADDR_HP_LP_SPEC> {
        BOOT_ADDR_HP_LP_W::new(self, 0)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`boot_addr_hp_lp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`boot_addr_hp_lp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BOOT_ADDR_HP_LP_SPEC;
impl crate::RegisterSpec for BOOT_ADDR_HP_LP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boot_addr_hp_lp::R`](R) reader structure"]
impl crate::Readable for BOOT_ADDR_HP_LP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`boot_addr_hp_lp::W`](W) writer structure"]
impl crate::Writable for BOOT_ADDR_HP_LP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BOOT_ADDR_HP_LP to value 0"]
impl crate::Resettable for BOOT_ADDR_HP_LP_SPEC {
    const RESET_VALUE: u32 = 0;
}
