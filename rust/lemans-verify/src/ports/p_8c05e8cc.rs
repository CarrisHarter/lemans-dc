//! 0x8c05e8cc — unlink/lookup helper on the struct at r4.
//!
//! CFG: r0 = [r4]; r0 = [r0+4]. If that field is non-zero, walk the
//! linked list (r1 = [r0]; while r1 != 0 { r0 = r1 }) and store the last
//! node back to [r4] in the rts delay slot; T ends 1 (tst r1,r1 saw zero).
//! If the field is zero, load r5 = [0x8c05e908] (=0xfffffffe) and run a
//! masked hash-bucket walk: h = [r4]; loop { next = [h+8]; b = next & r5;
//! if [b] == h { [r4] = (next>>1)<<1; break } else { [r4] = b; h = b } }.
//!
//! Leftovers the C does not show: the list path leaves r1 = 0, r0 = last
//! node, T = 1; the hash path leaves r5 = 0xfffffffe, r3 = current bucket,
//! r6 = last bucket, r1 = next with bit 0 cleared, r0 = [bucket] == r3,
//! and T = 0 (the shlr/shll pair before the final store always leaves the
//! shll carry-out, which is 0 after a logical shift right). Both paths
//! store through r4, and the hash loop stores [r4] = bucket on every
//! iteration (write order matters).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let head = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r0
    let mut r0 = mem.read_u32(head.wrapping_add(4)).unwrap(); // mov.l @(4,r0),r0
    if r0 != 0 {
        // bra 0x8c05e8da ; nop — walk: r1 = [r0]; while r1 != 0 { r0 = r1 }
        loop {
            let r1 = mem.read_u32(r0).unwrap(); // mov.l @r0,r1
            cpu.r[1] = r1;
            cpu.set_t(r1 == 0); // tst r1,r1
            if r1 != 0 {
                r0 = r1; // bf 0x8c05e8d8 (plain bf: immediate) ; mov r1,r0
                continue;
            }
            break;
        }
        cpu.r[0] = r0;
        let _ = mem.write_u32(cpu.r[4], r0); // rts delay slot: mov.l r0,@r4
        return; // T = 1
    }
    // bt 0x8c05e8fe (plain bt: immediate): r5 = mask
    let mask = mem.read_u32(0x8c05e908).unwrap(); // mov.l 0x8c05e908,r5
    cpu.r[5] = mask;
    // bra 0x8c05e8e6 ; nop
    let mut h = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r3
    cpu.r[3] = h;
    loop {
        let mut r1 = mem.read_u32(h.wrapping_add(8)).unwrap(); // mov.l @(8,r3),r1
        cpu.r[1] = r1;
        let r6 = r1 & mask; // mov r1,r6 ; and r5,r6
        cpu.r[6] = r6;
        let v = mem.read_u32(r6).unwrap(); // mov.l @r6,r0
        cpu.r[0] = v;
        cpu.set_t(v == h); // cmp/eq r0,r3
        if v == h {
            // bf not taken: shlr r1 ; shll r1 ; mov.l r1,@r4 ; rts ; nop
            cpu.set_t(r1 & 1 != 0); // shlr r1
            r1 >>= 1;
            cpu.set_t(r1 >> 31 != 0); // shll r1 (always 0 here)
            r1 <<= 1;
            cpu.r[1] = r1;
            let _ = mem.write_u32(cpu.r[4], r1); // mov.l r1,@r4
            return; // T = shll carry-out = 0
        }
        // bf taken (immediate) to 0x8c05e8e4: mov.l r6,@r4 ; then r3 = [r4]
        let _ = mem.write_u32(cpu.r[4], r6);
        h = mem.read_u32(cpu.r[4]).unwrap(); // mov.l @r4,r3 (now the bucket)
        cpu.r[3] = h;
    }
}
