//! 0x8c017d88 — decode one tag-prefixed field into the object block at r4.
//!
//! Incoming: r4 = object, r5 = record whose `+4` word points at the encoded
//! stream. The tag at [stream] is split into the object's bit fields, an
//! optional second header byte is stored sign-extended, then the varint at the
//! advanced cursor is unpacked by 0x8c017b94 into `object+0xa0`.
//!
//! Branch polarity: `tst r7,r7` leaves T = 1 when tag bit6 is clear, so the
//! `bt.s` takes the one-byte-header path (cursor +1, `[obj+0xac] = 0`) exactly
//! when bit6 == 0; otherwise the second byte is read sign-extended and the
//! cursor moves +2. The final `bt` uses T from `tst` on `[obj+0xb4]`, i.e. it
//! copies `[obj+0x14]` when tag bit5 == 0 and `[obj+0x54]` when bit5 != 0.
//!
//! Side effects the decompiled C hides:
//!   * r7 = tag & 0x40, r5 = object+0xa0, r4 = the unpacker's cursor, r6 = the
//!     unpacked value, r3/r1 as the unpacker (and the header path) leave them,
//!   * T = ([obj+0xb4] == 0) from the final `tst`; all other SR bits keep
//!     their seeded values,
//!   * stores: [obj+0xb4] = tag & 0x20, [obj+0xb0] = tag & 0x10,
//!     [obj+0xa8] = tag & 0x0f (delay slot of `bt.s`, always), [obj+0xac] = 0
//!     or sign_extend(second byte) (delay slot of `bra`), [obj+0xa0] = unpacked
//!     word (inside the callee), [obj+0xa4] = 0, [obj+0x18] = [obj+0x14] or
//!     [obj+0x54].

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c017d88: mov.l r14,@-r15 ; mov.l r13,@-r15 ; sts.l pr,@-r15
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[14]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.r[13]);
    cpu.r[15] = cpu.r[15].wrapping_sub(4);
    let _ = mem.write_u32(cpu.r[15], cpu.pr);
    // 8c017d8e: mov r15,r14 ; 8c017d90: mov r4,r13
    cpu.r[14] = cpu.r[15];
    let obj = cpu.r[4];
    cpu.r[13] = obj;
    // 8c017d92: mov.l @(4,r5),r4 — the encoded stream lives at [r5+4]
    cpu.r[4] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    // 8c017d94/8c017d98: r6 = 0xb4, r5 = 0xb0
    cpu.r[6] = 0xb4;
    cpu.r[5] = 0xb0;
    // 8c017d96: mov.b @r4,r0 — sign-extends
    let tag = i32::from(mem.read_u8(cpu.r[4]).unwrap_or(0) as i8) as u32;
    // 8c017d9a..8c017da6: r7 = tag & 0x40, r1 = tag & 0x20
    cpu.r[7] = tag & 0x40;
    cpu.r[1] = tag & 0x20;
    // 8c017da8: r3 = 0xa8
    cpu.r[3] = 0xa8;
    // 8c017dae (delay slot of `tst r7,r7`): [obj+0xb4] = tag & 0x20
    let _ = mem.write_u32(obj.wrapping_add(0xb4), cpu.r[1]);
    // 8c017db0..8c017dba: [obj+0xb0] = tag & 0x10
    cpu.r[1] = tag & 0x10;
    let _ = mem.write_u32(obj.wrapping_add(0xb0), cpu.r[1]);
    // 8c017dbc..8c017dc8: r1 = tag & 0x0f, [obj+0xa8] = r1 in the `bt.s`
    // delay slot, which runs on both edges.
    cpu.r[1] = tag & 0x0f;
    let _ = mem.write_u32(obj.wrapping_add(0xa8), cpu.r[1]);
    if tag & 0x40 == 0 {
        // 8c017dd8: r0 = 0xac ; mov #0,r1 ; add #1,r4 ; mov.l r1,@(r0,r13)
        cpu.r[0] = 0xac;
        cpu.r[1] = 0;
        cpu.r[4] = cpu.r[4].wrapping_add(1);
        let _ = mem.write_u32(obj.wrapping_add(0xac), 0);
    } else {
        // 8c017dca: mov.b @(1,r4),r0 — sign-extended second header byte
        let extra = i32::from(mem.read_u8(cpu.r[4].wrapping_add(1)).unwrap_or(0) as i8) as u32;
        // 8c017dcc: r3 = 0xac ; 8c017dce: exts.b r0,r1
        cpu.r[3] = 0xac;
        cpu.r[1] = extra;
        // 8c017dd0: add #2,r4 ; 8c017dd2: mov r3,r0
        cpu.r[4] = cpu.r[4].wrapping_add(2);
        cpu.r[0] = 0xac;
        // 8c017dd6 (delay slot of `bra 0x8c017de0`): [obj+0xac] = r1
        let _ = mem.write_u32(obj.wrapping_add(0xac), extra);
    }
    // 8c017de0..8c017de8: r4 = cursor, r5 = obj+0xa0, jsr @0x8c017b94
    cpu.r[0] = 0xa0;
    cpu.r[5] = obj.wrapping_add(0xa0);
    cpu.r[0] = 0x8c017b94;
    cpu.pr = 0x8c017dec;
    crate::registry::p_8c017b94::run(cpu, mem);
    // 8c017dec: r0 = 0xa4 ; mov #0,r1 ; mov.l r1,@(r0,r13)
    cpu.r[0] = 0xa4;
    cpu.r[1] = 0;
    let _ = mem.write_u32(obj.wrapping_add(0xa4), 0);
    // 8c017df2..8c017df8: r0 = [obj+0xb4]; tst r0,r0; bt 0x8c017e04
    let bit5 = mem.read_u32(obj.wrapping_add(0xb4)).unwrap_or(0);
    if bit5 != 0 {
        // 8c017dfa: r0 = obj+0x54; mov.l @r0,r0 ; delay slot stores [obj+0x18]
        cpu.r[0] = mem.read_u32(obj.wrapping_add(0x54)).unwrap_or(0);
    } else {
        // 8c017e04: r0 = [obj+0x14] ; mov.l r0,@(24,r13)
        cpu.r[0] = mem.read_u32(obj.wrapping_add(0x14)).unwrap_or(0);
    }
    let _ = mem.write_u32(obj.wrapping_add(0x18), cpu.r[0]);
    cpu.set_t(bit5 == 0);
    // 8c017e08: mov r14,r15 ; lds.l @r15+,pr ; mov.l @r15+,r13 ; rts ; delay
    // slot pops r14.
    cpu.r[15] = cpu.r[14];
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[13] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    cpu.r[14] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
}
