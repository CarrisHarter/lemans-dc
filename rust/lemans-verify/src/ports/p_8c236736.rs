//! 0x8c236736 - claim the first free 52-byte record.
//!
//! Records live at 0x8c2c8740, stride 52, limit +0xd0. A zero word at +40 is
//! free. On a hit the slot is marked used and filled from the globals at
//! 0x8c2c867c, 0x8c2c86e8 and 0x8c2c86ec plus a word of the struct at
//! 0x8c23d318. Returns the slot pointer, or 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(24);
    cpu.r[3] = mem.read_u32(0x8c2367a8).unwrap_or(0);
    cpu.r[2] = cpu.r[15];
    cpu.r[7] = mem.read_u32(0x8c2367a4).unwrap_or(0);
    cpu.r[13] = 0;
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(4)).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[3]).unwrap_or(0);
    cpu.r[5] = cpu.r[7];
    cpu.r[6] = cpu.r[7];
    let _ = mem.write_u32(cpu.r[2], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[2].wrapping_add(4), cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(12)).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[3].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[2].wrapping_add(8), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[2].wrapping_add(12), cpu.r[0]);
    cpu.r[0] = mem.read_u32(cpu.r[3].wrapping_add(20)).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[3].wrapping_add(16)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[2].wrapping_add(16), cpu.r[1]);
    let _ = mem.write_u32(cpu.r[2].wrapping_add(20), cpu.r[0]);
    cpu.r[0] = mem.read_u16(0x8c2367a0).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[7]);
    cpu.set_t(cpu.r[5] >= cpu.r[0]); // cmp/hs r0,r5
    cpu.r[7] = 1; // bt.s delay
    if !cpu.t() {
        loop {
            cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(40)).unwrap_or(0);
            cpu.set_t(cpu.r[3] == 0); // tst r3,r3
            if !cpu.t() {
                cpu.r[5] = cpu.r[5].wrapping_add(52);
                cpu.set_t(cpu.r[5] >= cpu.r[0]); // cmp/hs r0,r5
                cpu.r[6] = cpu.r[6].wrapping_add(52); // bf.s delay
                if cpu.t() {
                    break;
                }
                continue;
            }
            cpu.r[13] = cpu.r[7];
            cpu.r[5] = cpu.r[6]; // bra delay
            break;
        }
    }
    cpu.set_t(cpu.r[13] == 0); // tst r13,r13
    if !cpu.t() {
        cpu.r[1] = mem.read_u32(0x8c236864).unwrap_or(0);
        cpu.r[2] = 0;
        let _ = mem.write_u32(cpu.r[5].wrapping_add(40), cpu.r[7]);
        let _ = mem.write_u32(cpu.r[5].wrapping_add(28), cpu.r[4]);
        let _ = mem.write_u32(cpu.r[5].wrapping_add(36), cpu.r[4]);
        let _ = mem.write_u32(cpu.r[5], cpu.r[2]);
        cpu.r[2] = cpu.r[15];
        cpu.r[3] = mem.read_u32(cpu.r[1]).unwrap_or(0);
        cpu.r[3] = cpu.r[3].wrapping_shl(2);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[2]);
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        cpu.r[0] = cpu.r[0].wrapping_shl(2);
        cpu.r[0] = cpu.r[0].wrapping_shl(1);
        let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[0]);
        cpu.r[0] = cpu.r[1];
        cpu.r[3] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[3]);
        cpu.r[3] = mem.read_u32(0x8c236868).unwrap_or(0);
        cpu.r[2] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[5].wrapping_add(12), cpu.r[2]);
        cpu.r[2] = mem.read_u32(0x8c23686c).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[2]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[5].wrapping_add(16), cpu.r[1]);
        cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[3]).unwrap_or(0);
        let p = (cpu.r[0] as i32 as i64) * (cpu.r[1] as i32 as i64); // mul.l r1,r0
        cpu.mach = (p >> 32) as u32;
        cpu.macl = p as u32;
        cpu.r[0] = cpu.r[5];
        cpu.r[1] = cpu.macl;
        cpu.r[1] >>= 2;
        cpu.r[1] >>= 2;
        cpu.r[1] >>= 2;
        let _ = mem.write_u32(cpu.r[5].wrapping_add(44), cpu.r[1]);
    } else {
        cpu.r[0] = 0;
    }
    cpu.r[15] = cpu.r[15].wrapping_add(24);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
