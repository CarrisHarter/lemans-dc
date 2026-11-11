//! 0x8c02ca10 — `return *(double?)(p+0x3a0) / *(float*)(p->next+0x278) * *(float*)(p->next+0x27c);`
//!
//! ```text
//! mov.l  @(4,r4),r3
//! mov.l  0x8c02ca2c,r0        ; r0 = 0x274
//! mov.l  0x8c02ca30,r1        ; r1 = 0x3a0
//! add    r0,r3                ; r3 = [r4+4] + 0x274
//! mov    r3,r0
//! add    r1,r4                ; r4 += 0x3a0
//! add    #4,r0                ; r0 = [r4+4] + 0x278
//! add    #8,r3                ; r3 = [r4+4] + 0x27c
//! fmov   @r4,fr0
//! fmov   @r0,fr2
//! fmov   @r3,fr1
//! fdiv   fr2,fr0              ; fr0 = fr0 / fr2
//! rts
//! fmul   fr1,fr0              ; delay slot: fr0 *= fr1
//! ```
//! Hidden side effects the C hides: r0/r1/r3/r4 are all left advanced
//! (r0 = base+0x278, r1 = 0x3a0, r3 = base+0x27c, r4 = param+0x3a0), fr1 and
//! fr2 stay loaded, and the multiply happens in the delay slot after `rts`.
//! No T-bit update: sr keeps whatever the caller had.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[3] = mem.read_u32(base.wrapping_add(4)).unwrap();
    cpu.r[0] = mem.read_u32(0x8C02CA2C).unwrap(); // 0x274
    cpu.r[1] = mem.read_u32(0x8C02CA30).unwrap(); // 0x3a0
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.r[0] = cpu.r[3];
    cpu.r[4] = base.wrapping_add(cpu.r[1]);
    cpu.r[0] = cpu.r[0].wrapping_add(4);
    cpu.r[3] = cpu.r[3].wrapping_add(8);
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap();
    cpu.fr[2] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.fr[1] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.fr[0] = cpu.fr[0] / cpu.fr[2]; // fdiv fr2,fr0
    cpu.fr[0] = cpu.fr[0] * cpu.fr[1]; // delay slot of `rts`: fmul fr1,fr0
}
