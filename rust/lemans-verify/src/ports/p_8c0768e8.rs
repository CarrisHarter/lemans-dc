//! 0x8c0768e8 — clamp: return r5 if r5 > r4, else r6 if r4 > r6 (with r6 := r4), else r4.
//!
//! Original side effects: r0 = result (rts delay mov r6,r0), r6 = r4 on the
//! low path, T = last cmp/gt.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    cpu.set_t((cpu.r[5] as i32) > (cpu.r[4] as i32)); // cmp/gt r4,r5
    if (cpu.r[5] as i32) > (cpu.r[4] as i32) {
        cpu.r[0] = cpu.r[5]; // rts delay slot
        return;
    }
    cpu.set_t((cpu.r[4] as i32) > (cpu.r[6] as i32)); // cmp/gt r6,r4
    if (cpu.r[4] as i32) > (cpu.r[6] as i32) {
        cpu.r[0] = cpu.r[6]; // rts delay slot (via bra)
        return;
    }
    cpu.r[6] = cpu.r[4];
    cpu.r[0] = cpu.r[6]; // rts delay slot
}
