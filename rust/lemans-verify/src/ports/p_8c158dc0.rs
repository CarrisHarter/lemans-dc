//! 0x8c158dc0 - scan stride-40 groups for a -1 and zero the ones that have it.
//!
//! [r4] is the group count. Counts live at [r4+4], records at [r4+20].
//! Each group checks three words at record+12. A -1 makes the group call
//! 0x8c0b4ff0, which clears the three words at record+24. The running index
//! in r11 is popped away; the index that survives is the previous group's
//! count, carried in r14 into the next pass. The last cmp/gt leaves T clear.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved = [
        cpu.r[14], cpu.r[13], cpu.r[12], cpu.r[11], cpu.r[10], cpu.r[9], cpu.r[8], cpu.pr,
    ];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[4];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[10]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[9]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[8]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);

    cpu.r[11] = 0;
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), 0);
    cpu.r[14] = 0;
    let _ = mem.write_u32(cpu.r[15], 0);
    cpu.r[13] = 0xffff_ffff; // mov #-1, in the bra delay

    loop {
        cpu.r[0] = mem.read_u32(cpu.r[12]).unwrap_or(0);
        cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
        cpu.set_t((cpu.r[0] as i32) > (cpu.r[1] as i32));
        if !cpu.t() {
            break;
        }

        cpu.r[1] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
        cpu.r[6] = cpu.r[14];
        cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[6] = cpu.r[6].wrapping_shl(2);
        cpu.r[6] = cpu.r[6].wrapping_add(cpu.r[14]);
        cpu.r[6] = cpu.r[6].wrapping_shl(2);
        cpu.r[8] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
        cpu.r[3] = 0;
        cpu.r[4] = cpu.r[14];
        cpu.r[1] = 0;
        cpu.set_t(cpu.r[6] & 0x8000_0000 != 0);
        cpu.r[6] = cpu.r[6].wrapping_shl(1);
        cpu.r[10] = 1;
        cpu.r[7] = 3;
        loop {
            cpu.set_t((cpu.r[8] as i32) > (cpu.r[4] as i32));
            if !cpu.t() {
                break;
            }
            cpu.r[5] = cpu.r[1];
            cpu.r[0] = cpu.r[1];
            loop {
                cpu.r[9] = mem.read_u32(cpu.r[12].wrapping_add(20)).unwrap_or(0);
                cpu.r[9] = cpu.r[9].wrapping_add(12);
                cpu.r[9] = cpu.r[9].wrapping_add(cpu.r[6]);
                cpu.r[9] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[9])).unwrap_or(0);
                cpu.set_t(cpu.r[13] == cpu.r[9]);
                if cpu.t() {
                    cpu.r[3] = cpu.r[10];
                    cpu.r[11] = cpu.r[11].wrapping_add(1);
                }
                cpu.r[5] = cpu.r[5].wrapping_add(1);
                let cont = (cpu.r[7] as i32) > (cpu.r[5] as i32);
                cpu.set_t(cont);
                cpu.r[0] = cpu.r[0].wrapping_add(4); // bt.s delay
                if !cont {
                    break;
                }
            }
            cpu.r[6] = cpu.r[6].wrapping_add(40);
            cpu.r[4] = cpu.r[4].wrapping_add(1);
        }

        cpu.r[0] = cpu.r[3] & 0xff;
        cpu.set_t(cpu.r[0] == 0);
        if !cpu.t() {
            cpu.r[8] = cpu.r[14];
            cpu.r[8] = cpu.r[8].wrapping_shl(2);
            cpu.r[8] = cpu.r[8].wrapping_add(cpu.r[14]);
            cpu.r[8] = cpu.r[8].wrapping_shl(2);
            cpu.set_t(cpu.r[8] & 0x8000_0000 != 0);
            cpu.r[8] = cpu.r[8].wrapping_shl(1);
            loop {
                cpu.r[1] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
                cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
                cpu.r[0] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[1])).unwrap_or(0);
                cpu.set_t((cpu.r[0] as i32) > (cpu.r[14] as i32));
                if !cpu.t() {
                    break;
                }
                cpu.r[4] = mem.read_u32(cpu.r[12].wrapping_add(20)).unwrap_or(0);
                cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[8]);
                cpu.r[4] = cpu.r[4].wrapping_add(24); // jsr delay
                cpu.pr = 0x8c158e3c;
                crate::registry::p_8c0b4ff0::run(cpu, mem);
                cpu.r[8] = cpu.r[8].wrapping_add(40);
                cpu.r[14] = cpu.r[14].wrapping_add(1);
            }
        }

        cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
        cpu.r[3] = mem.read_u32(cpu.r[12].wrapping_add(4)).unwrap_or(0);
        cpu.r[1] = cpu.r[0].wrapping_add(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[1]);
        cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap_or(0);
        cpu.r[14] = mem.read_u32(cpu.r[0].wrapping_add(cpu.r[3])).unwrap_or(0);
        cpu.r[1] = cpu.r[1].wrapping_add(1);
        let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[1]);
    }

    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = saved[7];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[8] = saved[6];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[9] = saved[5];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[10] = saved[4];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = saved[3];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = saved[2];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = saved[1];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = saved[0];
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
