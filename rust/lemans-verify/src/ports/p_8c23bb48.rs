//! 0x8c23bb48 — clear bit 0 of the flags word for the record matching r4.
//!
//! `base = [0x8c23bbb0]`, `count = [base+12]`, stride 76, records hang off
//! `[base+72] + i*76` (computed twice with `mul.l`, so macl is rewritten).
//! A match clears bit 0 of the 16-bit word at the record (`mov.w` sign-extends,
//! `and 0xfffe`, `mov.w` stores the low half). r14 is restored on exit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c23bbb0).unwrap_or(0x8c23d798); // mov.l 0x8c23bbb0,r5
    cpu.r[6] = 0; // mov #0,r6
    let saved14 = cpu.r[14]; // mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    cpu.r[2] = cpu.r[6]; // mov r6,r2
    cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r5),r3
    cpu.r[14] = mem.read_u32(0x8c23bbac).unwrap_or(0x0000fffe); // mov.l 0x8c23bbac,r14
    cpu.set_t(cpu.r[2] >= cpu.r[3]); // cmp/hs r3,r2
    cpu.r[7] = 76; // bt.s delay slot: mov #76,r7
    if !cpu.t() {
        loop {
            // 0x8c23bb5a
            let product = cpu.r[6].wrapping_mul(cpu.r[7]); // mul.l r7,r6
            cpu.macl = product;
            cpu.r[3] = 72; // mov #72,r3
            cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[5]); // add r5,r3
            cpu.r[3] = mem.read_u32(cpu.r[3]).unwrap_or(0); // mov.l @r3,r3
            cpu.r[0] = cpu.macl; // sts macl,r0
            cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[3]); // add r3,r0
            cpu.set_t(cpu.r[0] == cpu.r[4]); // cmp/eq r4,r0
            if cpu.r[0] == cpu.r[4] {
                // 0x8c23bb6a
                cpu.macl = cpu.r[6].wrapping_mul(cpu.r[7]); // mul.l r7,r6 (again)
                cpu.r[0] = 72; // mov #72,r0
                cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(72)).unwrap_or(0); // mov.l @(r0,r5),r2
                cpu.r[3] = cpu.macl; // sts macl,r3
                cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]); // add r2,r3
                cpu.r[1] = mem.read_u16(cpu.r[3]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r3,r1
                cpu.r[1] &= cpu.r[14]; // and r14,r1
                let _ = mem.write_u16(cpu.r[3], cpu.r[1] as u16); // mov.w r1,@r3
            }
            // 0x8c23bb7a
            cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0); // mov.l @(12,r5),r3
            cpu.r[6] = cpu.r[6].wrapping_add(1); // add #1,r6
            cpu.set_t(cpu.r[6] >= cpu.r[3]); // cmp/hs r3,r6
            if cpu.t() {
                break; // bf 0x8c23bb5a
            }
        }
    }
    cpu.r[15] = cpu.r[15].wrapping_add(4); // rts delay: mov.l @r15+,r14
    cpu.r[14] = saved14;
}
