//! 0x8c014810 — classify fr4 (sign/exponent/mantissa) and scale by 2^r4,
//! returning a float in fr0; saturation to ±Inf / ±0.0 at the range ends.
//!
//! ```text
//! mov.l r14,@-r15 ; add #-4,r15 ; mov r15,r6
//! r2=0x8c1ccfa0 r3=0x7f800000 r1=0x8c1ccfd0 r5=0x80000000   (literal pool)
//! fmov fr4,@r6 ; r7=[r6] ; fr7=f32[0x8c1ccfa0](+Inf) ; r14=[r6]
//! r7 &= 0x7f800000 ; T = (r7 == 0x7f800000) ; bf.s normal ; fmov @r1,fr5 (-Inf, delay)
//! ; exp==0xff: r0=0x7fffff ; T=(r14&r0)==0 ; bf NaN
//! ;   T=(r14&r5)==0 ; bf -Inf(fr5) ; else +Inf(fr7)
//! ; NaN: r3=0x8c1ccf98 ; fr0=f32[0x8c1ccf98]
//! normal: r2=0x8c1ccf9c ; T=r7>0 ; r3=0x8c1ccfcc ; fr4=f32[r2](0.0) ; bt.s has delay fr6=f32[r3](-0.0)
//! ; exp!=0: r7=(r7>>23)+r4 ; if 0<r7<255 → build sign|((exp+r4)<<23)|mantissa into fr0
//! ;   else clamp by sign of r4 and sign of input to fr7/fr5/fr6/fr4
//! ; exp==0: clamp to fr6(-0.0) if sign else fr4(0.0)
//! every exit: add #4,r15 ; rts ; mov.l @r15+,r14
//! ```
//!
//! Hidden side effects the C does not show: fr4 is overwritten with 0.0 on the
//! normal path, fr5 = -Inf and fr6 = -0.0 are loaded unconditionally (delay
//! slots), fr7 = +Inf always, r0 = 0x7fffff only on the Inf/NaN path,
//! r1/r2/r3/r5/r6/r7 are literal-pool leftovers, r14 is saved/restored through
//! the stack, and T ends as the last tst/cmp of the taken path (not always 0).

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let saved_r14 = cpu.r[14];
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // mov.l r14,@-r15 (stack store elided)
    cpu.r[15] = cpu.r[15].wrapping_sub(4); // add #-4,r15
    cpu.r[6] = cpu.r[15];
    cpu.r[2] = mem.read_u32(0x8c0148b8).unwrap(); // 0x8c1ccfa0
    cpu.r[3] = mem.read_u32(0x8c0148b0).unwrap(); // 0x7f800000
    cpu.r[1] = mem.read_u32(0x8c0148bc).unwrap(); // 0x8c1ccfd0
    cpu.r[5] = mem.read_u32(0x8c0148b4).unwrap(); // 0x80000000
    let bits = cpu.fr[4].to_bits(); // fmov fr4,@r6 ; mov.l @r6,r7 ; mov.l @r6,r14
    cpu.r[7] = bits;
    cpu.fr[7] = mem.read_f32(cpu.r[2]).unwrap(); // fmov @r2,fr7 = +Inf
    cpu.r[14] = bits;
    cpu.r[7] &= cpu.r[3]; // and r3,r7
    cpu.set_t(cpu.r[7] == cpu.r[3]); // cmp/eq r3,r7
    cpu.fr[5] = mem.read_f32(cpu.r[1]).unwrap(); // bf.s delay slot: fmov @r1,fr5 = -Inf

    macro_rules! epilogue {
        () => {{
            cpu.r[15] = cpu.r[15].wrapping_add(4); // add #4,r15
            cpu.r[14] = saved_r14; // rts delay slot: mov.l @r15+,r14
            cpu.r[15] = cpu.r[15].wrapping_add(4);
        }};
    }

    if cpu.t() {
        // exponent field == 0x7f800000: Inf or NaN
        cpu.r[0] = mem.read_u32(0x8c0148c0).unwrap(); // 0x007fffff
        cpu.set_t(cpu.r[14] & cpu.r[0] == 0); // tst r14,r0
        if !cpu.t() {
            cpu.r[3] = mem.read_u32(0x8c0148c4).unwrap(); // 0x8c1ccf98
            cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap(); // NaN
            epilogue!();
            return;
        }
        cpu.set_t(cpu.r[14] & cpu.r[5] == 0); // tst r14,r5
        if !cpu.t() {
            cpu.fr[0] = cpu.fr[5]; // -Inf
            epilogue!();
            return;
        }
        cpu.fr[0] = cpu.fr[7]; // +Inf
        epilogue!();
        return;
    }

    // exponent field != 0x7f800000
    cpu.r[2] = mem.read_u32(0x8c0148c8).unwrap(); // 0x8c1ccf9c
    cpu.set_t((cpu.r[7] as i32) > 0); // cmp/pl r7
    cpu.r[3] = mem.read_u32(0x8c0148cc).unwrap(); // 0x8c1ccfcc
    cpu.fr[4] = mem.read_f32(cpu.r[2]).unwrap(); // fmov @r2,fr4 = 0.0
    cpu.fr[6] = mem.read_f32(cpu.r[3]).unwrap(); // bt.s delay slot: fmov @r3,fr6 = -0.0
    if cpu.t() {
        // exponent != 0
        cpu.r[3] = (-23i32) as u32;
        cpu.r[7] = ((cpu.r[7] as i32) >> 23) as u32; // shad r3,r7
        cpu.r[7] = cpu.r[7].wrapping_add(cpu.r[4]); // add r4,r7
        cpu.set_t((cpu.r[7] as i32) > 0); // cmp/pl r7
        if cpu.t() {
            cpu.r[3] = mem.read_u16(0x8c0148ae).unwrap() as u16 as i32 as u32; // mov.w = 255
            cpu.set_t((cpu.r[7] as i32) >= (cpu.r[3] as i32)); // cmp/ge r3,r7
            if !cpu.t() {
                // in-range: rebuild fr4 with exponent+r4
                cpu.r[2] = mem.read_u32(0x8c0148c0).unwrap(); // 0x007fffff
                cpu.r[3] = 23;
                cpu.r[5] = cpu.r[14] & 0x8000_0000; // and r14,r5
                cpu.r[7] = cpu.r[7] << 23; // shad r3,r7
                cpu.r[2] = cpu.r[14] & cpu.r[2]; // and r14,r2
                cpu.r[5] |= cpu.r[7]; // or r7,r5
                cpu.r[5] |= cpu.r[2]; // or r2,r5
                cpu.fr[0] = f32::from_bits(cpu.r[5]); // mov.l r5,@r6 ; fmov @r6,fr0
                epilogue!();
                return;
            }
        }
        // exponent+r4 out of range: clamp by sign of r4, then sign of input
        cpu.set_t((cpu.r[4] as i32) >= 0); // cmp/pz r4
        if !cpu.t() {
            cpu.set_t(cpu.r[14] & cpu.r[5] == 0); // tst r14,r5
            if !cpu.t() {
                cpu.fr[0] = cpu.fr[6]; // -0.0
                epilogue!();
                return;
            }
            cpu.fr[0] = cpu.fr[4]; // 0.0
            epilogue!();
            return;
        }
        cpu.set_t(cpu.r[14] & cpu.r[5] == 0); // tst r14,r5
        if !cpu.t() {
            cpu.fr[0] = cpu.fr[5]; // -Inf
            epilogue!();
            return;
        }
        cpu.fr[0] = cpu.fr[7]; // +Inf
        epilogue!();
        return;
    }
    // exponent == 0 (zero / denormal)
    cpu.set_t(cpu.r[14] & cpu.r[5] == 0); // tst r14,r5
    if !cpu.t() {
        cpu.fr[0] = cpu.fr[6]; // -0.0
        epilogue!();
        return;
    }
    cpu.fr[0] = cpu.fr[4]; // 0.0
    epilogue!();
}
