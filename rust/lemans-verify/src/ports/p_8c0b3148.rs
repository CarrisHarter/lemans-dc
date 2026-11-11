//! 0x8c0b3148 — 4x4-ish float matrix op: C(r4)[0..36) = f(A(r5), B(r6)),
//! with an aliased variant that stages results on the stack when r4==r5 or r4==r6.
//!
//! Literal transcription of 0x8c0b3148-0x8c0b34b9. Two code paths:
//!   path A (r4 != r5 && r4 != r6): dot products stored straight to r4, T=0 at exit;
//!   path B (r4 == r5 || r4 == r6): results staged at r15+48..r15+80, then copied
//!     to r4[0..36); T=1 at exit.
//!
//! Hidden side effects the C does not show: every scratch register's final value
//! (r0=36, r1/r2/r3 leftover pointers, r4/r5/r6 advanced by 28/32/32 in path A,
//! r5=r15+48 in path B), the stack slots r15+0..r15+40 holding pointers,
//! the delay-slot store `fmov fr3,@r4` after `bra`, and the per-path T bit.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.r[15] = cpu.r[15].wrapping_sub(4); let _ = mem.write_u32(cpu.r[15], cpu.r[14]); // 0x8c0b3148
    cpu.r[15] = cpu.r[15].wrapping_sub(4); let _ = mem.write_u32(cpu.r[15], cpu.r[13]); // 0x8c0b314a
    cpu.r[15] = cpu.r[15].wrapping_sub(4); let _ = mem.write_u32(cpu.r[15], cpu.r[12]); // 0x8c0b314c
    cpu.r[15] = cpu.r[15].wrapping_sub(4); let _ = mem.write_u32(cpu.r[15], cpu.r[11]); // 0x8c0b314e
    cpu.r[15] = cpu.r[15].wrapping_sub(4); let _ = mem.write_u32(cpu.r[15], cpu.r[10]); // 0x8c0b3150
    cpu.r[15] = cpu.r[15].wrapping_sub(4); let _ = mem.write_u32(cpu.r[15], cpu.r[9]); // 0x8c0b3152
    cpu.r[15] = cpu.r[15].wrapping_sub(4); let _ = mem.write_u32(cpu.r[15], cpu.r[8]); // 0x8c0b3154
    cpu.r[15] = cpu.r[15].wrapping_add((-84i32) as u32); // 0x8c0b3156
    cpu.set_t(cpu.r[4] == cpu.r[5]); // 0x8c0b3158 cmp/eq r4,r5
    if cpu.t() {
        // 0x8c0b315c bra 0x8c0b32fe (r4 == r5)
        path_b(cpu, mem);
    } else {
        cpu.set_t(cpu.r[4] == cpu.r[6]); // 0x8c0b3160 cmp/eq r4,r6
        if cpu.t() {
            // 0x8c0b3164 bra 0x8c0b32fe (r4 == r6)
            path_b(cpu, mem);
        } else {
            // 0x8c0b3162 bf 0x8c0b3168 (r4 != r5 && r4 != r6)
            path_a(cpu, mem);
        }
    }
    // epilogue 0x8c0b34a8-0x8c0b34b8 (bra target 0x8c0b34a8; rts delay slot pops r14)
    cpu.r[15] = cpu.r[15].wrapping_add(84); // 0x8c0b34a8
    cpu.r[8] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4); // 0x8c0b34aa
    cpu.r[9] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4); // 0x8c0b34ac
    cpu.r[10] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4); // 0x8c0b34ae
    cpu.r[11] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4); // 0x8c0b34b0
    cpu.r[12] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4); // 0x8c0b34b2
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4); // 0x8c0b34b4
    // rts  // 0x8c0b34b6
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap(); cpu.r[15] = cpu.r[15].wrapping_add(4); // 0x8c0b34b8
}

fn path_a(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[4] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap()); // 0x8c0b3168
    cpu.r[11] = cpu.r[5]; // 0x8c0b316a
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b316c
    cpu.r[10] = cpu.r[6]; // 0x8c0b316e
    cpu.r[9] = cpu.r[5]; // 0x8c0b3170
    cpu.r[8] = cpu.r[6]; // 0x8c0b3172
    cpu.r[11] = cpu.r[11].wrapping_add(12); // 0x8c0b3174
    cpu.r[10] = cpu.r[10].wrapping_add(4); // 0x8c0b3176
    cpu.r[9] = cpu.r[9].wrapping_add(24); // 0x8c0b3178
    cpu.r[7] = cpu.r[5]; // 0x8c0b317a
    cpu.r[8] = cpu.r[8].wrapping_add(8); // 0x8c0b317c
    cpu.fr[4] = cpu.fr[4] * cpu.fr[0]; // 0x8c0b317e
    cpu.r[12] = cpu.r[4]; // 0x8c0b3180
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[11]).unwrap()); // 0x8c0b3182
    cpu.r[7] = cpu.r[7].wrapping_add(4); // 0x8c0b3184
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b3186
    cpu.r[12] = cpu.r[12].wrapping_add(4); // 0x8c0b3188
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[9]).unwrap()); // 0x8c0b318a
    cpu.fr[3] = cpu.fr[3] * cpu.fr[2]; // 0x8c0b318c
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b318e
    cpu.r[3] = cpu.r[5]; // 0x8c0b3190
    cpu.fr[4] = cpu.fr[4] + cpu.fr[3]; // 0x8c0b3192
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b3194
    cpu.fr[4] = cpu.fr[4] + cpu.fr[1]; // 0x8c0b3196
    let _ = mem.write_f32(cpu.r[4], cpu.fr[4]); // 0x8c0b3198
    cpu.r[1] = cpu.r[5]; // 0x8c0b319a
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[7]).unwrap()); // 0x8c0b319c
    cpu.r[1] = cpu.r[1].wrapping_add(28); // 0x8c0b319e
    cpu.r[3] = cpu.r[3].wrapping_add(16); // 0x8c0b31a0
    let _ = mem.write_u32(cpu.r[15], cpu.r[12]); // 0x8c0b31a2
    cpu.r[2] = cpu.r[15]; // 0x8c0b31a4
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b31a6
    cpu.r[2] = cpu.r[2].wrapping_add(36); // 0x8c0b31a8
    cpu.r[0] = cpu.r[5]; // 0x8c0b31aa
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b31ac
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap()); // 0x8c0b31ae
    cpu.r[0] = cpu.r[0].wrapping_add(8); // 0x8c0b31b0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b31b2
    cpu.r[13] = cpu.r[5]; // 0x8c0b31b4
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap()); // 0x8c0b31b6
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b31b8
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b31ba
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b31bc
    cpu.r[13] = cpu.r[13].wrapping_add(20); // 0x8c0b31be
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b31c0
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b31c2
    let _ = mem.write_f32(cpu.r[12], cpu.fr[3]); // 0x8c0b31c4
    cpu.r[12] = cpu.r[5]; // 0x8c0b31c6
    cpu.r[12] = cpu.r[12].wrapping_add(32); // 0x8c0b31c8
    let _ = mem.write_u32(cpu.r[2], cpu.r[12]); // 0x8c0b31ca
    cpu.r[12] = cpu.r[4]; // 0x8c0b31cc
    cpu.r[2] = cpu.r[15]; // 0x8c0b31ce
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap()); // 0x8c0b31d0
    cpu.r[12] = cpu.r[12].wrapping_add(8); // 0x8c0b31d2
    cpu.r[2] = cpu.r[2].wrapping_add(40); // 0x8c0b31d4
    let _ = mem.write_u32(cpu.r[15].wrapping_add(4), cpu.r[12]); // 0x8c0b31d6
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b31d8
    cpu.r[12] = cpu.r[6]; // 0x8c0b31da
    cpu.r[12] = cpu.r[12].wrapping_add(12); // 0x8c0b31dc
    let _ = mem.write_u32(cpu.r[2], cpu.r[12]); // 0x8c0b31de
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b31e0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b31e2
    cpu.r[2] = cpu.r[15]; // 0x8c0b31e4
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[13]).unwrap()); // 0x8c0b31e6
    cpu.r[10] = cpu.r[6]; // 0x8c0b31e8
    cpu.r[10] = cpu.r[10].wrapping_add(16); // 0x8c0b31ea
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b31ec
    let _ = mem.write_u32(cpu.r[15].wrapping_add(12), cpu.r[10]); // 0x8c0b31ee
    cpu.r[2] = cpu.r[2].wrapping_add(36); // 0x8c0b31f0
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b31f2
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b31f4
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b31f6
    cpu.r[2] = cpu.r[15]; // 0x8c0b31f8
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b31fa
    cpu.r[8] = cpu.r[6]; // 0x8c0b31fc
    cpu.r[8] = cpu.r[8].wrapping_add(20); // 0x8c0b31fe
    cpu.r[2] = cpu.r[2].wrapping_add(16); // 0x8c0b3200
    let _ = mem.write_u32(cpu.r[2], cpu.r[8]); // 0x8c0b3202
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b3204
    cpu.r[2] = cpu.r[15]; // 0x8c0b3206
    cpu.r[8] = mem.read_u32(cpu.r[15].wrapping_add(4)).unwrap(); // 0x8c0b3208
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b320a
    cpu.r[2] = cpu.r[2].wrapping_add(20); // 0x8c0b320c
    let _ = mem.write_f32(cpu.r[8], cpu.fr[3]); // 0x8c0b320e
    cpu.r[8] = cpu.r[4]; // 0x8c0b3210
    cpu.r[8] = cpu.r[8].wrapping_add(12); // 0x8c0b3212
    let _ = mem.write_u32(cpu.r[15].wrapping_add(8), cpu.r[8]); // 0x8c0b3214
    cpu.r[8] = cpu.r[4]; // 0x8c0b3216
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap()); // 0x8c0b3218
    cpu.r[8] = cpu.r[8].wrapping_add(16); // 0x8c0b321a
    let _ = mem.write_u32(cpu.r[2], cpu.r[8]); // 0x8c0b321c
    cpu.r[8] = cpu.r[12]; // 0x8c0b321e
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3220
    cpu.r[2] = cpu.r[15]; // 0x8c0b3222
    cpu.r[2] = cpu.r[2].wrapping_add(24); // 0x8c0b3224
    cpu.r[8] = cpu.r[4]; // 0x8c0b3226
    cpu.r[8] = cpu.r[8].wrapping_add(20); // 0x8c0b3228
    let _ = mem.write_u32(cpu.r[2], cpu.r[8]); // 0x8c0b322a
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b322c
    cpu.r[8] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap(); // 0x8c0b322e
    cpu.r[2] = cpu.r[15]; // 0x8c0b3230
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[11]).unwrap()); // 0x8c0b3232
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3234
    cpu.r[2] = cpu.r[2].wrapping_add(16); // 0x8c0b3236
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[9]).unwrap()); // 0x8c0b3238
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b323a
    cpu.r[8] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b323c
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b323e
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3240
    cpu.r[2] = cpu.r[15]; // 0x8c0b3242
    cpu.r[2] = cpu.r[2].wrapping_add(16); // 0x8c0b3244
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b3246
    cpu.r[8] = mem.read_u32(cpu.r[15].wrapping_add(8)).unwrap(); // 0x8c0b3248
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b324a
    let _ = mem.write_f32(cpu.r[8], cpu.fr[3]); // 0x8c0b324c
    cpu.r[8] = cpu.r[12]; // 0x8c0b324e
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3250
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[7]).unwrap()); // 0x8c0b3252
    cpu.r[8] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap(); // 0x8c0b3254
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b3256
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap()); // 0x8c0b3258
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b325a
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap()); // 0x8c0b325c
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b325e
    cpu.r[8] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b3260
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b3262
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3264
    cpu.r[2] = cpu.r[15]; // 0x8c0b3266
    cpu.r[2] = cpu.r[2].wrapping_add(20); // 0x8c0b3268
    cpu.r[8] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b326a
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b326c
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b326e
    let _ = mem.write_f32(cpu.r[8], cpu.fr[3]); // 0x8c0b3270
    cpu.r[2] = cpu.r[15]; // 0x8c0b3272
    cpu.r[8] = cpu.r[12]; // 0x8c0b3274
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3276
    cpu.r[2] = cpu.r[2].wrapping_add(16); // 0x8c0b3278
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap()); // 0x8c0b327a
    cpu.r[8] = mem.read_u32(cpu.r[15].wrapping_add(12)).unwrap(); // 0x8c0b327c
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b327e
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[13]).unwrap()); // 0x8c0b3280
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3282
    cpu.r[8] = cpu.r[10]; // 0x8c0b3284
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3286
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b3288
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b328a
    cpu.r[8] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b328c
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b328e
    cpu.r[2] = cpu.r[15]; // 0x8c0b3290
    cpu.r[2] = cpu.r[2].wrapping_add(24); // 0x8c0b3292
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b3294
    cpu.r[8] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b3296
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b3298
    let _ = mem.write_f32(cpu.r[8], cpu.fr[3]); // 0x8c0b329a
    cpu.r[10] = cpu.r[6]; // 0x8c0b329c
    cpu.r[10] = cpu.r[10].wrapping_add(24); // 0x8c0b329e
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap()); // 0x8c0b32a0
    cpu.r[8] = cpu.r[6]; // 0x8c0b32a2
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b32a4
    cpu.r[8] = cpu.r[8].wrapping_add(28); // 0x8c0b32a6
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[11]).unwrap()); // 0x8c0b32a8
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b32aa
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b32ac
    cpu.r[6] = cpu.r[6].wrapping_add(32); // 0x8c0b32ae
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[9]).unwrap()); // 0x8c0b32b0
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b32b2
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b32b4
    cpu.r[12] = cpu.r[4]; // 0x8c0b32b6
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b32b8
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b32ba
    cpu.r[12] = cpu.r[12].wrapping_add(24); // 0x8c0b32bc
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b32be
    let _ = mem.write_f32(cpu.r[12], cpu.fr[3]); // 0x8c0b32c0
    cpu.r[5] = cpu.r[4]; // 0x8c0b32c2
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[7]).unwrap()); // 0x8c0b32c4
    cpu.r[5] = cpu.r[5].wrapping_add(28); // 0x8c0b32c6
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b32c8
    cpu.r[4] = cpu.r[4].wrapping_add(32); // 0x8c0b32ca
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap()); // 0x8c0b32cc
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b32ce
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b32d0
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap()); // 0x8c0b32d2
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b32d4
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b32d6
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b32d8
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b32da
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b32dc
    let _ = mem.write_f32(cpu.r[5], cpu.fr[3]); // 0x8c0b32de
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap()); // 0x8c0b32e0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b32e2
    cpu.r[0] = 36; // 0x8c0b32e4
    cpu.r[1] = mem.read_u32(cpu.r[15].wrapping_add(cpu.r[0])).unwrap(); // 0x8c0b32e6
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b32e8
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[13]).unwrap()); // 0x8c0b32ea
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b32ec
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap()); // 0x8c0b32ee
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b32f0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b32f2
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b32f4
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b32f6
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b32f8
    // bra 0x8c0b34a8 // 0x8c0b32fa
    let _ = mem.write_f32(cpu.r[4], cpu.fr[3]); // 0x8c0b32fc
}

fn path_b(cpu: &mut Cpu, mem: &mut Memory) {
    cpu.fr[4] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap()); // 0x8c0b32fe
    cpu.r[11] = cpu.r[5]; // 0x8c0b3300
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b3302
    cpu.r[12] = cpu.r[6]; // 0x8c0b3304
    cpu.r[13] = cpu.r[5]; // 0x8c0b3306
    cpu.r[14] = cpu.r[6]; // 0x8c0b3308
    cpu.r[11] = cpu.r[11].wrapping_add(12); // 0x8c0b330a
    cpu.r[12] = cpu.r[12].wrapping_add(4); // 0x8c0b330c
    cpu.r[13] = cpu.r[13].wrapping_add(24); // 0x8c0b330e
    cpu.r[2] = 48; // 0x8c0b3310
    cpu.r[7] = cpu.r[5]; // 0x8c0b3312
    cpu.r[14] = cpu.r[14].wrapping_add(8); // 0x8c0b3314
    cpu.fr[4] = cpu.fr[4] * cpu.fr[0]; // 0x8c0b3316
    cpu.r[3] = cpu.r[5]; // 0x8c0b3318
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[11]).unwrap()); // 0x8c0b331a
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b331c
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[12]).unwrap()); // 0x8c0b331e
    cpu.r[1] = cpu.r[5]; // 0x8c0b3320
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[13]).unwrap()); // 0x8c0b3322
    cpu.fr[3] = cpu.fr[3] * cpu.fr[2]; // 0x8c0b3324
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[14]).unwrap()); // 0x8c0b3326
    cpu.r[10] = cpu.r[6]; // 0x8c0b3328
    cpu.fr[4] = cpu.fr[4] + cpu.fr[3]; // 0x8c0b332a
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b332c
    cpu.fr[4] = cpu.fr[4] + cpu.fr[1]; // 0x8c0b332e
    let _ = mem.write_f32(cpu.r[2], cpu.fr[4]); // 0x8c0b3330
    cpu.r[7] = cpu.r[7].wrapping_add(4); // 0x8c0b3332
    cpu.r[3] = cpu.r[3].wrapping_add(16); // 0x8c0b3334
    cpu.r[1] = cpu.r[1].wrapping_add(28); // 0x8c0b3336
    cpu.r[2] = cpu.r[15]; // 0x8c0b3338
    cpu.r[0] = cpu.r[5]; // 0x8c0b333a
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[7]).unwrap()); // 0x8c0b333c
    cpu.r[10] = cpu.r[10].wrapping_add(12); // 0x8c0b333e
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b3340
    cpu.r[2] = cpu.r[2].wrapping_add(44); // 0x8c0b3342
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap()); // 0x8c0b3344
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b3346
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[12]).unwrap()); // 0x8c0b3348
    cpu.r[9] = cpu.r[5]; // 0x8c0b334a
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap()); // 0x8c0b334c
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b334e
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[14]).unwrap()); // 0x8c0b3350
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b3352
    cpu.r[8] = cpu.r[5]; // 0x8c0b3354
    let _ = mem.write_u32(cpu.r[2], cpu.r[10]); // 0x8c0b3356
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b3358
    cpu.r[2] = 52; // 0x8c0b335a
    cpu.r[10] = cpu.r[6]; // 0x8c0b335c
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b335e
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b3360
    let _ = mem.write_f32(cpu.r[2], cpu.fr[3]); // 0x8c0b3362
    cpu.r[10] = cpu.r[10].wrapping_add(16); // 0x8c0b3364
    cpu.r[2] = cpu.r[15]; // 0x8c0b3366
    cpu.r[2] = cpu.r[2].wrapping_add(28); // 0x8c0b3368
    let _ = mem.write_u32(cpu.r[2], cpu.r[10]); // 0x8c0b336a
    cpu.r[0] = cpu.r[0].wrapping_add(8); // 0x8c0b336c
    cpu.r[10] = cpu.r[6]; // 0x8c0b336e
    cpu.r[2] = cpu.r[15]; // 0x8c0b3370
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap()); // 0x8c0b3372
    cpu.r[10] = cpu.r[10].wrapping_add(20); // 0x8c0b3374
    cpu.r[2] = cpu.r[2].wrapping_add(32); // 0x8c0b3376
    let _ = mem.write_u32(cpu.r[2], cpu.r[10]); // 0x8c0b3378
    cpu.r[9] = cpu.r[9].wrapping_add(20); // 0x8c0b337a
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b337c
    cpu.r[8] = cpu.r[8].wrapping_add(32); // 0x8c0b337e
    cpu.r[2] = 56; // 0x8c0b3380
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b3382
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[9]).unwrap()); // 0x8c0b3384
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b3386
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[12]).unwrap()); // 0x8c0b3388
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b338a
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b338c
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[14]).unwrap()); // 0x8c0b338e
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b3390
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b3392
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b3394
    let _ = mem.write_f32(cpu.r[2], cpu.fr[3]); // 0x8c0b3396
    cpu.r[2] = cpu.r[15]; // 0x8c0b3398
    cpu.r[2] = cpu.r[2].wrapping_add(44); // 0x8c0b339a
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b339c
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap()); // 0x8c0b339e
    cpu.r[2] = cpu.r[15]; // 0x8c0b33a0
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b33a2
    cpu.r[2] = cpu.r[2].wrapping_add(28); // 0x8c0b33a4
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[11]).unwrap()); // 0x8c0b33a6
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b33a8
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b33aa
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[13]).unwrap()); // 0x8c0b33ac
    cpu.r[2] = cpu.r[15]; // 0x8c0b33ae
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b33b0
    cpu.r[2] = cpu.r[2].wrapping_add(32); // 0x8c0b33b2
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b33b4
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b33b6
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b33b8
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b33ba
    cpu.r[2] = 60; // 0x8c0b33bc
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b33be
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b33c0
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b33c2
    let _ = mem.write_f32(cpu.r[2], cpu.fr[3]); // 0x8c0b33c4
    cpu.r[2] = cpu.r[15]; // 0x8c0b33c6
    cpu.r[2] = cpu.r[2].wrapping_add(44); // 0x8c0b33c8
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b33ca
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[7]).unwrap()); // 0x8c0b33cc
    cpu.r[2] = cpu.r[15]; // 0x8c0b33ce
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b33d0
    cpu.r[2] = cpu.r[2].wrapping_add(28); // 0x8c0b33d2
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap()); // 0x8c0b33d4
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b33d6
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b33d8
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap()); // 0x8c0b33da
    cpu.r[2] = cpu.r[15]; // 0x8c0b33dc
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b33de
    cpu.r[2] = cpu.r[2].wrapping_add(32); // 0x8c0b33e0
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b33e2
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b33e4
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b33e6
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b33e8
    cpu.r[2] = 64; // 0x8c0b33ea
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b33ec
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b33ee
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b33f0
    let _ = mem.write_f32(cpu.r[2], cpu.fr[3]); // 0x8c0b33f2
    cpu.r[2] = cpu.r[15]; // 0x8c0b33f4
    cpu.r[2] = cpu.r[2].wrapping_add(44); // 0x8c0b33f6
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b33f8
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap()); // 0x8c0b33fa
    cpu.r[2] = cpu.r[15]; // 0x8c0b33fc
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b33fe
    cpu.r[2] = cpu.r[2].wrapping_add(28); // 0x8c0b3400
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[9]).unwrap()); // 0x8c0b3402
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b3404
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b3406
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3408
    cpu.r[2] = cpu.r[15]; // 0x8c0b340a
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b340c
    cpu.r[2] = cpu.r[2].wrapping_add(32); // 0x8c0b340e
    cpu.r[10] = mem.read_u32(cpu.r[2]).unwrap(); // 0x8c0b3410
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b3412
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b3414
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b3416
    cpu.r[2] = 68; // 0x8c0b3418
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b341a
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b341c
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b341e
    let _ = mem.write_f32(cpu.r[2], cpu.fr[3]); // 0x8c0b3420
    cpu.r[12] = cpu.r[6]; // 0x8c0b3422
    cpu.r[12] = cpu.r[12].wrapping_add(24); // 0x8c0b3424
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[5]).unwrap()); // 0x8c0b3426
    cpu.r[10] = cpu.r[6]; // 0x8c0b3428
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[12]).unwrap()); // 0x8c0b342a
    cpu.r[10] = cpu.r[10].wrapping_add(28); // 0x8c0b342c
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[11]).unwrap()); // 0x8c0b342e
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b3430
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b3432
    cpu.r[6] = cpu.r[6].wrapping_add(32); // 0x8c0b3434
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[13]).unwrap()); // 0x8c0b3436
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b3438
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b343a
    cpu.r[2] = 72; // 0x8c0b343c
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b343e
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b3440
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b3442
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b3444
    let _ = mem.write_f32(cpu.r[2], cpu.fr[3]); // 0x8c0b3446
    cpu.r[5] = cpu.r[15]; // 0x8c0b3448
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[7]).unwrap()); // 0x8c0b344a
    cpu.r[2] = 76; // 0x8c0b344c
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[12]).unwrap()); // 0x8c0b344e
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b3450
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[3]).unwrap()); // 0x8c0b3452
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b3454
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b3456
    cpu.r[5] = cpu.r[5].wrapping_add(48); // 0x8c0b3458
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[1]).unwrap()); // 0x8c0b345a
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b345c
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b345e
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b3460
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b3462
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b3464
    let _ = mem.write_f32(cpu.r[2], cpu.fr[3]); // 0x8c0b3466
    cpu.fr[3] = f32::from_bits(mem.read_u32(cpu.r[0]).unwrap()); // 0x8c0b3468
    cpu.r[2] = 80; // 0x8c0b346a
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[12]).unwrap()); // 0x8c0b346c
    cpu.r[2] = cpu.r[2].wrapping_add(cpu.r[15]); // 0x8c0b346e
    cpu.fr[2] = f32::from_bits(mem.read_u32(cpu.r[9]).unwrap()); // 0x8c0b3470
    cpu.fr[3] = cpu.fr[3] * cpu.fr[0]; // 0x8c0b3472
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[10]).unwrap()); // 0x8c0b3474
    cpu.fr[1] = f32::from_bits(mem.read_u32(cpu.r[8]).unwrap()); // 0x8c0b3476
    cpu.fr[2] = cpu.fr[2] * cpu.fr[0]; // 0x8c0b3478
    cpu.fr[0] = f32::from_bits(mem.read_u32(cpu.r[6]).unwrap()); // 0x8c0b347a
    cpu.fr[3] = cpu.fr[3] + cpu.fr[2]; // 0x8c0b347c
    cpu.fr[1] = cpu.fr[1] * cpu.fr[0]; // 0x8c0b347e
    cpu.fr[3] = cpu.fr[3] + cpu.fr[1]; // 0x8c0b3480
    let _ = mem.write_f32(cpu.r[2], cpu.fr[3]); // 0x8c0b3482
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap(); // 0x8c0b3484
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap(); // 0x8c0b3486
    let _ = mem.write_u32(cpu.r[4], cpu.r[1]); // 0x8c0b3488
    let _ = mem.write_u32(cpu.r[4].wrapping_add(4), cpu.r[0]); // 0x8c0b348a
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap(); // 0x8c0b348c
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(12)).unwrap(); // 0x8c0b348e
    let _ = mem.write_u32(cpu.r[4].wrapping_add(8), cpu.r[1]); // 0x8c0b3490
    let _ = mem.write_u32(cpu.r[4].wrapping_add(12), cpu.r[0]); // 0x8c0b3492
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(16)).unwrap(); // 0x8c0b3494
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(20)).unwrap(); // 0x8c0b3496
    let _ = mem.write_u32(cpu.r[4].wrapping_add(16), cpu.r[1]); // 0x8c0b3498
    let _ = mem.write_u32(cpu.r[4].wrapping_add(20), cpu.r[0]); // 0x8c0b349a
    cpu.r[1] = mem.read_u32(cpu.r[5].wrapping_add(24)).unwrap(); // 0x8c0b349c
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(28)).unwrap(); // 0x8c0b349e
    let _ = mem.write_u32(cpu.r[4].wrapping_add(24), cpu.r[1]); // 0x8c0b34a0
    let _ = mem.write_u32(cpu.r[4].wrapping_add(28), cpu.r[0]); // 0x8c0b34a2
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(32)).unwrap(); // 0x8c0b34a4
    let _ = mem.write_u32(cpu.r[4].wrapping_add(32), cpu.r[0]); // 0x8c0b34a6
}
