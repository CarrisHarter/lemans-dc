//! 0x8c1cba54 — free a circular list of payloads, then the container if asked.
//!
//! A null container returns 0. Otherwise `[container+36]` is a header whose
//! first word is the sentinel. The cursor starts at `sentinel->next`. Each
//! node before the sentinel may own a record at `node+8`; that record's two
//! pointers and the record itself are freed. `0x8c1cbb14` then drains the
//! nodes, and the sentinel and the header are freed too.
//!
//! `[container+36] = 0` runs in the `bf.s` delay slot on both outcomes.
//! `sext16(r5) > 0` is what also frees the container: `0x00010000` does not,
//! `0x80000001` does. `r12`, `r13` and `r14` are restored and `r0` is the
//! container.

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

fn dispatch(cpu: &mut Cpu, mem: &mut Memory) {
    match cpu.r[0] {
        0x8c1cb4d0 => crate::registry::p_8c1cb4d0::run(cpu, mem),
        0x8c1cb4c8 => crate::registry::p_8c1cb4c8::run(cpu, mem),
        0x8c01032c => crate::registry::p_8c01032c::run(cpu, mem),
        0x8c1cbb14 => crate::registry::p_8c1cbb14::run(cpu, mem),
        _ => {}
    }
}

fn call_pool(cpu: &mut Cpu, mem: &mut Memory, pool: u32, ret: u32, r4: u32) {
    cpu.r[0] = mem.read_u32(pool).unwrap_or(0);
    cpu.pr = ret;
    cpu.r[4] = r4;
    dispatch(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    push(cpu, mem, cpu.r[12]);
    cpu.r[13] = cpu.r[4];
    push(cpu, mem, cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(12);
    cpu.set_t(cpu.r[13] == 0);
    if !cpu.t() {
        cpu.r[0] = 0;
        let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[0]);
        // 0x8c1cb4d0 writes [[header]] through the pointer we push.
        let slot = cpu.r[15].wrapping_add(4);
        push(cpu, mem, slot);
        call_pool(
            cpu,
            mem,
            0x8c1cbb04,
            0x8c1cba76,
            mem.read_u32(cpu.r[13].wrapping_add(36)).unwrap_or(0),
        );
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[0] = cpu.r[15].wrapping_add(4);
        cpu.r[1] = mem.read_u32(cpu.r[0]).unwrap_or(0);
        cpu.r[3] = cpu.r[15].wrapping_add(8);
        let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
        // 0x8c1cb4c8 writes [header] through the frame base.
        let frame = cpu.r[15];
        push(cpu, mem, frame);
        call_pool(
            cpu,
            mem,
            0x8c1cbb08,
            0x8c1cba8e,
            mem.read_u32(cpu.r[13].wrapping_add(36)).unwrap_or(0),
        );
        cpu.r[15] = cpu.r[15].wrapping_add(4);
        cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
        cpu.set_t(cpu.r[0] == cpu.r[1]);
        if !cpu.t() {
            loop {
                cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
                cpu.r[12] = mem.read_u32(cpu.r[0].wrapping_add(8)).unwrap_or(0);
                cpu.set_t(cpu.r[12] == 0);
                if !cpu.t() {
                    call_pool(
                        cpu,
                        mem,
                        0x8c1cbb0c,
                        0x8c1cbaa6,
                        mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0),
                    );
                    call_pool(
                        cpu,
                        mem,
                        0x8c1cbb0c,
                        0x8c1cbaac,
                        mem.read_u32(cpu.r[12]).unwrap_or(0),
                    );
                    call_pool(cpu, mem, 0x8c1cbb0c, 0x8c1cbab2, cpu.r[12]);
                }
                cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
                cpu.r[0] = cpu.r[15];
                cpu.r[1] = mem.read_u32(cpu.r[1]).unwrap_or(0);
                let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[1]);
                push(cpu, mem, cpu.r[0]);
                call_pool(
                    cpu,
                    mem,
                    0x8c1cbb08,
                    0x8c1cbac2,
                    mem.read_u32(cpu.r[13].wrapping_add(36)).unwrap_or(0),
                );
                cpu.r[15] = cpu.r[15].wrapping_add(4);
                cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap_or(0);
                cpu.set_t(cpu.r[0] == cpu.r[1]);
                if cpu.t() {
                    break;
                }
            }
        }
        cpu.r[12] = mem.read_u32(cpu.r[13].wrapping_add(36)).unwrap_or(0);
        cpu.set_t(cpu.r[12] == 0);
        if !cpu.t() {
            call_pool(cpu, mem, 0x8c1cbb10, 0x8c1cbad8, cpu.r[12]);
            call_pool(
                cpu,
                mem,
                0x8c1cbb0c,
                0x8c1cbade,
                mem.read_u32(cpu.r[12]).unwrap_or(0),
            );
            call_pool(cpu, mem, 0x8c1cbb0c, 0x8c1cbae4, cpu.r[12]);
        }
        cpu.r[1] = 0;
        cpu.r[0] = cpu.r[14] as i16 as i32 as u32;
        cpu.set_t((cpu.r[0] as i32) > 0);
        let _ = mem.write_u32(cpu.r[13].wrapping_add(36), cpu.r[1]); // bf.s delay
        if cpu.t() {
            call_pool(cpu, mem, 0x8c1cbb0c, 0x8c1cbaf4, cpu.r[13]);
        }
    }
    cpu.r[0] = cpu.r[13];
    cpu.r[15] = cpu.r[15].wrapping_add(12);
    cpu.pr = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem); // rts delay
}
