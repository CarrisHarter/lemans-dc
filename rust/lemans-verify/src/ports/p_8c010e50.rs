//! 0x8c010e50 — memcmp-style byte compare returning the last-byte difference.
//!
//! ```text
//! tst   r6,r6
//! bf.s  0x8c010e5a
//! mov   r5,r7          ; delay slot: r7 = r5 on BOTH paths
//! rts
//! mov   #0,r0          ; delay slot: r0 = 0 when r6 == 0
//! mov   #0,r5
//! mov   r5,r2
//! cmp/hs r6,r2
//! bt    0x8c010e70
//! loop: mov.b @r7+,r2 ; mov.b @r4+,r3 ; cmp/eq r2,r3 ; bf done
//!       add #1,r5 ; cmp/hs r6,r5 ; bf loop
//! done: mov r4,r0 ; add #-1,r7 ; add #-1,r0
//!       mov.b @r7,r3 ; mov.b @r0,r0 ; extu.b r3,r3 ; extu.b r0,r0
//!       sub r3,r0 ; rts ; nop
//! ```
//! Hidden side effects: r7 is always initialized from r5 (delay slot of
//! `bf.s` runs even when not taken); r4/r7 end advanced by the number of
//! loop iterations; r2/r3 hold the last values loaded by the (oracle-u32)
//! post-increment loads, with r3 re-truncated by `extu.b` at the end; r5 is
//! the match count; T is 1 only when the loop ran to r6 matches (or r6 == 0),
//! else 0 from the mismatching cmp/eq.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let n = cpu.r[6];
    cpu.r[7] = cpu.r[5]; // delay slot of `bf.s`, both paths
    cpu.set_t(n == 0); // tst r6,r6
    if n == 0 {
        cpu.r[0] = 0; // delay slot of `rts`
        return;
    }

    cpu.r[5] = 0; // mov #0,r5
    loop {
        // The oracle's `mov.b @Rm+,Rn` (0x6 family, low nibble 4/5/6) passes the
        // raw nibble as load size, and its load() treats any size other than
        // 0/1 as a 32-bit read — so both post-increment loads below fetch a
        // full u32 and advance the pointer by 1, not by the element size.
        cpu.r[2] = mem.read_u32(cpu.r[7]).unwrap(); // mov.b @r7+,r2
        cpu.r[7] = cpu.r[7].wrapping_add(1);
        cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap(); // mov.b @r4+,r3
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[2] == cpu.r[3]); // cmp/eq r2,r3
        if cpu.r[2] != cpu.r[3] {
            break; // bf 0x8c010e70
        }
        cpu.r[5] = cpu.r[5].wrapping_add(1); // add #1,r5
        cpu.set_t(cpu.r[5] >= n); // cmp/hs r6,r5
        if cpu.r[5] >= n {
            break; // bf not taken -> fall through to 0x8c010e70
        }
    }

    let mut r0 = cpu.r[4]; // mov r4,r0
    cpu.r[7] = cpu.r[7].wrapping_sub(1); // add #-1,r7
    r0 = r0.wrapping_sub(1); // add #-1,r0
    cpu.r[3] = mem.read_u8(cpu.r[7]).unwrap() as u32; // mov.b @r7,r3 ; extu.b r3,r3
    let last = mem.read_u8(r0).unwrap() as u32; // mov.b @r0,r0 ; extu.b r0,r0
    cpu.r[0] = last.wrapping_sub(cpu.r[3]); // sub r3,r0
}
