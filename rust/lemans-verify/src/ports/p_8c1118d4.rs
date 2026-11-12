//! 0x8c1118d4 - insertion-swap three pointer slots by a double-indirect key.
//!
//! Each slot holds a pointer. The key is `[[pointer+4]]` via 0x8c10e374.
//! A null in the first of a pair forces the swap flag; a null in the second
//! clears it. The third comparison is fused into the second swap.

use sh4_oracle::{Cpu, Memory};

fn key(cpu: &mut Cpu, mem: &mut Memory, ret: u32) {
    cpu.pr = ret;
    crate::registry::p_8c10e374::run(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[13] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[11] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0);
    cpu.r[1] = mem.read_u32(cpu.r[11]).unwrap_or(0); // bt.s delay
    if !cpu.t() {
        cpu.set_t(cpu.r[1] == 0);
        if !cpu.t() {
            cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
            cpu.r[10] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0); // jsr delay
            key(cpu, mem, 0x8c1118fa);
            cpu.r[14] = cpu.r[0];
            cpu.r[4] = cpu.r[10]; // jsr delay
            key(cpu, mem, 0x8c111902);
            cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32)); // cmp/gt r14,r0
            cpu.r[0] = u32::from(cpu.t());
        } else {
            cpu.set_t(cpu.r[0] == 0);
            cpu.r[0] = u32::from(cpu.t());
        }
    } else {
        cpu.set_t(cpu.r[0] == 0);
        cpu.r[0] = u32::from(cpu.t());
    }
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[1] = mem.read_u32(cpu.r[11]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        let _ = mem.write_u32(cpu.r[11], cpu.r[0]);
        let _ = mem.write_u32(cpu.r[12], cpu.r[1]);
    }

    cpu.r[0] = mem.read_u32(cpu.r[13]).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0);
    cpu.r[1] = mem.read_u32(cpu.r[12]).unwrap_or(0); // bt.s delay
    if !cpu.t() {
        cpu.set_t(cpu.r[1] == 0);
        if !cpu.t() {
            cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(4)).unwrap_or(0);
            cpu.r[10] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0);
            key(cpu, mem, 0x8c11192e);
            cpu.r[14] = cpu.r[0];
            cpu.r[4] = cpu.r[10];
            key(cpu, mem, 0x8c111936);
            cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32));
            cpu.r[0] = u32::from(cpu.t());
        } else {
            cpu.set_t(cpu.r[0] == 0);
            cpu.r[0] = u32::from(cpu.t());
        }
    } else {
        cpu.set_t(cpu.r[0] == 0);
        cpu.r[0] = u32::from(cpu.t());
    }
    cpu.r[0] &= 0xff;
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        cpu.r[14] = mem.read_u32(cpu.r[13]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        cpu.set_t(cpu.r[14] == 0);
        let _ = mem.write_u32(cpu.r[13], cpu.r[0]);
        cpu.r[1] = mem.read_u32(cpu.r[11]).unwrap_or(0); // bt.s delay
        if !cpu.t() {
            cpu.set_t(cpu.r[1] == 0);
            if !cpu.t() {
                cpu.r[4] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap_or(0);
                cpu.r[10] = mem.read_u32(cpu.r[1].wrapping_add(4)).unwrap_or(0);
                key(cpu, mem, 0x8c11195e);
                cpu.r[13] = cpu.r[0];
                cpu.r[4] = cpu.r[10];
                key(cpu, mem, 0x8c111966);
                cpu.set_t((cpu.r[0] as i32) > (cpu.r[13] as i32));
                cpu.r[0] = u32::from(cpu.t());
            } else {
                cpu.set_t(cpu.r[14] == 0);
                cpu.r[0] = u32::from(cpu.t());
            }
        } else {
            cpu.set_t(cpu.r[14] == 0);
            cpu.r[0] = u32::from(cpu.t());
        }
        cpu.r[0] &= 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            cpu.r[0] = mem.read_u32(cpu.r[11]).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[12], cpu.r[0]);
            let _ = mem.write_u32(cpu.r[11], cpu.r[14]); // bra delay
        } else {
            let _ = mem.write_u32(cpu.r[12], cpu.r[14]);
        }
    }

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
