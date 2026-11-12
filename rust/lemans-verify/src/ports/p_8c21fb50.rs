//! 0x8c21fb50 - publish eight hex digits of the pointer stored at the frame.
//!
//! The value shifted is `r4` itself (reloaded from `sp+0`), not `*r4`. Nibble
//! 7 down to nibble 0: `>= 10` becomes `'A'+n-10` (add 55), else `'0'+n`.
//! Each digit is published through 0x8c21f9d8. `shld` is logical.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[13] = 28; // mov #28,r13
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]); // mov.l r4,@r15
    cpu.r[12] = 10; // mov #10,r12
    loop {
        cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0); // mov.l @r15,r0
        cpu.r[3] = cpu.r[13].wrapping_neg(); // neg r13,r3
        cpu.r[14] = 15; // mov #15,r14
        let amount = cpu.r[3] as i32;
        if amount >= 0 {
            cpu.r[0] <<= (amount as u32) & 31;
        } else {
            cpu.r[0] >>= ((-amount) as u32) & 31;
        }
        cpu.r[14] &= cpu.r[0]; // and r0,r14
        cpu.r[3] = cpu.r[14] & 0xff; // extu.b r14,r3
        cpu.set_t((cpu.r[3] as i32) >= (cpu.r[12] as i32)); // cmp/ge r12,r3
        if cpu.t() {
            cpu.r[14] = cpu.r[14].wrapping_add(55); // bra delay: add #55,r14
        } else {
            cpu.r[14] = cpu.r[14].wrapping_add(48); // add #48,r14
        }
        cpu.r[4] = cpu.r[14]; // bsr delay: mov r14,r4
        cpu.pr = 0x8c21fb7a;
        crate::registry::p_8c21f9d8::run(cpu, mem);
        cpu.r[13] = cpu.r[13].wrapping_add((-4i32) as u32); // add #-4,r13
        cpu.set_t((cpu.r[13] as i32) >= 0); // cmp/pz r13
        if !cpu.t() {
            break;
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
