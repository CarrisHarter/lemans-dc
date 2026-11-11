//! 0x8c21c902 — init a 44-byte struct at r4 from a global flag.
//!
//! r2 = 0x8c2bab84, r5 = 0; *r4 = *(r4+4) = 0. r3 = *0x8c2bab84; tst r3,r3.
//! bf.s (delay slot mov #-1,r6 runs always). If r3 == 0: *(r4+8)=0,
//! *(r4+12)=-1, *(r4+16)=0, bra delay *(r4+20)=0. Else: *(r4+8)=-1,
//! *(r4+12)=0, *(r4+16)=-1, *(r4+20)=-1. Then zero the u16s at r4+24..30,
//! +32, +34, the u32 at r4+40, and the rts delay slot stores r3' = 0x8c23c514
//! (literal) at r4+36. Final r0 = 0, T = (r3 == 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    cpu.r[2] = 0x8c2bab84; // mov.l literal,r2
    let r5: u32 = 0; // mov #0,r5
    mem.write_u32(r4, r5).unwrap(); // mov.l r5,@r4
    mem.write_u32(r4 + 4, r5).unwrap(); // mov.l r5,@(4,r4)
    let r3 = mem.read_u32(0x8c2bab84).unwrap_or(0); // mov.l @r2,r3
    let t = r3 == 0; // tst r3,r3
    cpu.set_t(t);
    let r6: u32 = 0xffffffff; // bf.s delay slot: mov #-1,r6
    if t {
        mem.write_u32(r4 + 8, r5).unwrap();
        mem.write_u32(r4 + 12, r6).unwrap();
        mem.write_u32(r4 + 16, r5).unwrap();
        mem.write_u32(r4 + 20, r5).unwrap(); // bra delay slot
    } else {
        mem.write_u32(r4 + 8, r6).unwrap();
        mem.write_u32(r4 + 12, r5).unwrap();
        mem.write_u32(r4 + 16, r6).unwrap();
        mem.write_u32(r4 + 20, r6).unwrap();
    }
    let r0 = r5; // mov r5,r0
    mem.write_u16(r4 + 24, r0 as u16).unwrap();
    mem.write_u16(r4 + 26, r0 as u16).unwrap();
    mem.write_u16(r4 + 34, r5 as u16).unwrap(); // mov #34,r0; mov.w r5,@(r0,r4)
    mem.write_u16(r4 + 32, r5 as u16).unwrap(); // mov #32,r0; mov.w r5,@(r0,r4)
    let r3n: u32 = 0x8c23c514; // mov.l literal,r3
    mem.write_u16(r4 + 30, r0 as u16).unwrap();
    mem.write_u16(r4 + 28, r0 as u16).unwrap();
    mem.write_u32(r4 + 40, r0).unwrap();
    mem.write_u32(r4 + 36, r3n).unwrap(); // rts delay slot: mov.l r3,@(36,r4)
    cpu.r[0] = 0;
    cpu.r[2] = 0x8c2bab84;
    cpu.r[3] = r3n;
    cpu.r[5] = r5;
    cpu.r[6] = r6;
}
