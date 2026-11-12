//! 0x8c155fbc - store a flag byte, then tail into the bit-set helper with a guard.
//!
//! `[r4+89] = r5` first. `r6 = 0` when `[r4+88]` is zero (`bt.s`, whose delay
//! still runs) or when the freshly stored `[r4+89]` is zero; otherwise `r6 = 1`.
//! 0x8c05d274 gets `r4` unchanged, `r5 = 1`, and that r6 via the `jmp`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = mem.read_u32(0x8c155fe4).unwrap_or(89); // mov.l 0x8c155fe4,r1
    cpu.r[0] = cpu.r[1]; // mov r1,r0
    let _ = mem.write_u8(cpu.r[4].wrapping_add(cpu.r[0]), cpu.r[5] as u8); // mov.b r5,@(r0,r4)
    cpu.r[0] = mem.read_u32(0x8c155fe8).unwrap_or(88); // mov.l 0x8c155fe8,r0
    cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0) as u32; // mov.b @(r0,r4),r0
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 0); // tst r0,r0
    cpu.r[6] = 0; // bt.s delay: mov #0,r6
    if cpu.r[0] != 0 {
        cpu.r[0] = cpu.r[1]; // mov r1,r0
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0) as u32; // mov.b @(r0,r4),r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if cpu.r[0] != 0 {
            cpu.r[6] = 1; // mov #1,r6
        }
    }
    cpu.r[0] = mem.read_u32(0x8c155fec).unwrap_or(0x8c05d274); // mov.l 0x8c155fec,r0
    cpu.r[5] = 1; // jmp delay: mov #1,r5
    crate::registry::p_8c05d274::run(cpu, mem);
}
