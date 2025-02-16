#[doc = "Register `UPDATE` reader"]
pub type R = crate::R<UPDATE_SPEC>;
#[doc = "Register `UPDATE` writer"]
pub type W = crate::W<UPDATE_SPEC>;
#[doc = "Field `MAIN_TIMER_UPDATE` writer - need_des"]
pub type MAIN_TIMER_UPDATE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_XTAL_OFF` reader - need_des"]
pub type MAIN_TIMER_XTAL_OFF_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_XTAL_OFF` writer - need_des"]
pub type MAIN_TIMER_XTAL_OFF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_SYS_STALL` reader - need_des"]
pub type MAIN_TIMER_SYS_STALL_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_SYS_STALL` writer - need_des"]
pub type MAIN_TIMER_SYS_STALL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MAIN_TIMER_SYS_RST` reader - need_des"]
pub type MAIN_TIMER_SYS_RST_R = crate::BitReader;
#[doc = "Field `MAIN_TIMER_SYS_RST` writer - need_des"]
pub type MAIN_TIMER_SYS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn main_timer_xtal_off(&self) -> MAIN_TIMER_XTAL_OFF_R {
        MAIN_TIMER_XTAL_OFF_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn main_timer_sys_stall(&self) -> MAIN_TIMER_SYS_STALL_R {
        MAIN_TIMER_SYS_STALL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn main_timer_sys_rst(&self) -> MAIN_TIMER_SYS_RST_R {
        MAIN_TIMER_SYS_RST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPDATE")
            .field(
                "main_timer_xtal_off",
                &format_args!("{}", self.main_timer_xtal_off().bit()),
            )
            .field(
                "main_timer_sys_stall",
                &format_args!("{}", self.main_timer_sys_stall().bit()),
            )
            .field(
                "main_timer_sys_rst",
                &format_args!("{}", self.main_timer_sys_rst().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_update(&mut self) -> MAIN_TIMER_UPDATE_W<UPDATE_SPEC> {
        MAIN_TIMER_UPDATE_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_xtal_off(&mut self) -> MAIN_TIMER_XTAL_OFF_W<UPDATE_SPEC> {
        MAIN_TIMER_XTAL_OFF_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_sys_stall(&mut self) -> MAIN_TIMER_SYS_STALL_W<UPDATE_SPEC> {
        MAIN_TIMER_SYS_STALL_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn main_timer_sys_rst(&mut self) -> MAIN_TIMER_SYS_RST_W<UPDATE_SPEC> {
        MAIN_TIMER_SYS_RST_W::new(self, 31)
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
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPDATE_SPEC;
impl crate::RegisterSpec for UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`update::R`](R) reader structure"]
impl crate::Readable for UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`update::W`](W) writer structure"]
impl crate::Writable for UPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UPDATE to value 0"]
impl crate::Resettable for UPDATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
