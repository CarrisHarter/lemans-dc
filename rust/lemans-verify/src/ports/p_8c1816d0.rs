//! 0x8c1816d0 — clamped float interpolation writer.
//!
//! r7 = *r15; r6 = *(r4+12); if r6 == r5 store 0.0 twice at @r7 and return.
//! Otherwise load r0 = *(r4+24): if non-zero the same double zero store runs
//! (T=0). If zero, branch on signed r6 vs r5: the "greater" arm multiplies
//! float(r6-r5) by -0.25 (pool 0x8c181748), the "less/equal" arm multiplies
//! float(r5-r6) by 0.25 (pool 0x8c181750); each arm adds/subtracts 1.0 when
//! the small operand passes a `cmp/gt`/`cmp/ge` against 4, then stores fr1 at
//! @r7 and fr2 at @r7+4 (rts delay slot), leaving r7 += 4 and fpul = delta.
//! r0 tracks the last mova (pool address).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r15 = cpu.r[15];
    let mut r7 = mem.read_u32(r15).unwrap(); // mov.l @r15,r7
    cpu.r[7] = r7;
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let r6 = mem.read_u32(r4 + 12).unwrap(); // mov.l @(12,r4),r6
    cpu.r[6] = r6;
    cpu.set_t(r6 == r5); // cmp/eq r6,r5
    if r6 == r5 {
        // bf not taken: fldi0 fr0; fmov fr0,@r7; add #4,r7; rts; fmov fr0,@r7
        cpu.fr[0] = 0.0;
        let _ = mem.write_u32(r7, cpu.fr[0].to_bits());
        r7 = r7.wrapping_add(4);
        cpu.r[7] = r7;
        let _ = mem.write_u32(r7, cpu.fr[0].to_bits());
        return;
    }
    let r0 = mem.read_u32(r4 + 24).unwrap(); // mov.l @(24,r4),r0
    cpu.r[0] = r0;
    cpu.set_t(r0 == 0); // tst r0,r0
    if r0 != 0 {
        // bt not taken -> bra 0x8c18173c: fldi0 fr0; store 0 twice
        cpu.fr[0] = 0.0;
        let _ = mem.write_u32(r7, cpu.fr[0].to_bits());
        r7 = r7.wrapping_add(4);
        cpu.r[7] = r7;
        let _ = mem.write_u32(r7, cpu.fr[0].to_bits());
        return;
    }
    // r0 == 0 -> 0x8c1816ec
    if (r6 as i32) > (r5 as i32) {
        // cmp/gt r5,r6 true: bf.s not taken; delay slot fldi0 fr2
        cpu.set_t(true);
        cpu.fr[2] = 0.0;
        let r3 = r6.wrapping_sub(r5); // sub r5,r3
        cpu.r[3] = r3;
        cpu.r[1] = 4; // mov #4,r1
        cpu.r[0] = 0x8c181748; // mova 0x8c181748,r0
        cpu.fr[0] = f32::from_bits(mem.read_u32(0x8c181748).unwrap()); // fmov @r0,fr0
        cpu.fpul = r3; // lds r3,fpul
        cpu.set_t(4 > r5 as i32); // cmp/gt r5,r1
        cpu.fr[1] = cpu.fpul as i32 as f32; // float fpul,fr1
        cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // bf.s delay slot: fmul fr0,fr1
        if 4 > r5 as i32 {
            cpu.set_t(r6 as i32 >= 4); // cmp/ge r1,r6
            if r6 as i32 >= 4 {
                cpu.r[0] = 0x8c18174c; // mova 0x8c18174c,r0
                cpu.fr[2] = f32::from_bits(mem.read_u32(0x8c18174c).unwrap());
                cpu.fr[0] = 1.0; // fldi1 fr0
                cpu.fr[1] = cpu.fr[1] + cpu.fr[0]; // bra delay slot: fadd fr0,fr1
            }
        }
    } else {
        // bf.s taken to 0x8c181714; delay slot fldi0 fr2
        cpu.set_t(false); // cmp/gt r5,r6
        cpu.fr[2] = 0.0;
        let r3 = r5.wrapping_sub(r6); // sub r6,r3
        cpu.r[3] = r3;
        cpu.r[1] = 4;
        cpu.r[0] = 0x8c181750; // mova 0x8c181750,r0
        cpu.fr[0] = f32::from_bits(mem.read_u32(0x8c181750).unwrap());
        cpu.fpul = r3;
        cpu.set_t(4 > r6 as i32); // cmp/gt r6,r1
        cpu.fr[1] = cpu.fpul as i32 as f32;
        cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // bf.s delay slot: fmul fr0,fr1
        if 4 > r6 as i32 {
            cpu.set_t(r5 as i32 >= 4); // cmp/ge r1,r5
            if r5 as i32 >= 4 {
                cpu.r[0] = 0x8c181754; // mova 0x8c181754,r0
                cpu.fr[2] = f32::from_bits(mem.read_u32(0x8c181754).unwrap());
                cpu.fr[0] = 1.0;
                cpu.fr[1] = cpu.fr[1] - cpu.fr[0]; // fsub fr0,fr1
            }
        }
    }
    // 0x8c181734: fmov fr1,@r7; add #4,r7; rts; delay fmov fr2,@r7
    let _ = mem.write_u32(r7, cpu.fr[1].to_bits());
    r7 = r7.wrapping_add(4);
    cpu.r[7] = r7;
    let _ = mem.write_u32(r7, cpu.fr[2].to_bits());
}
