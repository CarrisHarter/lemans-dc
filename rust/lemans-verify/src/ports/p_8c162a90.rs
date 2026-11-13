//! 0x8c162a90 — order three record pointers with one comparator.
//!
//! Slots are `r4`, `r5`, `r6`. A pair swaps when the right-hand record should
//! precede the left:
//! - unequal words at `+20`: the greater signed word comes first;
//! - equal words, unequal dwords through `+12`: a key of 2 comes first;
//! - both equal: `0x8c085480` on the 4 halfwords at `+32`, and the smaller
//!   halfword sequence comes first.
//!
//! The second swap (slot 2 with slot 3) reorders slot 1 against the displaced
//! record. `movt` is the `bra` delay on the key path. `extu.b` then `tst`
//! leaves T set when that comparison does not swap. `r11`..`r14` and `pr`
//! are restored.

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

fn extu_tst(cpu: &mut Cpu) {
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
}

/// Compare records in `r5` (left) and `r4` (right). Leaves T set when they stay.
fn precedes_r4(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.r[0] = rd(mem, cpu.r[5].wrapping_add(20));
    cpu.r[1] = rd(mem, cpu.r[4].wrapping_add(20));
    cpu.set_t(cpu.r[1] == cpu.r[0]);
    if !cpu.t() {
        cpu.set_t((cpu.r[1] as i32) > (cpu.r[0] as i32)); // cmp/gt r0, r1
        cpu.r[0] = cpu.t() as u32;
    } else {
        cpu.r[1] = rd(mem, cpu.r[5].wrapping_add(12));
        cpu.r[0] = rd(mem, cpu.r[4].wrapping_add(12));
        cpu.r[1] = rd(mem, cpu.r[1]);
        cpu.r[0] = rd(mem, cpu.r[0]);
        cpu.set_t(cpu.r[0] == cpu.r[1]);
        if !cpu.t() {
            cpu.set_t(cpu.r[0] == 2); // cmp/eq #2, r0
            cpu.r[0] = cpu.t() as u32; // bra delay
        } else {
            cpu.r[0] = rd(mem, 0x8c162b60);
            cpu.r[5] = cpu.r[5].wrapping_add(32);
            cpu.pr = ret;
            cpu.r[4] = cpu.r[4].wrapping_add(32);
            crate::registry::p_8c085480::run(cpu, mem);
        }
    }
    extu_tst(cpu);
}

/// Compare `r14` (displaced right record) against `r5` (current left record).
fn precedes_r14(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[1] = rd(mem, cpu.r[14].wrapping_add(20));
    cpu.r[0] = rd(mem, cpu.r[5].wrapping_add(20));
    cpu.set_t(cpu.r[1] == cpu.r[0]);
    if !cpu.t() {
        cpu.set_t((cpu.r[1] as i32) > (cpu.r[0] as i32));
        cpu.r[0] = cpu.t() as u32;
    } else {
        cpu.r[1] = rd(mem, cpu.r[5].wrapping_add(12));
        cpu.r[0] = rd(mem, cpu.r[14].wrapping_add(12));
        cpu.r[1] = rd(mem, cpu.r[1]);
        cpu.r[0] = rd(mem, cpu.r[0]);
        cpu.set_t(cpu.r[0] == cpu.r[1]);
        if !cpu.t() {
            cpu.set_t(cpu.r[0] == 2);
            cpu.r[0] = cpu.t() as u32;
        } else {
            cpu.r[0] = rd(mem, 0x8c162b60);
            cpu.r[4] = cpu.r[14];
            cpu.r[5] = cpu.r[5].wrapping_add(32);
            cpu.pr = 0x8c162b36;
            cpu.r[4] = cpu.r[4].wrapping_add(32);
            crate::registry::p_8c085480::run(cpu, mem);
        }
    }
    extu_tst(cpu);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    cpu.r[13] = cpu.r[6];
    push(cpu, mem, cpu.r[11]);
    cpu.r[12] = cpu.r[5];
    push(cpu, mem, cpu.pr);
    cpu.r[11] = cpu.r[4];

    cpu.r[5] = rd(mem, cpu.r[11]);
    cpu.r[4] = rd(mem, cpu.r[12]);
    precedes_r4(cpu, mem, 0x8c162ac0);
    if !cpu.t() {
        cpu.r[1] = rd(mem, cpu.r[11]);
        cpu.r[0] = rd(mem, cpu.r[12]);
        let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
        let _ = mem.write_u32(cpu.r[12], cpu.r[1]);
    }

    cpu.r[5] = rd(mem, cpu.r[12]);
    cpu.r[4] = rd(mem, cpu.r[13]);
    precedes_r4(cpu, mem, 0x8c162afc);
    if !cpu.t() {
        cpu.r[14] = rd(mem, cpu.r[13]);
        cpu.r[0] = rd(mem, cpu.r[12]);
        let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
        cpu.r[5] = rd(mem, cpu.r[11]);
        precedes_r14(cpu, mem);
        if !cpu.t() {
            cpu.r[0] = rd(mem, cpu.r[11]);
            let _ = mem.write_u32(cpu.r[12], cpu.r[0]);
            let _ = mem.write_u32(cpu.r[11], cpu.r[14]); // bra delay
        } else {
            let _ = mem.write_u32(cpu.r[12], cpu.r[14]);
        }
    }

    cpu.pr = pop(cpu, mem);
    cpu.r[11] = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}
