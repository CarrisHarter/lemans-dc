//! 0x8c018fec — `*g_slot = r4; return r4;`
//!
//! The slot pointer is read from the literal cell 0x8c018ff4 at runtime
//! (0x8c23da98 in the ROM, redirected by the spec). Hidden side effect:
//! the `rts` delay slot is `mov r4,r0`, so r0 ends as the stored value
//! even though r0 was loaded with the pointer first; T is never touched.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let ptr = mem.read_u32(0x8c018ff4).unwrap_or(0); // mov.l @0x8c018ff4,r0
    let _ = mem.write_u32(ptr, cpu.r[4]); // mov.l r4,@r0
    cpu.r[0] = cpu.r[4]; // rts delay slot: mov r4,r0
}
