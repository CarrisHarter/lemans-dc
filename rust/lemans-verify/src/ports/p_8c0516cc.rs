//! 0x8c0516cc — for i in 0..count: clear bit 8 of `base[idx] + 80*elem[i]`
//! and store `(float)r5 / const` as a float at `base[idx] + 80*elem[i] + 16`,
//! where elem[] is the list at `table2[idx]+4` and count is `*table2[idx]`.
//!
//! ABI: r4 = ptr to a struct whose dword at +4 is the index, r5 = value
//! loaded into fpul then converted with `float`.
//!
//! Side effects the decompiled C does not show: `lds r5,fpul` leaves fpul =
//! r5; the entry `bra`'s delay slot runs `float fpul,fr2` once, so fr2 =
//! (float)(i32)r5 (NOT the bit pattern as a float) and fr0 = fr2/fr1 (fr1 =
//! the 3.6f literal at 0x8c051750) on every iteration; r1 = base1 table,
//! r5 = 0xfffffeff mask, r3 = iteration count, r6 = 4*i, r7 = table2[idx]
//! and r0 = count are left visible on exit; T = 0 (the final `cmp/gt r3,r0`
//! failed); r12/r13/r14 are stack-restored. The C's FPSCR PR/SZ
//! double-precision paths do not exist in the machine code.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let fpul_in = cpu.r[5];

    cpu.fpul = fpul_in; // lds r5,fpul
    let base1 = mem.read_u32(0x8c051744).unwrap_or(0); // r1 = 0x8c1f6bd8
    let table2 = mem.read_u32(0x8c051748).unwrap_or(0); // r12 = 0x8c1f6be8
    let mask = mem.read_u32(0x8c05174c).unwrap_or(0); // r5 = 0xfffffeff
    let fconst = mem.read_f32(0x8c051750).unwrap_or(0.0); // fr1

    cpu.r[1] = base1;
    cpu.r[5] = mask;

    // delay slot of the entry `bra`: float fpul,fr2 (runs once)
    let fr2 = fpul_in as i32 as f32;
    cpu.fr[2] = fr2;

    let mut r3: u32 = 0; // i
    let mut r6: u32 = 0; // 4*i

    loop {
        // loop head 0x8c05172c
        let idx = mem.read_u32(r4.wrapping_add(4)).unwrap();
        let r14 = idx << 2; // shll2
        let r7 = mem.read_u32(table2.wrapping_add(r14)).unwrap();
        let count = mem.read_u32(r7).unwrap();
        cpu.r[7] = r7;
        cpu.r[0] = count;
        if !((count as i32) > (r3 as i32)) {
            break; // cmp/gt r3,r0 ; bt body — T = 0 on exit
        }

        // body 0x8c0516e2..
        let list = mem.read_u32(r7.wrapping_add(4)).unwrap(); // mov.l @(4,r7),r7
        cpu.fr[0] = fr2; // fmov fr2,fr0
        let e = mem.read_u32(r6.wrapping_add(list)).unwrap(); // mov.l @(r0,r7),r7 with r0=r6
        r3 = r3.wrapping_add(1);
        let b = mem.read_u32(base1.wrapping_add(r14)).unwrap(); // mov.l @(r0,r14),r0 with r0=r1
        // r13 = (((e<<2)+e)<<2)<<2 + b  == 80*e + b
        let r13 = (((e << 2).wrapping_add(e)) << 2 << 2).wrapping_add(b);
        let v = mem.read_u32(r13).unwrap() & mask; // and r5,r7
        mem.write_u32(r13, v).unwrap();

        // second half: same element again via r6 (not yet incremented)
        let r14b = idx << 2; // mov.l @(4,r4),r14 ; shll2 r14
        let cnt_struct = mem.read_u32(table2.wrapping_add(r14b)).unwrap(); // mov.l @(r0,r14),r7 with r0=r12
        let list2 = mem.read_u32(cnt_struct.wrapping_add(4)).unwrap(); // mov.l @(4,r7),r7
        let e2 = mem.read_u32(r6.wrapping_add(list2)).unwrap(); // mov.l @(r0,r7),r13 with r0=r6
        r6 = r6.wrapping_add(4); // add #4,r6
        let b2 = mem.read_u32(base1.wrapping_add(r14b)).unwrap(); // mov.l @(r0,r14),r7 with r0=r1
        // r14 = (((e2<<2)+e2)<<2)<<2 + (b2+16) == 80*e2 + b2 + 16
        let r14out = (((e2 << 2).wrapping_add(e2)) << 2 << 2).wrapping_add(b2.wrapping_add(16));
        cpu.fr[1] = fconst; // fmov @r0,fr1 (mova 0x8c051750,r0)
        cpu.fr[0] = cpu.fr[0] / fconst; // fdiv fr1,fr0
        mem.write_f32(r14out, cpu.fr[0]).unwrap();
    }

    cpu.r[3] = r3;
    cpu.r[6] = r6;
    cpu.set_t(false);
    // r12/r13/r14 popped from the stack: net unchanged.
}
