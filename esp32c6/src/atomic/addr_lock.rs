#[doc = "Register `ADDR_LOCK` reader"]
pub type R = crate::R<ADDR_LOCK_SPEC>;
#[doc = "Register `ADDR_LOCK` writer"]
pub type W = crate::W<ADDR_LOCK_SPEC>;
#[doc = "Field `LOCK` reader - read to acquire hardware lock, write to release hardware lock"]
pub type LOCK_R = crate::FieldReader;
#[doc = "Field `LOCK` writer - read to acquire hardware lock, write to release hardware lock"]
pub type LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - read to acquire hardware lock, write to release hardware lock"]
    #[inline(always)]
    pub fn lock(&self) -> LOCK_R {
        LOCK_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ADDR_LOCK")
            .field("lock", &format_args!("{}", self.lock().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ADDR_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:1 - read to acquire hardware lock, write to release hardware lock"]
    #[inline(always)]
    #[must_use]
    pub fn lock(&mut self) -> LOCK_W<ADDR_LOCK_SPEC> {
        LOCK_W::new(self, 0)
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
#[doc = "hardware lock regsiter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`addr_lock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`addr_lock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ADDR_LOCK_SPEC;
impl crate::RegisterSpec for ADDR_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`addr_lock::R`](R) reader structure"]
impl crate::Readable for ADDR_LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`addr_lock::W`](W) writer structure"]
impl crate::Writable for ADDR_LOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ADDR_LOCK to value 0"]
impl crate::Resettable for ADDR_LOCK_SPEC {
    const RESET_VALUE: u32 = 0;
}
