//! 0x8c069260 — select: returns r5 if r5 > r4, else r6 if r4 > r6, else r4.
//!
//! Original:
//! ```text
//! cmp/gt r4,r5 ; bf 0x8c069268 ; rts ; mov r5,r0
//! cmp/gt r6,r4 ; bf 0x8c069270 ; bra 0x8c069272 ; nop
//! mov r4,r6    ; rts ; mov r6,r0
//! ```
//! In this oracle `bf` is an immediate branch, so the `rts` at 0x8c069264 runs
//! only on the fall-through path (r5 > r4); a taken `bf` skips it entirely.
//! T ends as the last `cmp/gt` evaluated: set on the r5/r6 paths, (r4 > r6) on
//! the r4 path. `mov r4,r6` on the last path is a real side effect (r6 = r4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    let (r4, r5, r6) = (cpu.r[4], cpu.r[5], cpu.r[6]);
    if (r5 as i32) > (r4 as i32) {
        cpu.set_t(true); // cmp/gt r4,r5
        cpu.r[0] = r5; // delay slot of the fall-through rts
        return;
    }
    if (r4 as i32) > (r6 as i32) {
        cpu.set_t(true); // cmp/gt r6,r4
        cpu.r[0] = r6; // bra skips `mov r4,r6`; rts delay slot still copies r6
    } else {
        cpu.set_t(false); // cmp/gt r6,r4
        cpu.r[6] = r4; // mov r4,r6
        cpu.r[0] = cpu.r[6]; // delay slot of rts
    }
}
