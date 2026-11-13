//! 0x8c237440 — sum three lists and keep the larger of two running maxima.
//!
//! The anchor is the word at `0x8c23d798`. `[anchor+44]` and `[anchor+60]` are
//! lists of nodes whose value is at `+16` and whose link is at `+8`; each list
//! contributes its sum and its max. `[anchor+68]` is a third list, linked at
//! `+4`, whose leading halfword selects a weight and a slot count:
//!
//! | tag | weight | slots |
//! | --- | --- | --- |
//! | `0x1000` | `0x90` | 16 |
//! | `0x2000` | `0xa0` | 16 |
//! | `0x3000` | `0xc0` | 16 |
//! | `0x4000` | `0xe0` | 16 |
//! | `0x5000` | `0x200` | 8 |
//! | `0x6000` | `0x360` | 4 |
//! | `0x7000` | `0x800` | 2 |
//! | `0x8000` | `0xd60` | 1 |
//!
//! A null word in those slots adds the weight. `[r4]` receives the three sums.
//! `[r5]` receives whichever list-max is larger (`r8` over `r9` on a tie).
//! `r8`–`r14` are restored and `r0` is cleared.

use sh4_oracle::{Cpu, Memory};

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let v = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    v
}

fn sx16(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    for reg in [14, 13, 12, 11, 10, 9, 8] {
        push(cpu, mem, cpu.r[reg]);
    }
    // 0x8c23744e: add #-12,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    // 0x8c237450: mov.l 0x8c2374f0,r7
    cpu.r[7] = mem.read_u32(0x8c2374f0).unwrap_or(0);
    // 0x8c237452: mov #0,r13 and zero the three frame slots
    cpu.r[13] = 0;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), 0);
    cpu.r[9] = 0;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(8), 0);
    let _ = mem.write_u32(cpu.r[15], 0);
    // List A at [anchor+44]. Sum at frame+8, max in r8.
    cpu.r[6] = mem.read_u32(cpu.r[7].wrapping_add(44)).unwrap_or(0);
    cpu.set_t(cpu.r[6] == 0);
    cpu.r[8] = 0;
    if cpu.r[6] != 0 {
        loop {
            cpu.r[2] = mem.read_u32(cpu.r[6].wrapping_add(16)).unwrap_or(0);
            cpu.set_t(cpu.r[2] > cpu.r[8]);
            if cpu.t() {
                cpu.r[8] = mem.read_u32(cpu.r[6].wrapping_add(16)).unwrap_or(0);
            }
            cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
            cpu.r[3] = mem.read_u32(cpu.r[6].wrapping_add(16)).unwrap_or(0);
            cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
            let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[2]);
            cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[6] == 0);
            if cpu.r[6] == 0 {
                break;
            }
        }
    }
    // List B at [anchor+60]. Sum at frame+4, max in r9.
    cpu.r[6] = mem.read_u32(cpu.r[7].wrapping_add(60)).unwrap_or(0);
    cpu.set_t(cpu.r[6] == 0);
    if cpu.r[6] != 0 {
        loop {
            cpu.r[3] = mem.read_u32(cpu.r[6].wrapping_add(16)).unwrap_or(0);
            cpu.set_t(cpu.r[3] > cpu.r[9]);
            // bf.s delay reloads r3 whether or not the max updates.
            cpu.r[3] = mem.read_u32(cpu.r[6].wrapping_add(16)).unwrap_or(0);
            if cpu.t() {
                cpu.r[9] = mem.read_u32(cpu.r[6].wrapping_add(16)).unwrap_or(0);
            }
            cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
            cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
            let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[2]);
            cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[6] == 0);
            if cpu.r[6] == 0 {
                break;
            }
        }
    }
    // List C at [anchor+68].
    cpu.r[0] = 68;
    cpu.r[12] = mem.read_u32(cpu.r[7].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.set_t(cpu.r[12] == 0);
    if cpu.r[12] != 0 {
        cpu.r[14] = 16;
        loop {
            cpu.r[0] = mem.read_u16(cpu.r[12]).unwrap_or(0) as i16 as i32 as u32;
            cpu.r[3] = mem.read_u32(0x8c2374f4).unwrap_or(0);
            cpu.r[0] &= 0xffff;
            cpu.r[0] &= cpu.r[3];
            let tag = cpu.r[0];
            let c = |addr| sx16(mem, addr);
            if tag == c(0x8c2374e0) {
                cpu.r[1] = c(0x8c2374e0);
                cpu.r[7] = c(0x8c237540);
                cpu.r[6] = cpu.r[14];
            } else if tag == c(0x8c2374e2) {
                cpu.r[1] = c(0x8c2374e2);
                cpu.r[7] = c(0x8c237542);
                cpu.r[6] = cpu.r[14];
            } else if tag == c(0x8c2374e4) {
                cpu.r[1] = c(0x8c2374e4);
                cpu.r[7] = c(0x8c237544);
                cpu.r[6] = cpu.r[14];
            } else if tag == c(0x8c2374e6) {
                cpu.r[1] = c(0x8c2374e6);
                cpu.r[7] = c(0x8c237546);
                cpu.r[6] = cpu.r[14];
            } else if tag == c(0x8c2374e8) {
                cpu.r[1] = c(0x8c2374e8);
                cpu.r[7] = c(0x8c237548);
                cpu.r[6] = 8;
            } else if tag == c(0x8c2374ea) {
                cpu.r[1] = c(0x8c2374ea);
                cpu.r[7] = c(0x8c23754a);
                cpu.r[6] = 4;
            } else if tag == c(0x8c2374ec) {
                cpu.r[1] = c(0x8c2374ec);
                cpu.r[7] = c(0x8c2375e0);
                cpu.r[6] = 2;
            } else if tag == mem.read_u32(0x8c2374f8).unwrap_or(0) {
                cpu.r[1] = mem.read_u32(0x8c2374f8).unwrap_or(0);
                cpu.r[7] = c(0x8c2375e2);
                cpu.r[6] = 1;
            } else {
                cpu.r[1] = mem.read_u32(0x8c2374f8).unwrap_or(0);
                cpu.r[6] = 0;
                cpu.r[7] = 0;
            }
            // 0x8c237570: walk `r6` slots at r12+12. A null slot adds r7.
            cpu.r[3] = 0;
            cpu.r[10] = cpu.r[13];
            cpu.set_t(cpu.r[3] >= cpu.r[6]);
            cpu.r[11] = cpu.r[12].wrapping_add(12);
            if !cpu.t() {
                loop {
                    cpu.r[1] = mem.read_u32(cpu.r[11]).unwrap_or(0);
                    cpu.set_t(cpu.r[1] == 0);
                    cpu.r[10] = cpu.r[10].wrapping_add(1);
                    if cpu.r[1] == 0 {
                        cpu.r[2] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[7]);
                        let _ = mem.write_u32(cpu.r[15], cpu.r[2]);
                    }
                    let cont = cpu.r[10] < cpu.r[6];
                    cpu.set_t(!cont);
                    cpu.r[11] = cpu.r[11].wrapping_add(4);
                    if !cont {
                        break;
                    }
                }
            }
            cpu.r[12] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
            cpu.set_t(cpu.r[12] == 0);
            if cpu.r[12] == 0 {
                break;
            }
        }
    }
    // 0x8c237596: total into [r4], larger max into [r5].
    cpu.r[2] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
    cpu.set_t(cpu.r[8] >= cpu.r[9]);
    cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[3]);
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[1]);
    let _ = mem.write_u32(cpu.r[4], cpu.r[2]);
    if cpu.t() {
        cpu.r[3] = cpu.r[8];
    } else {
        cpu.r[3] = cpu.r[9];
    }
    cpu.r[0] = 0;
    let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    for reg in [8, 9, 10, 11, 12, 13, 14] {
        cpu.r[reg] = pop(cpu, mem);
    }
}
