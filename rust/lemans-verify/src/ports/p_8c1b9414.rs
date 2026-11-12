//! 0x8c1b9414 — construct the object at `param_1`: two vtable stores, init the
//! embedded sub-object at +28 through 0x8c1261d4, four zeroed 16-bit fields,
//! a 12-byte template copy from 0x8c2121e0 into +4..+12, and a self-pointer.
//!
//! ```text
//! sts.l pr,@-r15 ; add #-16,r15 ; [r15] = param_1
//! *param_1 = 0x8c1eb6a0      ; base vtable, immediately overwritten
//! *param_1 = 0x8c1f1ba4      ; base vtable
//! r4 = param_1 + 28          ; jsr delay slot
//! jsr 0x8c1261d4             ; r5 is still the second argument
//! *param_1 = 0x8c1f1b8c      ; final vtable
//! u16[param_1+40..+46] = 0   ; four halfword fields
//! 12-byte template at 0x8c2121e0 -> stack scratch -> param_1+4..+12
//! u32[param_1+16] = 0
//! u8 [param_1+20] = 0
//! u32[param_1+24] = param_1
//! return param_1
//! ```
//!
//! The `jsr` target comes from the literal at 0x8c1b9490 (0x8c1261d4) and is
//! dispatched through the registry; the delay slot adds 28 to r4 first, and
//! `pr` receives the instruction after the delay slot. The literal pointers at
//! 0x8c1b9488/0x8c1b948c/0x8c1b9494/0x8c1b9498 are real loads.
//!
//! Exit state: r0 = param_1, r1 = template word 0, r3 = param_1 + 4,
//! r4 = 0x8c2121e0, r5 = entry_sp - 16, r6 = param_1, r7 = 0, r15/pr restored;
//! the callee's r1 store is subsequently overwritten by the template copy.

use sh4_oracle::{Cpu, Memory};

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    // 8c1b9414: sts.l pr,@-r15 ; 8c1b9416: add #-16,r15
    let sp = cpu.r[15];
    let _ = mem.write_u32(sp.wrapping_sub(4), cpu.pr);
    cpu.r[15] = sp.wrapping_sub(20);
    // 8c1b9418: mov.l r4,@r15 ; 8c1b941a: mov.l @r15,r3
    let _ = mem.write_u32(cpu.r[15], cpu.r[4]);
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 8c1b941c: mov.l 0x8c1b9488,r0 ; 8c1b941e: mov.l 0x8c1b948c,r1
    cpu.r[0] = mem.read_u32(0x8c1b9488).unwrap_or(0);
    cpu.r[1] = mem.read_u32(0x8c1b948c).unwrap_or(0);
    // 8c1b9420: mov.l r0,@r3 — the discarded base vtable store.
    let _ = mem.write_u32(cpu.r[3], cpu.r[0]);
    // 8c1b9422: mov.l 0x8c1b9490,r0 — call target.
    cpu.r[0] = mem.read_u32(0x8c1b9490).unwrap_or(0);
    // 8c1b9424: mov.l r1,@r3 — the second base vtable pointer.
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    // 8c1b9426: mov.l @r15,r4 — first argument for the callee.
    cpu.r[4] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 8c1b9428: jsr @r0 — return address; 8c1b942a delay slot: add #28,r4.
    let target = cpu.r[0];
    cpu.pr = 0x8c1b942c;
    cpu.r[4] = cpu.r[4].wrapping_add(28);
    match crate::registry::lookup(target) {
        Some(port) => port(cpu, mem),
        None => panic!("0x8c1b9414: call to unported target 0x{target:08x}"),
    }

    // 8c1b942c: mov.l @r15,r3 ; 8c1b942e: mov r15,r5
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[5] = cpu.r[15];
    // 8c1b9430: mov.l 0x8c1b9494,r1 ; 8c1b9432: mov #40,r0
    cpu.r[1] = mem.read_u32(0x8c1b9494).unwrap_or(0);
    cpu.r[0] = 40;
    // 8c1b9434: mov #0,r7 ; 8c1b9436: mov.l r1,@r3 — final vtable.
    cpu.r[7] = 0;
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    // 8c1b9438: add #4,r5 — scratch descriptor now points at r15 + 4.
    cpu.r[5] = cpu.r[5].wrapping_add(4);
    // 8c1b943a: mov.l @r15,r1 ; 8c1b943c: mov.l 0x8c1b9498,r4
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[4] = mem.read_u32(0x8c1b9498).unwrap_or(0);
    // 8c1b943e/9444/944a/9450: mov.w r7,@(r0,r1) with r0 = 40, 42, 44, 46.
    let base = cpu.r[1];
    let _ = mem.write_u16(base.wrapping_add(40), 0);
    let _ = mem.write_u16(base.wrapping_add(42), 0);
    let _ = mem.write_u16(base.wrapping_add(44), 0);
    let _ = mem.write_u16(base.wrapping_add(46), 0);
    // 8c1b9452: mov.l @r15,r6 ; 8c1b9454: mov #20,r0
    cpu.r[6] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[0] = 20;
    // 8c1b9456: mov.l @r4,r3 ; 8c1b9458: mov.l @(4,r4),r1
    cpu.r[3] = mem.read_u32(cpu.r[4]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(4)).unwrap_or(0);
    // 8c1b945a: mov.l r3,@r5 ; 8c1b945c: mov.l r1,@(4,r5)
    let _ = mem.write_u32(cpu.r[5], cpu.r[3]);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(4), cpu.r[1]);
    // 8c1b945e: mov.l @(8,r4),r1 ; 8c1b9460: mov.l r1,@(8,r5)
    cpu.r[1] = mem.read_u32(cpu.r[4].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[5].wrapping_add(8), cpu.r[1]);
    // 8c1b9462: mov.l r7,@(16,r6) — u32[param_1+16] = 0.
    let _ = mem.write_u32(cpu.r[6].wrapping_add(16), cpu.r[7]);
    // 8c1b9464: mov.l @r15,r1 ; 8c1b9466: mov.b r7,@(r0,r1), r0 = 20.
    cpu.r[1] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u8(cpu.r[1].wrapping_add(20), cpu.r[7] as u8);
    // 8c1b9468: mov.l @r15,r3 ; 8c1b946a: mov.l @r5,r1
    cpu.r[3] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[1] = mem.read_u32(cpu.r[5]).unwrap_or(0);
    // 8c1b946c: add #4,r3 ; 8c1b946e: mov.l @(4,r5),r0
    cpu.r[3] = cpu.r[3].wrapping_add(4);
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(4)).unwrap_or(0);
    // 8c1b9470: mov.l r1,@r3 ; 8c1b9472: mov.l r0,@(4,r3)
    let _ = mem.write_u32(cpu.r[3], cpu.r[1]);
    let _ = mem.write_u32(cpu.r[3].wrapping_add(4), cpu.r[0]);
    // 8c1b9474: mov.l @(8,r5),r0 ; 8c1b9476: mov.l r0,@(8,r3)
    cpu.r[0] = mem.read_u32(cpu.r[5].wrapping_add(8)).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[3].wrapping_add(8), cpu.r[0]);
    // 8c1b9478: mov.l @r15,r0 ; 8c1b947a: mov.l r0,@(24,r0)
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    let _ = mem.write_u32(cpu.r[0].wrapping_add(24), cpu.r[0]);
    // 8c1b947c: mov.l @r15,r0 — return value.
    cpu.r[0] = mem.read_u32(cpu.r[15]).unwrap_or(0);
    // 8c1b947e: add #16,r15 ; 8c1b9480: lds.l @r15+,pr
    cpu.r[15] = cpu.r[15].wrapping_add(16);
    cpu.pr = mem.read_u32(cpu.r[15]).unwrap_or(0);
    cpu.r[15] = cpu.r[15].wrapping_add(4);
    // 8c1b9482: rts (delay 8c1b9484: nop)
}
