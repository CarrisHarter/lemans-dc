//! 0x8c221e18 — fill slot `r4` of the table at 0x8c2bde98 (stride 128):
//! zero bytes [44..57) and [60..73), then copy up to 12 bytes from `r5`
//! to slot+44 and from `r6` to slot+60, stopping a copy early at the first
//! zero byte read (the zero itself is not written).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[7] = cpu.r[4];
    cpu.r[7] <<= 2; // shll2
    cpu.r[7] <<= 2; // shll2
    cpu.r[3] = mem.read_u32(0x8c221ea8).unwrap_or(0); // 0x8c2bde98
    cpu.r[7] <<= 2; // shll2
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[7] <<= 1; // shll
    cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[3]);
    cpu.r[4] = cpu.r[7];
    cpu.r[4] = cpu.r[4].wrapping_add(44);
    cpu.r[14] = 0;
    cpu.r[2] = cpu.r[7];
    // zero loop 1: [r7+44 .. r7+57)
    loop {
        let _ = mem.write_u8(cpu.r[4], cpu.r[14] as u8);
        cpu.r[2] = cpu.r[7].wrapping_add(57);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[4] >= cpu.r[2]); // cmp/hs r2,r4
        if !cpu.t() {
            continue;
        }
        break;
    }
    cpu.r[4] = cpu.r[7];
    cpu.r[4] = cpu.r[4].wrapping_add(60);
    cpu.r[2] = cpu.r[7];
    // zero loop 2: [r7+60 .. r7+73)
    loop {
        let _ = mem.write_u8(cpu.r[4], cpu.r[14] as u8);
        cpu.r[2] = cpu.r[7].wrapping_add(73);
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        cpu.set_t(cpu.r[4] >= cpu.r[2]); // cmp/hs r2,r4
        if !cpu.t() {
            continue;
        }
        break;
    }
    cpu.r[0] = cpu.r[5];
    cpu.r[4] = cpu.r[5];
    cpu.r[5] = cpu.r[5].wrapping_add(12);
    cpu.r[14] = cpu.r[7];
    cpu.set_t(cpu.r[4] >= cpu.r[5]); // cmp/hs r5,r4
    cpu.r[14] = cpu.r[14].wrapping_add(44); // bt.s delay slot: always runs
    if !cpu.t() {
        // copy loop 1: r0 src, r4 cursor, r5 end, r14 dst
        loop {
            cpu.r[3] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r4,r3
            cpu.set_t(cpu.r[3] & cpu.r[3] == 0); // tst r3,r3
            if cpu.t() {
                break;
            }
            cpu.r[3] = mem.read_u8(cpu.r[0]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r0+,r3
            cpu.r[0] = cpu.r[0].wrapping_add(1);
            cpu.r[4] = cpu.r[4].wrapping_add(1);
            cpu.set_t(cpu.r[4] >= cpu.r[5]); // cmp/hs r5,r4
            let _ = mem.write_u8(cpu.r[14], cpu.r[3] as u8);
            cpu.r[14] = cpu.r[14].wrapping_add(1); // bf.s delay slot: always runs
            if !cpu.t() {
                continue;
            }
            break;
        }
    }
    cpu.r[14] = cpu.r[6];
    cpu.r[4] = cpu.r[6];
    cpu.r[6] = cpu.r[6].wrapping_add(12);
    cpu.r[5] = cpu.r[7];
    cpu.set_t(cpu.r[4] >= cpu.r[6]); // cmp/hs r6,r4
    cpu.r[5] = cpu.r[5].wrapping_add(60); // bt.s delay slot: always runs
    if !cpu.t() {
        // copy loop 2: r14 src (post-increment), r4 cursor, r6 end, r5 dst
        loop {
            cpu.r[2] = mem.read_u8(cpu.r[4]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r4,r2
            cpu.set_t(cpu.r[2] & cpu.r[2] == 0); // tst r2,r2
            if cpu.t() {
                break;
            }
            cpu.r[3] = mem.read_u8(cpu.r[14]).unwrap_or(0) as i8 as i32 as u32; // mov.b @r14+,r3
            cpu.r[14] = cpu.r[14].wrapping_add(1);
            cpu.r[4] = cpu.r[4].wrapping_add(1);
            cpu.set_t(cpu.r[4] >= cpu.r[6]); // cmp/hs r6,r4
            let _ = mem.write_u8(cpu.r[5], cpu.r[3] as u8);
            cpu.r[5] = cpu.r[5].wrapping_add(1); // bf.s delay slot: always runs
            if !cpu.t() {
                continue;
            }
            break;
        }
    }
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0); // rts delay slot
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
