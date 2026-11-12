//! 0x8c21fb1a - publish each byte of the string at `r4` through 0x8c21f9d8.
//!
//! Stops on a zero byte. The call's delay slot is `mov.b @r14+,r4`, so the
//! callee sees the byte and the cursor has already advanced. r14 is restored
//! by the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4]; // bra delay: mov r4,r14
    loop {
        cpu.r[2] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r14,r2
        cpu.set_t((cpu.r[2] & cpu.r[2]) == 0); // tst r2,r2
        if cpu.t() {
            break;
        }
        cpu.r[4] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // bsr delay: mov.b @r14+,r4
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.pr = 0x8c21fb44;
        crate::registry::p_8c21f9d8::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
