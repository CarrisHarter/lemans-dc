//! 0x8c027500 — copy 4 u16 + 1 u32 from (r4,r6) to (r4+0x114+r5*8, r4+0x144+r5*4)
//!
//! Hidden side effects the C does not show:
//! - r5 is scaled in place (`shll2 r5`), r0 ends as that scaled value
//! - r7 = r4+0x114+r5*8 and r3 = r4+0x144 are left behind as base pointers
//! - r1 ends holding the loaded u32 from [r4+0x164]
//! - `shll r1` (the *8 scale) sets T to bit 31 of the intermediate, and
//!   nothing later clears it, so T is that bit on exit
//! - the final u32 store is the `rts` delay slot
//!
//! NOTE: blocked against the current oracle — its group-8 decode reads the
//! store base from bits 8-11 and an 8-bit disp from bits 0-7, so `mov.w
//! r0,@(2,r7)` (0x8171) is executed as a store at r1+226 = 0x226, which is
//! unmapped; the oracle faults before halting. This port follows the real
//! SH-4 encoding (reg = bits 4-7, disp = bits 0-3, per QEMU/Ghidra).

use sh4_oracle::{Cpu, Memory};

fn sext16(v: u16) -> u32 {
    v as u16 as i16 as i32 as u32
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    cpu.r[3] = mem.read_u32(0x8C027530).unwrap(); // 0x114
    cpu.r[1] = cpu.r[5];
    cpu.r[1] = cpu.r[1] << 2; // shll2 r1
    cpu.r[7] = r4;
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
    let t = (cpu.r[1] >> 31) != 0;
    cpu.r[1] = cpu.r[1] << 1; // shll r1 — sets T
    cpu.set_t(t);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[6]).unwrap());
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[1]);
    cpu.r[3] = r4;
    let _ = mem.write_u16(cpu.r[7], cpu.r[0] as u16);
    cpu.r[5] = cpu.r[5] << 2; // shll2 r5
    cpu.r[1] = mem.read_u32(0x8C027534).unwrap(); // 0x144
    cpu.r[0] = sext16(mem.read_u16(cpu.r[6] + 2).unwrap());
    cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[1]);
    let _ = mem.write_u16(cpu.r[7] + 2, cpu.r[0] as u16);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[6] + 4).unwrap());
    let _ = mem.write_u16(cpu.r[7] + 4, cpu.r[0] as u16);
    cpu.r[0] = sext16(mem.read_u16(cpu.r[6] + 6).unwrap());
    let _ = mem.write_u16(cpu.r[7] + 6, cpu.r[0] as u16);
    cpu.r[0] = mem.read_u32(0x8C027538).unwrap(); // 0x164
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(r4)).unwrap();
    cpu.r[0] = cpu.r[5];
    // rts delay slot: mov.l r1,@(r0,r3)
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[3]), cpu.r[1]);
}
