pub const CLINT_BASE: usize = 0x2000000;

// CLINT_MSIP(hartid) (CLINT_BASE + 4 * (hartid))
#[inline(always)]
pub fn clint_msip(hartid: usize) -> usize {
    CLINT_BASE + 4 * (hartid)
}

// CLINT_MTIMECMP(hartid) (CLINT_BASE + 0x4000 + 8 * (hartid))
#[inline(always)]
pub fn clint_mtimecmp(hartid: usize) -> usize {
    CLINT_BASE + 0x4000 + 8 * hartid
}

// CLINT_MTIME (CLINT_BASE + 0xBFF8)
#[inline(always)]
pub fn clint_mtime() -> usize {
    CLINT_BASE + 0xBFF8
}
