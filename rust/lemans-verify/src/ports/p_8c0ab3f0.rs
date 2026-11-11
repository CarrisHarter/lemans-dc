//! 0x8c0ab3f0 — mode switch on the caller's r5, stored first at [r4+0x6c].
//! Entry writes: [r4+0x6c] = r5 (32-bit), byte [r4+0x84] = 1, float
//! [r4+0x88] = 0.0 (fldi0 fr1). Then the value at [r4+0x6c] selects:
//!   0: byte [r4+0x74] ? [r4+0x7c]=0 : [r4+0x7c]=180.0 (f32 @0x8c0ab4d8);
//!      [r4+0x80] = 90.0 (f32 @0x8c0ab4d0); [r4+0x8c] = 4.0 (@0x8c0ab4dc)
//!   1: t = f32[r4+0x80]; [r4+0x7c] = t; [r4+0x80] = 0.0; [r4+0x8c] = 2.0
//!   2: t = f32[r4+0x80]; [r4+0x7c] = t; [r4+0x80] = 90.0; [r4+0x8c] = 4.0
//!   3: byte [r4+0x84] = 0 (rts delay slot)
//!   other: same stores as case 2.
//! All case tails do `r4 += [0x8c0ab4e0]` (0x8c) and store fr0 through the
//! rts delay slot, so the last store lands at [r4_old+0x8c].
//!
//! Hidden side effects the C does not show: r0/r1/r3 are left holding the
//! last pointer constants (e.g. r0 = 0x8c0ab4dc, r1 = 0x8c, r3 = r4+0x88,
//! r4 = r4+0x8c, r5 = 0x88, r6 = 1, r7 = 0x84); fr1 stays 0.0 from fldi0 and
//! fr0 holds the last table constant loaded; T ends as the byte test
//! (case 0), the matching cmp/eq (cases 1-3), or 0 (default case, whose
//! bra comes after a failed cmp/eq #3); the rts delay slot is what performs
//! the final store in every case except 3.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let obj = cpu.r[4];
    cpu.r[0] = 108; // mov #108,r0
    mem.write_u32(obj.wrapping_add(cpu.r[0]), cpu.r[5]).unwrap(); // mov.l r5,@(r0,r4)
    cpu.r[1] = obj; // mov r4,r1
    cpu.r[7] = mem.read_u32(0x8c0ab4c8).unwrap(); // 0x84
    cpu.r[6] = 1; // mov #1,r6
    cpu.r[5] = mem.read_u32(0x8c0ab4cc).unwrap(); // 0x88
    cpu.r[0] = cpu.r[7]; // mov r7,r0
    cpu.r[3] = obj; // mov r4,r3
    mem.write_u8(obj.wrapping_add(cpu.r[0]), cpu.r[6] as u8).unwrap(); // mov.b r6,@(r0,r4)
    cpu.r[3] = obj.wrapping_add(cpu.r[5]); // add r5,r3
    cpu.fr[1] = 0.0; // fldi0 fr1
    mem.write_f32(cpu.r[3], cpu.fr[1]).unwrap(); // fmov fr1,@r3
    cpu.r[1] = obj.wrapping_add(108); // add #108,r1
    cpu.r[0] = mem.read_u32(cpu.r[1]).unwrap(); // mov.l @r1,r0
    let mode = cpu.r[0];

    cpu.set_t(mode == 0); // tst r0,r0
    if cpu.t() {
        // ---- case 0, 0x8c0ab420 ----
        let b = mem.read_u8(obj.wrapping_add(116)).unwrap() as u32; // extu.b
        cpu.set_t(b == 0); // tst r0,r0
        if cpu.t() {
            // byte == 0: 0x8c0ab442
            cpu.fr[0] = mem.read_f32(0x8c0ab4d8).unwrap(); // 180.0
            mem.write_f32(obj.wrapping_add(124), cpu.fr[0]).unwrap();
            cpu.fr[0] = mem.read_f32(0x8c0ab4d0).unwrap(); // 90.0
            cpu.r[1] = mem.read_u32(0x8c0ab4d4).unwrap(); // 0x80
            let r0c = obj.wrapping_add(cpu.r[1]);
            mem.write_f32(r0c, cpu.fr[0]).unwrap();
            cpu.r[0] = 0x8c0ab4d0;
        } else {
            // byte != 0: 0x8c0ab42e
            mem.write_f32(obj.wrapping_add(124), cpu.fr[1]).unwrap(); // 0.0
            cpu.fr[0] = mem.read_f32(0x8c0ab4d0).unwrap(); // 90.0
            cpu.r[1] = mem.read_u32(0x8c0ab4d4).unwrap(); // 0x80
            let r0c = obj.wrapping_add(cpu.r[1]);
            // bra 0x8c0ab458; delay slot stores fr0 through r0c
            mem.write_f32(r0c, cpu.fr[0]).unwrap();
            cpu.r[0] = 0x8c0ab4d0;
        }
        // ---- common tail of case 0, 0x8c0ab458 ----
        cpu.fr[0] = mem.read_f32(0x8c0ab4dc).unwrap(); // 4.0
        cpu.r[0] = mem.read_u32(0x8c0ab4e0).unwrap(); // 0x8c
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[0]); // add r0,r4
        mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap(); // rts delay: fmov fr0,@r4
        return;
    }
    cpu.set_t(mode == 1); // cmp/eq #1,r0
    if cpu.t() {
        // ---- case 1, 0x8c0ab464 ----
        cpu.r[0] = mem.read_u32(0x8c0ab4d4).unwrap(); // 0x80
        cpu.r[3] = obj;
        cpu.r[1] = mem.read_u32(0x8c0ab4e0).unwrap(); // 0x8c
        cpu.r[3] = cpu.r[3].wrapping_add(cpu.r[0]); // r4+0x80
        cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
        cpu.r[0] = obj.wrapping_add(124);
        mem.write_f32(cpu.r[0], cpu.fr[0]).unwrap(); // [r4+0x7c] = old [r4+0x80]
        cpu.r[4] = cpu.r[4].wrapping_add(cpu.r[1]); // r4 += 0x8c
        mem.write_f32(cpu.r[3], cpu.fr[1]).unwrap(); // [r4+0x80] = 0.0
        cpu.r[0] = 0x8c0ab4e4; // mova
        cpu.fr[0] = mem.read_f32(0x8c0ab4e4).unwrap(); // 2.0
        mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap(); // rts delay: fmov fr0,@r4
        return;
    }
    cpu.set_t(mode == 2); // cmp/eq #2,r0
    if cpu.t() {
        case2_default(cpu, mem, obj);
        return;
    }
    cpu.set_t(mode == 3); // cmp/eq #3,r0
    if cpu.t() {
        // ---- case 3, 0x8c0ab4a0 ----
        cpu.r[0] = cpu.r[7]; // mov r7,r0 -> 0x84
        cpu.r[1] = 0; // mov #0,r1
        mem.write_u8(obj.wrapping_add(cpu.r[0]), cpu.r[1] as u8).unwrap(); // rts delay
        return;
    }
    // ---- default case, 0x8c0ab4a8 (T stays 0 from the failed cmp/eq #3) ----
    case2_default(cpu, mem, obj);
}

fn case2_default(cpu: &mut Cpu, mem: &mut Memory, obj: u32) {
    // ---- case 2 (0x8c0ab480) and default (0x8c0ab4a8) are identical ----
    cpu.r[0] = mem.read_u32(0x8c0ab4d4).unwrap(); // 0x80
    cpu.r[3] = obj.wrapping_add(cpu.r[0]);
    cpu.r[1] = obj;
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap();
    cpu.r[1] = cpu.r[1].wrapping_add(124);
    cpu.r[0] = 0x8c0ab4d0; // mova
    mem.write_f32(cpu.r[1], cpu.fr[0]).unwrap(); // [r4+0x7c] = old [r4+0x80]
    cpu.r[1] = mem.read_u32(0x8c0ab4e0).unwrap(); // 0x8c
    cpu.fr[0] = mem.read_f32(0x8c0ab4d0).unwrap(); // 90.0
    cpu.r[4] = obj.wrapping_add(cpu.r[1]);
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap(); // [r4+0x80] = 90.0
    cpu.r[0] = 0x8c0ab4dc; // mova
    cpu.fr[0] = mem.read_f32(0x8c0ab4dc).unwrap(); // 4.0
    mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap(); // rts delay: fmov fr0,@r4
}
