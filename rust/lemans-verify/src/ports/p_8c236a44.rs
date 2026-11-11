//! 0x8c236a44 — hash_lookup.
//!
//! Bounds `r4` against `[0x8c23d7b0]` into a 16-bit word at `r5`, then walks the
//! chain at `0x8c23d7bc + 16*bucket` for a node whose `[+12]` equals `r4`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[2] = mem.read_u32(0x8c236b00).unwrap_or(0);
    cpu.r[3] = mem.read_u32(cpu.r[2]).unwrap_or(0);
    let hs = cpu.r[4] >= cpu.r[3];
    cpu.set_t(hs);
    cpu.r[6] = 0; // delay slot of `bt.s`
    if hs {
        cpu.r[1] = 1;
        let _ = mem.write_u16(cpu.r[5], cpu.r[1] as u16);
    } else {
        let _ = mem.write_u16(cpu.r[5], cpu.r[6] as u16);
    }
    cpu.r[3] = mem.read_u16(cpu.r[5]).unwrap_or(0) as i16 as i32 as u32;
    cpu.r[0] = mem.read_u32(0x8c236b04).unwrap_or(0);
    cpu.r[3] = cpu.r[3] << 2;
    cpu.r[3] = cpu.r[3] << 2;
    cpu.r[5] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
    let t = cpu.r[5] == 0;
    cpu.set_t(t);
    cpu.r[7] = cpu.r[6]; // delay slot of `bt.s`
    if !t {
        loop {
            cpu.r[1] = mem.read_u32(cpu.r[5] + 12).unwrap_or(0);
            let eq = cpu.r[4] == cpu.r[1];
            cpu.set_t(eq);
            if eq {
                cpu.r[7] = cpu.r[5];
                cpu.r[5] = cpu.r[6]; // delay slot of `bra`
                break;
            }
            cpu.r[5] = mem.read_u32(cpu.r[5] + 8).unwrap_or(0);
            let t2 = cpu.r[5] == 0;
            cpu.set_t(t2);
            if t2 {
                break;
            }
        }
    }
    cpu.r[0] = cpu.r[7]; // delay slot of `rts`
}
