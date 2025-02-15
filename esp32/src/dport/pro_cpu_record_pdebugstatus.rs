#[doc = "Register `PRO_CPU_RECORD_PDEBUGSTATUS` reader"]
pub type R = crate::R<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>;
#[doc = "Register `PRO_CPU_RECORD_PDEBUGSTATUS` writer"]
pub type W = crate::W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC>;
#[doc = "Field `RECORD_PRO_PDEBUGSTATUS` reader - "]
pub type RECORD_PRO_PDEBUGSTATUS_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGSTATUS_BBCAUSE` reader - "]
pub type RECORD_PDEBUGSTATUS_BBCAUSE_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGSTATUS_BBCAUSE` writer - "]
pub type RECORD_PDEBUGSTATUS_BBCAUSE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RECORD_PDEBUGSTATUS_INSNTYPE` reader - "]
pub type RECORD_PDEBUGSTATUS_INSNTYPE_R = crate::FieldReader;
#[doc = "Field `RECORD_PDEBUGSTATUS_INSNTYPE` writer - "]
pub type RECORD_PDEBUGSTATUS_INSNTYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn record_pro_pdebugstatus(&self) -> RECORD_PRO_PDEBUGSTATUS_R {
        RECORD_PRO_PDEBUGSTATUS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn record_pdebugstatus_bbcause(&self) -> RECORD_PDEBUGSTATUS_BBCAUSE_R {
        RECORD_PDEBUGSTATUS_BBCAUSE_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn record_pdebugstatus_insntype(&self) -> RECORD_PDEBUGSTATUS_INSNTYPE_R {
        RECORD_PDEBUGSTATUS_INSNTYPE_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_CPU_RECORD_PDEBUGSTATUS")
            .field(
                "record_pro_pdebugstatus",
                &format_args!("{}", self.record_pro_pdebugstatus().bits()),
            )
            .field(
                "record_pdebugstatus_bbcause",
                &format_args!("{}", self.record_pdebugstatus_bbcause().bits()),
            )
            .field(
                "record_pdebugstatus_insntype",
                &format_args!("{}", self.record_pdebugstatus_insntype().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugstatus_bbcause(
        &mut self,
    ) -> RECORD_PDEBUGSTATUS_BBCAUSE_W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC> {
        RECORD_PDEBUGSTATUS_BBCAUSE_W::new(self, 0)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn record_pdebugstatus_insntype(
        &mut self,
    ) -> RECORD_PDEBUGSTATUS_INSNTYPE_W<PRO_CPU_RECORD_PDEBUGSTATUS_SPEC> {
        RECORD_PDEBUGSTATUS_INSNTYPE_W::new(self, 0)
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
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_cpu_record_pdebugstatus::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pro_cpu_record_pdebugstatus::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_CPU_RECORD_PDEBUGSTATUS_SPEC;
impl crate::RegisterSpec for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_cpu_record_pdebugstatus::R`](R) reader structure"]
impl crate::Readable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pro_cpu_record_pdebugstatus::W`](W) writer structure"]
impl crate::Writable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PRO_CPU_RECORD_PDEBUGSTATUS to value 0"]
impl crate::Resettable for PRO_CPU_RECORD_PDEBUGSTATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
