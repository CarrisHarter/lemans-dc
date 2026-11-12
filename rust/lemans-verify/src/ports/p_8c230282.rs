//! 0x8c230282 - copy three float triples onto the stack, then build a packet.
//!
//! Three iterations (r4 = 0, 28, 56) walk the pointers stored at `sp+4`,
//! `sp+8` and `sp+12` — the entry `r5`, `r6` and `r7`. Each copy takes the
//! floats at +4/+8/+12 and the word at +24. 0x8c2395c0 then reads the block
//! at `sp+16`. The `mov.l @r5+,r2` advances to the next pointer.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(116);
    cpu.r[1] = cpu.r[15].wrapping_add(4);
    cpu.r[3] = cpu.r[1];
    let _ = mem.write_u32(cpu.r[15], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[3], cpu.r[5]);
    let _ = mem.write_u32(cpu.r[1].wrapping_add(4), cpu.r[6]);
    cpu.r[6] = 84;
    let _ = mem.write_u32(cpu.r[1].wrapping_add(8), cpu.r[7]);
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(16), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(20), cpu.r[2]);
    cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(24), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(12)).unwrap_or(0);
    cpu.r[4] = 0;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(28), cpu.r[2]);
    cpu.r[5] = mem.read_u32(cpu.r[15]).unwrap_or(0); // once, before the loop
    loop {
        cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.r[0] = 4;
        cpu.r[3] = cpu.r[15].wrapping_add(32);
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[2])).unwrap_or(0.0);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        let _ = mem.write_f32(cpu.r[3], cpu.fr[3]);
        cpu.r[3] = cpu.r[15].wrapping_add(32);
        cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        cpu.r[0] = 8;
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[2])).unwrap_or(0.0);
        cpu.r[0] = 4;
        let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.fr[3]);
        cpu.r[0] = 12;
        cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.r[3] = cpu.r[15].wrapping_add(32);
        cpu.fr[3] = mem.read_f32(cpu.r[0].wrapping_add(cpu.r[2])).unwrap_or(0.0);
        cpu.r[0] = 8;
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        let _ = mem.write_f32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.fr[3]);
        cpu.r[3] = cpu.r[15].wrapping_add(32);
        cpu.r[2] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.r[5] = cpu.r[5].wrapping_add(4); // mov.l @r5+,r2
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[4]);
        cpu.r[1] = mem.read_u32(cpu.r[2].wrapping_add(24)).unwrap_or(0);
        cpu.r[4] = cpu.r[4].wrapping_add(28);
        cpu.set_t(cpu.r[4] >= cpu.r[6]); // cmp/hs r6,r4
        let _ = mem.write_u32(cpu.r[3].wrapping_add(20), cpu.r[1]);
        if cpu.t() {
            break;
        }
    }
    cpu.r[2] = mem.read_u32(0x8c23030c).unwrap_or(0x8c2395c0);
    cpu.r[4] = cpu.r[15];
    cpu.r[4] = cpu.r[4].wrapping_add(16); // jsr delay
    cpu.pr = 0x8c2302f2;
    crate::registry::p_8c2395c0::run(cpu, mem);
    cpu.r[15] = cpu.r[15].wrapping_add(116);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
