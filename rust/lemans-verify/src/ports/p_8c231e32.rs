//! 0x8c231e32 - map min(r4, r5) onto a small code.
//!
//! 1→0, 2→1, 4→2, 16→4, 32→5, 64→6, 0x80→7, 0x100→8, 0x200→9, 0x400→10.
//! 8 and every other value return 3. The code is the `rts` delay slot.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.set_t((cpu.r[4] as i32) >= (cpu.r[5] as i32)); // cmp/ge r5,r4
    if cpu.t() {
        cpu.r[4] = cpu.r[5];
    }
    cpu.r[0] = cpu.r[4];
    let value = cpu.r[0];
    let early = [(1, 0), (2, 1), (4, 2)];
    for (imm, code) in early {
        cpu.set_t(value == imm as u32);
        if cpu.t() {
            cpu.r[0] = code;
            return;
        }
    }
    cpu.set_t(value == 8);
    if cpu.t() {
        cpu.r[0] = 3;
        return;
    }
    let later = [(16, 4), (32, 5), (64, 6)];
    for (imm, code) in later {
        cpu.set_t(value == imm as u32);
        if cpu.t() {
            cpu.r[0] = code;
            return;
        }
    }
    let words = [
        (0x8c231e80, 7u32),
        (0x8c231e82, 8),
        (0x8c231e84, 9),
        (0x8c231e86, 10),
    ];
    for (addr, code) in words {
        cpu.r[1] = mem.read_u16(addr).unwrap_or(0) as i16 as i32 as u32;
        cpu.set_t(cpu.r[1] == value);
        if cpu.t() {
            cpu.r[0] = code;
            return;
        }
    }
    cpu.r[0] = 3;
}
