//! 0x8c226868 — crc16.
//!
//! CRC-16/CCITT over `r5` bytes at `r4`, polynomial `0x1021`, starting from 0.
//! Returns the CRC in `r0`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[6] = 0;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[11] = 8;
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[10] = cpu.r[4];
    cpu.r[1] = 0x1021;
    cpu.r[4] = cpu.r[6];
    cpu.r[0] = 0x8000;
    cpu.r[12] = cpu.r[6]; // delay slot of `bra`
    loop {
        let hs = cpu.r[12] >= cpu.r[5];
        cpu.set_t(hs);
        if hs {
            break;
        }
        cpu.r[3] = mem.read_u8(cpu.r[10]).unwrap_or(0) as u32;
        cpu.r[10] = cpu.r[10].wrapping_add(1);
        cpu.r[7] = cpu.r[11];
        cpu.r[3] = cpu.r[3] << 8;
        cpu.r[4] = cpu.r[4] ^ cpu.r[3];
        loop {
            cpu.r[2] = cpu.r[4];
            cpu.r[6] = cpu.r[4];
            cpu.r[6] = cpu.r[6] << 1;
            let t = (cpu.r[0] & cpu.r[2]) == 0;
            cpu.set_t(t);
            if t {
                cpu.r[4] = cpu.r[6];
            } else {
                cpu.r[4] = cpu.r[6];
                cpu.r[4] = cpu.r[4] ^ cpu.r[1]; // delay slot of `bra`
            }
            cpu.r[7] = cpu.r[7].wrapping_sub(1);
            cpu.set_t(cpu.r[7] == 0);
            if cpu.r[7] == 0 {
                break;
            }
        }
        cpu.r[12] = cpu.r[12].wrapping_add(1);
    }
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[0] = cpu.r[4];
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap_or(0); // delay slot of `rts`
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
