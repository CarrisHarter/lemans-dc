//! 0x8c183274 — walk one or two index lists and remap each live slot.
//!
//! `r14 = *[0x8c183338]` (the global at `0x8c2ab268`). `r9` is the object,
//! `r10` the parallel pointer array.
//!
//! When `[object+28] == 0`, the count is the signed word at `object+20`.
//! Each index `i` loads a record through `0x8c059054(object+8, i)` and, when
//! `[record+12] != 0`, calls `0x8c0fda3c(r14, [record+12], array[i])`.
//!
//! Otherwise `0x8c166d98(object+28)` is the first count and `[header+72]` the
//! second. The first walk uses `0x8c166d8c` and `[record+36]`; the second uses
//! `0x8c059054` and `[record+12]`, indexing `array[first_count + i]`.
//!
//! `bt.s`/`bf.s` delay slots run on both outcomes: the global load, `r12 = 0`,
//! the trailing `r11 += 4`, and `r11 = 0` before the second walk. `r8`..`r14`
//! and `pr` are restored. Every exit leaves T clear.

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

fn signed_gt(rn: u32, rm: u32) -> bool {
    (rn as i32) > (rm as i32)
}

/// `jsr 0x8c0fda3c` with the delay slot `mov r14, r4`. `r5` and `r6` are already set.
fn call_remap(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = rd(mem, 0x8c183344);
    cpu.pr = ret;
    cpu.r[4] = cpu.r[14];
    crate::registry::p_8c0fda3c::run(cpu, mem);
}

/// Null `[object+28]`: reload `[object+20]` every test, index the array by `r11`.
fn walk_direct(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[8] = 0;
    cpu.r[11] = cpu.r[8];
    cpu.r[11] = cpu.r[11].wrapping_shl(2); // bra delay slot
    loop {
        cpu.r[0] = rd(mem, cpu.r[9].wrapping_add(20));
        cpu.set_t(signed_gt(cpu.r[0], cpu.r[8]));
        if !cpu.t() {
            break;
        }
        cpu.r[4] = rd(mem, cpu.r[9].wrapping_add(8));
        cpu.r[0] = rd(mem, 0x8c183348);
        cpu.pr = 0x8c18330a;
        cpu.r[5] = cpu.r[8];
        crate::registry::p_8c059054::run(cpu, mem);
        cpu.r[5] = rd(mem, cpu.r[0].wrapping_add(12));
        cpu.set_t(cpu.r[5] == 0);
        if !cpu.t() {
            cpu.r[0] = cpu.r[11];
            cpu.r[6] = rd(mem, cpu.r[0].wrapping_add(cpu.r[10]));
            call_remap(cpu, mem, 0x8c183318);
        }
        cpu.r[11] = cpu.r[11].wrapping_add(4);
        cpu.r[8] = cpu.r[8].wrapping_add(1);
    }
}

/// First walk: `0x8c166d8c`, gate `[record+36]`, array index `r11`.
fn walk_header_slots(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[11] = cpu.r[12];
    cpu.r[11] = cpu.r[11].wrapping_shl(2);
    loop {
        cpu.r[4] = rd(mem, cpu.r[9].wrapping_add(28));
        cpu.r[0] = rd(mem, 0x8c183340);
        cpu.pr = 0x8c1832b2;
        cpu.r[5] = cpu.r[12];
        crate::registry::p_8c166d8c::run(cpu, mem);
        cpu.r[0] = cpu.r[0].wrapping_add(24);
        cpu.r[5] = rd(mem, cpu.r[0].wrapping_add(12));
        cpu.set_t(cpu.r[5] == 0);
        if !cpu.t() {
            cpu.r[0] = cpu.r[11];
            cpu.r[6] = rd(mem, cpu.r[0].wrapping_add(cpu.r[10]));
            call_remap(cpu, mem, 0x8c1832c4);
        }
        cpu.r[12] = cpu.r[12].wrapping_add(1);
        cpu.set_t(signed_gt(cpu.r[13], cpu.r[12]));
        cpu.r[11] = cpu.r[11].wrapping_add(4); // bt.s delay, including the exit
        if !cpu.t() {
            break;
        }
    }
}

/// Second walk: `0x8c059054`, gate `[record+12]`, array index `(r13 + r11) << 2`.
fn walk_header_tail(cpu: &mut Cpu, mem: &mut Memory) {
    loop {
        cpu.r[4] = rd(mem, cpu.r[9].wrapping_add(8));
        cpu.r[0] = rd(mem, 0x8c183348);
        cpu.pr = 0x8c1832da;
        cpu.r[5] = cpu.r[11];
        crate::registry::p_8c059054::run(cpu, mem);
        cpu.r[5] = rd(mem, cpu.r[0].wrapping_add(12));
        cpu.set_t(cpu.r[5] == 0);
        if !cpu.t() {
            cpu.r[1] = cpu.r[13];
            cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[11]);
            cpu.r[0] = cpu.r[10];
            cpu.r[1] = cpu.r[1].wrapping_shl(2);
            cpu.r[6] = rd(mem, cpu.r[0].wrapping_add(cpu.r[1]));
            call_remap(cpu, mem, 0x8c1832f0);
        }
        cpu.r[11] = cpu.r[11].wrapping_add(1);
        cpu.set_t(signed_gt(cpu.r[8], cpu.r[11]));
        if !cpu.t() {
            break;
        }
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    push(cpu, mem, cpu.r[11]);
    push(cpu, mem, cpu.r[10]);
    push(cpu, mem, cpu.r[9]);
    cpu.r[10] = cpu.r[5];
    push(cpu, mem, cpu.r[8]);
    cpu.r[9] = cpu.r[4];
    push(cpu, mem, cpu.pr);

    cpu.r[0] = rd(mem, 0x8c183338);
    cpu.r[4] = rd(mem, cpu.r[9].wrapping_add(28));
    cpu.set_t(cpu.r[4] == 0);
    cpu.r[14] = rd(mem, cpu.r[0]); // bt.s delay: always load the global
    if cpu.t() {
        walk_direct(cpu, mem);
    } else {
        cpu.r[0] = rd(mem, 0x8c18333c);
        cpu.pr = 0x8c183298;
        crate::registry::p_8c166d98::run(cpu, mem);
        cpu.r[1] = rd(mem, cpu.r[9].wrapping_add(28));
        cpu.r[13] = cpu.r[0];
        cpu.set_t(signed_gt(cpu.r[0], 0));
        cpu.r[1] = cpu.r[1].wrapping_add(72);
        cpu.r[8] = rd(mem, cpu.r[1]);
        let slots = cpu.t();
        cpu.r[12] = 0; // bf.s delay
        if slots {
            walk_header_slots(cpu, mem);
        }
        cpu.set_t(signed_gt(cpu.r[8], 0));
        let tail = cpu.t();
        cpu.r[11] = 0; // bf.s delay
        if tail {
            walk_header_tail(cpu, mem);
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
