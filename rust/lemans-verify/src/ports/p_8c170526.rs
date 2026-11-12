//! 0x8c170526 - set bit `r4` in one of the four bitmaps hanging off 0x8c2b5260.
//!
//! When `[0x8c2b52dc] == 2` only two words are live: `+0x90` holds bits 0..15 and
//! `+0x92` holds bits 16..30 (bit 31 and above are ignored). Otherwise four words
//! are live: `+0x90` bits 0..15, `+0x92` bits 16..31, `+0x94` bits 32..47 and
//! `+0x96` bits 48..62. Each branch ORs `1 << (r4 - base)` into the word and
//! returns; `r0` is left holding whatever the branch last loaded.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c170578).unwrap_or(0x8c2b5260); // mov.l 0x8c170578,r5
    cpu.r[0] = 124; // mov #124,r0
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[3] = cpu.r[4]; // mov r4,r3
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[6] = 1; // mov #1,r6
    cpu.r[13] = mem.read_u16(0x8c170564).unwrap_or(0x90) as i16 as i32 as u32; // mov.w
    cpu.r[3] = cpu.r[3].wrapping_add(0xffff_fff0); // add #-16,r3
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[14] = cpu.r[6]; // mov r6,r14
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0); // mov.l @(r0,r5),r0
    cpu.r[12] = cpu.r[6]; // mov r6,r12
    cpu.r[7] = mem.read_u16(0x8c170564).unwrap_or(0x90) as i16 as i32 as u32; // mov.w
    cpu.r[14] = shad(cpu.r[14], cpu.r[3]); // shad r3,r14
    cpu.set_t(cpu.r[0] == 2); // cmp/eq #2,r0
    cpu.r[12] = shad(cpu.r[12], cpu.r[4]); // shad r4,r12
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[5]); // add r5,r7
    cpu.r[13] = cpu.r[13].wrapping_add(cpu.r[5]); // add r5,r13
    cpu.r[13] = cpu.r[13].wrapping_add(2); // bf.s delay slot runs either way: add #2,r13
    if !cpu.t() {
        cpu.r[2] = 16; // mov #16,r2
        cpu.set_t(cpu.r[4] >= cpu.r[2]); // cmp/hs r2,r4
        if cpu.t() {
            cpu.r[0] = 32; // mov #32,r0
            cpu.set_t(cpu.r[4] >= cpu.r[0]); // cmp/hs r0,r4
            if cpu.t() {
                cpu.r[3] = 48; // mov #48,r3
                cpu.set_t(cpu.r[4] >= cpu.r[3]); // cmp/hs r3,r4
                if cpu.t() {
                    cpu.r[2] = 63; // mov #63,r2
                    cpu.set_t(cpu.r[4] >= cpu.r[2]); // cmp/hs r2,r4
                    if !cpu.t() {
                        cpu.r[0] = mem.read_u16(0x8c17065e).unwrap_or(0x96) as i16 as i32 as u32; // mov.w
                        cpu.r[4] = cpu.r[4].wrapping_add(0xffff_ffd0); // add #-48,r4
                        cpu.r[1] = mem.read_u16(0x8c17065e).unwrap_or(0x96) as i16 as i32 as u32; // mov.w
                        cpu.r[6] = shad(cpu.r[6], cpu.r[4]); // shad r4,r6
                        cpu.r[0] = mem.read_u16(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0) as i16 as i32 as u32; // mov.w @(r0,r5),r0
                        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[5]); // add r5,r1
                        cpu.r[0] |= cpu.r[6]; // or r6,r0
                        let _ = mem.write_u16(cpu.r[1], cpu.r[0] as u16); // mov.w r0,@r1
                    }
                } else {
                    cpu.r[0] = mem.read_u16(0x8c17065c).unwrap_or(0x94) as i16 as i32 as u32; // mov.w
                    cpu.r[4] = cpu.r[4].wrapping_add(0xffff_ffe0); // add #-32,r4
                    cpu.r[6] = shad(cpu.r[6], cpu.r[4]); // shad r4,r6
                    cpu.r[1] = mem.read_u16(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0) as i16 as i32 as u32; // mov.w @(r0,r5),r1
                    cpu.r[1] |= cpu.r[6]; // or r6,r1
                    let _ = mem.write_u16(cpu.r[0].wrapping_add(cpu.r[5]), cpu.r[1] as u16); // mov.w r1,@(r0,r5)
                }
            } else {
                cpu.r[1] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r13,r1
                cpu.r[1] |= cpu.r[14]; // or r14,r1
                let _ = mem.write_u16(cpu.r[13], cpu.r[1] as u16); // bra delay: mov.w r1,@r13
            }
        } else {
            cpu.r[1] = mem.read_u16(cpu.r[7]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r7,r1
            cpu.r[1] |= cpu.r[12]; // or r12,r1
            let _ = mem.write_u16(cpu.r[7], cpu.r[1] as u16); // bra delay: mov.w r1,@r7
        }
    } else {
        cpu.r[2] = 16; // mov #16,r2
        cpu.set_t(cpu.r[4] >= cpu.r[2]); // cmp/hs r2,r4
        if !cpu.t() {
            cpu.r[1] = mem.read_u16(cpu.r[7]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r7,r1
            cpu.r[1] |= cpu.r[12]; // or r12,r1
            let _ = mem.write_u16(cpu.r[7], cpu.r[1] as u16); // bra delay: mov.w r1,@r7
        } else {
            cpu.r[0] = 31; // mov #31,r0
            cpu.set_t(cpu.r[4] >= cpu.r[0]); // cmp/hs r0,r4
            if !cpu.t() {
                cpu.r[1] = mem.read_u16(cpu.r[13]).unwrap_or(0) as i16 as i32 as u32; // mov.w @r13,r1
                cpu.r[1] |= cpu.r[14]; // or r14,r1
                let _ = mem.write_u16(cpu.r[13], cpu.r[1] as u16); // bra delay: mov.w r1,@r13
            }
        }
    }
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

fn shad(value: u32, amount: u32) -> u32 {
    let amount = amount as i32;
    if amount >= 0 {
        value.wrapping_shl((amount as u32) & 31)
    } else {
        ((value as i32) >> ((-amount) as u32 & 31)) as u32
    }
}
