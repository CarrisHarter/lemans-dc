//! 0x8c05d274 — set/clear bits of `*(u32*)(param_1 + 0x10)` by mask `param_2`:
//! OR when `param_3` (r6) is nonzero, AND-NOT otherwise.
//!
//! ```text
//! extu.b r6,r0            ; only the low byte of r6 decides
//! tst    r0,r0
//! bt     0x8c05d282       ; immediate
//! mov.l  @(16,r4),r0
//! or     r5,r0
//! rts
//! mov.l  r0,@(16,r4)      ; delay slot: store OR result
//! 0x8c05d282:
//! mov.l  @(16,r4),r1
//! not    r5,r0            ; r0 = ~mask
//! and    r0,r1
//! rts
//! mov.l  r1,@(16,r4)      ; delay slot: store AND result
//! ```
//! Hidden side effects: `extu.b` means r6's upper bytes are ignored; on the
//! set path r0 ends as the OR result, on the clear path r0 ends as `~mask` and
//! r1 as the cleared value; both stores happen in the rts delay slot; T is 0
//! on set, 1 on clear.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let r5 = cpu.r[5];
    let flag = cpu.r[6] & 0xFF; // extu.b r6,r0
    cpu.r[0] = flag;
    if flag != 0 {
        cpu.set_t(false); // tst r0,r0, r0 != 0
        let v = mem.read_u32(r4.wrapping_add(16)).unwrap_or(0);
        cpu.r[0] = v | r5; // or r5,r0
        let _ = mem.write_u32(r4.wrapping_add(16), cpu.r[0]); // rts delay slot
    } else {
        cpu.set_t(true); // tst r0,r0, r0 == 0 -> bt taken
        let mut r1 = mem.read_u32(r4.wrapping_add(16)).unwrap_or(0);
        cpu.r[0] = !r5; // not r5,r0
        r1 &= cpu.r[0]; // and r0,r1
        cpu.r[1] = r1;
        let _ = mem.write_u32(r4.wrapping_add(16), r1); // rts delay slot
    }
}
