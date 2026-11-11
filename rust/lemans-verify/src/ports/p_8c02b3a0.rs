//! 0x8c02b3a0 — three-way state update on the struct based at r4 (largest leaf
//! in the binary, 321 instructions, straight-line: no loops).
//!
//! CFG (all paths converge on the epilogue at 0x8c02b618):
//! ```text
//! prologue: push r14,r13,r12,r11 (stack window only; epilogue pops them back,
//!           so r11-r15 keep their entry values and the port never touches them)
//! a = *(u32*)(*(u32*)0x8c02b624 + r4)   // field at r4+0x494
//! b = *(u32*)(*(u32*)0x8c02b628 + r4)   // field at r4+0x498
//! a==1 && b==1 -> path1 0x8c02b3c2: state [r4+0x3f8]=0, scale 17 floats at
//!                [r4+4]+off (1.1/0.4/0.9/1.2 block, then 1.25 x4, 1.15 x2),
//!                byte [r4+0x405]=0, [r4+0x4f8]=0.0, [r4+0x4f0]=[r4+0x4ec],
//!                delay-slot store [r4+0x4f4]=1.0f, r4 += 0x4f4, T=1
//! a==1 && b==0 -> path2 0x8c02b4e2: state=1, same 17-float block, then 1.25 x4,
//!                1.07 x2, byte [r4+0x405]=0, [r4+0x4f8]=0.0,
//!                [r4+0x4f0]=[r4+0x4ec], delay-slot [r4+0x4f4]=0.99f, r4+=0x4f4, T=1
//! else         -> path3 0x8c02b5f0: state=2, [r4+0x4f8]=0.0,
//!                [r4+0x4f0]=[r4+0x4ec], [r4+0x4f4]=1.0f, r4+=0x4f4, T=0
//! ```
//!
//! Hidden side effects the decompiled C does not show:
//! - r4 is advanced by 0x4f4 on every path (add before the final store on
//!   paths 1/2, add r0,r4 on path 3), so the final stores land at r4+0x4f4 of
//!   the *old* base.
//! - paths 1/2 store through the delay slot of their `bra` (fmov fr0,@r4):
//!   path1 stores 1.0f (fr0 from fldi1), path2 stores 0.99f (fr0 loaded from
//!   the literal at 0x8c02b694 in the instruction before the delay slot).
//! - the byte at [r4+0x405] is written from r1 on path1 (r1 was zeroed by
//!   `mov #0,r1` for the state store, so the byte is 0, not 1) and from r7=0
//!   on path2; path3 does not write it.
//! - final r0 differs per path: 0x405 (path1), 0x8c02b694 (path2, mova before
//!   the delay slot), 0x4f4 (path3).
//! - leftover working registers: path1 r3=0x4f4, r5=r4+0x4ec, r6=r4+0x4f0,
//!   r7=r4+0x4f8; path2 r1=0x4f4, r3=r4+0x4ec, r5=r4+0x4f0, r6=r4+0x4f8, r7=0;
//!   path3 r1=r4+0x4f0, r3=r4+0x4f8, r5=2. fr leftovers: path1 fr1=[r4+0x4ec],
//!   fr2=[P+0x380]*1.15, fr3=1.15; path2 fr1=[P+0x380]*1.07, fr2=1.07,
//!   fr3=1.1 (never reloaded on path2); path3 only fr0=1.0.
//! - T is 1 on paths 1/2 (last flag op cmp/eq #1 / tst with zero) and 0 on
//!   path3.

use sh4_oracle::{Cpu, Memory};

/// Read-modify-write `*addr *= k`, returning the stored value (the original
/// keeps it in an fr register).
fn mul(mem: &mut Memory, addr: u32, k: f32) -> f32 {
    let v = mem.read_f32(addr).unwrap();
    let r = v * k;
    mem.write_f32(addr, r).unwrap();
    r
}

fn add(mem: &mut Memory, addr: u32, k: f32) {
    let v = mem.read_f32(addr).unwrap();
    mem.write_f32(addr, v + k).unwrap();
}

/// The 11 float updates shared verbatim by path1 (0x8c02b3c8-0x8c02b452) and
/// path2 (0x8c02b4e8-0x8c02b572), in original store order.
fn scale_block(mem: &mut Memory, p: u32) {
    let k11 = mem.read_f32(0x8C02B634).unwrap(); // 1.1
    let k04 = mem.read_f32(0x8C02B638).unwrap(); // 0.4
    let k09 = mem.read_f32(0x8C02B640).unwrap(); // 0.9
    let k12 = mem.read_f32(0x8C02B650).unwrap(); // 1.2
    mul(mem, p + 0xC8, k11);
    add(mem, p + 0xCC, k04);
    mul(mem, p + 0xD0, k09);
    mul(mem, p + 0x158, k11);
    add(mem, p + 0x15C, k04);
    mul(mem, p + 0x160, k09);
    mul(mem, p + 0x40, k12);
    mul(mem, p + 0x44, k12);
    mul(mem, p + 0x48, k12);
    mul(mem, p + 0x4C, k12);
    mul(mem, p + 0x300, k09);
}

pub fn run(cpu: &mut Cpu, mem: &mut Memory) {
    let s = cpu.r[4];
    let base_a = mem.read_u32(0x8C02B624).unwrap(); // 0x494
    cpu.r[1] = mem.read_u32(base_a.wrapping_add(s)).unwrap();
    if cpu.r[1] != 1 {
        // 0x8c02b3ae cmp/eq #1 -> T=0; 0x8c02b4ce cmp/eq -> T=0; bra path3
        cpu.set_t(false);
        return path3(cpu, mem, s);
    }
    let base_b = mem.read_u32(0x8C02B628).unwrap(); // 0x498
    let b = mem.read_u32(base_b.wrapping_add(s)).unwrap();
    if b != 1 {
        // 0x8c02b3ba cmp/eq -> T=0, bra 0x8c02b4cc; there cmp/eq r1==1 -> T=1,
        // then 0x8c02b4da tst b,b decides path2 (b==0) vs path3.
        cpu.set_t(b == 0);
        if b == 0 {
            return path2(cpu, mem, s);
        }
        return path3(cpu, mem, s);
    }
    cpu.set_t(true); // 0x8c02b3ba cmp/eq #1 with b==1
    path1(cpu, mem, s)
}

/// 0x8c02b3c2-0x8c02b4ca: a==1 && b==1.
fn path1(cpu: &mut Cpu, mem: &mut Memory, s: u32) {
    let p = mem.read_u32(s.wrapping_add(4)).unwrap();
    let state_off = mem.read_u32(0x8C02B62C).unwrap(); // 0x3f8
    cpu.r[1] = 0; // mov #0,r1 — clobbers the field value loaded into r1
    mem.write_u32(state_off.wrapping_add(s), 0).unwrap(); // [S+0x3f8] = 0
    scale_block(mem, p);
    let k125 = mem.read_f32(0x8C02B65C).unwrap(); // 1.25
    let k115 = mem.read_f32(0x8C02B664).unwrap(); // 1.15
    mul(mem, p + 0x388, k125);
    mul(mem, p + 0x38C, k125);
    mul(mem, p + 0x390, k125);
    mul(mem, p + 0x394, k125);
    mul(mem, p + 0x37C, k115);
    let fr2 = mul(mem, p + 0x380, k115); // fr2 final

    // leftover registers (r11-r14 are popped back by the epilogue)
    cpu.r[0] = mem.read_u32(0x8C02B68C).unwrap(); // 0x405
    cpu.r[3] = mem.read_u32(0x8C02B688).unwrap(); // 0x4f4
    cpu.r[7] = s.wrapping_add(mem.read_u32(0x8C02B678).unwrap()); // S+0x4f8
    cpu.r[5] = s.wrapping_add(mem.read_u32(0x8C02B680).unwrap()); // S+0x4ec
    cpu.r[6] = s.wrapping_add(mem.read_u32(0x8C02B684).unwrap()); // S+0x4f0
    cpu.r[4] = s.wrapping_add(cpu.r[3]); // S+0x4f4

    // tail stores in original order (0x8c02b4be .. delay slot 0x8c02b4ca)
    mem.write_u8(cpu.r[0].wrapping_add(s), cpu.r[1] as u8).unwrap(); // byte[S+0x405] = 0
    cpu.fr[1] = 0.0; // fldi0 at 0x8c02b45e
    mem.write_f32(cpu.r[7], cpu.fr[1]).unwrap(); // [S+0x4f8] = 0.0
    cpu.fr[1] = mem.read_f32(cpu.r[5]).unwrap(); // fmov @r5,fr1 (0x8c02b4c4)
    mem.write_f32(cpu.r[6], cpu.fr[1]).unwrap(); // [S+0x4f0] = [S+0x4ec]
    cpu.fr[0] = 1.0; // fldi1 at 0x8c02b496
    mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap(); // delay slot: [S+0x4f4] = 1.0

    cpu.fr[2] = fr2;
    cpu.fr[3] = k115;
}

/// 0x8c02b4e2-0x8c02b5ee: a==1 && b==0.
fn path2(cpu: &mut Cpu, mem: &mut Memory, s: u32) {
    let p = mem.read_u32(s.wrapping_add(4)).unwrap();
    let state_off = mem.read_u32(0x8C02B62C).unwrap(); // 0x3f8
    cpu.r[1] = 1;
    mem.write_u32(state_off.wrapping_add(s), 1).unwrap(); // [S+0x3f8] = 1
    scale_block(mem, p);
    let k125 = mem.read_f32(0x8C02B65C).unwrap(); // 1.25
    let k107 = mem.read_f32(0x8C02B690).unwrap(); // 1.07
    let k099 = mem.read_f32(0x8C02B694).unwrap(); // 0.99
    mul(mem, p + 0x388, k125);
    mul(mem, p + 0x38C, k125);
    mul(mem, p + 0x390, k125);
    mul(mem, p + 0x394, k125);
    mul(mem, p + 0x37C, k107);
    let fr1 = mul(mem, p + 0x380, k107); // fr1 final

    // leftover registers
    cpu.r[7] = 0; // mov #0,r7 at 0x8c02b58a
    cpu.r[0] = mem.read_u32(0x8C02B68C).unwrap(); // 0x405 (0x8c02b5ce)
    mem.write_u8(cpu.r[0].wrapping_add(s), cpu.r[7] as u8).unwrap(); // byte[S+0x405] = 0 (0x8c02b5de)
    cpu.r[6] = s.wrapping_add(mem.read_u32(0x8C02B678).unwrap()); // S+0x4f8
    cpu.fr[0] = 0.0; // fldi0 at 0x8c02b5a0
    mem.write_f32(cpu.r[6], cpu.fr[0]).unwrap(); // [S+0x4f8] = 0.0 (0x8c02b5e0)
    cpu.r[3] = s.wrapping_add(mem.read_u32(0x8C02B680).unwrap()); // S+0x4ec
    cpu.fr[0] = mem.read_f32(cpu.r[3]).unwrap(); // fmov @r3,fr0 (0x8c02b5e4)
    cpu.r[5] = s.wrapping_add(mem.read_u32(0x8C02B684).unwrap()); // S+0x4f0
    cpu.r[1] = mem.read_u32(0x8C02B688).unwrap(); // 0x4f4
    cpu.r[4] = s.wrapping_add(cpu.r[1]); // S+0x4f4 (0x8c02b5e6)
    mem.write_f32(cpu.r[5], cpu.fr[0]).unwrap(); // [S+0x4f0] = [S+0x4ec] (0x8c02b5e8)
    cpu.r[0] = 0x8C02B694; // mova at 0x8c02b5e2 — final r0
    cpu.fr[0] = k099; // fmov @r0,fr0 (0x8c02b5ea)
    mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap(); // delay slot: [S+0x4f4] = 0.99

    cpu.fr[1] = fr1;
    cpu.fr[2] = k107;
    cpu.fr[3] = mem.read_f32(0x8C02B634).unwrap(); // 1.1, set at 0x8c02b4ee, never reloaded
}

/// 0x8c02b5f0-0x8c02b616: fall-through path (a != 1, or b nonzero).
fn path3(cpu: &mut Cpu, mem: &mut Memory, s: u32) {
    let state_off = mem.read_u32(0x8C02B62C).unwrap(); // 0x3f8
    cpu.r[5] = 2;
    cpu.r[3] = s.wrapping_add(mem.read_u32(0x8C02B678).unwrap()); // S+0x4f8
    cpu.r[0] = s.wrapping_add(mem.read_u32(0x8C02B680).unwrap()); // S+0x4ec
    mem.write_u32(state_off.wrapping_add(s), cpu.r[5]).unwrap(); // [S+0x3f8] = 2
    cpu.fr[0] = 0.0; // fldi0
    mem.write_f32(cpu.r[3], cpu.fr[0]).unwrap(); // [S+0x4f8] = 0.0
    let v = mem.read_f32(cpu.r[0]).unwrap();
    cpu.fr[0] = v;
    cpu.r[1] = s.wrapping_add(mem.read_u32(0x8C02B684).unwrap()); // S+0x4f0
    cpu.r[0] = mem.read_u32(0x8C02B688).unwrap(); // 0x4f4 — final r0
    mem.write_f32(cpu.r[1], v).unwrap(); // [S+0x4f0] = old [S+0x4ec]
    cpu.r[4] = s.wrapping_add(cpu.r[0]); // S+0x4f4
    cpu.fr[0] = 1.0; // fldi1
    mem.write_f32(cpu.r[4], cpu.fr[0]).unwrap(); // [S+0x4f4] = 1.0
}
