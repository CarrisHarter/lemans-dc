//! 0x8c21bb5e - write 0 or 1 at `r4` from the bytes at +7 and +8.
//!
//! Byte +7 equal to 1 stores 1 when byte +8 is 5, else 0. Otherwise it stores
//! 0 when byte +8 is 0 or 0xff, and 1 for any other value. Returns 0. The
//! `bf.s` delay slot clears r5 on every entry.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(7)).unwrap_or(0) as i8 as i32 as u32; // mov.b
    cpu.r[6] = 1; // mov #1,r6
    cpu.r[0] &= 0xff; // extu.b r0,r0
    cpu.set_t(cpu.r[0] == 1); // cmp/eq #1,r0
    cpu.r[5] = 0; // bf.s delay: mov #0,r5
    if cpu.t() {
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32; // mov.b
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == 5); // cmp/eq #5,r0
        let store = if cpu.t() { cpu.r[6] } else { cpu.r[5] };
        let _ = mem.write_u8(cpu.r[4], store as u8); // bra delay: mov.b r6/@r4 or r5
    } else {
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(8)).unwrap_or(0) as i8 as i32 as u32; // mov.b
        cpu.r[1] = mem.read_u16(0x8c21bb96).unwrap_or(0x00ff) as i16 as i32 as u32; // mov.w
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.set_t(cpu.r[0] == cpu.r[1]); // cmp/eq r1,r0
        if !cpu.t() {
            cpu.set_t(cpu.r[0] == 0); // cmp/eq #0,r0
        }
        let store = if cpu.t() { cpu.r[5] } else { cpu.r[6] };
        let _ = mem.write_u8(cpu.r[4], store as u8);
    }
    cpu.r[0] = 0; // rts delay: mov #0,r0
}
