//! 0x8c0326a0 — `p->a = 0.0f; p->b = p->c; p->d = param_2(float);`
//!
//! ```text
//! mov.l  0x8c0326c4,r0        ; r0 = 0x4f8
//! mov    r4,r3
//! mov.l  0x8c0326c8,r1        ; r1 = 0x4ec
//! add    r0,r3                ; r3 = r4 + 0x4f8
//! fldi0  fr0
//! mov    r4,r0
//! add    r1,r0                ; r0 = r4 + 0x4ec
//! fmov   fr0,@r3              ; [r4+0x4f8] = 0.0f
//! mov    r4,r1
//! fmov   @r0,fr0              ; fr0 = [r4+0x4ec]
//! mov.l  0x8c0326cc,r0        ; r0 = 0x4f0
//! add    r0,r1                ; r1 = r4 + 0x4f0
//! mov.l  0x8c0326d0,r0        ; r0 = 0x4f4
//! fmov   fr0,@r1              ; [r4+0x4f0] = old [r4+0x4ec]
//! add    r0,r4                ; r4 += 0x4f4
//! rts
//! fmov   fr4,@r4              ; delay slot: [r4+0x4f4] = fr4 (r4 already advanced)
//! ```
//! Hidden side effects: three float stores at r4+0x4f8/0x4f0/0x4f4, the last one
//! in the `rts` delay slot after r4 was advanced; r0 = 0x4f4, r1 = r4+0x4f0,
//! r3 = r4+0x4f8, r4 = param+0x4f4 all stay behind; fr0 keeps the loaded value.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    cpu.r[0] = mem.read_u32(0x8C0326C4).unwrap(); // 0x4f8
    cpu.r[3] = base;
    cpu.r[1] = mem.read_u32(0x8C0326C8).unwrap(); // 0x4ec
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]);
    cpu.fr[0] = 0.0; // fldi0
    cpu.r[0] = base;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    let _ = mem.write_f32(cpu.r[3], cpu.fr[0]);
    cpu.r[1] = base;
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    cpu.r[0] = mem.read_u32(0x8C0326CC).unwrap(); // 0x4f0
    cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
    cpu.r[0] = mem.read_u32(0x8C0326D0).unwrap(); // 0x4f4
    let _ = mem.write_f32(cpu.r[1], cpu.fr[0]);
    cpu.r[4] = base.wrapping_add(cpu.r[0]);
    let _ = mem.write_f32(cpu.r[4], cpu.fr[4]); // delay slot of `rts`
}
