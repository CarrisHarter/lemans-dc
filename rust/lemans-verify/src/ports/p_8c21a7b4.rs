//! 0x8c21a7b4 — pack bit 3 of each source byte into a destination image.
//!
//! `r6` selects the tail:
//! 1 walks backward (`0x8c21a612`), 2 walks forward from `dest+0xba`
//! (`0x8c21a5e0`), 3 pre-decrements from `dest+0xc0` (`0x8c21a63e`), and any
//! other value writes 192 forward bytes (`0x8c21a66a`). Each output byte is
//! eight source bytes, bit 3 tested, gathered from `0x80` downward or from
//! `1` upward. The outer counters are 32 rows of 6 bytes, or 192 bytes for
//! the default. `r8`–`r15` are not used.

use sh4_oracle::{Cpu, Memory};

fn hw(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

fn ld_byte(cpu: &mut Cpu, mem: &Memory) {
    let b = mem.read_u8(cpu.r[5]).unwrap_or(0) as i8 as i32 as u32;
    cpu.r[5] = cpu.r[5].wrapping_add(1);
    cpu.r[0] = b;
    cpu.set_t(cpu.r[0] & 8 == 0); // tst #8, r0
    if !cpu.t() {
        cpu.r[6] |= cpu.r[1];
    }
}

fn shll(cpu: &mut Cpu) {
    let out = cpu.r[1] >> 31 != 0;
    cpu.r[1] <<= 1;
    cpu.set_t(out);
}

fn shlr(cpu: &mut Cpu) {
    let out = cpu.r[1] & 1 != 0;
    cpu.r[1] >>= 1;
    cpu.set_t(out);
}

/// Returns true when the decremented register is zero.
fn dt(cpu: &mut Cpu, reg: usize) -> bool {
    cpu.r[reg] = cpu.r[reg].wrapping_sub(1);
    let zero = cpu.r[reg] == 0;
    cpu.set_t(zero);
    zero
}

fn store(cpu: &mut Cpu, mem: &mut Memory) {
    let _ = mem.write_u8(cpu.r[4], cpu.r[6] as u8);
}

/// 0x8c21a5e0 — mode 2. Forward bytes from `dest+0xba`, then step back 12.
fn mode2(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = hw(mem, 0x8c21a60e); // 0xba
    cpu.r[2] = 32;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
    loop {
        cpu.r[3] = 6;
        loop {
            cpu.r[6] = 0;
            cpu.r[1] = hw(mem, 0x8c21a610); // 0x80
            cpu.r[7] = 8;
            loop {
                ld_byte(cpu, mem);
                shlr(cpu);
                if dt(cpu, 7) {
                    break;
                }
            }
            store(cpu, mem);
            cpu.r[4] = cpu.r[4].wrapping_add(1);
            if dt(cpu, 3) {
                break;
            }
        }
        let done = dt(cpu, 2);
        cpu.r[4] = cpu.r[4].wrapping_add((-12i32) as u32); // bf.s delay
        if done {
            break;
        }
    }
}

/// 0x8c21a612 — mode 1. Backward bytes from `dest+5`, then step forward 12.
fn mode1(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = 32;
    cpu.r[4] = cpu.r[4].wrapping_add(5);
    loop {
        cpu.r[3] = 6;
        loop {
            cpu.r[6] = 0;
            cpu.r[1] = 1;
            cpu.r[7] = 8;
            loop {
                ld_byte(cpu, mem);
                shll(cpu);
                if dt(cpu, 7) {
                    break;
                }
            }
            store(cpu, mem);
            cpu.r[4] = cpu.r[4].wrapping_sub(1);
            if dt(cpu, 3) {
                break;
            }
        }
        let done = dt(cpu, 2);
        cpu.r[4] = cpu.r[4].wrapping_add(12); // bf.s delay
        if done {
            break;
        }
    }
}

/// 0x8c21a63e — mode 3. Pre-decrement stores from `dest+0xc0`.
fn mode3(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = hw(mem, 0x8c21a668); // 0xc0
    cpu.r[2] = 32;
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
    loop {
        cpu.r[3] = 6;
        loop {
            cpu.r[6] = 0;
            cpu.r[1] = 1;
            cpu.r[7] = 8;
            loop {
                ld_byte(cpu, mem);
                let done = dt(cpu, 7);
                shll(cpu); // bf.s delay
                if done {
                    break;
                }
            }
            cpu.r[4] = cpu.r[4].wrapping_sub(1);
            store(cpu, mem);
            if dt(cpu, 3) {
                break;
            }
        }
        if dt(cpu, 2) {
            break;
        }
    }
}

/// 0x8c21a66a — any other mode. 192 forward bytes.
fn mode_default(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[3] = hw(mem, 0x8c21a68c); // 0xc0
    loop {
        cpu.r[6] = 0;
        cpu.r[1] = hw(mem, 0x8c21a68e); // 0x80
        cpu.r[7] = 8;
        loop {
            ld_byte(cpu, mem);
            shlr(cpu);
            if dt(cpu, 7) {
                break;
            }
        }
        store(cpu, mem);
        let done = dt(cpu, 3);
        cpu.r[4] = cpu.r[4].wrapping_add(1); // bf.s delay
        if done {
            break;
        }
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[6];
    cpu.set_t(cpu.r[0] == 1);
    if cpu.t() {
        mode1(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 2);
    if cpu.t() {
        mode2(cpu, mem);
        return;
    }
    cpu.set_t(cpu.r[0] == 3);
    if cpu.t() {
        mode3(cpu, mem);
        return;
    }
    mode_default(cpu, mem);
}
