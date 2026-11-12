//! 0x8c03c8bc — store_param_and_fanout: stores `r5` at `r4+200`, then
//! fans the same value out to five optional records plus two optional callees.
//!
//! Each of the seven gate words is loaded from `r4+offset` and tested; a zero
//! gate skips that store/call. `tst` sets T on equality-with-zero (so `bt`
//! means "gate is zero"). All seven tests use plain `bt` (no delay slot): a
//! zero gate jumps over its store/call body, a nonzero gate falls through.
//! - gate @+0xE0 -> `*(gate)+0x48 = r5` (mov #72 scratch in r0)
//! - gate @+0xE4 / @+0xE8 -> `*(gate)+0x38 = r5`
//! - gate @+0xF0 / @+0xF4 -> `*(gate)+0x78 = r5` (mov #120 scratch in r0)
//! - gate @+0x108 (r4=gate), r5=param -> `jsr 0x8c143e88` (writes r5 @ r4+92)
//! - gate @+0xFC (r4=gate), r5=param -> `jsr 0x8c197840` (writes r5 @ r4+56)

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[14] = cpu.r[5];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[13] = cpu.r[4];
    cpu.r[0] = 0xC8;
    let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[13]), cpu.r[14]);
    cpu.r[0] = 0xE0;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
    cpu.set_t(cpu.r[1] == 0);
    if !cpu.t() {
        cpu.r[0] = 72;
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), cpu.r[14]);
    }
    cpu.r[0] = 0xE4;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        let _ = mem.write_u32(cpu.r[0].wrapping_add(56), cpu.r[14]);
    }
    cpu.r[0] = 0xE8;
    cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
    cpu.set_t(cpu.r[0] == 0);
    if !cpu.t() {
        let _ = mem.write_u32(cpu.r[0].wrapping_add(56), cpu.r[14]);
    }
    cpu.r[0] = 0xF0;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
    cpu.set_t(cpu.r[1] == 0);
    if !cpu.t() {
        cpu.r[0] = 120;
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), cpu.r[14]);
    }
    cpu.r[0] = 0xF4;
    cpu.r[1] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
    cpu.set_t(cpu.r[1] == 0);
    if !cpu.t() {
        cpu.r[0] = 120;
        let _ = mem.write_u32(cpu.r[0].wrapping_add(cpu.r[1]), cpu.r[14]);
    }
    cpu.r[0] = 0x108;
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
    cpu.set_t(cpu.r[4] == 0);
    if !cpu.t() {
        cpu.r[0] = 0x8c143e88;
        cpu.pr = 0x8c03c910;
        cpu.r[5] = cpu.r[14]; // jsr delay slot
        crate::registry::p_8c143e88::run(cpu, mem);
    }
    cpu.r[0] = 0xFC;
    cpu.r[4] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[13])).unwrap_or(0);
    cpu.set_t(cpu.r[4] == 0);
    if !cpu.t() {
        cpu.r[0] = 0x8c197840;
        cpu.pr = 0x8c03c91e;
        cpu.r[5] = cpu.r[14]; // jsr delay slot
        crate::registry::p_8c197840::run(cpu, mem);
    }
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
