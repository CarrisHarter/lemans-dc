//! 0x8c0b4764 — build a 36-byte transform record from a 12-word descriptor.
//!
//! `param_1` (r4) is the destination, `param_2` (r5) the source. The nine
//! leading source floats are the 3x3 block handed to 0x8c0b3b3c; that routine
//! writes four normal floats through its stack-passed out pointer, so the caller
//! stages the block at r15+16 and pushes the old r15 (the staging base) as the
//! pointer. The four results land at [r15+0..r15+12] and are copied to
//! `out+4..out+16`. Words 9..11 of the source go to `out+20..out+28`, a 1.0f
//! goes to out+32, and only byte out+0 is zeroed (`mov.b`), leaving out+1..3
//! and out+17..19 untouched.
//!
//! ```text
//! push r14 ; push r13 ; r14=r5 ; push pr ; add #-52,r15 ; r13=r4
//! [r15+16..48] = r14[0..8] (fmov pairs)
//! r0=r15 ; push r0 (out arg) ; r4=r15+20 (delay slot: add #20,r4)
//! jsr 0x8c0b3b3c ; r15+=4
//! out+4=[r15+0] out+8=[r15+4] out+12=[r15+8] out+16=[r15+12]
//! out+20..28 = [r14+36..44] ; r14+=36
//! fr0=1.0 ; out+32=fr0 ; out[0]=0 (byte)
//! add #52,r15 ; lds pr ; pop r13 ; rts ; delay: pop r14
//! ```
//!
//! r2 keeps the callee's last pointer (positive arm leaves it at r15+48 before
//! the push; the negative arm sets its own r15+24); r0=0, r1=out+32, r3=[src+44],
//! r4=[src+36], r5=out+20, r6=r15, r7=out+4, fr0=1.0f at the end.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 0x8c0b4764: mov.l r14,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    mem.write_u32(cpu.r[15], cpu.r[14]).unwrap();
    // 0x8c0b4766: mov.l r13,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    mem.write_u32(cpu.r[15], cpu.r[13]).unwrap();
    // 0x8c0b4768: mov r5,r14
    cpu.r[14] = cpu.r[5];
    // 0x8c0b476a: sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    mem.write_u32(cpu.r[15], cpu.pr).unwrap();
    // 0x8c0b476c: add #-52,r15
    cpu.r[15] = cpu.r[15].wrapping_sub(52);
    // 0x8c0b476e: mov r4,r13
    cpu.r[13] = cpu.r[4];

    // 0x8c0b4770: fmov @r14,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[14]).unwrap();
    // 0x8c0b4772: mov #16,r2
    cpu.r[2] = 16;
    // 0x8c0b4774: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b4776: mov r14,r1
    cpu.r[1] = cpu.r[14];
    // 0x8c0b4778: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();
    // 0x8c0b477a: add #4,r1
    cpu.r[1] = cpu.r[1].wrapping_add(4);
    // 0x8c0b477c: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c0b477e: mov #20,r2
    cpu.r[2] = 20;
    // 0x8c0b4780: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b4782: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0b4784: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();
    // 0x8c0b4786: add #8,r0
    cpu.r[0] = cpu.r[0].wrapping_add(8);
    // 0x8c0b4788: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c0b478a: mov #24,r2
    cpu.r[2] = 24;
    // 0x8c0b478c: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b478e: mov r14,r1
    cpu.r[1] = cpu.r[14];
    // 0x8c0b4790: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();
    // 0x8c0b4792: add #12,r1
    cpu.r[1] = cpu.r[1].wrapping_add(12);
    // 0x8c0b4794: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c0b4796: mov #28,r2
    cpu.r[2] = 28;
    // 0x8c0b4798: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b479a: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();
    // 0x8c0b479c: add #16,r5
    cpu.r[5] = cpu.r[5].wrapping_add(16);
    // 0x8c0b479e: mov #32,r2
    cpu.r[2] = 32;
    // 0x8c0b47a0: mov r14,r0
    cpu.r[0] = cpu.r[14];
    // 0x8c0b47a2: fmov @r5,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[5]).unwrap();
    // 0x8c0b47a4: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b47a6: add #20,r0
    cpu.r[0] = cpu.r[0].wrapping_add(20);
    // 0x8c0b47a8: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();
    // 0x8c0b47aa: mov r14,r4
    cpu.r[4] = cpu.r[14];
    // 0x8c0b47ac: fmov @r0,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[0]).unwrap();
    // 0x8c0b47ae: mov #36,r2
    cpu.r[2] = 36;
    // 0x8c0b47b0: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b47b2: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();
    // 0x8c0b47b4: add #24,r4
    cpu.r[4] = cpu.r[4].wrapping_add(24);
    // 0x8c0b47b6: mov #40,r2
    cpu.r[2] = 40;
    // 0x8c0b47b8: mov r14,r3
    cpu.r[3] = cpu.r[14];
    // 0x8c0b47ba: fmov @r4,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[4]).unwrap();
    // 0x8c0b47bc: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b47be: mov r14,r1
    cpu.r[1] = cpu.r[14];
    // 0x8c0b47c0: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();
    // 0x8c0b47c2: add #28,r3
    cpu.r[3] = cpu.r[3].wrapping_add(28);
    // 0x8c0b47c4: mov #44,r2
    cpu.r[2] = 44;
    // 0x8c0b47c6: fmov @r3,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    // 0x8c0b47c8: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b47ca: add #32,r1
    cpu.r[1] = cpu.r[1].wrapping_add(32);
    // 0x8c0b47cc: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();
    // 0x8c0b47ce: mov r15,r0
    cpu.r[0] = cpu.r[15];
    // 0x8c0b47d0: mov #48,r2
    cpu.r[2] = 48;
    // 0x8c0b47d2: fmov @r1,fr0
    cpu.fr[0] = mem.read_f32(cpu.r[1]).unwrap();
    // 0x8c0b47d4: add r15,r2
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
    // 0x8c0b47d6: fmov fr0,@r2
    mem.write_f32(cpu.r[2], cpu.fr[0]).unwrap();

    // 0x8c0b47d8: mov.l r0,@-r15  (stack arg = staging base, used as out)
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    mem.write_u32(cpu.r[15], cpu.r[0]).unwrap();
    // 0x8c0b47da: mov r15,r4
    cpu.r[4] = cpu.r[15];
    // 0x8c0b47dc: mov.l 0x8c0b4824,r0 (0x8c0b3b3c)
    cpu.r[0] = mem.read_u32(0x8c0b4824).unwrap();
    // 0x8c0b47de: jsr @r0 ; 0x8c0b47e0: add #20,r4 (delay slot)
    cpu.pr = 0x8c0b47e2;
    cpu.r[4] = cpu.r[4].wrapping_add(20);
    crate::registry::p_8c0b3b3c::run(cpu, mem);

    // 0x8c0b47e2: add #4,r15  (discard the stack argument)
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b47e4: mov r15,r6
    cpu.r[6] = cpu.r[15];
    // 0x8c0b47e6: mov r13,r7
    cpu.r[7] = cpu.r[13];
    // 0x8c0b47e8: mov.l @r6,r3
    cpu.r[3] = mem.read_u32(cpu.r[6]).unwrap();
    // 0x8c0b47ea: add #4,r7
    cpu.r[7] = cpu.r[7].wrapping_add(4);
    // 0x8c0b47ec: mov.l @(4,r6),r0
    cpu.r[0] = mem.read_u32(cpu.r[6].wrapping_add(4)).unwrap();
    // 0x8c0b47ee: mov r13,r5
    cpu.r[5] = cpu.r[13];
    // 0x8c0b47f0: mov r13,r1
    cpu.r[1] = cpu.r[13];
    // 0x8c0b47f2: mov.l r3,@r7
    mem.write_u32(cpu.r[7], cpu.r[3]).unwrap();
    // 0x8c0b47f4: add #36,r14
    cpu.r[14] = cpu.r[14].wrapping_add(36);
    // 0x8c0b47f6: mov.l r0,@(4,r7)
    mem.write_u32(cpu.r[7].wrapping_add(4), cpu.r[0]).unwrap();
    // 0x8c0b47f8: add #20,r5
    cpu.r[5] = cpu.r[5].wrapping_add(20);
    // 0x8c0b47fa: mov.l @(8,r6),r4
    cpu.r[4] = mem.read_u32(cpu.r[6].wrapping_add(8)).unwrap();
    // 0x8c0b47fc: add #32,r1
    cpu.r[1] = cpu.r[1].wrapping_add(32);
    // 0x8c0b47fe: mov.l @(12,r6),r3
    cpu.r[3] = mem.read_u32(cpu.r[6].wrapping_add(12)).unwrap();
    // 0x8c0b4800: fldi1 fr0
    cpu.fr[0] = 1.0;
    // 0x8c0b4802: mov #0,r0
    cpu.r[0] = 0;
    // 0x8c0b4804: mov.l r4,@(8,r7)
    mem.write_u32(cpu.r[7].wrapping_add(8), cpu.r[4]).unwrap();
    // 0x8c0b4806: mov.l r3,@(12,r7)
    mem.write_u32(cpu.r[7].wrapping_add(12), cpu.r[3]).unwrap();
    // 0x8c0b4808: mov.l @r14,r4
    cpu.r[4] = mem.read_u32(cpu.r[14]).unwrap();
    // 0x8c0b480a: mov.l @(4,r14),r3
    cpu.r[3] = mem.read_u32(cpu.r[14].wrapping_add(4)).unwrap();
    // 0x8c0b480c: mov.l r4,@r5
    mem.write_u32(cpu.r[5], cpu.r[4]).unwrap();
    // 0x8c0b480e: mov.l r3,@(4,r5)
    mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[3]).unwrap();
    // 0x8c0b4810: mov.l @(8,r14),r3
    cpu.r[3] = mem.read_u32(cpu.r[14].wrapping_add(8)).unwrap();
    // 0x8c0b4812: mov.l r3,@(8,r5)
    mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[3]).unwrap();
    // 0x8c0b4814: fmov fr0,@r1
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap();
    // 0x8c0b4816: mov.b r0,@r13
    mem.write_u8(cpu.r[13], cpu.r[0] as u8).unwrap();
    // 0x8c0b4818: add #52,r15
    cpu.r[15] = cpu.r[15].wrapping_add(52);
    // 0x8c0b481a: lds.l @r15+,pr
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b481c: mov.l @r15+,r13
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 0x8c0b481e: rts ; 0x8c0b4820: mov.l @r15+,r14 (delay slot)
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
