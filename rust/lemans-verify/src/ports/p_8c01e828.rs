//! 0x8c01e828 — decrement a counter word; return 0 unless it hits zero.
//!
//! `mov.l @(8,r4),r0 ; tst r0,r0 ; bf .skip ; tst r0,r0 ; bt .set1 ;
//!  bra .store ; nop ; .set1: mov #1,r0 ; .store: mov.l r0,@(8,r4) ;
//!  .skip: mov.l @(8,r4),r0 ; add #-1,r0 ; tst r0,r0 ; bf.s .zero ;
//!  mov.l r0,@(8,r4) ; bra .done ; nop ; .zero: mov #0,r4 ; .done: rts ;
//!  mov r4,r0`
//! Hidden side effects: the `bf.s` delay slot stores the decremented value
//! unconditionally (even when the branch is taken), T ends as the last
//! `tst r0,r0`, and r4 is zeroed exactly when the decremented value is
//! nonzero. r0 in the rts delay slot is the *possibly zeroed* r4.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let v = mem.read_u32(base + 8).unwrap_or(0);
    cpu.set_t(v == 0); // tst at 0x8c01e82a
    if v == 0 {
        // second tst at 0x8c01e82e also sets T=1; bt taken to `mov #1,r0`
        cpu.set_t(true);
        let _ = mem.write_u32(base + 8, 1); // 0x8c01e838
    }
    let mut r0 = mem.read_u32(base + 8).unwrap_or(0); // 0x8c01e83a
    r0 = r0.wrapping_sub(1); // add #-1,r0
    cpu.set_t(r0 == 0); // tst at 0x8c01e83e
    // delay slot of `bf.s` runs on BOTH paths
    let _ = mem.write_u32(base + 8, r0);
    if r0 != 0 {
        cpu.r[4] = 0; // 0x8c01e848
    }
    cpu.r[0] = cpu.r[4]; // delay slot of `rts`
}
