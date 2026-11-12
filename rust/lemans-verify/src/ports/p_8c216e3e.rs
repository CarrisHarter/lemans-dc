//! 0x8c216e3e - folded-byte checksum of a NUL-terminated string.
//!
//! Starts from 0xda, adds each byte after folding it through 0x8c216d44
//! (`extu.b` on the result), then returns the low byte of `sum >> 2`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = mem.read_u16(0x8c216f4a).unwrap_or(0xda) as u32; // mov.w 0x8c216f4a,r13
    cpu.r[14] = cpu.r[4]; // bra delay: mov r4,r14
    loop {
        cpu.r[2] = mem.read_u8(cpu.r[14]).unwrap_or(0) as u32; // mov.b @r14,r2
        cpu.set_t(cpu.r[2] == 0); // tst r2,r2
        if cpu.t() {
            break;
        }
        cpu.r[4] = mem.read_u8(cpu.r[14]).unwrap_or(0) as u32; // bsr delay: mov.b @r14+,r4
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.pr = 0x8c216e4e;
        crate::registry::p_8c216d44::run(cpu, mem);
        cpu.r[0] &= 0xff; // extu.b r0,r0
        cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[0]); // add r0,r13
    }
    cpu.r[13] = cpu.r[13].wrapping_shr(2); // shlr2 r13
    cpu.r[0] = cpu.r[13] & 0xff; // extu.b r13,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
