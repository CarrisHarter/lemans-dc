//! 0x8c0ba554 — zero four 4-byte slots and four 24-byte structs.
//!
//! ```text
//! push r14/r13/r12 (stack; restored at the end)
//! r14 = 0 ; r13 = r14 ; r12 = r14
//! r7 = [0x8c27a028] ; r6 = r14 ; r5 = [0x8c27a098] ; r1 = 4 ; r4 = [0x8c27a038]
//! loop 0x8c0ba56a:
//!   r0 = r7 ; mov.l r6,@(r0,r13)      ; G1[i*4] = 0
//!   r3 = r12 ; r0 = r5 ; mov.l r6,@(r0,r13) ; G2[i*4] = 0
//!   add #1,r14 ; r0 = r4 ; mov.l r6,@(r0,r12) ; r3 += r4
//!   mov.l r6,@(4,r3) ; cmp/gt r14,r1 ; mov.l r6,@(8,r3)
//!   add #4,r13 ; mov.l r6,@(12,r3) ; add #24,r12
//!   bt.s 0x8c0ba56a ; mov.l r6,@(16,r3)   ; delay slot: last store of the block
//! pop r12/r13 ; rts ; delay: pop r14
//! ```
//! Hidden side effects: r0/r1/r3/r12/r13/r14 all end with loop residue
//! (r12/r13/r14 are then restored from the stack), r6 = 0, and T ends 0
//! (final `cmp/gt r14,r1` is `4 > 4` = false). The delay-slot store
//! `@(16,r3)` runs on every iteration including the last.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let (s12, s13, s14) = (cpu.r[12], cpu.r[13], cpu.r[14]);
    cpu.r[14] = 0;
    cpu.r[13] = cpu.r[14];
    cpu.r[12] = cpu.r[14];
    cpu.r[7] = mem.read_u32(0x8C0BA594).unwrap();
    cpu.r[6] = cpu.r[14];
    cpu.r[5] = mem.read_u32(0x8C0BA598).unwrap();
    cpu.r[1] = 4;
    cpu.r[4] = mem.read_u32(0x8C0BA59C).unwrap();
    loop {
        cpu.r[0] = cpu.r[7];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[6]);
        cpu.r[3] = cpu.r[12];
        cpu.r[0] = cpu.r[5];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[6]);
        cpu.r[14] = cpu.r[14].wrapping_add(1);
        cpu.r[0] = cpu.r[4];
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[12]), cpu.r[6]);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[6]);
        let t = (cpu.r[1] as i32) > (cpu.r[14] as i32); // cmp/gt r14,r1
        let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[6]);
        cpu.r[13] = cpu.r[13].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[3].wrapping_add(12), cpu.r[6]);
        cpu.r[12] = cpu.r[12].wrapping_add(24);
        let _ = mem.write_u32(cpu.r[3].wrapping_add(16), cpu.r[6]); // bt.s delay slot
        cpu.set_t(t);
        if !t {
            break;
        }
    }
    cpu.r[12] = s12; // pop r12
    cpu.r[13] = s13; // pop r13
    cpu.r[14] = s14; // rts delay: pop r14
}
