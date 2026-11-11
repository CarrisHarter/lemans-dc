//! 0x8c02c2a4 — angle wrap over 4 structs of stride 0xac: for each,
//! `*(f32*)(p+0x70) = *(f32*)(p+0xcc) * -v + *(f32*)(p+0x70)`, then reduce it
//! modulo 2π into (π, -π] by subtracting/adding fr3 at p+0x70 and p+0x88.
//!
//! Hidden side effects the C does not show: r5 stays p0+0x3e0 for the whole
//! call (it is not re-based as r4 advances), r4 ends advanced by 4*0xac (the
//! `bt.s` delay slot runs even on the final pass), r14/r15 are preserved via
//! the stack even though r14 is used as a cursor, r6 ends at the last p+0x70,
//! r0 at the last p+0xcc, and fr4 keeps the negated load. T ends clear
//! (final `cmp/gt r7,r1` is 4 > 4).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 (stack only; r14 restored by the rts delay slot)
    cpu.r[1] = mem.read_u32(0x8C02C304).unwrap(); // 0x3e0
    cpu.r[7] = 0; // mov #0,r7
    cpu.fr[3] = f32::from_bits(mem.read_u32(0x8C02C308).unwrap()); // 2*pi
    cpu.r[5] = cpu.r[4].wrapping_add(cpu.r[1]); // r5 = r4 + 0x3e0
    cpu.fr[0] = f32::from_bits(mem.read_u32(0x8C02C30C).unwrap()); // -pi
    cpu.r[1] = 4; // mov #4,r1
    cpu.r[3] = mem.read_u32(0x8C02C310).unwrap(); // 0xac
    cpu.fr[2] = f32::from_bits(mem.read_u32(0x8C02C314).unwrap()); // pi
    loop {
        // 0x8c02c2be
        let mut r14 = cpu.r[4].wrapping_add(88); // mov r4,r14 ; add #88,r14
        cpu.r[0] = r14.wrapping_add(116); // mov r14,r0 ; add #116,r0
        cpu.r[6] = r14.wrapping_add(24); // mov r14,r6 ; add #24,r6
        cpu.fr[4] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap()); // fmov @r0,fr4
        cpu.fr[5] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap()); // fmov @r5,fr5
        cpu.fr[4] = -cpu.fr[4]; // fneg fr4
        cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // fmov @r6,fr1
        cpu.fr[5] = cpu.fr[5] * cpu.fr[4]; // fmul fr4,fr5
        cpu.fr[5] = cpu.fr[5] + cpu.fr[1]; // fadd fr1,fr5
        let _ = mem.write_f32(cpu.r[6], cpu.fr[5]); // bra delay slot: fmov fr5,@r6
        // 0x8c02c2e0: reduce while fr1 > fr2
        loop {
            cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // fmov @r6,fr1
            cpu.set_t(cpu.fr[1] > cpu.fr[2]); // fcmp/gt fr2,fr1
            if !cpu.t() {
                break; // bt 0x8c02c2da not taken
            }
            cpu.fr[1] = cpu.fr[1] - cpu.fr[3]; // fsub fr3,fr1
            let _ = mem.write_f32(cpu.r[6], cpu.fr[1]); // fmov fr1,@r6
        }
        r14 = r14.wrapping_add(24); // bra delay slot: add #24,r14
        // 0x8c02c2f0: raise while fr0 > fr1
        loop {
            cpu.fr[1] = f32::from_bits(mem.read_u32(r14).unwrap()); // fmov @r14,fr1
            cpu.set_t(cpu.fr[0] > cpu.fr[1]); // fcmp/gt fr1,fr0
            if !cpu.t() {
                break; // bt 0x8c02c2ea not taken
            }
            cpu.fr[1] = cpu.fr[1] + cpu.fr[3]; // fadd fr3,fr1
            let _ = mem.write_f32(r14, cpu.fr[1]); // fmov fr1,@r14
        }
        cpu.r[7] = cpu.r[7].wrapping_add(1); // add #1,r7
        cpu.set_t((cpu.r[1] as i32) > (cpu.r[7] as i32)); // cmp/gt r7,r1
        let again = cpu.t();
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[3]); // bt.s delay slot: add r3,r4
        if !again {
            break;
        }
    }
    // rts ; mov.l @r15+,r14 (r14/r15 net unchanged)
}
