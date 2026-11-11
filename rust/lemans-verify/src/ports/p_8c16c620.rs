//! 0x8c16c620 — zero a 96-byte region at mem[0x8c16c660] (0x8c2b29c8).
//!
//! Saves r14/r13/r12, then loops 8 times (while 8 > r13 after increment):
//! per iteration stores r7 (0) as a word at base+12k and base+4+12k, and as
//! bytes at base+8+12k, base+9+12k and base+10+12k (the last is the bt.s
//! delay slot, which also runs on the final non-taken pass). Restores
//! r12/r13/r14, returns r0 = 1. Final T = 0 (last cmp/gt 8 > 8 is false).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // prologue: mov.l r14,r13,r12,@-r15 ; mov #0,r13
    for n in [14usize, 13, 12] {
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let v = cpu.r[n];
        let _ = mem.write_u32(cpu.r[15], v);
    }
    let base = mem.read_u32(0x8c16c660).unwrap_or(0); // 0x8c2b29c8
    let mut r13: u32 = 0; // mov #0,r13 ; mov r13,r12
    let mut r12: u32 = 0;
    let r7: u32 = 0; // mov r13,r7
    let r1: u32 = 8; // mov #8,r1
    loop {
        let r6 = base.wrapping_add(4);
        let r5 = base.wrapping_add(8);
        let r4 = base.wrapping_add(9);
        let r3 = base.wrapping_add(10);
        let _ = mem.write_u32(base.wrapping_add(r12), r7); // mov.l r7,@(r0,r12), r0 = r14
        r13 = r13.wrapping_add(1);
        let r0 = r12; // mov r12,r0
        let _ = mem.write_u32(r0.wrapping_add(r6), r7); // mov.l r7,@(r0,r6)
        let t = r1 > r13; // cmp/gt r13,r1 -> T = (r1 > r13)
        let _ = mem.write_u8(r0.wrapping_add(r5), r7 as u8); // mov.b r7,@(r0,r5)
        r12 = r12.wrapping_add(12);
        let _ = mem.write_u8(r0.wrapping_add(r4), r7 as u8); // mov.b r7,@(r0,r4)
        let _ = mem.write_u8(r0.wrapping_add(r3), r7 as u8); // bt.s delay: mov.b r7,@(r0,r3)
        cpu.set_t(t);
        if !t {
            break;
        }
    }
    // epilogue: pop r12 ; mov #1,r0 ; pop r13 ; rts ; delay pop r14
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = v;
    cpu.r[0] = 1;
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = v;
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = v;
    // leaked registers
    cpu.r[1] = r1;
    cpu.r[3] = base.wrapping_add(10);
    cpu.r[4] = base.wrapping_add(9);
    cpu.r[5] = base.wrapping_add(8);
    cpu.r[6] = base.wrapping_add(4);
    cpu.r[7] = r7;
}
