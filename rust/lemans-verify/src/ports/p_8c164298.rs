//! 0x8c164298 — classify mem[0xb0 + mem[mem[0x8c23dbf8]]] into *r4 / *(r4+4).
//!
//! v == 2 -> *r4 = 2, *(r4+4) = 1, return 1 (T=1)
//! v == 1 -> *r4 = 1, *(r4+4) = 2, return 2 (T=1)
//! v == 0 -> *r4 = 0, *(r4+4) = 3, return 3 (T=1)
//! else   -> *r4 = 2, *(r4+4) = 0, return 0 (T=0, from tst)
//! The *(r4+4) store is the rts delay slot in every path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let g = mem.read_u32(0x8c1642d4).unwrap_or(0); // 0x8c23dbf8
    let idx = mem.read_u32(g).unwrap_or(0);
    let off = mem.read_u32(0x8c1642d8).unwrap_or(0); // 0xb0
    let v = mem.read_u32(off.wrapping_add(idx)).unwrap_or(0);
    let r4 = cpu.r[4];
    let (a, b, t) = if v == 2 {
        (2u32, 1u32, true)
    } else if v == 1 {
        (1, 2, true)
    } else if v == 0 {
        (0, 3, true)
    } else {
        (2, 0, false) // tst r0,r0 -> T = 0
    };
    cpu.r[1] = idx; // mov.l @r0,r1
    cpu.r[0] = a;
    let _ = mem.write_u32(r4, a); // mov.l r0,@r4
    cpu.r[0] = b;
    let _ = mem.write_u32(r4.wrapping_add(4), b); // rts delay: mov.l r0,@(4,r4)
    cpu.set_t(t);
}
