#[repr(C)]
#[cfg_attr(feature = "impl-register-debug", derive(Debug))]
#[doc = "Register block"]
pub struct RegisterBlock {
    clk_cfg: CLK_CFG,
    timer0_cfg0: TIMER0_CFG0,
    timer0_cfg1: TIMER0_CFG1,
    timer0_sync: TIMER0_SYNC,
    timer0_status: TIMER0_STATUS,
    timer1_cfg0: TIMER1_CFG0,
    timer1_cfg1: TIMER1_CFG1,
    timer1_sync: TIMER1_SYNC,
    timer1_status: TIMER1_STATUS,
    timer2_cfg0: TIMER2_CFG0,
    timer2_cfg1: TIMER2_CFG1,
    timer2_sync: TIMER2_SYNC,
    timer2_status: TIMER2_STATUS,
    timer_synci_cfg: TIMER_SYNCI_CFG,
    operator_timersel: OPERATOR_TIMERSEL,
    gen0_stmp_cfg: GEN0_STMP_CFG,
    gen0_tstmp_a: GEN0_TSTMP_A,
    gen0_tstmp_b: GEN0_TSTMP_B,
    gen0_cfg0: GEN0_CFG0,
    gen0_force: GEN0_FORCE,
    gen0_a: GEN0_A,
    gen0_b: GEN0_B,
    dt0_cfg: DT0_CFG,
    dt0_fed_cfg: DT0_FED_CFG,
    dt0_red_cfg: DT0_RED_CFG,
    carrier0_cfg: CARRIER0_CFG,
    fh0_cfg0: FH0_CFG0,
    fh0_cfg1: FH0_CFG1,
    fh0_status: FH0_STATUS,
    gen1_stmp_cfg: GEN1_STMP_CFG,
    gen1_tstmp_a: GEN1_TSTMP_A,
    gen1_tstmp_b: GEN1_TSTMP_B,
    gen1_cfg0: GEN1_CFG0,
    gen1_force: GEN1_FORCE,
    gen1_a: GEN1_A,
    gen1_b: GEN1_B,
    dt1_cfg: DT1_CFG,
    dt1_fed_cfg: DT1_FED_CFG,
    dt1_red_cfg: DT1_RED_CFG,
    carrier1_cfg: CARRIER1_CFG,
    fh1_cfg0: FH1_CFG0,
    fh1_cfg1: FH1_CFG1,
    fh1_status: FH1_STATUS,
    gen2_stmp_cfg: GEN2_STMP_CFG,
    gen2_tstmp_a: GEN2_TSTMP_A,
    gen2_tstmp_b: GEN2_TSTMP_B,
    gen2_cfg0: GEN2_CFG0,
    gen2_force: GEN2_FORCE,
    gen2_a: GEN2_A,
    gen2_b: GEN2_B,
    dt2_cfg: DT2_CFG,
    dt2_fed_cfg: DT2_FED_CFG,
    dt2_red_cfg: DT2_RED_CFG,
    carrier2_cfg: CARRIER2_CFG,
    fh2_cfg0: FH2_CFG0,
    fh2_cfg1: FH2_CFG1,
    fh2_status: FH2_STATUS,
    fault_detect: FAULT_DETECT,
    cap_timer_cfg: CAP_TIMER_CFG,
    cap_timer_phase: CAP_TIMER_PHASE,
    cap_ch0_cfg: CAP_CH0_CFG,
    cap_ch1_cfg: CAP_CH1_CFG,
    cap_ch2_cfg: CAP_CH2_CFG,
    cap_ch0: CAP_CH0,
    cap_ch1: CAP_CH1,
    cap_ch2: CAP_CH2,
    cap_status: CAP_STATUS,
    update_cfg: UPDATE_CFG,
    int_ena: INT_ENA,
    int_raw: INT_RAW,
    int_st: INT_ST,
    int_clr: INT_CLR,
    clk: CLK,
    version: VERSION,
}
impl RegisterBlock {
    #[doc = "0x00 - "]
    #[inline(always)]
    pub const fn clk_cfg(&self) -> &CLK_CFG {
        &self.clk_cfg
    }
    #[doc = "0x04 - "]
    #[inline(always)]
    pub const fn timer0_cfg0(&self) -> &TIMER0_CFG0 {
        &self.timer0_cfg0
    }
    #[doc = "0x08 - "]
    #[inline(always)]
    pub const fn timer0_cfg1(&self) -> &TIMER0_CFG1 {
        &self.timer0_cfg1
    }
    #[doc = "0x0c - "]
    #[inline(always)]
    pub const fn timer0_sync(&self) -> &TIMER0_SYNC {
        &self.timer0_sync
    }
    #[doc = "0x10 - "]
    #[inline(always)]
    pub const fn timer0_status(&self) -> &TIMER0_STATUS {
        &self.timer0_status
    }
    #[doc = "0x14 - "]
    #[inline(always)]
    pub const fn timer1_cfg0(&self) -> &TIMER1_CFG0 {
        &self.timer1_cfg0
    }
    #[doc = "0x18 - "]
    #[inline(always)]
    pub const fn timer1_cfg1(&self) -> &TIMER1_CFG1 {
        &self.timer1_cfg1
    }
    #[doc = "0x1c - "]
    #[inline(always)]
    pub const fn timer1_sync(&self) -> &TIMER1_SYNC {
        &self.timer1_sync
    }
    #[doc = "0x20 - "]
    #[inline(always)]
    pub const fn timer1_status(&self) -> &TIMER1_STATUS {
        &self.timer1_status
    }
    #[doc = "0x24 - "]
    #[inline(always)]
    pub const fn timer2_cfg0(&self) -> &TIMER2_CFG0 {
        &self.timer2_cfg0
    }
    #[doc = "0x28 - "]
    #[inline(always)]
    pub const fn timer2_cfg1(&self) -> &TIMER2_CFG1 {
        &self.timer2_cfg1
    }
    #[doc = "0x2c - "]
    #[inline(always)]
    pub const fn timer2_sync(&self) -> &TIMER2_SYNC {
        &self.timer2_sync
    }
    #[doc = "0x30 - "]
    #[inline(always)]
    pub const fn timer2_status(&self) -> &TIMER2_STATUS {
        &self.timer2_status
    }
    #[doc = "0x34 - "]
    #[inline(always)]
    pub const fn timer_synci_cfg(&self) -> &TIMER_SYNCI_CFG {
        &self.timer_synci_cfg
    }
    #[doc = "0x38 - "]
    #[inline(always)]
    pub const fn operator_timersel(&self) -> &OPERATOR_TIMERSEL {
        &self.operator_timersel
    }
    #[doc = "0x3c - "]
    #[inline(always)]
    pub const fn gen0_stmp_cfg(&self) -> &GEN0_STMP_CFG {
        &self.gen0_stmp_cfg
    }
    #[doc = "0x40 - "]
    #[inline(always)]
    pub const fn gen0_tstmp_a(&self) -> &GEN0_TSTMP_A {
        &self.gen0_tstmp_a
    }
    #[doc = "0x44 - "]
    #[inline(always)]
    pub const fn gen0_tstmp_b(&self) -> &GEN0_TSTMP_B {
        &self.gen0_tstmp_b
    }
    #[doc = "0x48 - "]
    #[inline(always)]
    pub const fn gen0_cfg0(&self) -> &GEN0_CFG0 {
        &self.gen0_cfg0
    }
    #[doc = "0x4c - "]
    #[inline(always)]
    pub const fn gen0_force(&self) -> &GEN0_FORCE {
        &self.gen0_force
    }
    #[doc = "0x50 - "]
    #[inline(always)]
    pub const fn gen0_a(&self) -> &GEN0_A {
        &self.gen0_a
    }
    #[doc = "0x54 - "]
    #[inline(always)]
    pub const fn gen0_b(&self) -> &GEN0_B {
        &self.gen0_b
    }
    #[doc = "0x58 - "]
    #[inline(always)]
    pub const fn dt0_cfg(&self) -> &DT0_CFG {
        &self.dt0_cfg
    }
    #[doc = "0x5c - "]
    #[inline(always)]
    pub const fn dt0_fed_cfg(&self) -> &DT0_FED_CFG {
        &self.dt0_fed_cfg
    }
    #[doc = "0x60 - "]
    #[inline(always)]
    pub const fn dt0_red_cfg(&self) -> &DT0_RED_CFG {
        &self.dt0_red_cfg
    }
    #[doc = "0x64 - "]
    #[inline(always)]
    pub const fn carrier0_cfg(&self) -> &CARRIER0_CFG {
        &self.carrier0_cfg
    }
    #[doc = "0x68 - "]
    #[inline(always)]
    pub const fn fh0_cfg0(&self) -> &FH0_CFG0 {
        &self.fh0_cfg0
    }
    #[doc = "0x6c - "]
    #[inline(always)]
    pub const fn fh0_cfg1(&self) -> &FH0_CFG1 {
        &self.fh0_cfg1
    }
    #[doc = "0x70 - "]
    #[inline(always)]
    pub const fn fh0_status(&self) -> &FH0_STATUS {
        &self.fh0_status
    }
    #[doc = "0x74 - "]
    #[inline(always)]
    pub const fn gen1_stmp_cfg(&self) -> &GEN1_STMP_CFG {
        &self.gen1_stmp_cfg
    }
    #[doc = "0x78 - "]
    #[inline(always)]
    pub const fn gen1_tstmp_a(&self) -> &GEN1_TSTMP_A {
        &self.gen1_tstmp_a
    }
    #[doc = "0x7c - "]
    #[inline(always)]
    pub const fn gen1_tstmp_b(&self) -> &GEN1_TSTMP_B {
        &self.gen1_tstmp_b
    }
    #[doc = "0x80 - "]
    #[inline(always)]
    pub const fn gen1_cfg0(&self) -> &GEN1_CFG0 {
        &self.gen1_cfg0
    }
    #[doc = "0x84 - "]
    #[inline(always)]
    pub const fn gen1_force(&self) -> &GEN1_FORCE {
        &self.gen1_force
    }
    #[doc = "0x88 - "]
    #[inline(always)]
    pub const fn gen1_a(&self) -> &GEN1_A {
        &self.gen1_a
    }
    #[doc = "0x8c - "]
    #[inline(always)]
    pub const fn gen1_b(&self) -> &GEN1_B {
        &self.gen1_b
    }
    #[doc = "0x90 - "]
    #[inline(always)]
    pub const fn dt1_cfg(&self) -> &DT1_CFG {
        &self.dt1_cfg
    }
    #[doc = "0x94 - "]
    #[inline(always)]
    pub const fn dt1_fed_cfg(&self) -> &DT1_FED_CFG {
        &self.dt1_fed_cfg
    }
    #[doc = "0x98 - "]
    #[inline(always)]
    pub const fn dt1_red_cfg(&self) -> &DT1_RED_CFG {
        &self.dt1_red_cfg
    }
    #[doc = "0x9c - "]
    #[inline(always)]
    pub const fn carrier1_cfg(&self) -> &CARRIER1_CFG {
        &self.carrier1_cfg
    }
    #[doc = "0xa0 - "]
    #[inline(always)]
    pub const fn fh1_cfg0(&self) -> &FH1_CFG0 {
        &self.fh1_cfg0
    }
    #[doc = "0xa4 - "]
    #[inline(always)]
    pub const fn fh1_cfg1(&self) -> &FH1_CFG1 {
        &self.fh1_cfg1
    }
    #[doc = "0xa8 - "]
    #[inline(always)]
    pub const fn fh1_status(&self) -> &FH1_STATUS {
        &self.fh1_status
    }
    #[doc = "0xac - "]
    #[inline(always)]
    pub const fn gen2_stmp_cfg(&self) -> &GEN2_STMP_CFG {
        &self.gen2_stmp_cfg
    }
    #[doc = "0xb0 - "]
    #[inline(always)]
    pub const fn gen2_tstmp_a(&self) -> &GEN2_TSTMP_A {
        &self.gen2_tstmp_a
    }
    #[doc = "0xb4 - "]
    #[inline(always)]
    pub const fn gen2_tstmp_b(&self) -> &GEN2_TSTMP_B {
        &self.gen2_tstmp_b
    }
    #[doc = "0xb8 - "]
    #[inline(always)]
    pub const fn gen2_cfg0(&self) -> &GEN2_CFG0 {
        &self.gen2_cfg0
    }
    #[doc = "0xbc - "]
    #[inline(always)]
    pub const fn gen2_force(&self) -> &GEN2_FORCE {
        &self.gen2_force
    }
    #[doc = "0xc0 - "]
    #[inline(always)]
    pub const fn gen2_a(&self) -> &GEN2_A {
        &self.gen2_a
    }
    #[doc = "0xc4 - "]
    #[inline(always)]
    pub const fn gen2_b(&self) -> &GEN2_B {
        &self.gen2_b
    }
    #[doc = "0xc8 - "]
    #[inline(always)]
    pub const fn dt2_cfg(&self) -> &DT2_CFG {
        &self.dt2_cfg
    }
    #[doc = "0xcc - "]
    #[inline(always)]
    pub const fn dt2_fed_cfg(&self) -> &DT2_FED_CFG {
        &self.dt2_fed_cfg
    }
    #[doc = "0xd0 - "]
    #[inline(always)]
    pub const fn dt2_red_cfg(&self) -> &DT2_RED_CFG {
        &self.dt2_red_cfg
    }
    #[doc = "0xd4 - "]
    #[inline(always)]
    pub const fn carrier2_cfg(&self) -> &CARRIER2_CFG {
        &self.carrier2_cfg
    }
    #[doc = "0xd8 - "]
    #[inline(always)]
    pub const fn fh2_cfg0(&self) -> &FH2_CFG0 {
        &self.fh2_cfg0
    }
    #[doc = "0xdc - "]
    #[inline(always)]
    pub const fn fh2_cfg1(&self) -> &FH2_CFG1 {
        &self.fh2_cfg1
    }
    #[doc = "0xe0 - "]
    #[inline(always)]
    pub const fn fh2_status(&self) -> &FH2_STATUS {
        &self.fh2_status
    }
    #[doc = "0xe4 - "]
    #[inline(always)]
    pub const fn fault_detect(&self) -> &FAULT_DETECT {
        &self.fault_detect
    }
    #[doc = "0xe8 - "]
    #[inline(always)]
    pub const fn cap_timer_cfg(&self) -> &CAP_TIMER_CFG {
        &self.cap_timer_cfg
    }
    #[doc = "0xec - "]
    #[inline(always)]
    pub const fn cap_timer_phase(&self) -> &CAP_TIMER_PHASE {
        &self.cap_timer_phase
    }
    #[doc = "0xf0 - "]
    #[inline(always)]
    pub const fn cap_ch0_cfg(&self) -> &CAP_CH0_CFG {
        &self.cap_ch0_cfg
    }
    #[doc = "0xf4 - "]
    #[inline(always)]
    pub const fn cap_ch1_cfg(&self) -> &CAP_CH1_CFG {
        &self.cap_ch1_cfg
    }
    #[doc = "0xf8 - "]
    #[inline(always)]
    pub const fn cap_ch2_cfg(&self) -> &CAP_CH2_CFG {
        &self.cap_ch2_cfg
    }
    #[doc = "0xfc - "]
    #[inline(always)]
    pub const fn cap_ch0(&self) -> &CAP_CH0 {
        &self.cap_ch0
    }
    #[doc = "0x100 - "]
    #[inline(always)]
    pub const fn cap_ch1(&self) -> &CAP_CH1 {
        &self.cap_ch1
    }
    #[doc = "0x104 - "]
    #[inline(always)]
    pub const fn cap_ch2(&self) -> &CAP_CH2 {
        &self.cap_ch2
    }
    #[doc = "0x108 - "]
    #[inline(always)]
    pub const fn cap_status(&self) -> &CAP_STATUS {
        &self.cap_status
    }
    #[doc = "0x10c - "]
    #[inline(always)]
    pub const fn update_cfg(&self) -> &UPDATE_CFG {
        &self.update_cfg
    }
    #[doc = "0x110 - "]
    #[inline(always)]
    pub const fn int_ena(&self) -> &INT_ENA {
        &self.int_ena
    }
    #[doc = "0x114 - "]
    #[inline(always)]
    pub const fn int_raw(&self) -> &INT_RAW {
        &self.int_raw
    }
    #[doc = "0x118 - "]
    #[inline(always)]
    pub const fn int_st(&self) -> &INT_ST {
        &self.int_st
    }
    #[doc = "0x11c - "]
    #[inline(always)]
    pub const fn int_clr(&self) -> &INT_CLR {
        &self.int_clr
    }
    #[doc = "0x120 - "]
    #[inline(always)]
    pub const fn clk(&self) -> &CLK {
        &self.clk
    }
    #[doc = "0x124 - "]
    #[inline(always)]
    pub const fn version(&self) -> &VERSION {
        &self.version
    }
}
#[doc = "CLK_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk_cfg`] module"]
pub type CLK_CFG = crate::Reg<clk_cfg::CLK_CFG_SPEC>;
#[doc = ""]
pub mod clk_cfg;
#[doc = "TIMER0_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cfg0`] module"]
pub type TIMER0_CFG0 = crate::Reg<timer0_cfg0::TIMER0_CFG0_SPEC>;
#[doc = ""]
pub mod timer0_cfg0;
#[doc = "TIMER0_CFG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_cfg1`] module"]
pub type TIMER0_CFG1 = crate::Reg<timer0_cfg1::TIMER0_CFG1_SPEC>;
#[doc = ""]
pub mod timer0_cfg1;
#[doc = "TIMER0_SYNC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer0_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_sync`] module"]
pub type TIMER0_SYNC = crate::Reg<timer0_sync::TIMER0_SYNC_SPEC>;
#[doc = ""]
pub mod timer0_sync;
#[doc = "TIMER0_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer0_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer0_status`] module"]
pub type TIMER0_STATUS = crate::Reg<timer0_status::TIMER0_STATUS_SPEC>;
#[doc = ""]
pub mod timer0_status;
#[doc = "TIMER1_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cfg0`] module"]
pub type TIMER1_CFG0 = crate::Reg<timer1_cfg0::TIMER1_CFG0_SPEC>;
#[doc = ""]
pub mod timer1_cfg0;
#[doc = "TIMER1_CFG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_cfg1`] module"]
pub type TIMER1_CFG1 = crate::Reg<timer1_cfg1::TIMER1_CFG1_SPEC>;
#[doc = ""]
pub mod timer1_cfg1;
#[doc = "TIMER1_SYNC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer1_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_sync`] module"]
pub type TIMER1_SYNC = crate::Reg<timer1_sync::TIMER1_SYNC_SPEC>;
#[doc = ""]
pub mod timer1_sync;
#[doc = "TIMER1_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer1_status`] module"]
pub type TIMER1_STATUS = crate::Reg<timer1_status::TIMER1_STATUS_SPEC>;
#[doc = ""]
pub mod timer1_status;
#[doc = "TIMER2_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cfg0`] module"]
pub type TIMER2_CFG0 = crate::Reg<timer2_cfg0::TIMER2_CFG0_SPEC>;
#[doc = ""]
pub mod timer2_cfg0;
#[doc = "TIMER2_CFG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_cfg1`] module"]
pub type TIMER2_CFG1 = crate::Reg<timer2_cfg1::TIMER2_CFG1_SPEC>;
#[doc = ""]
pub mod timer2_cfg1;
#[doc = "TIMER2_SYNC (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_sync::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer2_sync::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_sync`] module"]
pub type TIMER2_SYNC = crate::Reg<timer2_sync::TIMER2_SYNC_SPEC>;
#[doc = ""]
pub mod timer2_sync;
#[doc = "TIMER2_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer2_status`] module"]
pub type TIMER2_STATUS = crate::Reg<timer2_status::TIMER2_STATUS_SPEC>;
#[doc = ""]
pub mod timer2_status;
#[doc = "TIMER_SYNCI_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer_synci_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`timer_synci_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@timer_synci_cfg`] module"]
pub type TIMER_SYNCI_CFG = crate::Reg<timer_synci_cfg::TIMER_SYNCI_CFG_SPEC>;
#[doc = ""]
pub mod timer_synci_cfg;
#[doc = "OPERATOR_TIMERSEL (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`operator_timersel::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`operator_timersel::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@operator_timersel`] module"]
pub type OPERATOR_TIMERSEL = crate::Reg<operator_timersel::OPERATOR_TIMERSEL_SPEC>;
#[doc = ""]
pub mod operator_timersel;
#[doc = "GEN0_STMP_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_stmp_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_stmp_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen0_stmp_cfg`] module"]
pub type GEN0_STMP_CFG = crate::Reg<gen0_stmp_cfg::GEN0_STMP_CFG_SPEC>;
#[doc = ""]
pub mod gen0_stmp_cfg;
#[doc = "GEN0_TSTMP_A (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_tstmp_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_tstmp_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen0_tstmp_a`] module"]
pub type GEN0_TSTMP_A = crate::Reg<gen0_tstmp_a::GEN0_TSTMP_A_SPEC>;
#[doc = ""]
pub mod gen0_tstmp_a;
#[doc = "GEN0_TSTMP_B (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_tstmp_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_tstmp_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen0_tstmp_b`] module"]
pub type GEN0_TSTMP_B = crate::Reg<gen0_tstmp_b::GEN0_TSTMP_B_SPEC>;
#[doc = ""]
pub mod gen0_tstmp_b;
#[doc = "GEN0_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen0_cfg0`] module"]
pub type GEN0_CFG0 = crate::Reg<gen0_cfg0::GEN0_CFG0_SPEC>;
#[doc = ""]
pub mod gen0_cfg0;
#[doc = "GEN0_FORCE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen0_force`] module"]
pub type GEN0_FORCE = crate::Reg<gen0_force::GEN0_FORCE_SPEC>;
#[doc = ""]
pub mod gen0_force;
#[doc = "GEN0_A (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen0_a`] module"]
pub type GEN0_A = crate::Reg<gen0_a::GEN0_A_SPEC>;
#[doc = ""]
pub mod gen0_a;
#[doc = "GEN0_B (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen0_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen0_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen0_b`] module"]
pub type GEN0_B = crate::Reg<gen0_b::GEN0_B_SPEC>;
#[doc = ""]
pub mod gen0_b;
#[doc = "DT0_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt0_cfg`] module"]
pub type DT0_CFG = crate::Reg<dt0_cfg::DT0_CFG_SPEC>;
#[doc = ""]
pub mod dt0_cfg;
#[doc = "DT0_FED_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt0_fed_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt0_fed_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt0_fed_cfg`] module"]
pub type DT0_FED_CFG = crate::Reg<dt0_fed_cfg::DT0_FED_CFG_SPEC>;
#[doc = ""]
pub mod dt0_fed_cfg;
#[doc = "DT0_RED_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt0_red_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt0_red_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt0_red_cfg`] module"]
pub type DT0_RED_CFG = crate::Reg<dt0_red_cfg::DT0_RED_CFG_SPEC>;
#[doc = ""]
pub mod dt0_red_cfg;
#[doc = "CARRIER0_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@carrier0_cfg`] module"]
pub type CARRIER0_CFG = crate::Reg<carrier0_cfg::CARRIER0_CFG_SPEC>;
#[doc = ""]
pub mod carrier0_cfg;
#[doc = "FH0_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh0_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh0_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh0_cfg0`] module"]
pub type FH0_CFG0 = crate::Reg<fh0_cfg0::FH0_CFG0_SPEC>;
#[doc = ""]
pub mod fh0_cfg0;
#[doc = "FH0_CFG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh0_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh0_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh0_cfg1`] module"]
pub type FH0_CFG1 = crate::Reg<fh0_cfg1::FH0_CFG1_SPEC>;
#[doc = ""]
pub mod fh0_cfg1;
#[doc = "FH0_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh0_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh0_status`] module"]
pub type FH0_STATUS = crate::Reg<fh0_status::FH0_STATUS_SPEC>;
#[doc = ""]
pub mod fh0_status;
#[doc = "GEN1_STMP_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_stmp_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_stmp_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen1_stmp_cfg`] module"]
pub type GEN1_STMP_CFG = crate::Reg<gen1_stmp_cfg::GEN1_STMP_CFG_SPEC>;
#[doc = ""]
pub mod gen1_stmp_cfg;
#[doc = "GEN1_TSTMP_A (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_tstmp_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_tstmp_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen1_tstmp_a`] module"]
pub type GEN1_TSTMP_A = crate::Reg<gen1_tstmp_a::GEN1_TSTMP_A_SPEC>;
#[doc = ""]
pub mod gen1_tstmp_a;
#[doc = "GEN1_TSTMP_B (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_tstmp_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_tstmp_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen1_tstmp_b`] module"]
pub type GEN1_TSTMP_B = crate::Reg<gen1_tstmp_b::GEN1_TSTMP_B_SPEC>;
#[doc = ""]
pub mod gen1_tstmp_b;
#[doc = "GEN1_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen1_cfg0`] module"]
pub type GEN1_CFG0 = crate::Reg<gen1_cfg0::GEN1_CFG0_SPEC>;
#[doc = ""]
pub mod gen1_cfg0;
#[doc = "GEN1_FORCE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen1_force`] module"]
pub type GEN1_FORCE = crate::Reg<gen1_force::GEN1_FORCE_SPEC>;
#[doc = ""]
pub mod gen1_force;
#[doc = "GEN1_A (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen1_a`] module"]
pub type GEN1_A = crate::Reg<gen1_a::GEN1_A_SPEC>;
#[doc = ""]
pub mod gen1_a;
#[doc = "GEN1_B (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen1_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen1_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen1_b`] module"]
pub type GEN1_B = crate::Reg<gen1_b::GEN1_B_SPEC>;
#[doc = ""]
pub mod gen1_b;
#[doc = "DT1_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt1_cfg`] module"]
pub type DT1_CFG = crate::Reg<dt1_cfg::DT1_CFG_SPEC>;
#[doc = ""]
pub mod dt1_cfg;
#[doc = "DT1_FED_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1_fed_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1_fed_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt1_fed_cfg`] module"]
pub type DT1_FED_CFG = crate::Reg<dt1_fed_cfg::DT1_FED_CFG_SPEC>;
#[doc = ""]
pub mod dt1_fed_cfg;
#[doc = "DT1_RED_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt1_red_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt1_red_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt1_red_cfg`] module"]
pub type DT1_RED_CFG = crate::Reg<dt1_red_cfg::DT1_RED_CFG_SPEC>;
#[doc = ""]
pub mod dt1_red_cfg;
#[doc = "CARRIER1_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@carrier1_cfg`] module"]
pub type CARRIER1_CFG = crate::Reg<carrier1_cfg::CARRIER1_CFG_SPEC>;
#[doc = ""]
pub mod carrier1_cfg;
#[doc = "FH1_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh1_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh1_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh1_cfg0`] module"]
pub type FH1_CFG0 = crate::Reg<fh1_cfg0::FH1_CFG0_SPEC>;
#[doc = ""]
pub mod fh1_cfg0;
#[doc = "FH1_CFG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh1_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh1_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh1_cfg1`] module"]
pub type FH1_CFG1 = crate::Reg<fh1_cfg1::FH1_CFG1_SPEC>;
#[doc = ""]
pub mod fh1_cfg1;
#[doc = "FH1_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh1_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh1_status`] module"]
pub type FH1_STATUS = crate::Reg<fh1_status::FH1_STATUS_SPEC>;
#[doc = ""]
pub mod fh1_status;
#[doc = "GEN2_STMP_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_stmp_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_stmp_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen2_stmp_cfg`] module"]
pub type GEN2_STMP_CFG = crate::Reg<gen2_stmp_cfg::GEN2_STMP_CFG_SPEC>;
#[doc = ""]
pub mod gen2_stmp_cfg;
#[doc = "GEN2_TSTMP_A (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_tstmp_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_tstmp_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen2_tstmp_a`] module"]
pub type GEN2_TSTMP_A = crate::Reg<gen2_tstmp_a::GEN2_TSTMP_A_SPEC>;
#[doc = ""]
pub mod gen2_tstmp_a;
#[doc = "GEN2_TSTMP_B (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_tstmp_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_tstmp_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen2_tstmp_b`] module"]
pub type GEN2_TSTMP_B = crate::Reg<gen2_tstmp_b::GEN2_TSTMP_B_SPEC>;
#[doc = ""]
pub mod gen2_tstmp_b;
#[doc = "GEN2_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen2_cfg0`] module"]
pub type GEN2_CFG0 = crate::Reg<gen2_cfg0::GEN2_CFG0_SPEC>;
#[doc = ""]
pub mod gen2_cfg0;
#[doc = "GEN2_FORCE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_force::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_force::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen2_force`] module"]
pub type GEN2_FORCE = crate::Reg<gen2_force::GEN2_FORCE_SPEC>;
#[doc = ""]
pub mod gen2_force;
#[doc = "GEN2_A (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_a::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_a::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen2_a`] module"]
pub type GEN2_A = crate::Reg<gen2_a::GEN2_A_SPEC>;
#[doc = ""]
pub mod gen2_a;
#[doc = "GEN2_B (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gen2_b::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gen2_b::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gen2_b`] module"]
pub type GEN2_B = crate::Reg<gen2_b::GEN2_B_SPEC>;
#[doc = ""]
pub mod gen2_b;
#[doc = "DT2_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt2_cfg`] module"]
pub type DT2_CFG = crate::Reg<dt2_cfg::DT2_CFG_SPEC>;
#[doc = ""]
pub mod dt2_cfg;
#[doc = "DT2_FED_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt2_fed_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt2_fed_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt2_fed_cfg`] module"]
pub type DT2_FED_CFG = crate::Reg<dt2_fed_cfg::DT2_FED_CFG_SPEC>;
#[doc = ""]
pub mod dt2_fed_cfg;
#[doc = "DT2_RED_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dt2_red_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dt2_red_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dt2_red_cfg`] module"]
pub type DT2_RED_CFG = crate::Reg<dt2_red_cfg::DT2_RED_CFG_SPEC>;
#[doc = ""]
pub mod dt2_red_cfg;
#[doc = "CARRIER2_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`carrier2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`carrier2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@carrier2_cfg`] module"]
pub type CARRIER2_CFG = crate::Reg<carrier2_cfg::CARRIER2_CFG_SPEC>;
#[doc = ""]
pub mod carrier2_cfg;
#[doc = "FH2_CFG0 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh2_cfg0::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh2_cfg0::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh2_cfg0`] module"]
pub type FH2_CFG0 = crate::Reg<fh2_cfg0::FH2_CFG0_SPEC>;
#[doc = ""]
pub mod fh2_cfg0;
#[doc = "FH2_CFG1 (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh2_cfg1::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fh2_cfg1::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh2_cfg1`] module"]
pub type FH2_CFG1 = crate::Reg<fh2_cfg1::FH2_CFG1_SPEC>;
#[doc = ""]
pub mod fh2_cfg1;
#[doc = "FH2_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fh2_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fh2_status`] module"]
pub type FH2_STATUS = crate::Reg<fh2_status::FH2_STATUS_SPEC>;
#[doc = ""]
pub mod fh2_status;
#[doc = "FAULT_DETECT (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fault_detect::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fault_detect::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fault_detect`] module"]
pub type FAULT_DETECT = crate::Reg<fault_detect::FAULT_DETECT_SPEC>;
#[doc = ""]
pub mod fault_detect;
#[doc = "CAP_TIMER_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_timer_cfg`] module"]
pub type CAP_TIMER_CFG = crate::Reg<cap_timer_cfg::CAP_TIMER_CFG_SPEC>;
#[doc = ""]
pub mod cap_timer_cfg;
#[doc = "CAP_TIMER_PHASE (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_phase::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_phase::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_timer_phase`] module"]
pub type CAP_TIMER_PHASE = crate::Reg<cap_timer_phase::CAP_TIMER_PHASE_SPEC>;
#[doc = ""]
pub mod cap_timer_phase;
#[doc = "CAP_CH0_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch0_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch0_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch0_cfg`] module"]
pub type CAP_CH0_CFG = crate::Reg<cap_ch0_cfg::CAP_CH0_CFG_SPEC>;
#[doc = ""]
pub mod cap_ch0_cfg;
#[doc = "CAP_CH1_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch1_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch1_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch1_cfg`] module"]
pub type CAP_CH1_CFG = crate::Reg<cap_ch1_cfg::CAP_CH1_CFG_SPEC>;
#[doc = ""]
pub mod cap_ch1_cfg;
#[doc = "CAP_CH2_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch2_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch2_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch2_cfg`] module"]
pub type CAP_CH2_CFG = crate::Reg<cap_ch2_cfg::CAP_CH2_CFG_SPEC>;
#[doc = ""]
pub mod cap_ch2_cfg;
#[doc = "CAP_CH0 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch0::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch0`] module"]
pub type CAP_CH0 = crate::Reg<cap_ch0::CAP_CH0_SPEC>;
#[doc = ""]
pub mod cap_ch0;
#[doc = "CAP_CH1 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch1::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch1`] module"]
pub type CAP_CH1 = crate::Reg<cap_ch1::CAP_CH1_SPEC>;
#[doc = ""]
pub mod cap_ch1;
#[doc = "CAP_CH2 (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch2::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_ch2`] module"]
pub type CAP_CH2 = crate::Reg<cap_ch2::CAP_CH2_SPEC>;
#[doc = ""]
pub mod cap_ch2;
#[doc = "CAP_STATUS (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_status::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cap_status`] module"]
pub type CAP_STATUS = crate::Reg<cap_status::CAP_STATUS_SPEC>;
#[doc = ""]
pub mod cap_status;
#[doc = "UPDATE_CFG (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`update_cfg::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`update_cfg::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@update_cfg`] module"]
pub type UPDATE_CFG = crate::Reg<update_cfg::UPDATE_CFG_SPEC>;
#[doc = ""]
pub mod update_cfg;
#[doc = "INT_ENA (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_ena`] module"]
pub type INT_ENA = crate::Reg<int_ena::INT_ENA_SPEC>;
#[doc = ""]
pub mod int_ena;
#[doc = "INT_RAW (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_raw`] module"]
pub type INT_RAW = crate::Reg<int_raw::INT_RAW_SPEC>;
#[doc = ""]
pub mod int_raw;
#[doc = "INT_ST (r) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_st`] module"]
pub type INT_ST = crate::Reg<int_st::INT_ST_SPEC>;
#[doc = ""]
pub mod int_st;
#[doc = "INT_CLR (w) register accessor: \n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@int_clr`] module"]
pub type INT_CLR = crate::Reg<int_clr::INT_CLR_SPEC>;
#[doc = ""]
pub mod int_clr;
#[doc = "CLK (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clk`] module"]
pub type CLK = crate::Reg<clk::CLK_SPEC>;
#[doc = ""]
pub mod clk;
#[doc = "VERSION (rw) register accessor: \n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`version::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@version`] module"]
pub type VERSION = crate::Reg<version::VERSION_SPEC>;
#[doc = ""]
pub mod version;
