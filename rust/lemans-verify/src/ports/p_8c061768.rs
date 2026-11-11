//! 0x8c061768 — store `fr4` at `0x8c1f7a18 + (r4 << 2)`.
//!
//! Original: `mov.l 0x8c061774,r0 ; shll2 r4 ; add r0,r4 ; rts ; fmov fr4,@r4`
//! Side effects the C hides: r0 keeps the table base 0x8c1f7a18, r4 becomes the
//! computed destination, and the store is the `rts` delay slot. `shll2`/`add`
//! leave T alone.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[0] = mem.read_u32(0x8c061774).unwrap(); // 0x8c1f7a18
    cpu.r[4] = cpu.r[4] << 2; // shll2 r4
    cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // add r0,r4
    let _ = mem.write_f32(cpu.r[4], cpu.fr[4]); // delay slot of rts
}
