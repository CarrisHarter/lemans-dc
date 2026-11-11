//! 0x8c13cfbc — search the table at [r4+8] + i*80 for i in [0, [r4+4]) for
//! the key r5; on every match copy 8 bytes from [[r4+104] + i*8] to r6/r6+4.
//!
//! The loop counter r7 is checked at the top (0x8c13cfec) and incremented at
//! 0x8c13cfea even after a match, so all matching slots are copied. Final
//! state: r0 = [r4+4], r7 = [r4+4], r3 = r4+104, T = 0 (last cmp/hi failed).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let r4 = cpu.r[4];
    cpu.r[3] = r4.wrapping_add(104); // mov r4,r3 ; add #104,r3
    cpu.r[7] = 0; // bra delay slot: mov #0,r7
    loop {
        // 0x8c13cfec: r0 = @(4,r4) ; cmp/hi r7,r0 ; bt 0x8c13cfc4
        cpu.r[0] = mem.read_u32(r4.wrapping_add(4)).unwrap_or(0);
        cpu.set_t(cpu.r[0] > cpu.r[7]); // cmp/hi r7,r0
        if !cpu.t() {
            break;
        }
        // 0x8c13cfc4: r1 = r7*80 + @(8,r4) ; cmp/eq r5,r1 ; bf 0x8c13cfea
        let i = cpu.r[7];
        let mut r1 = i << 2; // shll2 r1 -> 4i
        r1 = r1.wrapping_add(i); // add r7,r1 -> 5i
        r1 <<= 2; // shll2 -> 20i
        let base = mem.read_u32(r4.wrapping_add(8)).unwrap_or(0);
        r1 <<= 2; // shll2 -> 80i
        r1 = r1.wrapping_add(base);
        cpu.r[1] = r1;
        cpu.set_t(r1 == cpu.r[5]); // cmp/eq r5,r1
        if cpu.t() {
            // 0x8c13cfd6: r0 = r7*8 ; r1 = @r3 ; r1 = @(r0,r1) ; @r6 = r1 ;
            //            r1 = @r3 ; r1 += r0 ; r0 = @(4,r1) ; @(4,r6) = r0
            let mut r0 = i;
            r0 <<= 2; // shll2 r0 -> 4i
            let tbl = mem.read_u32(cpu.r[3]).unwrap_or(0);
            cpu.set_t(r0 >> 31 != 0); // shll r0: T = old bit 31 (clobbered later anyway)
            r0 <<= 1; // 8i
            cpu.r[0] = r0;
            let p = mem.read_u32(tbl.wrapping_add(r0)).unwrap_or(0);
            cpu.r[1] = p;
            let _ = mem.write_u32(cpu.r[6], p); // mov.l r1,@r6
            let tbl = mem.read_u32(cpu.r[3]).unwrap_or(0);
            let q = tbl.wrapping_add(r0);
            cpu.r[1] = q;
            cpu.r[0] = mem.read_u32(q.wrapping_add(4)).unwrap_or(0);
            let _ = mem.write_u32(cpu.r[6].wrapping_add(4), cpu.r[0]); // mov.l r0,@(4,r6)
        }
        // 0x8c13cfea: add #1,r7
        cpu.r[7] = cpu.r[7].wrapping_add(1);
    }
    // 0x8c13cff2: rts ; nop
}
