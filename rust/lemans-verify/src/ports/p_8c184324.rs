//! 0x8c184324 — bounds-checked copy of an 8-byte record.
//!
//! `r4` is a state struct: `[r4+0x138]` holds the record count, the records
//! start at `r4+0x13c` with a stride of 8 bytes, and `r5` receives the copy.
//! When `r6` is in range the record is copied; when `r6 >= count` the two words
//! at `r5` are zeroed by memset (0x8c010f88). The compare is signed (`cmp/ge`),
//! so a negative index goes down the copy path.
//!
//! Register side effects the C hides: in the copy path r4 ends as
//! `r4 + 0x13c + 8*r6`, r6 as `8*r6`, r1 and r0 as the two loaded words; in the
//! zero path memset leaves r4 = the destination pointer, r5 = 0, r6 = 8,
//! r7 = 8, r3 = 0, r0 = the destination and T set. In the copy path `shll2`
//! leaves T alone but the following `shll` sets it to the bit it shifted out,
//! so T ends as bit 29 of the original index (`r6 << 2`'s top bit), not as the
//! bounds compare.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // sts.l pr,@-r15
    let _ = mem.write_u32(cpu.r[15], cpu.pr);

    cpu.r[0] = 0x138; // mov.l 0x8c184354,r0
    cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0); // mov.l @(r0,r4),r0
    let in_range = (cpu.r[6] as i32) >= (cpu.r[0] as i32); // cmp/ge r0,r6
    cpu.set_t(in_range);

    if in_range {
        cpu.r[0] = 0x8c010f88; // mov.l 0x8c184358,r0
        cpu.r[4] = cpu.r[5]; // mov r5,r4 (destination)
        cpu.r[6] = 8; // mov #8,r6
        cpu.r[5] = 0; // delay slot: mov #0,r5
        cpu.pr = 0x8c184338;
        crate::registry::p_8c010f88::run(cpu, mem); // memset(r4, 0, 8)
    } else {
        cpu.r[0] = 0x13c; // mov.l 0x8c18435c,r0
        cpu.r[6] = cpu.r[6].wrapping_shl(2); // shll2 r6 (T unchanged)
        cpu.set_t(cpu.r[6] >> 31 != 0); // shll r6 shifts bit 31 out into T
        cpu.r[6] = cpu.r[6].wrapping_shl(1); // shll r6
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // add r0,r4
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[6]); // add r6,r4
        cpu.r[1] = mem.read_u32(cpu.r[4]).unwrap_or(0); // mov.l @r4,r1
        cpu.r[0] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0); // mov.l @(4,r4),r0
        let _ = mem.write_u32(cpu.r[5], cpu.r[1]); // mov.l r1,@r5
        let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r5)
    }

    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0); // lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
