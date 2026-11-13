//! 0x8c1287dc — scan `[r4+8]` for `4*[r4+4]` pointers.
//!
//! The first pass calls `0x8c128830` on the first object whose word at `+0`
//! equals `[r5]`. If none match, the second pass calls it on the first object
//! whose byte at `+4` is nonzero. Otherwise it returns without a call.
//! `r11`–`r14` are untouched.

use sh4_oracle::{Cpu, Memory};

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn call_rewrite(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = word(mem, 0x8c12882c); // 0x8c128830
    cpu.pr = ret;
    crate::registry::p_8c128830::run(cpu, mem);
}

fn epilogue(cpu: &mut Cpu, mem: &Memory) {
    cpu.pr = word(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_pr = cpu.pr;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], saved_pr);

    cpu.r[7] = word(mem, cpu.r[4].wrapping_add(8));
    cpu.r[0] = word(mem, cpu.r[4].wrapping_add(4));
    cpu.r[6] = cpu.r[7];
    cpu.r[3] = word(mem, cpu.r[5]);
    cpu.r[0] <<= 2; // shll2, T unchanged
    cpu.r[1] = cpu.r[7];
    cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[0]); // bra delay

    loop {
        cpu.set_t(cpu.r[6] > cpu.r[1]); // cmp/hi r1, r6
        if !cpu.t() {
            break;
        }
        cpu.r[4] = word(mem, cpu.r[1]);
        cpu.r[0] = word(mem, cpu.r[4]);
        cpu.set_t(cpu.r[3] == cpu.r[0]);
        if cpu.t() {
            call_rewrite(cpu, mem, 0x8c1287fc);
            epilogue(cpu, mem);
            return;
        }
        cpu.r[1] = cpu.r[1].wrapping_add(4);
    }

    loop {
        cpu.set_t(cpu.r[6] > cpu.r[7]); // cmp/hi r7, r6
        if !cpu.t() {
            break;
        }
        cpu.r[4] = word(mem, cpu.r[7]);
        cpu.r[0] = mem.read_u8(cpu.r[4].wrapping_add(4)).unwrap_or(0) as i8 as i32 as u32;
        cpu.r[0] &= 0xff; // extu.b
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            call_rewrite(cpu, mem, 0x8c12881a);
            epilogue(cpu, mem);
            return;
        }
        cpu.r[7] = cpu.r[7].wrapping_add(4);
    }
    epilogue(cpu, mem);
}
