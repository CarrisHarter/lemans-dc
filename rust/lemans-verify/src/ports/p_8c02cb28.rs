//! 0x8c02cb28 — `v = *(float*)(p+0x450) / *(float*)(p->next+0x3e0); return clamp(v, 0, 1)`
//!
//! ```text
//! mov.l  @(4,r4),r1
//! fldi0  fr0                  ; fr0 = 0.0
//! mov.l  0x8c02cb54,r0        ; r0 = 0x3e0
//! mov.l  0x8c02cb58,r3        ; r3 = 0x450
//! add    r0,r1
//! fmov   @r1,fr1
//! add    r3,r4                ; r4 += 0x450
//! fmov   @r4,fr2
//! fdiv   fr1,fr2              ; fr2 = fr2 / fr1
//! fcmp/gt fr2,fr0             ; T = fr0 > fr2  (i.e. quotient < 0)
//! bf     0x8c02cb42
//! rts
//! nop                         ; negative: return fr0 = 0.0, T = 1
//! fldi1  fr0                  ; fr0 = 1.0
//! fcmp/gt fr0,fr2             ; T = fr2 > fr0  (quotient > 1)
//! bf     0x8c02cb4c
//! rts
//! nop                         ; > 1: return fr0 = 1.0, T = 1
//! fmov   fr2,fr0
//! rts
//! nop                         ; in [0,1]: return the quotient, T = 0
//! ```
//! Hidden side effects: r0 = 0x3e0, r1 = [r4+4] + 0x3e0, r3 = 0x450 and r4
//! stays advanced; fr1/fr2 stay loaded; the returned value lives in fr0 and
//! the T bit differs per path: 1 when the quotient is negative or > 1,
//! 0 when it lands in [0, 1].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[1] = mem.read_u32(base.wrapping_add(4)).unwrap();
    cpu.fr[0] = 0.0; // fldi0
    cpu.r[0] = mem.read_u32(0x8C02CB54).unwrap(); // 0x3e0
    cpu.r[3] = mem.read_u32(0x8C02CB58).unwrap(); // 0x450
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.fr[1] = mem.read_f32(cpu.r[1]).unwrap();
    cpu.r[4] = base.wrapping_add(cpu.r[3]);
    cpu.fr[2] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.fr[2] = cpu.fr[2] / cpu.fr[1]; // fdiv fr1,fr2
    let neg = cpu.fr[0] > cpu.fr[2]; // fcmp/gt fr2,fr0 -> T = fr0 > fr2
    cpu.set_t(neg);
    if neg {
        return; // rts + nop, fr0 stays 0.0, T = 1
    }
    cpu.fr[0] = 1.0; // fldi1
    let over = cpu.fr[2] > cpu.fr[0]; // fcmp/gt fr0,fr2 -> T = fr2 > fr0
    cpu.set_t(over);
    if over {
        return; // rts + nop, fr0 stays 1.0, T = 1
    }
    cpu.fr[0] = cpu.fr[2]; // fmov fr2,fr0
}
