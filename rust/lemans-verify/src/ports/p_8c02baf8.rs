//! 0x8c02baf8 — copy a 6-float (24-byte, single precision) record from
//! param_1+0x484..0x4ac to param_2.
//!
//! Hidden side effects the C does not show:
//! - the prologue `mov.l r14,@-r15` / delay-slot `mov.l @r15+,r14` cancel,
//!   so r14/r15 end unchanged (the spill lands in the stack window)
//! - r5 is advanced in place twice (r5 += 40 final), r4 += 0x4ac
//! - r6 = r5+36, r7 = r4+0x4a4, r3 = r4+0x4a8, r1 = 0x4ac are left behind
//! - r0 ends as the raw u32 loaded from [r4+0x4a0]
//! - fr0 ends as the float loaded from [r4+0x4ac]
//! - 10 stores: seven mov.l (offsets 0..28) plus three fmov.s (32, 36, 40)

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let lit = |mem: &Memory, a: u32| mem.read_u32(a).unwrap();

    cpu.r[1] = lit(mem, 0x8C02BB58); // 0x484
    cpu.r[0] = r4;
    // cpu.r[14] = r5;  — r14 is spilled and restored by the rts delay slot,
    // so its final value is the original r14; only the store address uses r5+32.
    cpu.r[0] = cpu.r[0].wrapping_add(cpu.r[1]);
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
    cpu.r[6] = r5;
    cpu.r[7] = r4;
    let _ = mem.write_f32(r5, cpu.fr[0]); // fmov fr0,@r5
    cpu.r[3] = r4;
    cpu.r[0] = lit(mem, 0x8C02BB5C); // 0x488
    let r14_dst = r5.wrapping_add(32); // add #32,r14
    cpu.r[1] = lit(mem, 0x8C02BB60); // 0x4a4
    cpu.r[6] = cpu.r[6].wrapping_add(36); // add #36,r6
    cpu.r[0] = lit(mem, cpu.r[0].wrapping_add(r4));
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[1]);
    let _ = mem.write_u32(r5 + 4, cpu.r[0]);
    cpu.r[0] = lit(mem, 0x8C02BB64); // 0x48c
    cpu.r[0] = lit(mem, cpu.r[0].wrapping_add(r4));
    let _ = mem.write_u32(r5 + 8, cpu.r[0]);
    cpu.r[0] = lit(mem, 0x8C02BB68); // 0x490
    cpu.r[1] = lit(mem, 0x8C02BB6C); // 0x4a8
    cpu.r[0] = lit(mem, cpu.r[0].wrapping_add(r4));
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    let _ = mem.write_u32(r5 + 12, cpu.r[0]);
    cpu.r[0] = lit(mem, 0x8C02BB70); // 0x494
    cpu.r[0] = lit(mem, cpu.r[0].wrapping_add(r4));
    let _ = mem.write_u32(r5 + 16, cpu.r[0]);
    cpu.r[0] = lit(mem, 0x8C02BB74); // 0x498
    cpu.r[1] = lit(mem, 0x8C02BB78); // 0x4ac
    cpu.r[0] = lit(mem, cpu.r[0].wrapping_add(r4));
    let _ = mem.write_u32(r5 + 20, cpu.r[0]);
    cpu.r[0] = lit(mem, 0x8C02BB7C); // 0x49c
    cpu.r[0] = lit(mem, cpu.r[0].wrapping_add(r4));
    let _ = mem.write_u32(r5 + 24, cpu.r[0]);
    cpu.r[0] = lit(mem, 0x8C02BB80); // 0x4a0
    cpu.r[0] = lit(mem, cpu.r[0].wrapping_add(r4));
    let _ = mem.write_u32(r5 + 28, cpu.r[0]);
    cpu.r[4] = r4.wrapping_add(cpu.r[1]); // add r1,r4
    cpu.fr[0] = mem.read_f32(cpu.r[7]).unwrap(); // fmov @r7,fr0
    cpu.r[5] = r5.wrapping_add(40); // add #40,r5
    let _ = mem.write_f32(r14_dst, cpu.fr[0]); // fmov fr0,@r14
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr0
    let _ = mem.write_f32(cpu.r[6], cpu.fr[0]); // fmov fr0,@r6
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap(); // fmov @r4,fr0
    let _ = mem.write_f32(cpu.r[5], cpu.fr[0]); // fmov fr0,@r5
}
