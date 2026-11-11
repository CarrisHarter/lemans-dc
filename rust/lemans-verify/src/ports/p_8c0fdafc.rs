//! 0x8c0fdafc — copy one 16-byte block from a material to a float out pointer.
//!
//! ```text
//! CFG:
//!   push r14; r1=[r5+4] & 0x0001ffff (DAT_8c0fdbc0)
//!   T=((r1&4)==0)  ; bt 0x8c0fdb38
//!   PATH A (bit4 set): 4x fmov [r5+16..28] -> fr0 -> [arg..arg+12], arg=[r15+4]
//!     bra EPI (delay stores the 4th word); fr0 = [r5+28]; T=0
//!   0x8c0fdb38: r1 &= 0x00018000 (DAT_8c0fdbc4); T=(r1==0); bt 0x8c0fdb92
//!   PATH B (0x18000 set): r4=[r5+16] packed RGBA; bytes extracted with
//!     shlr16/shlr8 + extu.b into r1/r7/r5/r4; fr3=255.0f (DAT_8c0fdbc8);
//!     [arg]=float(b3)/255, [arg+4]=float(b2)/255, [arg+8]=float(b1)/255,
//!     [arg+12]=float(b0)/255 (last store is the bra delay slot); T=0
//!   PATH C (neither): 4x fmov [r5+32..44] -> [arg..arg+12]; T=1 (!)
//!   EPI (0x8c0fdbbc): rts; delay pop r14
//! ```
//! Hidden side effects: r0/r1/r3/r4/r5/r6/r7 leftovers differ per path
//! (path B leaves the extracted bytes and fpul=byte0), fr0/fr1/fr2/fr3 hold
//! the last loaded/divided values, and T is 0 on paths A/B but 1 on path C.
//! The oracle models every fmov as a 4-byte single move regardless of
//! FPSCR.SZ, so the port does the same.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // mov.l r14,@-r15

    let mut r1 = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap(); // mov.l @(4,r5),r1
    let mask = mem.read_u32(0x8C0FDBC0).unwrap(); // mov.l 0x8c0fdbc0,r0
    r1 &= mask; // and r0,r1
    cpu.r[1] = r1;
    cpu.r[0] = r1; // mov r1,r0
    cpu.r[0] &= 4; // and #4,r0
    cpu.set_t(cpu.r[0] & cpu.r[0] == 0); // tst r0,r0

    if cpu.r[0] != 0 {
        // PATH A, 0x8c0fdb0c: copy [r5+16..r5+28] to [arg..arg+12]
        cpu.r[0] = cpu.r[5].wrapping_add(16); // mov r5,r0 ; add #16,r0
        cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap(); // mov.l @(4,r15),r4
        cpu.r[1] = cpu.r[5]; // mov r5,r1
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
        cpu.r[3] = cpu.r[4]; // mov r4,r3
        cpu.r[1] = cpu.r[1].wrapping_add(20); // add #20,r1
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // fmov fr0,@r4
        cpu.r[0] = cpu.r[5]; // mov r5,r0
        cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr0
        cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
        cpu.r[0] = cpu.r[0].wrapping_add(24); // add #24,r0
        let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // fmov fr0,@r3
        cpu.r[1] = cpu.r[4]; // mov r4,r1
        cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
        cpu.r[1] = cpu.r[1].wrapping_add(8); // add #8,r1
        cpu.r[5] = cpu.r[5].wrapping_add(28); // add #28,r5
        let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1
        cpu.r[4] = cpu.r[4].wrapping_add(12); // add #12,r4
        cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap(); // fmov @r5,fr0
        // bra 0x8c0fdbbc; delay slot stores the last word
        let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // fmov fr0,@r4 (delay)
    } else {
        // 0x8c0fdb38
        let mask2 = mem.read_u32(0x8C0FDBC4).unwrap(); // mov.l 0x8c0fdbc4,r0
        r1 &= mask2; // and r0,r1
        cpu.r[1] = r1;
        cpu.set_t(r1 & r1 == 0); // tst r1,r1
        if r1 != 0 {
            // PATH B, 0x8c0fdb40: unpack RGBA at [r5+16] into 4 floats /255
            cpu.r[4] = mem.read_u32(cpu.r[5].wrapping_add(16)).unwrap(); // mov.l @(16,r5),r4
            cpu.r[0] = 0x8C0FDBC8; // mova 0x8c0fdbc8,r0
            cpu.r[3] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap(); // mov.l @(4,r15),r3
            cpu.r[1] = cpu.r[4]; // mov r4,r1
            cpu.r[1] >>= 16; // shlr16 r1
            cpu.r[6] = cpu.r[4]; // mov r4,r6
            cpu.r[5] = cpu.r[4]; // mov r4,r5
            cpu.r[1] >>= 8; // shlr8 r1
            cpu.r[6] >>= 16; // shlr16 r6
            cpu.r[1] &= 0xFF; // extu.b r1,r1
            cpu.r[6] &= 0xFF; // extu.b r6,r6
            cpu.r[5] >>= 8; // shlr8 r5
            cpu.r[7] = cpu.r[6] & 0xFF; // extu.b r6,r7
            cpu.r[5] &= 0xFF; // extu.b r5,r5
            cpu.r[4] &= 0xFF; // extu.b r4,r4
            cpu.r[1] &= 0xFF; // extu.b r1,r1
            cpu.r[14] = cpu.r[3]; // mov r3,r14
            cpu.r[5] &= 0xFF; // extu.b r5,r5
            cpu.r[6] = cpu.r[3]; // mov r3,r6
            cpu.r[4] &= 0xFF; // extu.b r4,r4
            cpu.fr[3] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr3 (255.0)
            cpu.fpul = cpu.r[1]; // lds r1,fpul
            cpu.fr[0] = cpu.fpul as i32 as f32; // float fpul,fr0
            cpu.fpul = cpu.r[7]; // lds r7,fpul
            cpu.fr[0] = cpu.fr[0] / cpu.fr[3]; // fdiv fr3,fr0
            let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // fmov fr0,@r3
            cpu.fr[2] = cpu.fpul as i32 as f32; // float fpul,fr2
            cpu.fpul = cpu.r[5]; // lds r5,fpul
            cpu.fr[1] = cpu.fpul as i32 as f32; // float fpul,fr1
            cpu.fpul = cpu.r[4]; // lds r4,fpul
            cpu.r[14] = cpu.r[14].wrapping_add(4); // add #4,r14
            cpu.fr[2] = cpu.fr[2] / cpu.fr[3]; // fdiv fr3,fr2
            let _ = mem.write_f32(cpu.r[14], cpu.fr[2]); // fmov fr2,@r14
            cpu.fr[0] = cpu.fpul as i32 as f32; // float fpul,fr0
            cpu.r[6] = cpu.r[6].wrapping_add(8); // add #8,r6
            cpu.fr[1] = cpu.fr[1] / cpu.fr[3]; // fdiv fr3,fr1
            let _ = mem.write_f32(cpu.r[6], cpu.fr[1]); // fmov fr1,@r6
            cpu.r[3] = cpu.r[3].wrapping_add(12); // add #12,r3
            cpu.fr[0] = cpu.fr[0] / cpu.fr[3]; // fdiv fr3,fr0
            // bra 0x8c0fdbbc; delay slot stores the last word
            let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // fmov fr0,@r3 (delay)
        } else {
            // PATH C, 0x8c0fdb92: copy [r5+32..r5+44] to [arg..arg+12]
            cpu.r[0] = cpu.r[5].wrapping_add(32); // mov r5,r0 ; add #32,r0
            cpu.r[4] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap(); // mov.l @(4,r15),r4
            cpu.r[1] = cpu.r[5]; // mov r5,r1
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
            cpu.r[3] = cpu.r[4]; // mov r4,r3
            cpu.r[1] = cpu.r[1].wrapping_add(36); // add #36,r1
            let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // fmov fr0,@r4
            cpu.r[0] = cpu.r[5]; // mov r5,r0
            cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap(); // fmov @r1,fr0
            cpu.r[3] = cpu.r[3].wrapping_add(4); // add #4,r3
            cpu.r[0] = cpu.r[0].wrapping_add(40); // add #40,r0
            let _ = mem.write_f32(cpu.r[3], cpu.fr[0]); // fmov fr0,@r3
            cpu.r[1] = cpu.r[4]; // mov r4,r1
            cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap(); // fmov @r0,fr0
            cpu.r[1] = cpu.r[1].wrapping_add(8); // add #8,r1
            cpu.r[5] = cpu.r[5].wrapping_add(44); // add #44,r5
            let _ = mem.write_f32(cpu.r[1], cpu.fr[0]); // fmov fr0,@r1
            cpu.r[4] = cpu.r[4].wrapping_add(12); // add #12,r4
            cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap(); // fmov @r5,fr0
            let _ = mem.write_f32(cpu.r[4], cpu.fr[0]); // fmov fr0,@r4
        }
    }
    // EPI, 0x8c0fdbbc: rts; delay pop r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
