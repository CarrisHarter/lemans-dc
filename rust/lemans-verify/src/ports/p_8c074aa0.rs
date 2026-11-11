//! 0x8c074aa0 — flag test: `(*(u32*)(*(base+8)+8 of rec) & 0x4000) != 0`.
//!
//! idx = [base + (0xf8 if [base+8]==3 else 0xe8) + r5*4]; if idx==5 return 0;
//! else rec = [base+16+idx*4] and the flags live at [[rec+8]+8].
//! Side effects the C hides: r5 = r5<<2, r4 = base+16, r6 = 0xf8 or 0xe8,
//! r3 = final flags word, r1 = result via `subc r1,r1 ; add #1,r1`, and the
//! exit T = (flags & 0x4000) == 0 (tst), or T=1 on the idx==5 path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let shifted = cpu.r[5] << 2; // shll2 r5
    cpu.r[5] = shifted;
    let mut r6 = mem.read_u32(0x8c074ae0).unwrap_or(0); // 0xe8
    cpu.r[6] = r6;
    let mut idx = mem
        .read_u32(base.wrapping_add(r6).wrapping_add(shifted))
        .unwrap_or(0);
    if mem.read_u32(base.wrapping_add(8)).unwrap_or(0) == 3 {
        r6 = mem.read_u32(0x8c074ae4).unwrap_or(0); // 0xf8
        cpu.r[6] = r6;
        idx = mem
            .read_u32(base.wrapping_add(r6).wrapping_add(shifted))
            .unwrap_or(0);
    }
    cpu.r[3] = idx;
    cpu.r[0] = idx; // mov r3,r0
    if idx == 5 {
        cpu.r[0] = 0; // rts delay slot: mov #0,r0
        cpu.set_t(true); // cmp/eq #5 taken
        return;
    }
    let rec = mem
        .read_u32(base.wrapping_add(16).wrapping_add(idx << 2))
        .unwrap_or(0);
    cpu.r[4] = base.wrapping_add(16); // add #16,r4
    let flags = mem
        .read_u32(
            mem.read_u32(rec.wrapping_add(8)).unwrap_or(0)
                .wrapping_add(8),
        )
        .unwrap_or(0);
    cpu.r[3] = flags; // mov.l @(8,r3),r3 twice
    let mask = mem.read_u32(0x8c074ae8).unwrap_or(0); // 0x4000
    cpu.r[0] = mask;
    let t = (flags & mask) == 0; // tst r0,r3
    cpu.set_t(t);
    let r1: u32 = if t { 0xffff_ffff } else { 0 }; // subc r1,r1
    cpu.r[1] = r1.wrapping_add(1); // add #1,r1
    cpu.r[0] = cpu.r[1]; // rts delay slot: mov r1,r0
}
