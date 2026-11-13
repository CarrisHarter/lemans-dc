//! 0x8c1ae6e8 — release each pointer in the array, then free the array.
//!
//! A null object returns 0. Otherwise `r14` walks `[object+8]` while it is
//! unsigned-below `base + count*4`. A null slot is skipped. A live record is
//! released at `+72`, `+60`, and the three handles at `+56/+52/+48`, then
//! freed. The saved count argument's low half frees the object itself; the
//! array pointer is freed whenever it is non-null. `0x8c2b8280` is cleared on
//! the way out. `r8`–`r15` are restored.

use sh4_oracle::{Cpu, Memory};

fn push(cpu: &mut Cpu, mem: &mut Memory, value: u32) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], value);
}

fn pop(cpu: &mut Cpu, mem: &Memory) -> u32 {
    let value = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    value
}

fn word(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

fn release_record(cpu: &mut Cpu, mem: &mut Memory) {
    let record = cpu.r[13];
    // 0x8c1ae708: mov r13,r4 ; add #48,r4 ; tst r4,r4
    cpu.r[4] = record.wrapping_add(48);
    cpu.set_t(cpu.r[4] == 0);
    if cpu.r[4] == 0 {
        return;
    }
    // 0x8c1ae710: the +72 pair. r4 is still record+48; the jsr delay adds 24.
    cpu.r[0] = record.wrapping_add(72);
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        cpu.r[0] = word(mem, 0x8c1ae7b8); // 0x8c1af3b4
        cpu.r[4] = cpu.r[4].wrapping_add(24);
        cpu.pr = 0x8c1ae71e;
        crate::registry::p_8c1af3b4::run(cpu, mem);
        cpu.r[0] = word(mem, 0x8c1ae7bc); // 0x8c1ade24
        cpu.r[4] = record.wrapping_add(72);
        cpu.r[5] = cpu.r[12];
        cpu.pr = 0x8c1ae728;
        crate::registry::p_8c1ade24::run(cpu, mem);
    }
    // 0x8c1ae728: the +60 pair. r4 reloaded to record+48, delay adds 12.
    cpu.r[0] = record.wrapping_add(60);
    cpu.set_t(cpu.r[0] == 0);
    if cpu.r[0] != 0 {
        cpu.r[4] = record.wrapping_add(48);
        cpu.r[0] = word(mem, 0x8c1ae7c0); // 0x8c1af164
        cpu.r[4] = cpu.r[4].wrapping_add(12);
        cpu.pr = 0x8c1ae73a;
        crate::registry::p_8c1af164::run(cpu, mem);
        cpu.r[0] = word(mem, 0x8c1ae7c4); // 0x8c1ade78
        cpu.r[4] = record.wrapping_add(60);
        cpu.r[5] = cpu.r[12];
        cpu.pr = 0x8c1ae744;
        crate::registry::p_8c1ade78::run(cpu, mem);
    }
    // Three handle releases, offsets 56, 52, 48. r5 stays -1.
    for (add, ret) in [(56u32, 0x8c1ae74e), (52, 0x8c1ae758), (48, 0x8c1ae762)] {
        cpu.r[0] = word(mem, 0x8c1ae7c8); // 0x8c018bd8
        cpu.r[4] = record.wrapping_add(add);
        cpu.r[5] = cpu.r[12];
        cpu.pr = ret;
        crate::registry::p_8c018bd8::run(cpu, mem);
    }
}

fn epilogue(cpu: &mut Cpu, mem: &Memory) {
    cpu.r[0] = cpu.r[10];
    cpu.pr = pop(cpu, mem);
    cpu.r[10] = pop(cpu, mem);
    cpu.r[11] = pop(cpu, mem);
    cpu.r[12] = pop(cpu, mem);
    cpu.r[13] = pop(cpu, mem);
    cpu.r[14] = pop(cpu, mem);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, cpu.r[14]);
    push(cpu, mem, cpu.r[13]);
    push(cpu, mem, cpu.r[12]);
    push(cpu, mem, cpu.r[11]);
    push(cpu, mem, cpu.r[10]);
    cpu.r[11] = cpu.r[5];
    push(cpu, mem, cpu.pr);
    cpu.r[10] = cpu.r[4];
    cpu.set_t(cpu.r[10] == 0);
    if cpu.r[10] == 0 {
        epilogue(cpu, mem);
        return;
    }
    cpu.r[14] = word(mem, cpu.r[10].wrapping_add(8));
    cpu.r[12] = (-1i32) as u32; // bra delay
    loop {
        cpu.r[0] = word(mem, cpu.r[10].wrapping_add(4));
        cpu.r[1] = word(mem, cpu.r[10].wrapping_add(8));
        cpu.r[0] <<= 2; // shll2
        cpu.r[1] = cpu.r[1].wrapping_add(cpu.r[0]);
        cpu.set_t(cpu.r[1] > cpu.r[14]); // cmp/hi r14,r1
        if !cpu.t() {
            break;
        }
        cpu.r[13] = word(mem, cpu.r[14]);
        cpu.set_t(cpu.r[13] == 0);
        if cpu.r[13] != 0 {
            release_record(cpu, mem);
            cpu.r[0] = word(mem, 0x8c1ae7cc); // 0x8c01032c
            cpu.r[4] = cpu.r[13];
            cpu.pr = 0x8c1ae768;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
        cpu.r[14] = cpu.r[14].wrapping_add(4);
    }
    cpu.r[1] = word(mem, 0x8c1ae7d0); // 0x8c2b8280
    cpu.r[0] = 0;
    cpu.set_t(cpu.r[10] == 0);
    let _ = mem.write_u32(cpu.r[1], cpu.r[0]); // bt.s delay, both ways
    if !cpu.t() {
        cpu.set_t(cpu.r[10] == 0);
        if !cpu.t() {
            cpu.r[4] = word(mem, cpu.r[10].wrapping_add(8));
            cpu.set_t(cpu.r[4] == 0);
            if !cpu.t() {
                cpu.r[0] = word(mem, 0x8c1ae7cc);
                cpu.pr = 0x8c1ae790;
                crate::registry::p_8c01032c::run(cpu, mem);
            }
            cpu.set_t(cpu.r[10] == 0);
            if !cpu.t() {
                cpu.r[0] = word(mem, 0x8c1ae7d4); // 0x8c01e078
                cpu.r[4] = cpu.r[10];
                cpu.r[5] = 0;
                cpu.pr = 0x8c1ae79c;
                crate::registry::p_8c01e078::run(cpu, mem);
            }
        }
    }
    cpu.r[0] = cpu.r[11] as u16 as i16 as i32 as u32; // exts.w
    cpu.set_t((cpu.r[0] as i32) > 0);
    if cpu.t() {
        cpu.r[0] = word(mem, 0x8c1ae7cc);
        cpu.r[4] = cpu.r[10];
        cpu.pr = 0x8c1ae7a8;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    epilogue(cpu, mem);
}
