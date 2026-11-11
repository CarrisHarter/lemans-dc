//! 0x8c093070 — set flag byte at r4+0x60 (literal 96) to 1 when the global
//! byte at `*(0x8c2ab460) + 0x345` is non-zero OR the low byte of r5 is
//! non-zero; otherwise return without storing.
//!
//! Original:
//! ```text
//! mov.l 0x8c093090,r0 ; ! 8c2ab460   r0 = &global
//! mov.l @r0,r1                       r1 = global ptr P
//! mov.l 0x8c093094,r0 ; ! 0x345      r0 = 0x345
//! mov.b @(r0,r1),r0 ; extu.b x2      r0 = byte[P+0x345]
//! tst r0,r0 ; bf 0x8c093086          store if byte != 0
//! extu.b r5,r0 ; tst r0,r0 ; bt 0x8c09308c   skip store if r5&0xff == 0
//! mov.l 0x8c093098,r0 ; ! 96 ; mov #1,r1 ; mov.b r1,@(r0,r4) ; rts ; nop
//! ```
//! Side effects the C hides: on the no-store path r0 keeps `r5 & 0xff` and
//! r1 keeps P with T=1; on the store path r0=96, r1=1, T=0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let p = mem.read_u32(mem.read_u32(0x8c093090).unwrap_or(0)).unwrap_or(0); // *(0x8c2ab460)
    let off = mem.read_u32(0x8c093094).unwrap_or(0); // 0x345
    let b = mem.read_u8(p.wrapping_add(off)).unwrap_or(0) as u32; // mov.b + extu.b x2
    cpu.set_t(b == 0); // tst r0,r0
    if b == 0 {
        let r0 = cpu.r[5] & 0xFF; // extu.b r5,r0
        cpu.set_t(r0 == 0); // tst r0,r0
        if r0 == 0 {
            // bt 0x8c09308c taken -> rts ; nop
            cpu.r[0] = r0;
            cpu.r[1] = p;
            return;
        }
        // bt not taken -> fall into the store
    }
    cpu.r[0] = mem.read_u32(0x8c093098).unwrap_or(0); // 96
    cpu.r[1] = 1; // mov #1,r1
    let _ = mem.write_u8(cpu.r[0].wrapping_add(cpu.r[4]), 1); // mov.b r1,@(r0,r4)
    // rts ; nop — T stays 0
}
