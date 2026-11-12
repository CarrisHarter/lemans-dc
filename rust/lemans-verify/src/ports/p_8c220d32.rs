//! 0x8c220d32 - -1 if any of 0x8c221da8(0..7) is nonzero.
//!
//! All eight returning zero falls through to 0x8c220d14, which is before this
//! entry: it clears a global and tail-jumps through 0x8c21a04a and 0x8c2219fc
//! into 0x8c222924. The `bra` delay slot has already popped r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[14] = 0;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[13] = 8;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[12] = mem.read_u32(0x8c220da0).unwrap_or(0x8c221da8);
    let mut early = false;
    loop {
        cpu.r[4] = cpu.r[14]; // jsr delay
        cpu.pr = 0x8c220d44;
        crate::registry::p_8c221da8::run(cpu, mem);
        cpu.set_t(cpu.r[0] == 0); // tst r0,r0
        if !cpu.t() {
            cpu.r[0] = (-1i32) as u32; // bra delay
            early = true;
            break;
        }
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.set_t((cpu.r[14] as i32) >= (cpu.r[13] as i32)); // cmp/ge r13,r14
        if cpu.t() {
            break;
        }
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // pop, also the bra delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    if early {
        return;
    }
    // 0x8c220d14
    cpu.r[3] = mem.read_u32(0x8c220d94).unwrap_or(0x8c23d47c);
    cpu.r[2] = 0;
    cpu.r[5] = cpu.r[2];
    cpu.r[1] = mem.read_u32(0x8c220d90).unwrap_or(0x8c21a04a);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    let _ = mem.write_u32(cpu.r[3], cpu.r[2]);
    cpu.r[2] = mem.read_u32(0x8c220d8c).unwrap_or(0x8c23cc68);
    cpu.r[4] = mem.read_u32(cpu.r[2]).unwrap_or(0); // jsr delay
    cpu.pr = 0x8c220d26;
    crate::registry::p_8c21a04a::run(cpu, mem);
    cpu.r[3] = mem.read_u32(0x8c220d98).unwrap_or(0x8c2219fc);
    cpu.pr = 0x8c220d2c;
    crate::registry::p_8c2219fc::run(cpu, mem);
    cpu.r[1] = mem.read_u32(0x8c220d9c).unwrap_or(0x8c222924);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // jmp delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    crate::registry::p_8c222924::run(cpu, mem);
}
