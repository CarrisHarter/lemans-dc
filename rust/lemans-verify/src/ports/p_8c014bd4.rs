//! 0x8c014bd4 — rotate a little-endian byte array left through carry,
//! masked with 0x0080, walking from the last byte down to `r4`.
//!
//! Hidden side effects the C does not show: the delay slot of the entry
//! `bf.s` leaves `r6 == r5` (end pointer), each loop-back delay slot
//! decrements `r6` in lockstep with `r5`, `r1` keeps `byte & 0x80`,
//! `r3` keeps the sign-extended byte shifted left (32-bit), `r7` keeps the
//! last masked bit and becomes the return value, and `r14` is saved and
//! restored so the 0x0080 mask load never survives.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let mask: u32 = 0x0080; // mov.w @0x8c014c18,r14 (saved/restored around it)
    let mut r5 = cpu.r[5].wrapping_sub(1);
    let mut r6 = r5;
    r5 = r5.wrapping_add(base);
    let mut r7: u32 = 0;
    let mut r1 = cpu.r[1];
    let mut r3 = cpu.r[3];
    let mut r0 = cpu.r[0];
    // cmp/hs r4,r5 ; bf.s (delay slot: r6 = r5, always executed)
    let enter = r5 >= base;
    r6 = r5;
    if enter {
        loop {
            let b = mem.read_u8(r5).unwrap_or(0);
            r1 = b as u32 & mask; // extu.b r1,r1 ; and r14,r1
            r3 = ((b as i8 as i32) << 1) as u32; // mov.b sign-extends, shll r3
            let carry_in = r7 != 0; // tst r7,r7
            // bt.s delay slot always stores the shifted byte
            let _ = mem.write_u8(r5, r3 as u8);
            if carry_in {
                r0 = ((mem.read_u8(r6).unwrap_or(0) as i8 as i32) as u32) | 1;
                let _ = mem.write_u8(r6, r0 as u8);
            }
            r5 = r5.wrapping_sub(1);
            r7 = r1;
            let cont = r5 >= base; // cmp/hs r4,r5
            r6 = r6.wrapping_sub(1); // delay slot of bt.s
            if !cont {
                break;
            }
        }
    }
    cpu.set_t(r7 == 0); // tst r7,r7 before the return-value branch
    cpu.r[0] = if r7 != 0 { 1 } else { 0 };
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
    cpu.r[7] = r7;
}
