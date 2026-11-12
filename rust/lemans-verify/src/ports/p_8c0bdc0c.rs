//! 0x8c0bdc0c — scan 108-byte records, returning the first that passes the
//! plane/ray test from 0x8c0c0328.
//!
//! ```text
//! push r14,r13,r12,r11,pr ; add #-8,r15
//! r13 = param_3 ; r12 = param_2 ; r11 = param_1
//! bra 0x8c0bdc64 (delay: r14 = 0)                    ; i = 0
//! 0x8c0bdc64: r0 = *(r11+12) ; cmp/hi r14,r0 ; bt body ; else r0 = -1, exit
//! body: r1 = 108*i ; mul.l (MACH/MACL) ; fr0 = 0.0
//!       *(r15) = 0.0 ; *(r15+4) = 0.0                 ; two float out-slots
//!       r4 = *(r11+16) + i*108
//!       push &slot1 ; push &slot0
//!       r5 = r4+20 ; r7 = r13 ; r6 = r12 ; jsr 0x8c0c0328 (delay: r4 += 8)
//!       r15 += 8 ; extu.b r0,r0 ; tst r0,r0
//!       bt 0x8c0bdc62                                 ; not found → i++, loop
//!       else r0 = i (bra delay) → exit
//! 0x8c0bdc6c: add #8,r15 ; pop pr,r11,r12,r13 ; rts ; pop r14
//! ```
//!
//! The callee advances r4..r7 by 4 and leaves its FPU intermediates behind;
//! the wrapper reloads r6/r7 from r12/r13 and recomputes r4/r5 from the table
//! on every iteration, so those advances do not accumulate. `mul.l` leaves the
//! scaled index in MACH/MACL (`r0` is 108, so the high word only matters for
//! negative `i`). Both exits leave T clear: the last flag-setting instruction
//! is `tst` with a nonzero result or the failing `cmp/hi`.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // mov.l r14,@-r15 ; mov.l r13,@-r15 ; mov.l r12,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]);
    // mov r6,r13 ; mov.l r11,@-r15 ; mov r5,r12
    cpu.r[13] = cpu.r[6];
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[11]);
    cpu.r[12] = cpu.r[5];
    // sts.l pr,@-r15 ; add #-8,r15 ; mov r4,r11
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    cpu.r[15] = cpu.r[15].wrapping_sub(8);
    cpu.r[11] = cpu.r[4];
    // bra 0x8c0bdc64 ; mov #0,r14 (delay slot)
    cpu.r[14] = 0;

    loop {
        // 0x8c0bdc64: mov.l @(12,r11),r0 ; cmp/hi r14,r0 ; bt body
        cpu.r[0] = mem.read_u32(cpu.r[11].wrapping_add(12)).unwrap();
        let t = cpu.r[0] > cpu.r[14];
        cpu.set_t(t);
        if !t {
            cpu.r[0] = 0xffff_ffff; // mov #-1,r0
            break;
        }
        // 0x8c0bdc22: mov r14,r1 ; mov #108,r0 ; mul.l r0,r1
        cpu.r[1] = cpu.r[14];
        cpu.r[0] = 108;
        let p = (cpu.r[1] as i32 as i64) * (cpu.r[0] as i32 as i64);
        cpu.mach = (p >> 32) as u32;
        cpu.macl = p as u32;
        // fldi0 fr0 ; sts macl,r1
        cpu.fr[0] = 0.0;
        cpu.r[1] = cpu.macl;
        // mov #0,r2 ; add r15,r2 ; fmov fr0,@r2
        cpu.r[2] = 0;
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
        let _ = mem.write_u32(cpu.r[2], cpu.fr[0].to_bits());
        // mov r15,r0 ; mov #4,r2 ; add r15,r2 ; fmov fr0,@r2
        cpu.r[0] = cpu.r[15];
        cpu.r[2] = 4;
        cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]);
        let _ = mem.write_u32(cpu.r[2], cpu.fr[0].to_bits());
        // add #4,r0
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        // mov.l @(16,r11),r4 ; add r1,r4
        cpu.r[4] = mem.read_u32(cpu.r[11].wrapping_add(16)).unwrap();
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]);
        // mov.l r0,@-r15 (slot1 pointer) ; mov r15,r0 ; add #4,r0
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        cpu.r[0] = cpu.r[15];
        cpu.r[0] = cpu.r[0].wrapping_add(4);
        // mov.l r0,@-r15 (slot0 pointer)
        cpu.r[15] = cpu.r[15].wrapping_sub(4);
        let _ = mem.write_u32(cpu.r[15], cpu.r[0]);
        // mov r4,r5 ; mov.l 0x8c0bdc7c,r0 ; mov r13,r7 ; mov r12,r6
        cpu.r[5] = cpu.r[4];
        cpu.r[0] = 0x8c0c0328;
        cpu.r[7] = cpu.r[13];
        cpu.r[6] = cpu.r[12];
        // add #20,r5
        cpu.r[5] = cpu.r[5].wrapping_add(20);
        // jsr @r0 ; add #8,r4 (delay slot)
        cpu.r[4] = cpu.r[4].wrapping_add(8);
        cpu.pr = 0x8c0bdc56;
        crate::registry::p_8c0c0328::run(cpu, mem);
        // add #8,r15 ; extu.b r0,r0 ; tst r0,r0
        cpu.r[15] = cpu.r[15].wrapping_add(8);
        cpu.r[0] &= 0xff;
        let t = cpu.r[0] == 0;
        cpu.set_t(t);
        if !t {
            // bra 0x8c0bdc6c ; mov r14,r0 (delay slot)
            cpu.r[0] = cpu.r[14];
            break;
        }
        // bt 0x8c0bdc62: add #1,r14
        cpu.r[14] = cpu.r[14].wrapping_add(1);
    }

    // 0x8c0bdc6c: add #8,r15 ; lds.l @r15+,pr ; pop r11,r12,r13 ; rts
    cpu.r[15] = cpu.r[15].wrapping_add(8);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // delay slot: mov.l @r15+,r14
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap();
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
