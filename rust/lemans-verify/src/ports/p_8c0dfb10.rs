//! 0x8c0dfb10 — high-score style table update. Zeroes the byte at r4+0x194,
//! scans `count` (= [r4+0x190]) 8-byte entries (u32 key at r4+i*8, f32 value
//! at r4+4+i*8) for key == r5; on a match the entry's f32 becomes
//! max(fr4, entry) and r4 advances past it. On a miss, if count+1 <= 50 the
//! key r5 is stored at r4+count*8, fr4 at r4+4+count*8, and count is
//! incremented.
//!
//! Residue: found path — r0 = r4+4+i*8, r1 = i*8, r3 = i*8, r4 = r4+4+i*8,
//! fr0 = entry f32, T = (fr4 > fr0); the rts delay slot stores fr4 at the new
//! r4. Miss path — r0 = 50, r1 = count+1, r3 = count, T = (50 >= count+1);
//! append sub-path additionally r6 = r0 = count*8, r3 = r4+4+count*8, T =
//! bit31 of count*4 (last `shll r1`).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    let off_flag = mem.read_u32(0x8c0dfb78).unwrap_or(0); // 0x194
    let off_cnt = mem.read_u32(0x8c0dfb7c).unwrap_or(0); // 0x190
    let mut r3: u32 = 0; // mov #0,r3
    let mut r1: u32 = 0; // mov r3,r1
    let _ = mem.write_u8(r4.wrapping_add(off_flag), 0); // mov.b r3,@(r0,r4)
    // bra 0x8c0dfb48 ; delay: mov.l @(r0,r4),r6
    let r6 = mem.read_u32(r4.wrapping_add(off_cnt)).unwrap_or(0);
    cpu.r[6] = r6;
    let found = loop {
        // 0x8c0dfb48: cmp/gt r3,r6 ; bt 0x8c0dfb1e
        if (r6 as i32) > (r3 as i32) {
            let key = mem.read_u32(r1.wrapping_add(r4)).unwrap_or(0); // mov r1,r0 ; mov.l @(r0,r4),r0
            cpu.set_t(key == cpu.r[5]); // cmp/eq r5,r0
            if key == cpu.r[5] {
                break Some((r1, r3));
            }
            r1 = r1.wrapping_add(8); // add #8,r1
            r3 = r3.wrapping_add(1); // add #1,r3
        } else {
            break None;
        }
    };
    match found {
        Some((r1, r3i)) => {
            // 0x8c0dfb26: mov r4,r0 ; shll2 r3 ; shll r3 ; add #4,r0 ; add r3,r0
            let r3 = (r3i << 2) << 1;
            let r0 = r4.wrapping_add(4).wrapping_add(r3);
            let fr0 = mem.read_f32(r0).unwrap_or(0.0); // fmov @r0,fr0
            cpu.fr[0] = fr0;
            cpu.set_t(cpu.fr[4] > fr0); // fcmp/gt fr0,fr4
            if !cpu.t() {
                cpu.fr[4] = fr0; // fmov fr0,fr4
            }
            // add #4,r4 ; add r3,r4 ; rts ; fmov fr4,@r4 (delay slot)
            let r4n = r4.wrapping_add(4).wrapping_add(r3);
            let _ = mem.write_f32(r4n, cpu.fr[4]);
            cpu.r[0] = r0;
            cpu.r[1] = r1;
            cpu.r[3] = r3;
            cpu.r[4] = r4n;
        }
        None => {
            // 0x8c0dfb4c: mov r6,r1 ; add #1,r1 ; mov #50,r0 ; cmp/ge r1,r0 ; bf 0x8c0dfb74
            let r1n = r6.wrapping_add(1);
            cpu.r[1] = r1n;
            cpu.r[0] = 50;
            cpu.set_t((50i32) >= (r1n as i32)); // cmp/ge r1,r0
            cpu.r[3] = r3;
            if (50i32) < (r1n as i32) {
                return; // bf 0x8c0dfb74 -> rts ; nop
            }
            // append path 0x8c0dfb56
            let r6s = (r6 << 2) << 1; // shll2 r6 ; shll r6
            cpu.set_t(((r6 << 2) >> 31) != 0); // shll r6 sets T
            cpu.r[6] = r6s;
            cpu.r[0] = r6s; // mov r6,r0
            let r3n = r4.wrapping_add(4); // mov r4,r3 ; add #4,r3
            let cnt = mem.read_u32(r4.wrapping_add(off_cnt)).unwrap_or(0); // mov.l @(r0,r4),r1
            let r1s = (cnt << 2) << 1; // shll2 r1 ; shll r1
            let _ = mem.write_u32(r6s.wrapping_add(r4), cpu.r[5]); // mov.l r5,@(r0,r4)
            let r3n = r3n.wrapping_add(r1s); // add r1,r3
            let _ = mem.write_f32(r3n, cpu.fr[4]); // fmov fr4,@r3
            let r1n2 = mem.read_u32(r4.wrapping_add(off_cnt)).unwrap_or(0).wrapping_add(1);
            let _ = mem.write_u32(r4.wrapping_add(off_cnt), r1n2); // mov.l r1,@(r0,r4)
            cpu.r[1] = r1n2;
            cpu.r[3] = r3n;
            cpu.r[0] = off_cnt; // mov.l 0x8c0dfb7c,r0 ! 0x190 (after the key store)
            // rts ; nop
        }
    }
}

