//! 0x8c105f2c — free the child tree at `[object+4]`, then the object when the count is positive.
//!
//! A null object returns 0 with T set. A non-null object whose child word is
//! non-zero calls `0x8c106700` with `r5 = [object+4]` and `r4` left as the
//! object. `tst` of `object+4` is dead: `exts.w` of the saved count and
//! `cmp/pl` replace T. A positive low half then frees the object through
//! `0x8c01032c`. `r0` is the object on every exit; `r13`/`r14` are restored.

use sh4_oracle::{Cpu, Memory};

fn rd(mem: &Memory, addr: u32) -> u32 {
    mem.read_u32(addr).unwrap_or(0)
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[14] = cpu.r[4];
    cpu.r[13] = cpu.r[5];
    cpu.set_t(cpu.r[14] == 0);
    if !cpu.t() {
        cpu.r[0] = rd(mem, cpu.r[14].wrapping_add(4));
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            cpu.r[0] = cpu.r[14].wrapping_add(4);
            cpu.r[5] = rd(mem, cpu.r[0]);
            cpu.r[0] = rd(mem, 0x8c105f68);
            cpu.pr = 0x8c105f4c;
            crate::registry::p_8c106700::run(cpu, mem);
        }
        cpu.r[0] = cpu.r[14].wrapping_add(4);
        cpu.set_t(cpu.r[0] == 0); // dead: object+4 is zero only for object == -4
        cpu.r[0] = cpu.r[13] as i16 as i32 as u32; // exts.w
        cpu.set_t((cpu.r[0] as i32) > 0); // cmp/pl
        if cpu.t() {
            cpu.r[0] = rd(mem, 0x8c105f6c);
            cpu.pr = 0x8c105f5e;
            cpu.r[4] = cpu.r[14]; // jsr delay slot
            crate::registry::p_8c01032c::run(cpu, mem);
        }
    }
    cpu.r[0] = cpu.r[14];
    cpu.pr = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = rd(mem, cpu.r[15]);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = rd(mem, cpu.r[15]); // rts delay
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
