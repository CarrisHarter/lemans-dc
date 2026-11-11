//! 0x8c173a6c — ocbwb cache-hint loop over a 32-byte-aligned range.
//!
//! r2 = r4 & ~31, r4 = (r4 + r5 - 1) & ~31 (r4 itself is mutated). Then loop
//! { ocbwb @r2; cmp/hs r4,r2; } with r2 += 32 in every bt.s/bf.s delay slot
//! until r2 >=u r4. Cache ops have no architectural effect; leaked:
//! r1=0xffffffe0, r2, mutated r4, T=1. rts delay slot is a nop.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, _mem: &mut Memory) {
    let mask = 0xFFFF_FFE0u32;
    let mut r2 = cpu.r[4] & mask;
    let r4 = cpu.r[4].wrapping_add(cpu.r[5]).wrapping_sub(1) & mask;

    loop {
        // 8c173a78: ocbwb @r2 (no-op); cmp/hs r4,r2 ; bt.s exit (delay +32)
        let t = r2 >= r4;
        cpu.set_t(t);
        r2 = r2.wrapping_add(32);
        if t {
            break;
        }
        // 8c173a80: ocbwb @r2 (no-op); cmp/hs r4,r2 ; bf.s loop (delay +32)
        let t = r2 >= r4;
        cpu.set_t(t);
        r2 = r2.wrapping_add(32);
        if !t {
            continue;
        }
        break;
    }
    cpu.r[1] = mask;
    cpu.r[2] = r2;
    cpu.r[4] = r4;
}
