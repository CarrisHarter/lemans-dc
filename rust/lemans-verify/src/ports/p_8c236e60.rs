//! 0x8c236e60 — fill the struct at `r4` from the descriptor at 0x8c23d798:
//! [r4+4] = len(list[44]) + len(list[60]) (next at node+8),
//! [r4+8] = len(list[36]) + len(list[52]) (next at node+8),
//! [r4+12/16/20/24/28] = desc[8], len(list[68]) (next at node+4), desc[12],
//! desc[76], desc[4]. Returns 0.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c236ee0).unwrap_or(0); // 0x8c23d798
    cpu.r[7] = 0;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[7]);
    cpu.r[6] = mem.read_u32(cpu.r[5].wrapping_add(44)).unwrap_or(0);
    cpu.set_t(cpu.r[6] & cpu.r[6] == 0); // tst r6,r6
    if !cpu.t() {
        loop {
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            cpu.r[3] = cpu.r[3].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[3]);
            cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[6] = mem.read_u32(cpu.r[5].wrapping_add(60)).unwrap_or(0);
    cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
    if !cpu.t() {
        loop {
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
            cpu.r[2] = cpu.r[2].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[2]);
            cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
            if cpu.t() {
                break;
            }
        }
    }
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[7]);
    cpu.r[6] = mem.read_u32(cpu.r[5].wrapping_add(36)).unwrap_or(0);
    cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
    if !cpu.t() {
        loop {
            cpu.r[3] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
            cpu.r[3] = cpu.r[3].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[3]);
            cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[6] = mem.read_u32(cpu.r[5].wrapping_add(52)).unwrap_or(0);
    cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
    if !cpu.t() {
        loop {
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
            cpu.r[2] = cpu.r[2].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[2]);
            cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap_or(0);
            cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[0] = 68;
    cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[2]);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[7]);
    cpu.r[6] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
    cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
    if !cpu.t() {
        loop {
            cpu.r[2] = mem.read_u32(cpu.r[4].wrapping_add(16)).unwrap_or(0);
            cpu.r[2] = cpu.r[2].wrapping_add(1);
            let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[2]);
            cpu.r[6] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap_or(0);
            cpu.set_t(cpu.r[6] & cpu.r[6] == 0);
            if cpu.t() {
                break;
            }
        }
    }
    cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap_or(0);
    cpu.r[0] = 76;
    let _ = mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[2]);
    cpu.r[3] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[5])).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[3]);
    cpu.r[2] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[4].wrapping_add(28), cpu.r[2]);
    cpu.r[0] = 0; // rts delay slot
}
