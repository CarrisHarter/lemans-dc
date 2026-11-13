//! 0x8c111b40 — bubble three slots into ascending order.
//!
//! `r4`, `r5` and `r6` are pointers to object pointers. Each pass swaps when
//! the later object's count at `+24` is positive and the earlier one's is not,
//! or when both counts are clear and the earlier dword at `+0` is greater, or
//! when both counts are positive and `0x8c085480` says the halfwords at `+40`
//! are greater. The third slot is inserted back through the first two.
//! `r11`–`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn write(mem: &mut Memory, addr: u32, value: u32) {
    let _ = mem.write_u32(addr, value);
}

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    write(mem, cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let value = word(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn pl(cpu: &mut Cpu, value: u32) {
    cpu.set_t((value as i32) > 0);
}

fn gt(cpu: &mut Cpu, rn: u32, rm: u32) {
    cpu.set_t((rn as i32) > (rm as i32));
}

/// Compare objects in `r4` and `r5`. `r0` is 1 when `r5` should precede `r4`.
fn compare_pair(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = word(mem, cpu.r[4].wrapping_add(24));
    pl(cpu, cpu.r[0]);
    if cpu.t() {
        cpu.r[0] = word(mem, cpu.r[5].wrapping_add(24));
        pl(cpu, cpu.r[0]);
        if cpu.t() {
            cpu.r[0] = word(mem, 0x8c111c2c); // 0x8c085480
            cpu.r[5] = cpu.r[5].wrapping_add(40);
            cpu.pr = 0x8c111b68;
            cpu.r[4] = cpu.r[4].wrapping_add(40); // jsr delay
            crate::registry::p_8c085480::run(cpu, mem);
            return;
        }
    }
    fallback(cpu, mem, false);
}

fn fallback(cpu: &mut Cpu, mem: &mut Memory, from_r14: bool) {
    if !from_r14 {
        cpu.r[3] = word(mem, cpu.r[4].wrapping_add(24));
    }
    cpu.r[1] = 1;
    gt(cpu, cpu.r[1], cpu.r[3]); // cmp/gt r3, r1
    if cpu.t() {
        cpu.r[0] = word(mem, cpu.r[5].wrapping_add(24));
        gt(cpu, cpu.r[1], cpu.r[0]); // cmp/gt r0, r1
        if cpu.t() {
            let base = if from_r14 { cpu.r[14] } else { cpu.r[4] };
            cpu.r[1] = word(mem, base);
            cpu.r[0] = word(mem, cpu.r[5]);
            gt(cpu, cpu.r[0], cpu.r[1]); // cmp/gt r1, r0
            cpu.r[0] = cpu.t() as u32; // movt
            return;
        }
    }
    pl(cpu, cpu.r[3]);
    cpu.r[0] = cpu.t() as u32;
}

fn swap_if(cpu: &mut Cpu, mem: &mut Memory, slot_a: u32, slot_b: u32) {
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[1] = word(mem, slot_a);
        cpu.r[0] = word(mem, slot_b);
        write(mem, slot_a, cpu.r[0]);
        write(mem, slot_b, cpu.r[1]);
    }
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

    cpu.r[4] = word(mem, cpu.r[12]);
    cpu.r[5] = word(mem, cpu.r[11]); // bf.s delay, always
    compare_pair(cpu, mem);
    swap_if(cpu, mem, cpu.r[11], cpu.r[12]);

    cpu.r[4] = word(mem, cpu.r[13]);
    cpu.r[5] = word(mem, cpu.r[12]);
    compare_pair(cpu, mem);
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[14] = word(mem, cpu.r[13]);
        cpu.r[0] = word(mem, cpu.r[12]);
        write(mem, cpu.r[13], cpu.r[0]);
        cpu.r[3] = word(mem, cpu.r[14].wrapping_add(24));
        pl(cpu, cpu.r[3]);
        cpu.r[5] = word(mem, cpu.r[11]); // bf.s delay
        if cpu.t() {
            cpu.r[0] = word(mem, cpu.r[5].wrapping_add(24));
            pl(cpu, cpu.r[0]);
            if cpu.t() {
                cpu.r[0] = word(mem, 0x8c111c2c);
                cpu.r[4] = cpu.r[14];
                cpu.r[5] = cpu.r[5].wrapping_add(40);
                cpu.pr = 0x8c111bf2;
                cpu.r[4] = cpu.r[4].wrapping_add(40);
                crate::registry::p_8c085480::run(cpu, mem);
            } else {
                fallback(cpu, mem, true);
            }
        } else {
            fallback(cpu, mem, true);
        }
        cpu.r[0] &= 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if cpu.t() {
            write(mem, cpu.r[12], cpu.r[14]);
        } else {
            cpu.r[0] = word(mem, cpu.r[11]);
            write(mem, cpu.r[12], cpu.r[0]);
            write(mem, cpu.r[11], cpu.r[14]); // bra delay
        }
    }

    cpu.pr = pop(cpu, mem);
    cpu.r[11] = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem); // rts delay
}
