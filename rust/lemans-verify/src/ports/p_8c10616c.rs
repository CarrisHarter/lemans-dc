//! 0x8c10616c — release six embedded handles, then free the container if asked.
//!
//! Null r4 exits at once (`tst` leaves T=1). Otherwise six `jsr` calls to the
//! release helper at 0x8c018bd8 with `r4 = object + {48,44,28,24,8,4}` and
//! `mov #-1,r5` in each delay slot, in that order. Then `exts.w r5` (the caller's
//! r5) gates the final free of the container through 0x8c01032c. `r0 = r13` at
//! exit; r13/r14 are pushed and end at their entry values.

use sh4_oracle::{Cpu, Memory};

const RELEASE_RETURNS: [u32; 6] = [
    0x8c106182, 0x8c10618c, 0x8c106196, 0x8c1061a0, 0x8c1061aa, 0x8c1061b4,
];
const OFFSETS: [u32; 6] = [48, 44, 28, 24, 8, 4];

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let object = cpu.r[4]; // mov r4,r13
    let count_reg = cpu.r[5]; // mov r5,r14
    cpu.set_t(object == 0); // tst r13,r13
    if object != 0 {
        for i in 0..6 {
            cpu.r[0] = mem.read_u32(0x8c1061cc).unwrap_or(0x8c018bd8);
            cpu.r[4] = object.wrapping_add(OFFSETS[i]);
            cpu.r[5] = u32::MAX; // delay slot: mov #-1,r5
            cpu.pr = RELEASE_RETURNS[i];
            crate::registry::p_8c018bd8::run(cpu, mem);
        }

        let count = (count_reg as u16 as i16) as i32; // exts.w r14,r0
        cpu.r[0] = count as u32;
        cpu.set_t(count > 0); // cmp/pl r0
        if count > 0 {
            cpu.r[0] = mem.read_u32(0x8c1061d0).unwrap_or(0x8c01032c);
            cpu.r[4] = object; // delay slot: mov r13,r4
            cpu.pr = 0x8c1061c0;
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = object; // mov r13,r0
}
