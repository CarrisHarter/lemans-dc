//! 0x8c1116cc — insertion-sort an exclusive pointer range `[r4, r5)`.
//!
//! The range is left alone when `r5 <= r4+4`. Otherwise each pointer from
//! `r4+4` is pulled left while it compares strictly before its neighbour.
//!
//! The neighbour test reads the byte at `record+50`:
//! - both bytes nonzero: if the 4 halfwords at `+32` all match (`0x8c0853e8`),
//!   order by the halfwords at `+40`, otherwise by the ones at `+32`
//!   (`0x8c085480`, true when the key is smaller);
//! - only the key's byte is nonzero: always pull;
//! - only the neighbour's byte is nonzero: never pull;
//! - both zero: pull when the neighbour's first word is signed-greater.
//!
//! The hole is filled by `[r11] = key` in the outer `bt.s` delay slot, including
//! the iteration that ends the sort. `r8`..`r14` and `pr` are restored.

use sh4_oracle::{Cpu, Memory};

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &mut Memory) -> u32 {
    let value = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn byte_zx(mem: &Memory, addr: u32) -> u32 {
    // mov.b sign-extends, then extu.b keeps the low 8 bits.
    (mem.read_u8(addr).unwrap_or(0) as i8 as i32 as u32) & 0xff
}

/// True when the key in `r13` should keep moving left past `[r12]`.
fn before_neighbour(cpu: &mut Cpu, mem: &mut Memory) -> bool {
    cpu.r[0] = cpu.r[13].wrapping_add(50);
    cpu.r[0] = byte_zx(mem, cpu.r[0]);
    cpu.r[1] = cpu.r[0] & 0xff;
    cpu.set_t(cpu.r[1] == 0);
    cpu.r[10] = mem.read_u32(cpu.r[12]).unwrap_or(0); // bt.s delay
    if !cpu.t() {
        cpu.r[1] = cpu.r[10].wrapping_add(50);
        cpu.r[1] = byte_zx(mem, cpu.r[1]);
        cpu.r[1] &= 0xff;
        cpu.set_t(cpu.r[1] == 0);
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(0x8c11178c).unwrap_or(0x8c0853e8);
            cpu.r[5] = cpu.r[10];
            cpu.r[4] = cpu.r[13];
            cpu.r[5] = cpu.r[5].wrapping_add(32);
            cpu.pr = 0x8c111728;
            cpu.r[4] = cpu.r[4].wrapping_add(32);
            crate::registry::p_8c0853e8::run(cpu, mem);
            cpu.r[0] &= 0xff;
            cpu.set_t(cpu.r[0] == 0);
            if cpu.t() {
                cpu.r[0] = mem.read_u32(0x8c111790).unwrap_or(0x8c085480);
                cpu.r[4] = cpu.r[13];
                cpu.r[10] = cpu.r[10].wrapping_add(32);
                cpu.r[5] = cpu.r[10];
                cpu.pr = 0x8c11174a;
                cpu.r[4] = cpu.r[4].wrapping_add(32);
            } else {
                cpu.r[0] = mem.read_u32(0x8c111790).unwrap_or(0x8c085480);
                cpu.r[4] = cpu.r[13];
                cpu.r[10] = cpu.r[10].wrapping_add(40);
                cpu.r[5] = cpu.r[10];
                cpu.pr = 0x8c11173a;
                cpu.r[4] = cpu.r[4].wrapping_add(40);
            }
            crate::registry::p_8c085480::run(cpu, mem);
        } else {
            finish_flags(cpu, mem);
        }
    } else {
        finish_flags(cpu, mem);
    }
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    !cpu.t()
}

/// Shared tail at 0x8c11174e: flag bytes already loaded, `r0` still the key's.
fn finish_flags(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = cpu.r[0] & 0xff;
    cpu.set_t(cpu.r[1] == 0);
    if !cpu.t() {
        return;
    }
    cpu.r[1] = cpu.r[10].wrapping_add(50);
    cpu.r[1] = byte_zx(mem, cpu.r[1]);
    cpu.r[1] &= 0xff;
    cpu.set_t(cpu.r[1] == 0);
    if !cpu.t() {
        return;
    }
    cpu.r[1] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[10]).unwrap_or(0);
    cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32));
    cpu.r[0] = cpu.t() as u32;
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    push(cpu, mem, cpu.r[11]);
    push(cpu, mem, cpu.r[10]);
    push(cpu, mem, cpu.r[9]);
    push(cpu, mem, cpu.r[8]);
    cpu.r[9] = cpu.r[4];
    push(cpu, mem, cpu.pr);
    cpu.r[8] = cpu.r[5];
    cpu.r[14] = cpu.r[9];
    cpu.r[14] = cpu.r[14].wrapping_add(4);
    cpu.set_t(cpu.r[8] > cpu.r[14]);
    if cpu.t() {
        loop {
            cpu.r[13] = mem.read_u32(cpu.r[14]).unwrap_or(0);
            cpu.r[12] = cpu.r[14];
            cpu.r[11] = cpu.r[14];
            cpu.r[12] = cpu.r[12].wrapping_sub(4); // bra delay into the test
            loop {
                cpu.set_t(cpu.r[11] > cpu.r[9]);
                if !cpu.t() || !before_neighbour(cpu, mem) {
                    break;
                }
                cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
                cpu.r[12] = cpu.r[12].wrapping_sub(4);
                cpu.r[11] = cpu.r[11].wrapping_sub(4);
            }
            cpu.r[14] = cpu.r[14].wrapping_add(4);
            cpu.set_t(cpu.r[8] > cpu.r[14]);
            let _ = mem.write_u32(cpu.r[11], cpu.r[13]); // bt.s delay
            if !cpu.t() {
                break;
            }
        }
    }
    cpu.pr = pop(cpu, mem);
    cpu.r[8] = pop(cpu, mem);
    cpu.r[9] = pop(cpu, mem);
    cpu.r[10] = pop(cpu, mem);
    cpu.r[11] = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem); // rts delay
}
