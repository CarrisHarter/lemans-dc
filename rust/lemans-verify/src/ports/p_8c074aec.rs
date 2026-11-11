//! 0x8c074aec — record setter: `[rec+56] = r5; [rec+60] = 0` (byte).
//!
//! idx comes from r6 (shll2 r6); table at base+0xe8, or base+0xf8 when
//! [base+8]==3. idx==5 stores nothing. Side effects the C hides: r6 = r6<<2,
//! r4 = base+16, r1 = 0, r3 = rec, r0 = 60, exit T = 0 (store path) / 1 (idx5).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let base = cpu.r[4];
    let shifted = cpu.r[6] << 2; // shll2 r6
    cpu.r[6] = shifted;
    let mut r3 = mem.read_u32(0x8c074b24).unwrap_or(0); // 0xe8
    let mut idx = mem
        .read_u32(base.wrapping_add(r3).wrapping_add(shifted))
        .unwrap_or(0);
    if mem.read_u32(base.wrapping_add(8)).unwrap_or(0) == 3 {
        r3 = mem.read_u32(0x8c074b28).unwrap_or(0); // 0xf8
        idx = mem
            .read_u32(base.wrapping_add(r3).wrapping_add(shifted))
            .unwrap_or(0);
    }
    cpu.r[3] = r3;
    cpu.r[1] = idx;
    cpu.r[0] = idx; // mov r1,r0
    if idx == 5 {
        cpu.set_t(true); // cmp/eq #5,r0 ; bt
        return;
    }
    let rec = mem
        .read_u32(base.wrapping_add(16).wrapping_add(idx << 2))
        .unwrap_or(0);
    cpu.r[4] = base.wrapping_add(16); // add #16,r4
    cpu.r[3] = rec;
    cpu.r[1] = 0; // mov #0,r1
    let _ = mem.write_u32(rec.wrapping_add(56), cpu.r[5]); // mov.l r5,@(56,r3)
    cpu.r[0] = 60; // mov #60,r0
    let _ = mem.write_u8(rec.wrapping_add(60), cpu.r[1] as u8); // mov.b r1,@(r0,r3)
    cpu.set_t(false);
}
