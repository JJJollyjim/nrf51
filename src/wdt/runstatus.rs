#[doc = "Reader of register RUNSTATUS"]
pub type R = crate::R<u32, super::RUNSTATUS>;
#[doc = "Watchdog running status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RUNSTATUS_A {
    #[doc = "0: Watchdog timer is not running."]
    NOTRUNNING,
    #[doc = "1: Watchdog timer is running."]
    RUNNING,
}
impl From<RUNSTATUS_A> for bool {
    #[inline(always)]
    fn from(variant: RUNSTATUS_A) -> Self {
        match variant {
            RUNSTATUS_A::NOTRUNNING => false,
            RUNSTATUS_A::RUNNING => true,
        }
    }
}
#[doc = "Reader of field `RUNSTATUS`"]
pub type RUNSTATUS_R = crate::R<bool, RUNSTATUS_A>;
impl RUNSTATUS_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RUNSTATUS_A {
        match self.bits {
            false => RUNSTATUS_A::NOTRUNNING,
            true => RUNSTATUS_A::RUNNING,
        }
    }
    #[doc = "Checks if the value of the field is `NOTRUNNING`"]
    #[inline(always)]
    pub fn is_not_running(&self) -> bool {
        *self == RUNSTATUS_A::NOTRUNNING
    }
    #[doc = "Checks if the value of the field is `RUNNING`"]
    #[inline(always)]
    pub fn is_running(&self) -> bool {
        *self == RUNSTATUS_A::RUNNING
    }
}
impl R {
    #[doc = "Bit 0 - Watchdog running status."]
    #[inline(always)]
    pub fn runstatus(&self) -> RUNSTATUS_R {
        RUNSTATUS_R::new((self.bits & 0x01) != 0)
    }
}
