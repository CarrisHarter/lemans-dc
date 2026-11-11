//! 0x8c06d2a0 — bit-test dispatcher: fr0 := the float at the pointer selected
//! by the first matching mask of r4 (0x07000000, 0x01000000, 0x02000000,
//! 0x04000000), defaulting to the 0x07000000 entry.
//!
//! ```text
//! mov.l 0x8c06d2e8,r1   ; r1 = 0x07000000
//! mov   r4,r0
//! and   r1,r0
//! tst   r0,r0
//! bt    0x8c06d2b0      ; not taken -> rts with fmov @r0,fr0 in the delay slot
//! ... (same pattern for 0x01000000 / 0x02000000) ...
//! mov.l 0x8c06d300,r0   ; r0 = 0x04000000
//! and   r0,r4           ; r4 IS CLOBBERED here
//! tst   r4,r4
//! bt    0x8c06d2de      ; default: reload r0 = 0x8c2400f0, fmov, rts
//! ```
//! Hidden side effects the decompiled C does not show: r0 ends as the selected
//! pointer, r1 keeps the last mask loaded (0x02000000 on the last two paths,
//! not the matching mask), r4 is truncated to `r4 & 0x04000000` on the last
//! two paths, and T is 1 only on the default path (every matched path leaves
//! T = 0 from its `tst`).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8c06d2e8).unwrap(); // 0x07000000
    cpu.r[0] = cpu.r[4] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(0x8c06d2ec).unwrap(); // 0x8c2400f0
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // rts delay slot
        return;
    }
    cpu.r[1] = mem.read_u32(0x8c06d2f0).unwrap(); // 0x01000000
    cpu.r[0] = cpu.r[4] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(0x8c06d2f4).unwrap(); // 0x8c240100
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        return;
    }
    cpu.r[1] = mem.read_u32(0x8c06d2f8).unwrap(); // 0x02000000
    cpu.r[0] = cpu.r[4] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(0x8c06d2fc).unwrap(); // 0x8c240110
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        return;
    }
    cpu.r[0] = mem.read_u32(0x8c06d300).unwrap(); // 0x04000000
    cpu.r[4] &= cpu.r[0]; // and r0,r4 clobbers the argument register
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] != 0 {
        cpu.r[0] = mem.read_u32(0x8c06d304).unwrap(); // 0x8c240120
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        return;
    }
    // default path: T stays 1 from the last tst
    cpu.r[0] = mem.read_u32(0x8c06d2ec).unwrap(); // 0x8c2400f0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
}
