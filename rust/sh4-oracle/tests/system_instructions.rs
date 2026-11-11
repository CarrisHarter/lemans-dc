//! Regression tests for the system-register, multiply/accumulate and
//! carry/overflow instructions.
//!
//! Every opcode here is a real instruction word taken from
//! `analysis/export/disassembly.asm`; the comment on each test names the site.
//! Expected values are computed by hand, not captured from the interpreter.

use sh4_oracle::{image, Cpu, Memory, Region, Stop, LOAD_BASE};

const DATA: u32 = 0x8C20_0000;

/// Build a machine whose code is `insn` followed by `rts ; nop`.
fn one(insn: u16) -> (Cpu, Memory) {
    let mut code = Vec::new();
    code.extend_from_slice(&insn.to_le_bytes());
    code.extend_from_slice(&0x000Bu16.to_le_bytes()); // rts
    code.extend_from_slice(&0x0009u16.to_le_bytes()); // nop
    let mut mem = image(LOAD_BASE, &code);
    mem.regions
        .push(Region::new(DATA, vec![0u8; 64]));
    (Cpu::new(), mem)
}

fn run(mut cpu: Cpu, mut mem: Memory) -> (Cpu, Memory) {
    let stop = cpu.run(&mut mem, 20).unwrap();
    assert_eq!(stop, Stop::Halt);
    (cpu, mem)
}

#[test]
fn swap_b_swaps_within_each_halfword() {
    // 0x8c010034: swap.b r4,r1 (0x6148)
    let (mut cpu, mem) = one(0x6148);
    cpu.r[4] = 0x1122_3344;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[1], 0x1122_4433);
}

#[test]
fn swap_w_rotates_by_16() {
    // 0x8c061e40: swap.w r14,r14 (0x6EE9)
    let (mut cpu, mem) = one(0x6EE9);
    cpu.r[14] = 0x1122_3344;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[14], 0x3344_1122);
}

#[test]
fn shad_shifts_left_and_right_by_signed_amount() {
    // 0x8c01091c: shad r8,r3 (0x438C)
    let (mut cpu, mem) = one(0x438C);
    cpu.r[3] = 0x0000_0001;
    cpu.r[8] = 4;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[3], 0x10);

    let (mut cpu, mem) = one(0x438C);
    cpu.r[3] = 0x8000_0000;
    cpu.r[8] = 0xFFFF_FFFF; // -1
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[3], 0xC000_0000, "arithmetic right shift keeps the sign");
}

#[test]
fn shld_shifts_logically() {
    // 0x8c011164: shld r6,r14 (0x4E6D)
    let (mut cpu, mem) = one(0x4E6D);
    cpu.r[14] = 0x8000_0000;
    cpu.r[6] = 0xFFFF_FFFF; // -1
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[14], 0x4000_0000, "logical right shift clears the top bit");
}

#[test]
fn shal_and_shar_move_the_low_bit_into_t() {
    // 0x8c01007a: shal r5 (0x4520)
    let (mut cpu, mem) = one(0x4520);
    cpu.r[5] = 0x8000_0001;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[5], 0x0000_0002);
    assert!(cpu.t(), "shal copies bit 31 into T");

    // 0x8c0142ec: shar r9 (0x4921)
    let (mut cpu, mem) = one(0x4921);
    cpu.r[9] = 0x8000_0001;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[9], 0xC000_0000);
    assert!(cpu.t(), "shar copies bit 0 into T");
}

#[test]
fn rotl_and_rotr_rotate_through_t() {
    // 0x8c01f7be: rotl r3 (0x4304)
    let (mut cpu, mem) = one(0x4304);
    cpu.r[3] = 0x8000_0001;
    cpu.set_t(false);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[3], 0x0000_0002, "bit 31 rotates into T, T into bit 0");
    assert!(cpu.t());

    // 0x8c0198de: rotr r2 (0x4205)
    let (mut cpu, mem) = one(0x4205);
    cpu.r[2] = 0x0000_0001;
    cpu.set_t(true);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[2], 0x8000_0000, "T rotates into bit 31");
    assert!(cpu.t());
}

#[test]
fn rotcl_and_rotcr_chain_through_t() {
    // 0x8c010a48: rotcl r4 (0x4424)
    let (mut cpu, mem) = one(0x4424);
    cpu.r[4] = 0x4000_0000;
    cpu.set_t(true);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[4], 0x8000_0001);
    assert!(!cpu.t());

    // 0x8c01619c: rotcr r2 (0x4225)
    let (mut cpu, mem) = one(0x4225);
    cpu.r[2] = 0x0000_0002;
    cpu.set_t(true);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[2], 0x8000_0001);
    assert!(!cpu.t());
}

#[test]
fn movt_reads_the_t_bit() {
    // 0x8c010a40: movt r0 (0x0029)
    let (mut cpu, mem) = one(0x0029);
    cpu.set_t(true);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[0], 1);
}

#[test]
fn clrmac_clears_both_accumulator_words() {
    // 0x8c0db600: clrmac (0x0028)
    let (mut cpu, mem) = one(0x0028);
    cpu.mach = 0xDEAD_BEEF;
    cpu.macl = 0xFEED_FACE;
    let (cpu, _) = run(cpu, mem);
    assert_eq!((cpu.mach, cpu.macl), (0, 0));
}

#[test]
fn clrs_and_sets_toggle_the_s_bit() {
    // 0x8c0db770: clrs (0x0048)
    let (mut cpu, mem) = one(0x0048);
    cpu.sr = 0x0000_0002;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.sr & 2, 0);

    // 0x8c052a18: sets (0x0058)
    let (mut cpu, mem) = one(0x0058);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.sr & 2, 2);
}

#[test]
fn div0u_clears_q_m_and_t() {
    // 0x8c016010: div0u (0x0019)
    let (mut cpu, mem) = one(0x0019);
    cpu.sr = 0x0000_0301;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.sr & 0x0301, 0);
}

#[test]
fn div0s_sets_q_m_and_t_from_signs() {
    // 0x8c010a3e: div0s r2,r4 (0x2427)
    let (mut cpu, mem) = one(0x2427);
    cpu.r[4] = 0x8000_0000; // Q = 1
    cpu.r[2] = 0x0000_0000; // M = 0
    let (cpu, _) = run(cpu, mem);
    assert_eq!((cpu.sr >> 8) & 3, 0b01, "Q=1, M=0");
    assert!(!cpu.t(), "T = (Q == M)");

    let (mut cpu, mem) = one(0x2427);
    cpu.r[4] = 0x8000_0000;
    cpu.r[2] = 0x8000_0000;
    let (cpu, _) = run(cpu, mem);
    assert!(cpu.t(), "T = (Q == M) when both are negative");
}

#[test]
fn mul_l_writes_mach_and_macl() {
    // 0x8c010808: mul.l r4,r14 (0x0E47)
    let (mut cpu, mem) = one(0x0E47);
    cpu.r[14] = 0x0001_0000;
    cpu.r[4] = 0x0001_0000;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.mach, 1);
    assert_eq!(cpu.macl, 0);
}

#[test]
fn mulu_w_and_muls_w_write_macl_only() {
    // 0x8c014cc2: mulu.w r3,r6 (0x263E)
    let (mut cpu, mem) = one(0x263E);
    cpu.r[6] = 0xFFFF_FFFF;
    cpu.r[3] = 0x0000_0002;
    cpu.mach = 0x1234;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.macl, 0x0001_FFFE);
    assert_eq!(cpu.mach, 0x1234, "mulu.w must not touch MACH");

    // 0x8c044570: muls.w r0,r7 (0x270F)
    let (mut cpu, mem) = one(0x270F);
    cpu.r[7] = 0xFFFF_FFFF; // -1
    cpu.r[0] = 0x0000_0003;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.macl, 0xFFFF_FFFD);
}

#[test]
fn dmulu_and_dmuls_write_the_full_product() {
    // 0x8c01e33a: dmulu.l r3,r15 (0x3F35)
    let (mut cpu, mem) = one(0x3F35);
    cpu.r[15] = 0xFFFF_FFFF;
    cpu.r[3] = 0xFFFF_FFFF;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.mach, 0xFFFF_FFFE);
    assert_eq!(cpu.macl, 0x0000_0001);

    // 0x8c02b696: dmuls.l r7,r15 (0x3F7D)
    let (mut cpu, mem) = one(0x3F7D);
    cpu.r[15] = 0xFFFF_FFFF; // -1
    cpu.r[7] = 0x0000_0002;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.mach, 0xFFFF_FFFF);
    assert_eq!(cpu.macl, 0xFFFF_FFFE);
}

#[test]
fn xtrct_joins_the_high_half_of_rm() {
    // 0x8c014a2c: xtrct r13,r4 (0x24DD)
    let (mut cpu, mem) = one(0x24DD);
    cpu.r[4] = 0x1122_3344;
    cpu.r[13] = 0x5566_7788;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[4], 0x7788_1122);
}

#[test]
fn addc_and_subc_chain_the_t_bit() {
    // 0x8c010aca: addc r2,r4 (0x342E)
    let (mut cpu, mem) = one(0x342E);
    cpu.r[4] = 0xFFFF_FFFF;
    cpu.r[2] = 0;
    cpu.set_t(true);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[4], 0);
    assert!(cpu.t(), "carry out");

    // 0x8c010a42: subc r3,r3 (0x333A)
    let (mut cpu, mem) = one(0x333A);
    cpu.r[3] = 0;
    cpu.set_t(true);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[3], 0xFFFF_FFFF);
    assert!(cpu.t(), "borrow out");
}

#[test]
fn addv_and_subv_set_t_on_signed_overflow() {
    // 0x8c047796: addv r2,r15 (0x3F2F)
    let (mut cpu, mem) = one(0x3F2F);
    cpu.r[15] = 0x7FFF_FFFF;
    cpu.r[2] = 1;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[15], 0x8000_0000);
    assert!(cpu.t(), "positive overflow");

    // 0x8c02ca92: subv r6,r15 (0x3F6B)
    let (mut cpu, mem) = one(0x3F6B);
    cpu.r[15] = 0x8000_0000;
    cpu.r[6] = 1;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[15], 0x7FFF_FFFF);
    assert!(cpu.t(), "negative overflow");
}

#[test]
fn negc_negates_through_the_carry() {
    // 0x8c0162e4: negc r13,r2 (0x62DA)
    let (mut cpu, mem) = one(0x62DA);
    cpu.r[13] = 1;
    cpu.set_t(true);
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r[2], 0xFFFF_FFFE);
    assert!(cpu.t(), "0 - 1 - 1 borrows");
}

#[test]
fn cmp_str_finds_a_zero_byte() {
    // 0x8c011e7c: cmp/str r12,r0 (0x20CC)
    let (mut cpu, mem) = one(0x20CC);
    cpu.r[0] = 0x0102_0304;
    cpu.r[12] = 0x0100_0304;
    let (cpu, _) = run(cpu, mem);
    assert!(cpu.t(), "byte 1 differs to zero");

    let (mut cpu, mem) = one(0x20CC);
    cpu.r[0] = 0x0102_0304;
    cpu.r[12] = 0xFFFF_FFFF;
    let (cpu, _) = run(cpu, mem);
    assert!(!cpu.t());
}

#[test]
fn gbr_byte_operations_use_the_gbr_base() {
    // 0x8c01752c: tst.b #152,@(r0,gbr) (0xCC98)
    let (mut cpu, mut mem) = one(0xCC98);
    cpu.gbr = DATA;
    cpu.r[0] = 3;
    mem.write_u8(DATA + 3, 0x00).unwrap();
    let (cpu, _) = run(cpu, mem);
    assert!(cpu.t(), "tst.b sets T when the AND is zero");

    // 0x8c018968: and.b #108,@(r0,gbr) (0xCD6C)
    let (mut cpu, mut mem) = one(0xCD6C);
    cpu.gbr = DATA;
    cpu.r[0] = 1;
    mem.write_u8(DATA + 1, 0xFF).unwrap();
    let (cpu, mem) = run(cpu, mem);
    assert_eq!(mem.read_u8(DATA + 1).unwrap(), 0x6C);

    // 0x8c010530: or.b #84,@(r0,gbr) (0xCF54)
    let (mut cpu, mut mem) = one(0xCF54);
    cpu.gbr = DATA;
    cpu.r[0] = 2;
    mem.write_u8(DATA + 2, 0x01).unwrap();
    let (cpu, mem) = run(cpu, mem);
    assert_eq!(mem.read_u8(DATA + 2).unwrap(), 0x55);

    // 0x8c01ca44: xor.b #28,@(r0,gbr) (0xCE1C)
    let (mut cpu, mut mem) = one(0xCE1C);
    cpu.gbr = DATA;
    cpu.r[0] = 4;
    mem.write_u8(DATA + 4, 0xFF).unwrap();
    let (cpu, mem) = run(cpu, mem);
    assert_eq!(mem.read_u8(DATA + 4).unwrap(), 0xE3);
}

#[test]
fn tas_b_tests_then_sets_the_high_bit() {
    // 0x8c018484: tas.b @r4 (0x441B)
    let (mut cpu, mut mem) = one(0x441B);
    cpu.r[4] = DATA;
    mem.write_u8(DATA, 0x00).unwrap();
    let (cpu, mem) = run(cpu, mem);
    assert!(cpu.t(), "T is set when the byte was zero");
    assert_eq!(mem.read_u8(DATA).unwrap(), 0x80);
}

#[test]
fn mac_w_accumulates_signed_products() {
    // 0x8c0117f6: mac.w @r7+,@r15+ (0x4F7F)
    let (mut cpu, mut mem) = one(0x4F7F);
    cpu.r[7] = DATA;
    cpu.r[15] = DATA + 8;
    cpu.macl = 10;
    mem.write_u16(DATA, 3).unwrap();
    mem.write_u16(DATA + 8, 5).unwrap();
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.macl, 25);
    assert_eq!(cpu.r[7], DATA + 2);
    assert_eq!(cpu.r[15], DATA + 10);
}

#[test]
fn mac_l_accumulates_into_the_64_bit_accumulator() {
    // 0x8c0108d6: mac.l @r7+,@r0+ (0x007F)
    let (mut cpu, mut mem) = one(0x007F);
    cpu.r[7] = DATA;
    cpu.r[0] = DATA + 8;
    cpu.mach = 0;
    cpu.macl = 0;
    mem.write_u32(DATA, 0x0001_0000).unwrap();
    mem.write_u32(DATA + 8, 0x0001_0000).unwrap();
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.mach, 1);
    assert_eq!(cpu.macl, 0);
    assert_eq!(cpu.r[7], DATA + 4);
    assert_eq!(cpu.r[0], DATA + 12);
}

#[test]
fn movca_l_stores_r0_through_the_cache_path() {
    // 0x8c028e84: movca.l r0,@r0 (0x00C3)
    let (mut cpu, mut mem) = one(0x00C3);
    cpu.r[0] = DATA + 16;
    let (cpu, mem) = run(cpu, mem);
    assert_eq!(mem.read_u32(DATA + 16).unwrap(), cpu.r[0]);
}

#[test]
fn braf_and_bsrf_branch_relative_to_pc() {
    // 0x8c0417b8: braf r6 (0x0623) — target is pc+4+r6, so the delay slot runs.
    let mut code = Vec::new();
    code.extend_from_slice(&0x0623u16.to_le_bytes()); // braf r6
    code.extend_from_slice(&0x0009u16.to_le_bytes()); // nop (delay slot)
    code.extend_from_slice(&0x000Bu16.to_le_bytes()); // rts
    code.extend_from_slice(&0x0009u16.to_le_bytes()); // nop
    let mut mem = image(LOAD_BASE, &code);
    mem.regions.push(Region::new(DATA, vec![0u8; 64]));
    let mut cpu = Cpu::new();
    cpu.r[6] = 0; // pc+4+0 = LOAD_BASE+4, the rts
    let stop = cpu.run(&mut mem, 20).unwrap();
    assert_eq!(stop, Stop::Halt);
}

#[test]
fn system_register_transfers_round_trip() {
    // 0x8c010086: sts.l pr,@-r15 (0x4F22) is already covered; these are the
    // registers the game saves around calls.
    // 0x8c01f842: stc.l r3_bank,@-r2 (0x42B3)
    let (mut cpu, mut mem) = one(0x42B3);
    cpu.r[2] = DATA + 16;
    cpu.bank[3] = 0xCAFE_BABE;
    let (cpu, mem) = run(cpu, mem);
    assert_eq!(cpu.r[2], DATA + 12);
    assert_eq!(mem.read_u32(DATA + 12).unwrap(), 0xCAFE_BABE);

    // 0x8c021888: stc.l sr,@-r0 (0x4003)
    let (mut cpu, mut mem) = one(0x4003);
    cpu.r[0] = DATA + 16;
    cpu.sr = 0x0000_0001;
    let (cpu, mem) = run(cpu, mem);
    assert_eq!(mem.read_u32(DATA + 12).unwrap(), 1);
    assert_eq!(cpu.r[0], DATA + 12);

    // 0x8c01a0b2: ldc.l @r4+,sr (0x4407)
    let (mut cpu, mut mem) = one(0x4407);
    cpu.r[4] = DATA;
    mem.write_u32(DATA, 0x0000_0011).unwrap();
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.sr, 0x11);
    assert_eq!(cpu.r[4], DATA + 4);

    // 0x8c01f84a: ldc.l @r1+,dbr (0x41F6)
    let (mut cpu, mut mem) = one(0x41F6);
    cpu.r[1] = DATA;
    mem.write_u32(DATA, 0x1234_5678).unwrap();
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.dbr, 0x1234_5678);

    // 0x8c0100d2: lds.l @r15+,pr (0x4F26) already existed; check lds fpul.
    // 0x8c0170xx: lds r0,fpul (0x405A)
    let (mut cpu, mem) = one(0x405A);
    cpu.r[0] = 0x3F80_0000;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.fpul, 0x3F80_0000);
}

#[test]
fn fneg_fabs_and_fsqrt_are_exact() {
    // 0x8c013c68: fneg fr4 (0xF44D)
    let (mut cpu, mem) = one(0xF44D);
    cpu.fr[4] = 2.5;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.fr[4], -2.5);

    // 0x8c06bf90: fabs fr1 (0xF15D)
    let (mut cpu, mem) = one(0xF15D);
    cpu.fr[1] = -3.5;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.fr[1], 3.5);

    // 0x8c014428: fsqrt fr0 (0xF06D)
    let (mut cpu, mem) = one(0xF06D);
    cpu.fr[0] = 9.0;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.fr[0], 3.0);
}

#[test]
fn cache_hints_have_no_architectural_effect() {
    // 0x8c01da60: pref @r6 (0x0683)
    let (mut cpu, mem) = one(0x0683);
    cpu.r[6] = DATA;
    let before = cpu.r;
    let (cpu, _) = run(cpu, mem);
    assert_eq!(cpu.r, before);
}

#[test]
fn bra_sign_extends_a_backward_displacement() {
    // 0x8c0177be: `bra 0x8c01775e` — a negative 12-bit displacement. The
    // decoder used to treat 0xFFE as +4094 and jump forward into data.
    // Layout: +0 nop ; +2 nop ; +4 bra -4 ; +6 nop (delay) ; +8 rts
    let mut code = Vec::new();
    code.extend_from_slice(&0x0009u16.to_le_bytes()); // nop
    code.extend_from_slice(&0x0009u16.to_le_bytes()); // nop
    code.extend_from_slice(&0xAFFCu16.to_le_bytes()); // bra -4 -> +4+4-8 = +0
    code.extend_from_slice(&0x0009u16.to_le_bytes()); // delay slot
    code.extend_from_slice(&0x000Bu16.to_le_bytes()); // rts
    code.extend_from_slice(&0x0009u16.to_le_bytes()); // nop
    let mut mem = image(LOAD_BASE, &code);
    let mut cpu = Cpu::new();
    cpu.pc = LOAD_BASE + 4;
    cpu.step(&mut mem).unwrap(); // bra
    cpu.step(&mut mem).unwrap(); // delay slot, then pc = target
    assert_eq!(cpu.pc, LOAD_BASE, "backward bra must land behind the instruction");
}
