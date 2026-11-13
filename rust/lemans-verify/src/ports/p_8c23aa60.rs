//! 0x8c23aa60 — after a table hit, merge a mode into the low half of a word.
//!
//! `0x8c237780` searches for `r4`. A miss returns `0x0000eeee` and writes
//! nothing. A hit loads the object at `[r4+0x88]` and `r0 =` the saved `r5`.
//! `r5 >= 35` (unsigned) takes the shared tail with mode `0x400` and leaves T
//! set. Otherwise `shll` / `mova` / `mov.w @(r0,r1)` / `braf` indexes the word
//! table at `0x8c23aac8` (base of the branch is `0x8c23aac2`). Each arm only
//! chooses the value that lands in `r4`:
//!
//! | index | mode | how |
//! | --- | --- | --- |
//! | 0, 5–9, 11–19, 25–29 | 0x400 | `mov r6,r4` (`r6` is the `0x400` literal) |
//! | 1 | 0x401 | `mov.w` |
//! | 2, 4 | 0x800 | `mov r7,r4` in the `bra` delay slot |
//! | 3 | 0x472 | `mov.w` |
//! | 10 | 0x8e3 | `mov.w` |
//! | 20 | 0x3de | `mov.w` |
//! | 21 | 0x3bf | `mov.w` |
//! | 22 | 0x3a2 | `mov.w` |
//! | 23 | 0x387 | `mov.w` |
//! | 24 | 0x36d | `mov.w` |
//! | 30 | 0x7bd | `mov.w` |
//! | 31 | 0x780 | `mov.w` |
//! | 32 | 0x745 | `mov.w` |
//! | 33 | 0x70f | `mov.w` |
//! | 34 | 0x6db | `mov.w` |
//!
//! The tail does `[object+36] = ([object+36] & 0xffff0000) | mode` and returns
//! 0. `and`/`or` leave T alone, so a table hit exits with T clear (`shll` of a
//! value below 35) and an out-of-range index exits with T set (`cmp/hs`).

use sh4_oracle::{Cpu, Memory};

fn sext16(mem: &Memory, addr: u32) -> u32 {
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

/// `mov.w @(disp,PC),r4` at `pc`: sign-extended literal the instruction names.
fn movw_pc(mem: &Memory, pc: u32) -> u32 {
    let op = mem.read_u16(pc).unwrap_or(0) as u32;
    let addr = (pc & !1).wrapping_add(4).wrapping_add((op & 0xff) * 2);
    mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32
}

fn mode_for(target: u32, cpu: &Cpu, mem: &Memory) -> u32 {
    match target {
        0x8c23ab18 => cpu.r[7],                 // bra delay: mov r7,r4
        0x8c23ab0e | 0x8c23abd2 => cpu.r[6],    // mov r6,r4
        _ => movw_pc(mem, target),
    }
}

fn epilogue(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[3] = mem.read_u32(0x8c23aa80).unwrap_or(0x8c237780);
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[5]);
    cpu.pr = 0x8c23aa6e;
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0); // jsr delay
    crate::registry::p_8c237780::run(cpu, mem);

    cpu.set_t(cpu.r[0] == 0);
    if cpu.t() {
        cpu.r[0] = mem.read_u32(0x8c23aa84).unwrap_or(0x0000eeee);
        epilogue(cpu, mem);
        return;
    }

    cpu.r[0] = sext16(mem, 0x8c23aac2); // 0x88
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[6] = sext16(mem, 0x8c23aac6); // 0x400
    cpu.r[5] = mem.read_u32(cpu.r[4].wrapping_add(cpu.r[0])).unwrap_or(0);
    cpu.r[0] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
    cpu.r[7] = sext16(mem, 0x8c23aac4); // 0x800
    cpu.r[1] = 35;
    cpu.set_t(cpu.r[0] >= cpu.r[1]); // cmp/hs
    if cpu.t() {
        cpu.r[4] = cpu.r[6]; // bra 0x8c23abd2: mov r6,r4
    } else {
        let out = cpu.r[0] >> 31 != 0;
        cpu.r[0] <<= 1; // shll
        cpu.set_t(out);
        cpu.r[1] = cpu.r[0];
        cpu.r[0] = 0x8c23aac8; // mova
        let disp = mem
            .read_u16(cpu.r[0].wrapping_add(cpu.r[1]))
            .unwrap_or(0) as i16 as i32 as u32;
        cpu.r[0] = disp;
        let target = 0x8c23aac2u32.wrapping_add(disp); // braf
        cpu.r[4] = mode_for(target, cpu, mem);
    }

    cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(36)).unwrap_or(0);
    cpu.r[0] = 0;
    cpu.r[3] = mem.read_u32(0x8c23ac08).unwrap_or(0xffff0000);
    cpu.r[2] &= cpu.r[3];
    cpu.r[1] = cpu.r[2];
    cpu.r[1] |= cpu.r[4];
    let _ = mem.write_u32(cpu.r[5].wrapping_add(36), cpu.r[1]);
    epilogue(cpu, mem);
}
