#[doc = "Register `HOST_SLCHOST_CONF_W7` reader"]
pub type R = crate::R<HOST_SLCHOST_CONF_W7_SPEC>;
#[doc = "Register `HOST_SLCHOST_CONF_W7` writer"]
pub type W = crate::W<HOST_SLCHOST_CONF_W7_SPEC>;
#[doc = "Field `HOST_SLCHOST_CONF28` reader - "]
pub type HOST_SLCHOST_CONF28_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF28` writer - "]
pub type HOST_SLCHOST_CONF28_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF29` reader - "]
pub type HOST_SLCHOST_CONF29_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF29` writer - "]
pub type HOST_SLCHOST_CONF29_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF30` reader - "]
pub type HOST_SLCHOST_CONF30_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF30` writer - "]
pub type HOST_SLCHOST_CONF30_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HOST_SLCHOST_CONF31` reader - "]
pub type HOST_SLCHOST_CONF31_R = crate::FieldReader;
#[doc = "Field `HOST_SLCHOST_CONF31` writer - "]
pub type HOST_SLCHOST_CONF31_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn host_slchost_conf28(&self) -> HOST_SLCHOST_CONF28_R {
        HOST_SLCHOST_CONF28_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn host_slchost_conf29(&self) -> HOST_SLCHOST_CONF29_R {
        HOST_SLCHOST_CONF29_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn host_slchost_conf30(&self) -> HOST_SLCHOST_CONF30_R {
        HOST_SLCHOST_CONF30_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn host_slchost_conf31(&self) -> HOST_SLCHOST_CONF31_R {
        HOST_SLCHOST_CONF31_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOST_SLCHOST_CONF_W7")
            .field(
                "host_slchost_conf28",
                &format_args!("{}", self.host_slchost_conf28().bits()),
            )
            .field(
                "host_slchost_conf29",
                &format_args!("{}", self.host_slchost_conf29().bits()),
            )
            .field(
                "host_slchost_conf30",
                &format_args!("{}", self.host_slchost_conf30().bits()),
            )
            .field(
                "host_slchost_conf31",
                &format_args!("{}", self.host_slchost_conf31().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HOST_SLCHOST_CONF_W7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf28(&mut self) -> HOST_SLCHOST_CONF28_W<HOST_SLCHOST_CONF_W7_SPEC> {
        HOST_SLCHOST_CONF28_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf29(&mut self) -> HOST_SLCHOST_CONF29_W<HOST_SLCHOST_CONF_W7_SPEC> {
        HOST_SLCHOST_CONF29_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf30(&mut self) -> HOST_SLCHOST_CONF30_W<HOST_SLCHOST_CONF_W7_SPEC> {
        HOST_SLCHOST_CONF30_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn host_slchost_conf31(&mut self) -> HOST_SLCHOST_CONF31_W<HOST_SLCHOST_CONF_W7_SPEC> {
        HOST_SLCHOST_CONF31_W::new(self, 24)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`host_slchost_conf_w7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`host_slchost_conf_w7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOST_SLCHOST_CONF_W7_SPEC;
impl crate::RegisterSpec for HOST_SLCHOST_CONF_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`host_slchost_conf_w7::R`](R) reader structure"]
impl crate::Readable for HOST_SLCHOST_CONF_W7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`host_slchost_conf_w7::W`](W) writer structure"]
impl crate::Writable for HOST_SLCHOST_CONF_W7_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOST_SLCHOST_CONF_W7 to value 0"]
impl crate::Resettable for HOST_SLCHOST_CONF_W7_SPEC {
    const RESET_VALUE: u32 = 0;
}
