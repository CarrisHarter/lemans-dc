//! 0x8c06c244 — remove from the pair table ([r4+0x1c], count [r4+0x24],
//! floats [r4+0x20]) the first entry whose field0 or field1 equals r5.
//!
//! CFG: COND(0x8c06c2aa) -> BODY(0x8c06c256) -> {FOUND(0x8c06c262) | SKIP(0x8c06c2a6)}
//!      -> NEXT(0x8c06c2a8: r1+=1) -> COND; COND falls through to the epilogue.
//! Removal copies the last entry (base-12+12*count) and last float
//! (base20-4+4*count) over slot i, then count--.
//!
//! Only bra/rts are delayed here: the delay slots that fire are
//! `mov #-12,r3` (0x8c06c254) and the count-- store (0x8c06c2a4).
//! Plain bt/bf do NOT run their delay slots in this ISA.
//!
//! Side effects the C does not show: r6 := -4, r3 := -12; on a removal
//! r1 is decremented then re-incremented (net: same slot retried), r0 ends
//! at base+12*scanned, r7 := count-1, fr0 := the moved float; shll at
//! 0x8c06c264/0x8c06c26a sets T to the shifted-out bit (later overwritten
//! by the final cmp/gt); exit T = 0.

use sh4_oracle::{Cpu, Memory};

#[inline]
fn ld(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}
#[inline]
fn st(mem: &mut Memory, addr: u32, v: u32) {
    let _ = mem.write_u32(addr, v);
}
#[inline]
fn push(cpu: &mut Cpu, mem: &mut Memory, n: usize) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    st(mem, cpu.r[15], cpu.r[n]);
}
#[inline]
fn pop(cpu: &mut Cpu, mem: &Memory, n: usize) {
    cpu.r[n] = ld(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
#[inline]
fn shll(cpu: &mut Cpu, n: usize) {
    let out = cpu.r[n] >> 31 != 0;
    cpu.r[n] <<= 1;
    cpu.set_t(out);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    push(cpu, mem, 14);
    push(cpu, mem, 13);
    push(cpu, mem, 12);
    push(cpu, mem, 11);
    cpu.r[0] = ld(mem, cpu.r[4].wrapping_add(28)); // mov.l @(28,r4),r0
    cpu.r[1] = 0; // mov #0,r1
    cpu.r[6] = 0xffff_fffc; // mov.l 0x8c06c2bc,r6
    cpu.r[3] = 0xffff_fff4; // bra delay: mov #-12,r3
    loop {
        // COND 0x8c06c2aa
        cpu.r[11] = ld(mem, cpu.r[4].wrapping_add(36));
        cpu.set_t((cpu.r[11] as i32) > (cpu.r[1] as i32)); // cmp/gt r1,r11
        if !cpu.t() {
            break; // fall through to epilogue 0x8c06c2b0
        }
        // BODY 0x8c06c256
        cpu.r[7] = ld(mem, cpu.r[0]);
        cpu.set_t(cpu.r[7] == cpu.r[5]); // cmp/eq r5,r7
        let matched = if cpu.t() {
            true // bt 0x8c06c262
        } else {
            cpu.r[7] = ld(mem, cpu.r[0].wrapping_add(4)); // mov.l @(4,r0),r7
            cpu.set_t(cpu.r[7] == cpu.r[5]); // cmp/eq r5,r7
            cpu.t() // bf 0x8c06c2a6 falls through on match
        };
        if !matched {
            // SKIP 0x8c06c2a6
            cpu.r[0] = cpu.r[0].wrapping_add(12);
            cpu.r[1] = cpu.r[1].wrapping_add(1); // NEXT 0x8c06c2a8
            continue;
        }
        // FOUND 0x8c06c262
        cpu.r[7] = cpu.r[11]; // mov r11,r7
        shll(cpu, 7);
        cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[11]); // 3*count
        cpu.r[14] = cpu.r[1]; // mov r1,r14
        shll(cpu, 14);
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[1]); // 3*i
        cpu.r[11] = ld(mem, cpu.r[4].wrapping_add(28)); // mov.l @(28,r4),r11
        cpu.r[14] <<= 2; // shll2 r14
        cpu.r[7] <<= 2; // shll2 r7
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[11]); // dest = base+12*i
        cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[3]); // base-12
        cpu.r[11] = cpu.r[11].wrapping_add(cpu.r[7]); // src = base-12+12*count
        cpu.r[12] = ld(mem, cpu.r[11]); // mov.l @r11,r12
        cpu.r[13] = cpu.r[1]; // mov r1,r13
        cpu.r[7] = ld(mem, cpu.r[11].wrapping_add(4)); // mov.l @(4,r11),r7
        cpu.r[13] <<= 2; // shll2 r13
        cpu.r[1] = cpu.r[1].wrapping_sub(1); // add #-1,r1
        st(mem, cpu.r[14], cpu.r[12]); // mov.l r12,@r14
        st(mem, cpu.r[14].wrapping_add(4), cpu.r[7]); // mov.l r7,@(4,r14)
        cpu.r[7] = ld(mem, cpu.r[11].wrapping_add(8)); // mov.l @(8,r11),r7
        st(mem, cpu.r[14].wrapping_add(8), cpu.r[7]); // mov.l r7,@(8,r14)
        cpu.r[12] = ld(mem, cpu.r[4].wrapping_add(32)); // mov.l @(32,r4),r12
        cpu.r[7] = ld(mem, cpu.r[4].wrapping_add(36)); // mov.l @(36,r4),r7
        cpu.r[14] = cpu.r[12]; // mov r12,r14
        cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[6]); // add r6,r12
        cpu.r[7] <<= 2; // shll2 r7
        cpu.r[12] = cpu.r[12].wrapping_add(cpu.r[7]); // src float
        cpu.fr[0] = f32::from_bits(ld(mem, cpu.r[12])); // fmov @r12,fr0
        cpu.r[14] = cpu.r[14].wrapping_add(cpu.r[13]); // add r13,r14
        st(mem, cpu.r[14], cpu.fr[0].to_bits()); // fmov fr0,@r14
        cpu.r[7] = ld(mem, cpu.r[4].wrapping_add(36));
        cpu.r[7] = cpu.r[7].wrapping_sub(1); // add #-1,r7
        // bra 0x8c06c2a8; delay slot 0x8c06c2a4 stores the new count
        st(mem, cpu.r[4].wrapping_add(36), cpu.r[7]); // mov.l r7,@(36,r4)
        cpu.r[1] = cpu.r[1].wrapping_add(1); // NEXT 0x8c06c2a8
    }
    // epilogue 0x8c06c2b0
    pop(cpu, mem, 11);
    pop(cpu, mem, 12);
    pop(cpu, mem, 13);
    pop(cpu, mem, 14); // rts delay slot 0x8c06c2b8
}
