//! 0x8c22c420 - scan up to 8 records of stride 0x1c4 for an id match.
//!
//! A record matches when its pointer word is nonzero and `[+4] == r4`. The
//! word at `+20` selects the result: 2 or 3 -> 0x100, 4 -> 0x101, 5 -> 102,
//! 6 -> 103, 7 -> 104, anything else or no match -> 105. The stride add is
//! the `bf.s` delay slot, so it runs on the exiting iteration too.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[5] = mem.read_u32(0x8c22c464).unwrap_or(0x8c2c6fa0);
    cpu.r[7] = 8;
    cpu.r[6] = 0;
    loop {
        cpu.r[3] = mem.read_u32(cpu.r[5]).unwrap_or(0);
        cpu.set_t(cpu.r[3] == 0); // tst r3,r3
        let mut miss = cpu.t();
        if !miss {
            cpu.r[3] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
            cpu.set_t(cpu.r[4] == cpu.r[3]); // cmp/eq r4,r3
            miss = !cpu.t();
        }
        if !miss {
            cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(20)).unwrap_or(0);
            cpu.set_t(cpu.r[0] == 2);
            if cpu.t() {
                cpu.r[0] = mem.read_u16(0x8c22c460).unwrap_or(0x0100) as i16 as i32 as u32;
                return;
            }
            cpu.set_t(cpu.r[0] == 3);
            if cpu.t() {
                cpu.r[0] = mem.read_u16(0x8c22c460).unwrap_or(0x0100) as i16 as i32 as u32;
                return;
            }
            cpu.set_t(cpu.r[0] == 4);
            if cpu.t() {
                cpu.r[0] = mem.read_u16(0x8c22c462).unwrap_or(0x0101) as i16 as i32 as u32;
                return;
            }
            cpu.set_t(cpu.r[0] == 5);
            if cpu.t() {
                cpu.r[0] = 102; // rts delay
                return;
            }
            cpu.set_t(cpu.r[0] == 6);
            if cpu.t() {
                cpu.r[0] = 103;
                return;
            }
            cpu.set_t(cpu.r[0] == 7);
            if cpu.t() {
                cpu.r[0] = 104;
                return;
            }
            break;
        }
        cpu.r[6] = cpu.r[6].wrapping_add(1);
        cpu.r[3] = mem.read_u16(0x8c22c4a0).unwrap_or(0x01c4) as i16 as i32 as u32;
        cpu.set_t(cpu.r[6] >= cpu.r[7]); // cmp/hs r7,r6
        cpu.r[5] = cpu.r[5].wrapping_add(cpu.r[3]); // bf.s delay
        if cpu.t() {
            break;
        }
    }
    cpu.r[0] = 105;
}
