#[doc = "Register `INFO_MEM[%s]` reader"]
pub type R = crate::R<INFO_MEM_SPEC>;
#[doc = "Register `INFO_MEM[%s]` writer"]
pub type W = crate::W<INFO_MEM_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INFO_MEM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
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
#[doc = "The memory that stores HUK info.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`info_mem::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`info_mem::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INFO_MEM_SPEC;
impl crate::RegisterSpec for INFO_MEM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info_mem::R`](R) reader structure"]
impl crate::Readable for INFO_MEM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`info_mem::W`](W) writer structure"]
impl crate::Writable for INFO_MEM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INFO_MEM[%s] to value 0"]
impl crate::Resettable for INFO_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
