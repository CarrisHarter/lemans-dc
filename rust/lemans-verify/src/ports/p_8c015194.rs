//! 0x8c015194 — shift each of r5 bytes at r4 right by r6 bits, carrying the
//! low bits into the next byte; r0 = 1 if the final carry byte is nonzero.
//!
//! Original side effects: r7 = 8-r6 then 0 then carry, r13 = exts.w(8-r6),
//! r12 = 0xff shad -r13 mask, r3 = -exts.w(r6), r2/r1 = last byte ops,
//! r14 = count, r4 advanced, T = (final r7 == 0); r12/r13/r14 pushed and
//! popped, net unchanged.

use sh4_oracle::{Cpu, Memory};

fn shld(amt: u32, v: u32) -> u32 {
    let a = amt as i32;
    if a >= 0 {
        v << a
    } else {
        v >> (-a)
    }
}

fn shad(amt: u32, v: u32) -> u32 {
    let a = amt as i32;
    if a >= 0 {
        v << a
    } else {
        ((v as i32) >> (-a)) as u32
    }
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r13 = ((8u32.wrapping_sub(cpu.r[6])) as u16 as i16) as u32; // exts.w
    cpu.set_t(r13 == 0); // tst r13,r13
    if r13 == 0 {
        cpu.r[7] = 8u32.wrapping_sub(cpu.r[6]);
        return; // r0 keeps its incoming value
    }
    let r12 = shad(r13.wrapping_neg(), 0xff);
    cpu.r[3] = r13.wrapping_neg(); // 0x8c0151a6: neg r13,r3 — survives the skip
    cpu.set_t((cpu.r[5] as i32) > 0); // cmp/pl r5
    cpu.r[7] = 0;
    if (cpu.r[5] as i32) > 0 {
        let mut r14 = 0u32;
        loop {
            let b = mem.read_u8(cpu.r[4]).unwrap_or(0);
            cpu.r[2] = b as u32; // mov.b @r4,r2
            cpu.r[3] = ((cpu.r[6] as u16 as i16) as i32).wrapping_neg() as u32;
            cpu.r[0] = b as u32; // mov.b @r4,r0
            cpu.r[2] = shld(cpu.r[3], cpu.r[2]); // extu.b then shld
            cpu.r[0] &= r12; // and r12,r0
            r14 = r14.wrapping_add(1);
            cpu.set_t(r14 >= cpu.r[5]); // cmp/ge r5,r14
            let _ = mem.write_u8(cpu.r[4], cpu.r[2] as u8);
            let mut r1 = mem.read_u8(cpu.r[4]).unwrap_or(0) as u32;
            r1 |= cpu.r[7];
            cpu.r[1] = r1; // or r7,r1
            cpu.r[7] = cpu.r[0] & 0xff; // extu.b r0,r7
            let _ = mem.write_u8(cpu.r[4], r1 as u8);
            cpu.r[7] = shld(r13, cpu.r[7]);
            cpu.r[4] = cpu.r[4].wrapping_add(1); // bf.s delay
            if r14 >= cpu.r[5] {
                break;
            }
        }
    }
    cpu.r[7] &= 0xff; // extu.b r7,r7
    cpu.set_t(cpu.r[7] == 0); // tst r7,r7
    cpu.r[0] = if cpu.r[7] == 0 { 0 } else { 1 };
}
