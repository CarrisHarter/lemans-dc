//! 0x8c06d308 — same bit-test dispatcher as 0x8c06d2a0, one table over:
//! masks at 0x8c06d350/358/360/368 (0x07000000/0x01000000/0x02000000/0x04000000),
//! float pointers 0x8c2400f8/0x8c240108/0x8c240118/0x8c240128.
//!
//! Hidden side effects the decompiled C does not show: r0 ends as the selected
//! pointer, r1 keeps the last mask loaded (0x02000000 on the last two paths),
//! r4 is truncated to `r4 & 0x04000000` on the last two paths, and T is 1
//! only on the default path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8c06d350).unwrap(); // 0x07000000
    cpu.r[0] = cpu.r[4] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(0x8c06d354).unwrap(); // 0x8c2400f8
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // rts delay slot
        return;
    }
    cpu.r[1] = mem.read_u32(0x8c06d358).unwrap(); // 0x01000000
    cpu.r[0] = cpu.r[4] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(0x8c06d35c).unwrap(); // 0x8c240108
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        return;
    }
    cpu.r[1] = mem.read_u32(0x8c06d360).unwrap(); // 0x02000000
    cpu.r[0] = cpu.r[4] & cpu.r[1];
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        cpu.r[0] = mem.read_u32(0x8c06d364).unwrap(); // 0x8c240118
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        return;
    }
    cpu.r[0] = mem.read_u32(0x8c06d368).unwrap(); // 0x04000000
    cpu.r[4] &= cpu.r[0]; // and r0,r4 clobbers the argument register
    cpu.set_t(cpu.r[4] == 0); // tst r4,r4
    if cpu.r[4] != 0 {
        cpu.r[0] = mem.read_u32(0x8c06d36c).unwrap(); // 0x8c240128
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
        return;
    }
    // default path: T stays 1 from the last tst
    cpu.r[0] = mem.read_u32(0x8c06d354).unwrap(); // 0x8c2400f8
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
}
