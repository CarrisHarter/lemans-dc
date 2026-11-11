//! 0x8c0c9fe8 — zero 40 bytes at r4 (5 iterations of two dwords), then store
//! r4 to the global at *0x8c0ca008 (0x8c27a6e0) and return r4.
//!
//! Residue the C does not show: r0 ends at 40 after the loop (the `bt.s`
//! delay slot `add #8,r0` runs on the final iteration too) but is then
//! overwritten — first by the global pointer, then by the rts delay slot
//! `mov r4,r0` — so r0 = r4. r1 = 10, r3 = r4 + 32, r5 = 0, r6 = 10,
//! T = 0 (final cmp/gt 10 > 10).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let mut r0: u32 = 0; // mov #0,r6 ; mov r6,r0
    let mut r6: u32 = 0;
    let r1: u32 = 10; // mov #10,r1
    let r5: u32 = 0; // mov r6,r5
    let mut r3 = r4;
    loop {
        r3 = r4; // mov r4,r3
        let _ = mem.write_u32(r0.wrapping_add(r4), r5); // mov.l r5,@(r0,r4)
        r3 = r3.wrapping_add(r0); // add r0,r3
        r6 = r6.wrapping_add(2); // add #2,r6
        let _ = mem.write_u32(r3.wrapping_add(4), r5); // mov.l r5,@(4,r3)
        let t = (r1 as i32) > (r6 as i32); // cmp/gt r6,r1
        r0 = r0.wrapping_add(8); // bt.s delay slot: add #8,r0 (runs every iteration)
        cpu.set_t(t);
        if !t {
            break;
        }
    }
    let global = mem.read_u32(0x8c0ca008).unwrap_or(0); // literal -> 0x8c27a6e0
    let _ = mem.write_u32(global, r4); // mov.l r4,@r0
    cpu.r[0] = r4; // rts delay slot: mov r4,r0
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
    cpu.set_t(false);
}
