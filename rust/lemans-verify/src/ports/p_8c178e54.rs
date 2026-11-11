//! 0x8c178e54 — build a 40-byte record at r4 from a 12-byte source at r5,
//! four f32 args (fr4..fr8) and two u32 args (r6,r7).
//!
//! Stores in original order: 3 u32 from [r5..r5+8] at r4+0/4/8, f32 fr4..fr8
//! at r4+12/16/20/24/28, then r6 at r4+32 and r7 at r4+36 (r6/r7 pass through
//! unchanged). r13/r14 are pushed and popped, so only r0, r1, r3, r5 leak:
//! r0=r4, r1=r4+28, r3=r4+24, r5=[r5+8]. The rts delay slot only pops r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let w0 = mem.read_u32(r5).unwrap();
    let w1 = mem.read_u32(r5 + 4).unwrap();
    let w2 = mem.read_u32(r5 + 8).unwrap();
    mem.write_u32(r4, w0).unwrap();
    mem.write_u32(r4 + 4, w1).unwrap();
    mem.write_u32(r4 + 8, w2).unwrap();
    mem.write_f32(r4 + 12, cpu.fr[4]).unwrap();
    mem.write_f32(r4 + 16, cpu.fr[5]).unwrap();
    mem.write_f32(r4 + 20, cpu.fr[6]).unwrap();
    mem.write_f32(r4 + 24, cpu.fr[7]).unwrap();
    mem.write_f32(r4 + 28, cpu.fr[8]).unwrap();
    mem.write_u32(r4 + 32, cpu.r[6]).unwrap();
    mem.write_u32(r4 + 36, cpu.r[7]).unwrap();
    cpu.r[0] = r4;
    cpu.r[1] = r4 + 28;
    cpu.r[3] = r4 + 24;
    cpu.r[5] = w2;
}
