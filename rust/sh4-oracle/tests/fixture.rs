use sh4_oracle::{image, Cpu, Error, Stop, HALT_PC, LOAD_BASE};

/// The hand-assembled 56-byte SH-4 fixture used throughout the project.
const FIXTURE: &[u8] = include_bytes!("../../../fixtures/sh4_fixture.bin");

fn machine() -> (Cpu, sh4_oracle::Memory) {
    let mem = image(LOAD_BASE, FIXTURE);
    // `mov.l @(2,pc),r1` at 0x14 reads a literal at 0x20 holding LOAD_BASE.
    assert_eq!(mem.read_u32(LOAD_BASE + 0x20).unwrap(), LOAD_BASE);
    (Cpu::new(), mem)
}

#[test]
fn sum_loop_counts_down_to_zero() {
    let (mut cpu, mut mem) = machine();
    cpu.pc = LOAD_BASE;
    cpu.r[1] = 5; // the fixture's loop counter
    let stop = cpu.run(&mut mem, 10_000).unwrap();
    assert_eq!(stop, Stop::Halt);
    assert_eq!(cpu.r[0], 5, "loop should iterate once per unit of r1");
    assert_eq!(cpu.r[1], 0, "dt should leave the counter at zero");
    assert!(cpu.t(), "dt sets T when the result is zero");
}

#[test]
fn mixed_arguments_are_not_clobbered() {
    let (mut cpu, mut mem) = machine();
    cpu.pc = LOAD_BASE;
    cpu.r[1] = 3;
    cpu.r[5] = 0xDEAD_BEEF; // untouched by the routine
    cpu.run(&mut mem, 10_000).unwrap();
    assert_eq!(cpu.r[0], 3);
    assert_eq!(cpu.r[5], 0xDEAD_BEEF);
}

#[test]
fn indirect_jump_lands_on_literal_target() {
    let (mut cpu, mut mem) = machine();
    cpu.pc = LOAD_BASE + 0x10;
    // mov #5,r4 ; nop ; mov.l @(pc),r1 ; jmp @r1 ; <delay slot nop>
    for _ in 0..5 {
        cpu.step(&mut mem).unwrap();
    }
    assert_eq!(cpu.r[1], LOAD_BASE, "literal load should yield the load base");
    assert_eq!(cpu.pc, LOAD_BASE, "jmp should arrive after the delay slot");
}

#[test]
fn delay_slot_runs_before_the_jump() {
    // jmp @r1 ; add #1,r0 (delay slot) ; rts ; nop
    let mut code = Vec::new();
    for op in [0x412Bu16, 0x7001, 0x000B, 0x0009] {
        code.extend_from_slice(&op.to_le_bytes());
    }
    let mut mem = image(LOAD_BASE, &code);
    let mut cpu = Cpu::new();
    cpu.pc = LOAD_BASE;
    cpu.r[1] = LOAD_BASE + 4; // jump to the rts
    cpu.run(&mut mem, 100).unwrap();
    assert_eq!(cpu.r[0], 1, "the instruction after jmp must execute before the jump");
}

#[test]
fn rts_returns_through_pr() {
    let mut code = Vec::new();
    for op in [0x000Bu16, 0x0009] {
        code.extend_from_slice(&op.to_le_bytes());
    }
    let mut mem = image(LOAD_BASE, &code);
    let mut cpu = Cpu::new();
    cpu.pc = LOAD_BASE;
    cpu.pr = HALT_PC;
    assert_eq!(cpu.run(&mut mem, 10).unwrap(), Stop::Halt);
}

#[test]
fn fpu_arithmetic_is_single_precision() {
    // fmul fr4,fr0 (0xF042) ; fadd fr4,fr0 is not what the fixture says, so build our own:
    // fmul fr4,fr0 ; fadd fr5,fr0 ; fldi1 fr6 ; rts
    let mut code = Vec::new();
    for op in [0xF042u16, 0xF050, 0xF69D, 0x000B, 0x0009] {
        code.extend_from_slice(&op.to_le_bytes());
    }
    let mut mem = image(LOAD_BASE, &code);
    let mut cpu = Cpu::new();
    cpu.pc = LOAD_BASE;
    cpu.fr[0] = 2.5;
    cpu.fr[4] = 4.0;
    cpu.fr[5] = 0.25;
    cpu.run(&mut mem, 100).unwrap();
    assert_eq!(cpu.fr[0], 10.25, "FR0 = 2.5*4.0 + 0.25");
    assert_eq!(cpu.fr[6], 1.0, "fldi1 loads 1.0");
}

#[test]
fn fmac_rounds_once() {
    // fmac fr0,fr4,fr5 : FR5 += FR0 * FR4 with a single rounding
    let mut code = Vec::new();
    for op in [0xF54Eu16, 0x000B, 0x0009] {
        code.extend_from_slice(&op.to_le_bytes());
    }
    let mut mem = image(LOAD_BASE, &code);
    let mut cpu = Cpu::new();
    cpu.pc = LOAD_BASE;
    cpu.fr[0] = 1.5;
    cpu.fr[4] = 2.0;
    cpu.fr[5] = -0.5;
    cpu.run(&mut mem, 100).unwrap();
    assert_eq!(cpu.fr[5], 2.5, "fused multiply-add: -0.5 + 1.5*2.0");
}

#[test]
fn store_records_memory_effects() {
    // fmov.s FR0,@R1 ; rts ; nop ; <1 f32 slot at +4>
    let mut code = vec![];
    for op in [0xF10Au16, 0x000B, 0x0009] {
        code.extend_from_slice(&op.to_le_bytes());
    }
    code.resize(64, 0);
    let mut mem = sh4_oracle::Memory::with_region(sh4_oracle::Region::new(LOAD_BASE, code));

    let mut cpu = Cpu::new();
    cpu.pc = LOAD_BASE;
    cpu.r[1] = LOAD_BASE + 8; // a mapped, non-code address
    cpu.fr[0] = 42.5;
    cpu.run(&mut mem, 100).unwrap();
    assert_eq!(mem.read_f32(LOAD_BASE + 8).unwrap(), 42.5);
    assert!(!mem.writes.is_empty(), "writes are logged for diffing");
}

#[test]
fn unknown_opcode_is_reported_with_pc() {
    let mut mem = image(LOAD_BASE, &[0x00, 0x00]); // 0x0000 is not an SH-4 instruction
    let mut cpu = Cpu::new();
    cpu.pc = LOAD_BASE;
    match cpu.step(&mut mem) {
        Err(Error::Unimplemented(pc, op, _)) => {
            assert_eq!(pc, LOAD_BASE);
            assert_eq!(op, 0x0000);
        }
        other => panic!("expected Unimplemented, got {other:?}"),
    }
}

#[test]
fn fetch_outside_image_is_reported() {
    // nop ; rts ; nop(delay slot) ; nop -> branch to an unmapped PR afterwards
    let mut mem = image(LOAD_BASE, &[0x09, 0x00, 0x0B, 0x00, 0x09, 0x00, 0x09, 0x00]);
    let mut cpu = Cpu::new();
    cpu.pc = LOAD_BASE;
    cpu.pr = 0x1234_5678; // not mapped
    match cpu.run(&mut mem, 10) {
        Err(Error::FetchOutOfRange(addr)) => assert_eq!(addr, 0x1234_5678),
        other => panic!("expected FetchOutOfRange, got {other:?}"),
    }
}

#[test]
fn call_helper_uses_r4_convention() {
    // mov #7,r0 ; rts  -> returns immediate in r0
    let mut code = Vec::new();
    for op in [0xE007u16, 0x000B, 0x0009] {
        code.extend_from_slice(&op.to_le_bytes());
    }
    let mut mem = image(LOAD_BASE, &code);
    let mut cpu = Cpu::new();
    assert_eq!(cpu.call(&mut mem, LOAD_BASE, &[1, 2, 3], 100).unwrap(), 7);
}
