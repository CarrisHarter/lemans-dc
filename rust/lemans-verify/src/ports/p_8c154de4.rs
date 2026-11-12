//! 0x8c154de4 - clear a global, drain the sub-array, then free when positive.
//!
//! Twin of 0x8c150878 with the counter at 0x8c2abd28 and the drain at 0x8c154e94.
//! The `bt.s` fragment is dead (r14 != 0) but its delay slot `[r1] = 0` always
//! runs. 0x8c154e94 keeps r4, then a positive exts.w(r13) frees r14.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[14] = cpu.r[4]; // mov r4,r14
    cpu.r[13] = cpu.r[5]; // mov r5,r13
    cpu.set_t(cpu.r[14] == 0); // tst r14,r14
    if cpu.r[14] != 0 {
        cpu.r[1] = mem.read_u32(0x8c154e18).unwrap_or(0x8c2abd28); // mov.l 0x8c154e18,r1
        cpu.r[0] = 0; // mov #0,r0
        cpu.set_t(cpu.r[14] == 0); // tst r14,r14 (dead)
        let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // bt.s delay: mov.l r0,@r1
        cpu.r[0] = mem.read_u32(0x8c154e1c).unwrap_or(0x8c154e94); // mov.l 0x8c154e1c,r0
        cpu.pr = 0x8c154e02;
        crate::registry::p_8c154e94::run(cpu, mem);
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32; // exts.w r13,r0
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl r0
        if cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c154e20).unwrap_or(0x8c01032c); // mov.l 0x8c154e20,r0
            cpu.r[4] = cpu.r[14]; // jsr delay: mov r14,r4
            cpu.pr = 0x8c154e0e;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14]; // mov r14,r0
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
