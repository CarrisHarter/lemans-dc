//! 0x8c22f514 - zero `r5` bytes at `r4`, picking the widest store the alignment allows.
//!
//! 8-aligned uses paired `fmov` after `fschg` (SZ=1). 4-aligned stores words,
//! 2-aligned stores halfwords, otherwise bytes. Each loop is `dt` / store /
//! `bf.s` with the pointer add in the delay slot, so a count of 0 wraps.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = cpu.r[4] | cpu.r[5];
    cpu.set_t((cpu.r[0] & 7) == 0); // tst #7,r0
    if cpu.t() {
        cpu.fr[0] = 0.0; // fldi0 fr0
        cpu.fr[1] = 0.0; // fldi0 fr1
        cpu.fpscr ^= 1 << 20; // fschg
        cpu.r[5] >>= 2; // shlr2, T unchanged
        cpu.set_t(cpu.r[5] & 1 != 0); // shlr r5
        cpu.r[5] >>= 1;
        loop {
            cpu.r[5] = cpu.r[5].wrapping_sub(1); // dt r5
            cpu.set_t(cpu.r[5] == 0);
            let _ = mem.write_u32(cpu.r[4], cpu.fr[0].to_bits());
            let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.fr[1].to_bits());
            cpu.r[4] = cpu.r[4].wrapping_add(8); // bf.s delay
            if cpu.t() {
                break;
            }
        }
        cpu.fpscr ^= 1 << 20; // rts delay: fschg
        return;
    }
    cpu.set_t((cpu.r[0] & 3) == 0); // tst #3,r0
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c22f548).unwrap_or(0);
        cpu.r[5] >>= 2; // shlr2
        loop {
            cpu.r[5] = cpu.r[5].wrapping_sub(1);
            cpu.set_t(cpu.r[5] == 0);
            let _ = mem.write_u32(cpu.r[4], cpu.r[0]);
            cpu.r[4] = cpu.r[4].wrapping_add(4);
            if cpu.t() {
                break;
            }
        }
        return;
    }
    cpu.set_t((cpu.r[0] & 1) == 0); // tst #1,r0
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c22f560).unwrap_or(0);
        cpu.set_t(cpu.r[5] & 1 != 0);
        cpu.r[5] >>= 1;
        loop {
            cpu.r[5] = cpu.r[5].wrapping_sub(1);
            cpu.set_t(cpu.r[5] == 0);
            let _ = mem.write_u16(cpu.r[4], cpu.r[0] as u16);
            cpu.r[4] = cpu.r[4].wrapping_add(2);
            if cpu.t() {
                break;
            }
        }
        return;
    }
    cpu.r[0] = mem.read_u32(0x8c22f574).unwrap_or(0);
    loop {
        cpu.r[5] = cpu.r[5].wrapping_sub(1);
        cpu.set_t(cpu.r[5] == 0);
        let _ = mem.write_u8(cpu.r[4], cpu.r[0] as u8);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        if cpu.t() {
            break;
        }
    }
}
