//! 0x8c19f578 — mode-flag transition on the global byte at `+0x345`.
//!
//! Two calls to `0x8c074d58` read flags 4 and 5 for the index at
//! `[object+16]`. `0x8c1cc3b4` then classifies the mode:
//!
//! - mode 1 or 2, flag 4 set: store 0 and call `0x8c1cc380`, return 1
//! - mode 3, flag 4 set: store 1 and call, return 1
//! - mode 3, flag 4 clear and flag 5 set: store 0 and call, return 1
//! - anything else: return 0
//!
//! `r14` is the 0/1 result and is copied to `r0` before the epilogue restores
//! it. `r12` holds flag 4 and `r13` flag 5 until that restore. T is the last
//! `tst` on a mode path, or the failing `cmp/eq #3` when the mode is neither.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &mut Memory) -> u32 {
    let value = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn flag_at(cpu: &mut Cpu, mem: &mut Memory, which: u32, ret: u32) {
    cpu.r[0] = rd(mem, 0x8c19f640);
    cpu.r[7] = 0;
    cpu.r[5] = rd(mem, cpu.r[13].wrapping_add(16));
    cpu.r[4] = rd(mem, cpu.r[0]);
    cpu.r[0] = rd(mem, 0x8c19f644);
    cpu.pr = ret;
    cpu.r[6] = which;
    crate::registry::p_8c074d58::run(cpu, mem);
}

fn store_byte(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[0] = rd(mem, 0x8c19f650);
    cpu.r[1] = value;
    cpu.r[3] = rd(mem, cpu.r[0]);
    cpu.r[0] = rd(mem, 0x8c19f654);
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1] as u8);
}

fn call_clear(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = rd(mem, 0x8c19f648);
    cpu.r[4] = rd(mem, cpu.r[0]);
    cpu.r[0] = rd(mem, 0x8c19f658);
    cpu.pr = ret;
    crate::registry::p_8c1cc380::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    push(cpu, mem, cpu.pr);

    flag_at(cpu, mem, 4, 0x8c19f590);
    cpu.r[1] = rd(mem, 0x8c19f640);
    cpu.r[12] = cpu.r[0];
    cpu.r[7] = 0;
    cpu.r[5] = rd(mem, cpu.r[13].wrapping_add(16));
    cpu.r[4] = rd(mem, cpu.r[1]);
    cpu.r[0] = rd(mem, 0x8c19f644);
    cpu.pr = 0x8c19f5a0;
    cpu.r[6] = 5;
    crate::registry::p_8c074d58::run(cpu, mem);

    cpu.r[1] = rd(mem, 0x8c19f648);
    cpu.r[13] = cpu.r[0];
    cpu.r[0] = rd(mem, 0x8c19f64c);
    cpu.r[14] = 0;
    cpu.pr = 0x8c19f5ac;
    cpu.r[4] = rd(mem, cpu.r[1]);
    crate::registry::p_8c1cc3b4::run(cpu, mem);

    cpu.set_t(cpu.r[0] == 1);
    if cpu.r[0] == 1 {
        apply_mode12(cpu, mem, 0x8c19f630);
    } else {
        cpu.set_t(cpu.r[0] == 2);
        if cpu.r[0] == 2 {
            apply_mode12(cpu, mem, 0x8c19f610);
        } else {
            cpu.set_t(cpu.r[0] == 3);
            if cpu.r[0] == 3 {
                cpu.r[0] = cpu.r[12] & 0xff;
                cpu.set_t(cpu.r[0] == 0);
                if !cpu.t() {
                    store_byte(cpu, mem, 1);
                    call_clear(cpu, mem, 0x8c19f5d6);
                    cpu.r[14] = 1;
                } else {
                    cpu.r[0] = cpu.r[13] & 0xff;
                    cpu.set_t(cpu.r[0] == 0);
                    if !cpu.t() {
                        store_byte(cpu, mem, cpu.r[14]);
                        call_clear(cpu, mem, 0x8c19f5f4);
                        cpu.r[14] = 1;
                    }
                }
            }
        }
    }

    cpu.r[0] = cpu.r[14];
    cpu.pr = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}

fn apply_mode12(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = cpu.r[12] & 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        store_byte(cpu, mem, cpu.r[14]);
        call_clear(cpu, mem, ret);
        cpu.r[14] = 1;
    }
}
