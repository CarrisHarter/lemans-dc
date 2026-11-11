//! 0x8c07496c — initialise a 0x3d-byte record at r4 from (r5, r6).
//!
//! Store order matters (the verifier diffs the write log): [r4+4] gets the
//! *incoming* r5 before r5 is zeroed, then +8,+12,+16,+17,+20,+28(=r6),
//! +40,+44,+48, then `fmov fr0,@r4` writes 1.0f at +0, then six bytes copied
//! from the pointer at 0x8c0749b8 land at +32..+37, then +52,+56,+60.
//! Register side effects the C hides: r1 = 0x8c1f82e8, r3 = r4+32, r5 = 0,
//! r0 = r4 (rts delay slot `mov r4,r0`). fr0 = 1.0 (fldi1). T untouched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let p2 = cpu.r[5];
    let p3 = cpu.r[6];
    let z = 0u32;

    let _ = mem.write_u32(r4.wrapping_add(4), p2); // mov.l r5,@(4,r4)
    cpu.r[5] = 0; // mov #0,r5
    let _ = mem.write_u32(r4.wrapping_add(8), z);
    let _ = mem.write_u32(r4.wrapping_add(12), z);
    cpu.fr[0] = 1.0; // fldi1 fr0
    let _ = mem.write_u8(r4.wrapping_add(16), 0);
    let r3 = r4.wrapping_add(32); // add #32,r3 (r3 was r4)
    let _ = mem.write_u8(r4.wrapping_add(17), 0);
    let ptr = mem.read_u32(0x8c0749b8).unwrap_or(0); // -> 0x8c1f82e8
    cpu.r[1] = ptr;
    let _ = mem.write_u32(r4.wrapping_add(20), z);
    let _ = mem.write_u32(r4.wrapping_add(28), p3); // mov.l r6,@(28,r4)
    let _ = mem.write_u32(r4.wrapping_add(40), z);
    let _ = mem.write_u32(r4.wrapping_add(44), z);
    let _ = mem.write_u32(r4.wrapping_add(48), z);
    let _ = mem.write_u32(r4, cpu.fr[0].to_bits()); // fmov fr0,@r4
    for i in 0..6u32 {
        let v = mem.read_u8(ptr.wrapping_add(i)).unwrap_or(0);
        let _ = mem.write_u8(r3.wrapping_add(i), v);
    }
    let _ = mem.write_u8(r4.wrapping_add(52), 0);
    let _ = mem.write_u32(r4.wrapping_add(56), z);
    let _ = mem.write_u8(r4.wrapping_add(60), 0);

    cpu.r[3] = r3;
    cpu.r[0] = r4; // rts delay slot: mov r4,r0
}
