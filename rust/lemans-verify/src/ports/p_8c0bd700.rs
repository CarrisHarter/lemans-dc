//! 0x8c0bd700 — add two 11-word vectors, with element 5 taking the max.
//!
//! ```text
//! for off in 0,4,8,12,16:  r1 = a[off] ; r0 = b[off] ; r1 += r0 ; a[off] = r1
//! r1 = b[20] ; r0 = a[20] ; cmp/gt r0,r1 (T = b[20] > a[20])
//! bf 0x8c0bd734 ; bra 0x8c0bd736 ; nop
//! 0x8c0bd734: mov r0,r1        ; else r1 = a[20]
//! 0x8c0bd736: mov.l r1,@(20,r4)
//! mov r4,r0                    ; r0 = a (kept for the rest)
//! for off in 24..44 step 4: r3 = a[off] ; r1 = b[off] ; r3 += r1 ; a[off] = r3
//! rts ; mov.l r3,@(44,r4)      ; the last store is the rts delay slot
//! ```
//! Hidden side effects: r0 ends as the `a` pointer, r1 as b[44], r3 as the
//! final sum, and T stays at the `cmp/gt` from the max step.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let a = cpu.r[4];
    let b = cpu.r[5];
    let mut r0: u32;
    let mut r1: u32;
    for off in [0u32, 4, 8, 12, 16] {
        r1 = mem.read_u32(a.wrapping_add(off)).unwrap();
        r0 = mem.read_u32(b.wrapping_add(off)).unwrap();
        r1 = r1.wrapping_add(r0);
        let _ = mem.write_u32(a.wrapping_add(off), r1);
    }
    // element 5: max(a[5], b[5])
    r1 = mem.read_u32(b.wrapping_add(20)).unwrap();
    r0 = mem.read_u32(a.wrapping_add(20)).unwrap();
    let t = (r1 as i32) > (r0 as i32); // cmp/gt r0,r1
    if !t {
        r1 = r0; // mov r0,r1
    }
    let _ = mem.write_u32(a.wrapping_add(20), r1);
    cpu.r[0] = a; // mov r4,r0
    r0 = a;
    let mut r3: u32 = 0;
    for off in [24u32, 28, 32, 36, 40, 44] {
        r3 = mem.read_u32(a.wrapping_add(off)).unwrap();
        r1 = mem.read_u32(b.wrapping_add(off)).unwrap();
        r3 = r3.wrapping_add(r1);
        let _ = mem.write_u32(a.wrapping_add(off), r3); // off 44 = rts delay slot
    }
    cpu.r[0] = r0;
    cpu.r[1] = r1;
    cpu.r[3] = r3;
    cpu.set_t(t);
}
