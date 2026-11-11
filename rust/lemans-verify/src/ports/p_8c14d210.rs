//! 0x8c14d210 — round `*(r4+8)` up past `*(r4+8) + 4**(r4+4)` in steps of 4,
//! then zero `*(r4+4)`.
//!
//! Original:
//! ```text
//! mov.l @(8,r4),r1 ; mov.l @(4,r4),r0 ; mov r1,r3 ; shll2 r0 ; add r0,r3
//! cmp/hi r1,r3 ; bf 0x8c14d224        <- plain bf: NO delay slot
//! add #4,r1 ; cmp/hi r1,r3 ; bt 0x8c14d21e   <- plain bt: NO delay slot
//! 0x8c14d224: mov #0,r0 ; rts ; mov.l r0,@(4,r4)   <- rts DOES delay
//! ```
//! So `add #4,r1` at 0x8c14d21e runs only via fall-through or as the bt target:
//! the whole thing is `while r3 >u r1 { r1 += 4 }`. The rts delay slot stores
//! r0=0 to `*(r4+4)`; the last cmp/hi leaves T=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r1 = mem.read_u32(r4 + 8).unwrap_or(0);
    let r0 = mem.read_u32(r4 + 4).unwrap_or(0);
    let r3 = r1.wrapping_add(r0 << 2); // shll2 r0 ; add r0,r3
    let mut r1 = r1;
    while r3 > r1 {
        // cmp/hi r1,r3 (T = r3 > r1) ; bt 0x8c14d21e / bf 0x8c14d224
        r1 = r1.wrapping_add(4); // 0x8c14d21e: add #4,r1
    }
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.r[0] = 0; // 0x8c14d224: mov #0,r0
    let _ = mem.write_u32(r4 + 4, cpu.r[0]); // rts delay slot: mov.l r0,@(4,r4)
    cpu.set_t(false); // exit cmp/hi leaves T=0 on both paths
}
