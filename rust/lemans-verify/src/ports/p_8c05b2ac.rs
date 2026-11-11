//! 0x8c05b2ac — `param_1->field_0x24 ?: PTR_DAT_8c05b2bc` (null-coalescing).
//!
//! ```text
//! mov.l @(36,r4),r0
//! tst   r0,r0           ; T = (r0 == 0)
//! bt    0x8c05b2b6      ; immediate in the oracle
//! rts
//! nop
//! 0x8c05b2b6:
//! mov.l 0x8c05b2bc,r0   ; r0 = [0x8c05b2bc] (literal pool = 0x40000840)
//! rts
//! nop
//! ```
//! Hidden side effects: T ends 0 on the non-null path and 1 on the null path;
//! the fallback pointer comes from the literal pool at 0x8c05b2bc (the
//! disassembly's `! 8c1f7328` annotation is stale — the bytes hold 0x40000840),
//! so it is loaded through memory.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let v = mem.read_u32(cpu.r[4].wrapping_add(0x24)).unwrap_or(0);
    cpu.r[0] = v;
    cpu.set_t(v == 0); // tst r0,r0
    if v == 0 {
        cpu.r[0] = mem.read_u32(0x8c05b2bc).unwrap_or(0); // bt target
    }
}
