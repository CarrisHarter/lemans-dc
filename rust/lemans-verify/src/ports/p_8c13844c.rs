//! 0x8c13844c — release every handle in the array at `[r4+8]`, then free the array.
//!
//! Same walk as 0x8c138494 (`end = base + count*4`, `0x8c018bd8(slot, -1)`),
//! then `0x8c01032c` on the array base reloaded from `[r4+8]`. A null base
//! skips both. `r12 = -1` only when the walk runs. Pushed r11–r14 and pr
//! return to their entry values; the exit T is the callee's on the free path
//! and the `tst` (1) on the null path.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved = [cpu.r[14], cpu.r[13], cpu.r[12], cpu.r[11], cpu.pr];
    cpu.r[15] = cpu.r[15].wrapping_sub(20);
    cpu.r[11] = cpu.r[4];
    cpu.r[13] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
    cpu.set_t(cpu.r[13] == 0);
    if cpu.r[13] != 0 {
        let count = mem.read_u32(cpu.r[11].wrapping_add(4)).unwrap_or(0);
        cpu.r[0] = count.wrapping_shl(2);
        cpu.r[14] = cpu.r[13].wrapping_add(cpu.r[0]);
        cpu.set_t(cpu.r[14] > cpu.r[13]);
        if cpu.t() {
            cpu.r[12] = 0xffff_ffff;
            loop {
                cpu.r[0] = mem.read_u32(0x8c13848c).unwrap_or(0x8c018bd8);
                cpu.r[5] = cpu.r[12];
                cpu.r[4] = cpu.r[13];
                cpu.pr = 0x8c138474;
                crate::registry::p_8c018bd8::run(cpu, mem);
                cpu.r[13] = cpu.r[13].wrapping_add(4);
                cpu.set_t(cpu.r[14] > cpu.r[13]);
                if !cpu.t() {
                    break;
                }
            }
        }
        cpu.r[0] = mem.read_u32(0x8c138490).unwrap_or(0x8c01032c);
        cpu.r[4] = mem.read_u32(cpu.r[11].wrapping_add(8)).unwrap_or(0);
        cpu.pr = 0x8c138480;
        crate::registry::p_8c01032c::run(cpu, mem);
    }
    cpu.r[15] = cpu.r[15].wrapping_add(20);
    cpu.pr = saved[4];
    cpu.r[11] = saved[3];
    cpu.r[12] = saved[2];
    cpu.r[13] = saved[1];
    cpu.r[14] = saved[0];
}
