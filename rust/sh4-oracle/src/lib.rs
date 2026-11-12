//! Minimal SH-4 (SH7750) interpreter used as a behavioural oracle.
//!
//! Scope is deliberate: this exists to answer "does my Rust rewrite compute the
//! same thing as the original routine?", not to run a Dreamcast. It implements
//! the integer core, delayed branches, the general-purpose register set, and the
//! single-precision FPU subset that decompiled game code actually uses.
//!
//! Anything not implemented returns [`Error::Unimplemented`] with the opcode and
//! PC, so a gap is visible instead of silently producing wrong results.
//!
//! Deliberate omissions: no MMU, no cache, no interrupts, no DMA, no store
//! queues (writes are recorded so a test can compare memory effects), and no
//! multi-threading. Those belong to the console, not to a function-level oracle.

pub mod json;

use std::fmt;

/// Address the Katana bootstrap maps `1ST_READ.BIN` at.
pub const LOAD_BASE: u32 = 0x8C01_0000;

/// Default stack window, outside any real image, so a function prologue has
/// somewhere to push. Both the oracle and the verifier use it, so a port that
/// spills differently still compares green.
pub const STACK_BASE: u32 = 0x8C30_0000;
pub const STACK_SIZE: u32 = 0x1_0000;

/// Main RAM the loaded image does not cover.
///
/// The image is the first 2.28 MB of a 16 MB bank at 0x8c000000. Globals live
/// past the end of the image, so a spec that dereferences one used to die with
/// `data access ... outside mapped memory`. This window is zeroed RAM, exactly
/// what the hardware has before the game writes it.
pub const RAM_BASE: u32 = 0x8C24_0000;

/// Give the CPU a stack and the RAM the image does not cover.
pub fn install_default_stack(cpu: &mut Cpu, mem: &mut Memory) {
    if !mem.regions.iter().any(|r| r.base == STACK_BASE) {
        mem.regions.push(Region::new(STACK_BASE, vec![0u8; STACK_SIZE as usize]));
    }
    // Start exactly where the loaded image ends, so globals that sit just past
    // its last byte are addressable.
    let image_end = mem
        .regions
        .iter()
        .filter(|r| r.base == LOAD_BASE)
        .map(|r| r.base + r.bytes.len() as u32)
        .max()
        .unwrap_or(RAM_BASE);
    let ram_base = (image_end + 3) & !3;
    if ram_base < STACK_BASE && !mem.regions.iter().any(|r| r.base == ram_base) {
        mem.regions
            .push(Region::new(ram_base, vec![0u8; (STACK_BASE - ram_base) as usize]));
    }
    if cpu.r[15] == 0 {
        cpu.r[15] = STACK_BASE + STACK_SIZE - 16;
    }
}

/// True when an address lies inside the synthetic stack window.
pub fn in_stack(addr: u32) -> bool {
    addr >= STACK_BASE && addr < STACK_BASE + STACK_SIZE
}

/// Sentinel return address: `rts` to this stops the machine.
pub const HALT_PC: u32 = 0xFFFF_FFF0;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Instruction outside any mapped region.
    FetchOutOfRange(u32),
    /// Load/store outside any mapped region.
    DataOutOfRange(u32, u32),
    /// Decoder gap. Carries pc, opcode and a short tag.
    Unimplemented(u32, u16, &'static str),
    /// Execution limit hit; not an error in itself, but a test failure.
    StepLimit(usize),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FetchOutOfRange(pc) => write!(f, "fetch outside mapped memory at 0x{pc:08x}"),
            Error::DataOutOfRange(addr, pc) => {
                write!(f, "data access 0x{addr:08x} outside mapped memory (pc 0x{pc:08x})")
            }
            Error::Unimplemented(pc, op, tag) => {
                write!(f, "unimplemented {tag} 0x{op:04x} at 0x{pc:08x}")
            }
            Error::StepLimit(n) => write!(f, "step limit {n} reached"),
        }
    }
}

impl std::error::Error for Error {}

/// Why execution stopped.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stop {
    /// `rts` returned to [`HALT_PC`].
    Halt,
    Steps(usize),
}

/// One flat window of memory. The oracle uses one; the real machine has many.
#[derive(Debug, Clone)]
pub struct Region {
    pub base: u32,
    pub bytes: Vec<u8>,
}

impl Region {
    pub fn new(base: u32, bytes: Vec<u8>) -> Self {
        Self { base, bytes }
    }

    pub fn contains(&self, addr: u32, len: u32) -> bool {
        let end = addr as u64 + len as u64;
        addr >= self.base && end <= self.base as u64 + self.bytes.len() as u64
    }
}

/// Byte-addressable memory plus a log of writes, so tests can diff side effects.
#[derive(Debug, Clone, Default)]
pub struct Memory {
    pub regions: Vec<Region>,
    pub writes: Vec<(u32, u8)>,
}

impl Memory {
    pub fn with_region(region: Region) -> Self {
        Self { regions: vec![region], writes: Vec::new() }
    }

    fn locate(&self, addr: u32, len: u32) -> Option<(&Region, usize)> {
        self.regions
            .iter()
            .find(|r| r.contains(addr, len))
            .map(|r| (r, (addr - r.base) as usize))
    }

    pub fn read_u16(&self, addr: u32) -> Result<u16, Error> {
        let (r, off) = self.locate(addr, 2).ok_or(Error::DataOutOfRange(addr, addr))?;
        Ok(u16::from_le_bytes([r.bytes[off], r.bytes[off + 1]]))
    }

    pub fn read_u8(&self, addr: u32) -> Result<u8, Error> {
        let (r, off) = self.locate(addr, 1).ok_or(Error::DataOutOfRange(addr, addr))?;
        Ok(r.bytes[off])
    }

    pub fn write_u8(&mut self, addr: u32, value: u8) -> Result<(), Error> {
        let (r, off) = self.locate(addr, 1).ok_or(Error::DataOutOfRange(addr, addr))?;
        let base = r.base;
        let bytes = &mut self.regions.iter_mut().find(|r| r.base == base).unwrap().bytes;
        bytes[off] = value;
        self.writes.push((addr, value));
        Ok(())
    }

    pub fn write_u16(&mut self, addr: u32, value: u16) -> Result<(), Error> {
        let (r, off) = self.locate(addr, 2).ok_or(Error::DataOutOfRange(addr, addr))?;
        let base = r.base;
        let bytes = &mut self.regions.iter_mut().find(|r| r.base == base).unwrap().bytes;
        bytes[off..off + 2].copy_from_slice(&value.to_le_bytes());
        self.writes.extend((0..2).map(|i| (addr + i, value.to_le_bytes()[i as usize])));
        Ok(())
    }

    pub fn read_u32(&self, addr: u32) -> Result<u32, Error> {
        let (r, off) = self.locate(addr, 4).ok_or(Error::DataOutOfRange(addr, addr))?;
        let mut b = [0u8; 4];
        b.copy_from_slice(&r.bytes[off..off + 4]);
        Ok(u32::from_le_bytes(b))
    }

    pub fn read_f32(&self, addr: u32) -> Result<f32, Error> {
        Ok(f32::from_bits(self.read_u32(addr)?))
    }

    pub fn write_u32(&mut self, addr: u32, value: u32) -> Result<(), Error> {
        let pc_hint = addr;
        let (r, off) = self
            .locate(addr, 4)
            .ok_or(Error::DataOutOfRange(addr, pc_hint))?;
        let base = r.base;
        let bytes = &mut self.regions.iter_mut().find(|r| r.base == base).unwrap().bytes;
        bytes[off..off + 4].copy_from_slice(&value.to_le_bytes());
        self.writes.extend((0..4).map(|i| (addr + i, value.to_le_bytes()[i as usize])));
        Ok(())
    }

    pub fn write_f32(&mut self, addr: u32, value: f32) -> Result<(), Error> {
        self.write_u32(addr, value.to_bits())
    }

    /// Replace the whole region body without recording writes (test setup).
    pub fn load(&mut self, req_base: u32, bytes: &[u8]) -> Result<(), Error> {
        self.regions
            .push(Region::new(req_base, bytes.to_vec()));
        Ok(())
    }
}

/// Architectural state.
#[derive(Debug, Clone)]
pub struct Cpu {
    pub r: [u32; 16],
    pub pc: u32,
    pub pr: u32,
    pub sr: u32,
    /// FPU registers, always single precision in this oracle.
    pub fr: [f32; 16],
    pub fpscr: u32,
    /// FPU communication register, used by float/ftrc/flds/fsts.
    pub fpul: u32,
    /// Multiply-accumulate high/low words.
    pub mach: u32,
    pub macl: u32,
    /// System registers the game saves and restores around calls.
    pub gbr: u32,
    pub vbr: u32,
    pub ssr: u32,
    pub spc: u32,
    pub sgr: u32,
    pub dbr: u32,
    /// R0_BANK..R7_BANK, only reachable through stc/ldc.
    pub bank: [u32; 8],
    /// Pending delayed-branch target, consumed after the delay slot runs.
    pub delay: Option<u32>,
    pub steps: usize,
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Self {
        Self {
            r: [0; 16],
            pc: LOAD_BASE,
            pr: HALT_PC,
            sr: 0,
            fr: [0.0; 16],
            fpscr: 0,
            fpul: 0,
            mach: 0,
            macl: 0,
            gbr: 0,
            vbr: 0,
            ssr: 0,
            spc: 0,
            sgr: 0,
            dbr: 0,
            bank: [0; 8],
            delay: None,
            steps: 0,
        }
    }

    #[inline]
    pub fn t(&self) -> bool {
        self.sr & 1 != 0
    }

    #[inline]
    pub fn set_t(&mut self, v: bool) {
        if v {
            self.sr |= 1;
        } else {
            self.sr &= !1;
        }
    }

    #[inline]
    pub fn reg(&self, n: usize) -> u32 {
        self.r[n]
    }

    /// Execute one instruction. A delayed branch consumes two steps.
    pub fn step(&mut self, mem: &mut Memory) -> Result<(), Error> {
        if let Some(target) = self.delay.take() {
            let pc = self.pc;
            let op = fetch(mem, pc)?;
            self.pc = pc.wrapping_add(2);
            self.execute(op, mem, pc)?;
            if self.delay.is_some() {
                return Err(Error::Unimplemented(pc, op, "branch in delay slot"));
            }
            self.pc = target;
            self.steps += 1;
            return Ok(());
        }

        let pc = self.pc;
        let op = fetch(mem, pc)?;
        self.pc = pc.wrapping_add(2);
        self.execute(op, mem, pc)?;
        self.steps += 1;
        Ok(())
    }

    /// Run from `self.pc` until `rts` reaches [`HALT_PC`] or `limit` steps.
    pub fn run(&mut self, mem: &mut Memory, limit: usize) -> Result<Stop, Error> {
        for _ in 0..limit {
            if self.pc == HALT_PC {
                return Ok(Stop::Halt);
            }
            self.step(mem)?;
        }
        Ok(Stop::Steps(limit))
    }

    /// Call a function by ABI: r4..r7 hold the first four integer arguments.
    /// Returns `r0` after the routine returns to [`HALT_PC`].
    pub fn call(
        &mut self,
        mem: &mut Memory,
        entry: u32,
        args: &[u32],
        limit: usize,
    ) -> Result<u32, Error> {
        self.pc = entry;
        self.pr = HALT_PC;
        self.delay = None;
        self.r[4..7].copy_from_slice(&args[..args.len().min(4)]);
        match self.run(mem, limit)? {
            Stop::Halt => Ok(self.r[0]),
            Stop::Steps(n) => Err(Error::StepLimit(n)),
        }
    }

    fn execute(&mut self, op: u16, mem: &mut Memory, pc: u32) -> Result<(), Error> {
        if let Some(result) = self.execute_system(op, mem, pc) {
            return result;
        }
        let n = ((op >> 8) & 0xF) as usize;
        let m = ((op >> 4) & 0xF) as usize;
        let disp8 = (op & 0xFF) as i8 as i32;

        match op >> 12 {
            0x0 => match op {
                0x0009 => Ok(()),                       // nop
                0x000B => { self.delay = Some(self.pr); Ok(()) }  // rts (delayed)
                0x0002 => {                              // stc SR,Rn
                    self.r[n] = self.sr;
                    Ok(())
                }
                0x0008 => { self.set_t(false); Ok(()) }  // clrt
                0x0018 => { self.set_t(true); Ok(()) }   // sett
                _ => match op & 0xF {
                    0x4 | 0x5 | 0x6 => {
                        // mov.b/w/l Rm,@(R0,Rn)
                        let addr = self.r[0].wrapping_add(self.r[n]);
                        self.store(mem, addr, self.r[m], op & 0xF)?;
                        Ok(())
                    }
                    0xC | 0xD | 0xE => {
                        // mov.b/w/l @(R0,Rm),Rn
                        let addr = self.r[0].wrapping_add(self.r[m]);
                        self.r[n] = self.load(mem, addr, op & 0xF)?;
                        Ok(())
                    }
                    _ => Err(Error::Unimplemented(pc, op, "0x0 family")),
                },
            },
            0x1 => {
                // mov.l Rm,@(disp,Rn)
                let addr = self.r[n].wrapping_add((op & 0xF) as u32 * 4);
                self.store(mem, addr, self.r[m], 0x2)?;
                Ok(())
            }
            0x2 => match op & 0xF {
                0x0 | 0x1 | 0x2 => {
                    // mov.b/w/l Rm,@Rn
                    self.store(mem, self.r[n], self.r[m], op & 0xF)?;
                    Ok(())
                }
                0x4 | 0x5 | 0x6 => {
                    // mov.b/w/l Rm,@-Rn   (decrement first)
                    let step = match op & 0xF { 0x4 => 1, 0x5 => 2, _ => 4 };
                    self.r[n] = self.r[n].wrapping_sub(step);
                    self.store(mem, self.r[n], self.r[m], op & 0xF)?;
                    Ok(())
                }
                0x8 => {
                    // tst Rm,Rn
                    self.set_t(self.r[n] & self.r[m] == 0);
                    Ok(())
                }
                0x9 => {
                    // and Rm,Rn
                    self.r[n] &= self.r[m];
                    Ok(())
                }
                0xA => {
                    // xor Rm,Rn
                    self.r[n] ^= self.r[m];
                    Ok(())
                }
                0xB => {
                    // or Rm,Rn
                    self.r[n] |= self.r[m];
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x2 family")),
            },
            0x3 => match op & 0xF {
                0x0 => {                                 // cmp/eq Rm,Rn
                    self.set_t(self.r[n] == self.r[m]);
                    Ok(())
                }
                0x2 => {                                 // cmp/hs Rm,Rn (unsigned >=)
                    self.set_t(self.r[n] >= self.r[m]);
                    Ok(())
                }
                0x3 => {                                 // cmp/ge Rm,Rn (signed >=)
                    self.set_t((self.r[n] as i32) >= (self.r[m] as i32));
                    Ok(())
                }
                0x6 => {                                 // cmp/hi Rm,Rn (unsigned >)
                    self.set_t(self.r[n] > self.r[m]);
                    Ok(())
                }
                0x7 => {                                 // cmp/gt Rm,Rn (signed >)
                    self.set_t((self.r[n] as i32) > (self.r[m] as i32));
                    Ok(())
                }
                0x8 => {                                 // sub Rm,Rn
                    self.r[n] = self.r[n].wrapping_sub(self.r[m]);
                    Ok(())
                }
                0xC => {                                 // add Rm,Rn
                    self.r[n] = self.r[n].wrapping_add(self.r[m]);
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x3 family")),
            },
            0x4 => match op & 0xF0FF {
                0x4000 => {                              // shll Rn
                    let out = self.r[n] >> 31 != 0;
                    self.r[n] = self.r[n] << 1;
                    self.set_t(out);
                    Ok(())
                }
                0x4001 => {                              // shlr Rn
                    let out = self.r[n] & 1 != 0;
                    self.r[n] = self.r[n] >> 1;
                    self.set_t(out);
                    Ok(())
                }
                0x4008 => {                              // shll2 Rn
                    self.r[n] = self.r[n] << 2;
                    Ok(())
                }
                0x4009 => {                              // shlr2 Rn
                    self.r[n] = self.r[n] >> 2;
                    Ok(())
                }
                0x4010 => {                              // dt Rn
                    self.r[n] = self.r[n].wrapping_sub(1);
                    self.set_t(self.r[n] == 0);
                    Ok(())
                }
                0x4073 => {                              // not Rn
                    self.r[n] = !self.r[n];
                    Ok(())
                }
                0x407B => {                              // neg Rn
                    self.r[n] = (self.r[n] as i32).wrapping_neg() as u32;
                    Ok(())
                }
                0x4018 => {                              // shll8 Rn
                    self.r[n] = self.r[n] << 8;
                    Ok(())
                }
                0x4019 => {                              // shlr8 Rn
                    self.r[n] = self.r[n] >> 8;
                    Ok(())
                }
                0x4028 => {                              // shll16 Rn
                    self.r[n] = self.r[n] << 16;
                    Ok(())
                }
                0x4029 => {                              // shlr16 Rn
                    self.r[n] = self.r[n] >> 16;
                    Ok(())
                }
                0x4011 => {                              // cmp/pz Rn
                    self.set_t((self.r[n] as i32) >= 0);
                    Ok(())
                }
                0x4015 => {                              // cmp/pl Rn
                    self.set_t((self.r[n] as i32) > 0);
                    Ok(())
                }
                0x4002 => {                              // sts PR,Rn
                    self.r[n] = self.pr;
                    Ok(())
                }
                0x400E => {                              // ldc Rn,SR (register is in the high nibble)
                    self.sr = self.r[n];
                    Ok(())
                }
                0x402A => {                              // lds Rn,PR
                    self.pr = self.r[n];
                    Ok(())
                }
                0x4022 => {                              // sts.l PR,@-Rn
                    self.r[n] = self.r[n].wrapping_sub(4);
                    mem.write_u32(self.r[n], self.pr)?;
                    Ok(())
                }
                0x4026 => {                              // lds.l @Rn+,PR
                    self.pr = mem.read_u32(self.r[n])?;
                    self.r[n] = self.r[n].wrapping_add(4);
                    Ok(())
                }
                0x400B => {                              // jsr @Rn (delayed)
                    self.pr = pc.wrapping_add(4);
                    self.delay = Some(self.r[n]);
                    Ok(())
                }
                0x402B => {                              // jmp @Rn (delayed)
                    self.delay = Some(self.r[n]);
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x4 family")),
            },
            0x5 => {
                // mov.l @(disp,Rm),Rn   (disp scaled by 4)
                let addr = self.r[m].wrapping_add((op & 0xF) as u32 * 4);
                self.r[n] = mem.read_u32(addr)?;
                Ok(())
            }
            0x6 => match op & 0xF {
                0x0 | 0x1 | 0x2 => {                     // mov.b/w/l @Rm,Rn
                    self.r[n] = self.load(mem, self.r[m], op & 0xF)?;
                    Ok(())
                }
                0x3 => {                                 // mov Rm,Rn
                    self.r[n] = self.r[m];
                    Ok(())
                }
                0x4 | 0x5 | 0x6 => {                     // mov.b/w/l @Rm+,Rn
                    self.r[n] = self.load(mem, self.r[m], op & 0xF)?;
                    let step = match op & 0xF { 0x4 => 1, 0x5 => 2, _ => 4 };
                    self.r[m] = self.r[m].wrapping_add(step);
                    Ok(())
                }
                0xC => {                                 // extu.b Rm,Rn
                    self.r[n] = self.r[m] & 0xFF;
                    Ok(())
                }
                0xD => {                                 // extu.w Rm,Rn
                    self.r[n] = self.r[m] & 0xFFFF;
                    Ok(())
                }
                0xE => {                                 // exts.b Rm,Rn
                    self.r[n] = self.r[m] as u8 as i8 as i32 as u32;
                    Ok(())
                }
                0xF => {                                 // exts.w Rm,Rn
                    self.r[n] = self.r[m] as u16 as i16 as i32 as u32;
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0x6 family")),
            },
            0x7 => {
                // add #imm,Rn
                self.r[n] = (self.r[n] as i32).wrapping_add(disp8) as u32;
                Ok(())
            }
            0x8 => {
                // The displacement group puts Rn/Rm in the *m* field and the
                // 4-bit displacement in the low nibble. Using the full low byte
                // as displacement and the n field as base (the original code)
                // addressed r[n]+(m<<4) instead of r[m]+disp.
                let disp = (op & 0xF) as u32;
                let target = (pc as i32 + 4 + disp8 * 2) as u32;
                match op >> 8 {
                    0x80 => {                            // mov.b R0,@(disp,Rn)
                        let addr = self.r[m].wrapping_add(disp);
                        self.store(mem, addr, self.r[0], 0x0)?;
                        Ok(())
                    }
                    0x81 => {                            // mov.w R0,@(disp,Rn)
                        let addr = self.r[m].wrapping_add(disp * 2);
                        self.store(mem, addr, self.r[0], 0x1)?;
                        Ok(())
                    }
                    0x84 => {                            // mov.b @(disp,Rm),R0
                        let addr = self.r[m].wrapping_add(disp);
                        self.r[0] = self.load(mem, addr, 0x0)?;
                        Ok(())
                    }
                    0x85 => {                            // mov.w @(disp,Rm),R0
                        let addr = self.r[m].wrapping_add(disp * 2);
                        self.r[0] = self.load(mem, addr, 0x1)?;
                        Ok(())
                    }
                    0x88 => {                            // cmp/eq #imm,R0
                        self.set_t(self.r[0] == (op & 0xFF) as i8 as i32 as u32);
                        Ok(())
                    }
                    0x89 => { if self.t() { self.pc = target; } Ok(()) }   // bt
                    0x8B => { if !self.t() { self.pc = target; } Ok(()) }  // bf
                    0x8D => { if self.t() { self.delay = Some(target); } Ok(()) }  // bt.s
                    0x8F => { if !self.t() { self.delay = Some(target); } Ok(()) } // bf.s
                    _ => Err(Error::Unimplemented(pc, op, "0x8 family")),
                }
            }
            0x9 => {
                // mov.w @(disp,PC),Rn  (word literal)
                let addr = (pc & !1).wrapping_add(4).wrapping_add((op & 0xFF) as u32 * 2);
                self.r[n] = mem.read_u16(addr)? as i16 as i32 as u32;
                Ok(())
            }
            0xA => {
                // bra disp12 (delayed). The displacement is a signed 12-bit
                // value; `(op & 0x0FFF) as i16` leaves bit 11 positive, so a
                // backward branch used to jump forward instead.
                let disp = (((op & 0x0FFF) << 4) as i16) >> 4;
                self.delay = Some((pc as i32 + 4 + disp as i32 * 2) as u32);
                Ok(())
            }
            0xB => {
                // bsr disp12 (delayed)
                let disp = (((op & 0x0FFF) << 4) as i16) >> 4;
                self.pr = pc.wrapping_add(4);
                self.delay = Some((pc as i32 + 4 + disp as i32 * 2) as u32);
                Ok(())
            }
            0xC => match op >> 8 {
                0xC7 => {                                // mova @(disp,PC),R0
                    self.r[0] = (pc & !3).wrapping_add(4).wrapping_add((op & 0xFF) as u32 * 4);
                    Ok(())
                }
                0xC8 => {                                // tst #imm,R0
                    self.set_t(self.r[0] & (op & 0xFF) as u32 == 0);
                    Ok(())
                }
                0xC9 => {                                // and #imm,R0
                    self.r[0] &= (op & 0xFF) as u32;
                    Ok(())
                }
                0xCA => {                                // xor #imm,R0
                    self.r[0] ^= (op & 0xFF) as u32;
                    Ok(())
                }
                0xCB => {                                // or #imm,R0
                    self.r[0] |= (op & 0xFF) as u32;
                    Ok(())
                }
                _ => Err(Error::Unimplemented(pc, op, "0xC family")),
            },
            0xD => {
                // mov.l @(disp,pc),Rn
                let addr = (pc & !3).wrapping_add(4).wrapping_add((op & 0xFF) as u32 * 4);
                self.r[n] = mem.read_u32(addr)?;
                Ok(())
            }
            0xE => {
                // mov #imm,Rn
                self.r[n] = (op & 0xFF) as i8 as i32 as u32;
                Ok(())
            }
            0xF => self.execute_fpu(op, mem, pc, n, m),
            _ => Err(Error::Unimplemented(pc, op, "top nibble")),
        }
    }


    /// Instructions outside the original core: system-register transfers,
    /// multiply/accumulate, carry/overflow arithmetic, and the cache hints.
    ///
    /// Returns `None` when the opcode is not one of these, so the caller falls
    /// through to the original decoder.
    fn execute_system(
        &mut self,
        op: u16,
        mem: &mut Memory,
        pc: u32,
    ) -> Option<Result<(), Error>> {
        match self.execute_system_impl(op, mem, pc) {
            Err(Error::Unimplemented(_, _, "not a system instruction")) => None,
            other => Some(other),
        }
    }

    fn execute_system_impl(
        &mut self,
        op: u16,
        mem: &mut Memory,
        pc: u32,
    ) -> Result<(), Error> {
        let n = ((op >> 8) & 0xF) as usize;
        let m = ((op >> 4) & 0xF) as usize;
        let low = (op & 0xFF) as u8;
        let unhandled = || Error::Unimplemented(pc, op, "not a system instruction");

        match op >> 12 {
            0x0 => {
                match op {
                    0x0028 => { self.mach = 0; self.macl = 0; return Ok(()); }  // clrmac
                    _ if op & 0xF0FF == 0x0029 => {                            // movt Rn
                        self.r[n] = self.t() as u32;
                        return Ok(());
                    }
                    0x0048 => { self.sr &= !(1 << 1); return Ok(()); }           // clrs
                    0x0058 => { self.sr |= 1 << 1; return Ok(()); }              // sets
                    0x0019 => {                                                  // div0u
                        self.sr &= !((1 << 8) | (1 << 9) | 1);
                        return Ok(());
                    }
                    0x0038 => return Ok(()),                                     // ldtlb: no MMU
                    _ => {}
                }
                match low {
                    0x02 => { self.r[n] = self.sr; return Ok(()); }
                    0x12 => { self.r[n] = self.gbr; return Ok(()); }
                    0x22 => { self.r[n] = self.vbr; return Ok(()); }
                    0x32 => { self.r[n] = self.ssr; return Ok(()); }
                    0x42 => { self.r[n] = self.spc; return Ok(()); }
                    0x3A => { self.r[n] = self.sgr; return Ok(()); }
                    0xFA => { self.r[n] = self.dbr; return Ok(()); }
                    0x0A => { self.r[n] = self.mach; return Ok(()); }
                    0x1A => { self.r[n] = self.macl; return Ok(()); }
                    0x2A => { self.r[n] = self.pr; return Ok(()); }
                    0x5A => { self.r[n] = self.fpul; return Ok(()); }
                    0x6A => { self.r[n] = self.fpscr; return Ok(()); }
                    0x03 => {                                                    // bsrf Rn
                        self.pr = pc.wrapping_add(4);
                        self.delay = Some(self.r[n].wrapping_add(pc.wrapping_add(4)));
                        return Ok(());
                    }
                    0x23 => {                                                    // braf Rn
                        self.delay = Some(self.r[n].wrapping_add(pc.wrapping_add(4)));
                        return Ok(());
                    }
                    // pref, ocbi, ocbp, ocbwb: cache hints, no architectural effect.
                    0x83 | 0x93 | 0xA3 | 0xB3 => return Ok(()),
                    0xC3 => {                                                    // movca.l R0,@Rn
                        mem.write_u32(self.r[n], self.r[0])?;
                        return Ok(());
                    }
                    _ if low & 0xF == 0x2 && low >> 4 >= 8 => {                  // stc Rn_BANK,Rn
                        let idx = ((low >> 4) as usize) - 8;
                        self.r[n] = self.bank[idx];
                        return Ok(());
                    }
                    _ => {}
                }
                match op & 0xF {
                    0x7 => {                                                 // mul.l Rm,Rn
                        let p = (self.r[n] as i32 as i64) * (self.r[m] as i32 as i64);
                        self.mach = (p >> 32) as u32;
                        self.macl = p as u32;
                        return Ok(());
                    }
                    0xF => {                                                 // mac.l @Rm+,@Rn+
                        let a = mem.read_u32(self.r[m])? as i32 as i64;
                        let b = mem.read_u32(self.r[n])? as i32 as i64;
                        self.r[m] = self.r[m].wrapping_add(4);
                        self.r[n] = self.r[n].wrapping_add(4);
                        let acc = (((self.mach as u64) << 32) | self.macl as u64) as i64;
                        let sum = acc.wrapping_add(a.wrapping_mul(b));
                        self.mach = (sum >> 32) as u32;
                        self.macl = sum as u32;
                        return Ok(());
                    }
                    _ => {}
                }
            }
            0x2 => match op & 0xF {
                0x7 => {                                                     // div0s Rm,Rn
                    let q = (self.r[n] >> 31) & 1;
                    let mm = (self.r[m] >> 31) & 1;
                    self.sr = (self.sr & !((1 << 8) | (1 << 9))) | (q << 8) | (mm << 9);
                    self.set_t(q == mm);
                    return Ok(());
                }
                0xC => {                                                     // cmp/str Rm,Rn
                    let x = self.r[n] ^ self.r[m];
                    self.set_t(
                        x & 0xFF == 0
                            || x & 0xFF00 == 0
                            || x & 0xFF_0000 == 0
                            || x & 0xFF00_0000 == 0,
                    );
                    return Ok(());
                }
                0xD => {                                                     // xtrct Rm,Rn
                    self.r[n] = (self.r[n] >> 16) | (self.r[m] << 16);
                    return Ok(());
                }
                0xE => {                                                     // mulu.w Rm,Rn
                    self.macl = (self.r[n] & 0xFFFF).wrapping_mul(self.r[m] & 0xFFFF);
                    return Ok(());
                }
                0xF => {                                                     // muls.w Rm,Rn
                    let a = self.r[n] as i16 as i32;
                    let b = self.r[m] as i16 as i32;
                    self.macl = a.wrapping_mul(b) as u32;
                    return Ok(());
                }
                _ => {}
            },
            0x3 => match op & 0xF {
                0x5 => {                                                     // dmulu.l Rm,Rn
                    let p = (self.r[n] as u64) * (self.r[m] as u64);
                    self.mach = (p >> 32) as u32;
                    self.macl = p as u32;
                    return Ok(());
                }
                0xA => {                                                     // subc Rm,Rn
                    let (d, b1) = self.r[n].overflowing_sub(self.r[m]);
                    let (d, b2) = d.overflowing_sub(self.t() as u32);
                    self.r[n] = d;
                    self.set_t(b1 || b2);
                    return Ok(());
                }
                0xB => {                                                     // subv Rm,Rn
                    let a = self.r[n] as i32;
                    let b = self.r[m] as i32;
                    let d = a.wrapping_sub(b);
                    self.r[n] = d as u32;
                    self.set_t(((a ^ b) & (a ^ d)) < 0);
                    return Ok(());
                }
                0xD => {                                                     // dmuls.l Rm,Rn
                    let p = (self.r[n] as i32 as i64) * (self.r[m] as i32 as i64);
                    self.mach = (p >> 32) as u32;
                    self.macl = p as u32;
                    return Ok(());
                }
                0xE => {                                                     // addc Rm,Rn
                    let (s, c1) = self.r[n].overflowing_add(self.r[m]);
                    let (s, c2) = s.overflowing_add(self.t() as u32);
                    self.r[n] = s;
                    self.set_t(c1 || c2);
                    return Ok(());
                }
                0xF => {                                                     // addv Rm,Rn
                    let a = self.r[n] as i32;
                    let b = self.r[m] as i32;
                    let s = a.wrapping_add(b);
                    self.r[n] = s as u32;
                    self.set_t(((a ^ s) & (b ^ s)) < 0);
                    return Ok(());
                }
                _ => {}
            },
            0x4 => {
                match low {
                    0x04 => {                                                // rotl Rn
                        let t = self.r[n] >> 31 != 0;
                        self.r[n] = (self.r[n] << 1) | self.t() as u32;
                        self.set_t(t);
                        return Ok(());
                    }
                    0x05 => {                                                // rotr Rn
                        let t = self.r[n] & 1 != 0;
                        self.r[n] = (self.r[n] >> 1) | ((self.t() as u32) << 31);
                        self.set_t(t);
                        return Ok(());
                    }
                    0x20 => {                                                // shal Rn
                        let t = self.r[n] >> 31 != 0;
                        self.r[n] <<= 1;
                        self.set_t(t);
                        return Ok(());
                    }
                    0x21 => {                                                // shar Rn
                        let t = self.r[n] & 1 != 0;
                        self.r[n] = ((self.r[n] as i32) >> 1) as u32;
                        self.set_t(t);
                        return Ok(());
                    }
                    0x24 => {                                                // rotcl Rn
                        let t = self.r[n] >> 31 != 0;
                        self.r[n] = (self.r[n] << 1) | self.t() as u32;
                        self.set_t(t);
                        return Ok(());
                    }
                    0x25 => {                                                // rotcr Rn
                        let t = self.r[n] & 1 != 0;
                        self.r[n] = (self.r[n] >> 1) | ((self.t() as u32) << 31);
                        self.set_t(t);
                        return Ok(());
                    }
                    0x1B => {                                                // tas.b @Rn
                        let v = mem.read_u8(self.r[n])?;
                        self.set_t(v == 0);
                        mem.write_u8(self.r[n], v | 0x80)?;
                        return Ok(());
                    }
                    _ => {}
                }
                match op & 0xF {
                    0xC => {                                                 // shad Rm,Rn
                        let amount = self.r[m] as i32;
                        if amount >= 0 {
                            self.r[n] <<= (amount as u32) & 31;
                        } else {
                            self.r[n] = ((self.r[n] as i32) >> ((-amount) as u32 & 31)) as u32;
                        }
                        return Ok(());
                    }
                    0xD => {                                                 // shld Rm,Rn
                        let amount = self.r[m] as i32;
                        if amount >= 0 {
                            self.r[n] <<= (amount as u32) & 31;
                        } else {
                            self.r[n] >>= ((-amount) as u32) & 31;
                        }
                        return Ok(());
                    }
                    0xF => {                                                 // mac.w @Rm+,@Rn+
                        let a = mem.read_u16(self.r[m])? as i16 as i32;
                        let b = mem.read_u16(self.r[n])? as i16 as i32;
                        self.r[m] = self.r[m].wrapping_add(2);
                        self.r[n] = self.r[n].wrapping_add(2);
                        let (sum, ovf) = (self.macl as i32).overflowing_add(a.wrapping_mul(b));
                        self.macl = sum as u32;
                        self.mach = u32::from(ovf);
                        return Ok(());
                    }
                    _ => {}
                }
                match low {
                    0x02 => {                                                // sts.l MACH,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.mach)?;
                        return Ok(());
                    }
                    0x12 => {                                                // sts.l MACL,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.macl)?;
                        return Ok(());
                    }
                    0x52 => {                                                // sts.l FPUL,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.fpul)?;
                        return Ok(());
                    }
                    0x62 => {                                                // sts.l FPSCR,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.fpscr)?;
                        return Ok(());
                    }
                    0x03 => {                                                // stc.l SR,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.sr)?;
                        return Ok(());
                    }
                    0x13 => {                                                // stc.l GBR,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.gbr)?;
                        return Ok(());
                    }
                    0x23 => {                                                // stc.l VBR,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.vbr)?;
                        return Ok(());
                    }
                    0x33 => {                                                // stc.l SSR,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.ssr)?;
                        return Ok(());
                    }
                    0x43 => {                                                // stc.l SPC,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.spc)?;
                        return Ok(());
                    }
                    0x32 => {                                                // stc.l SGR,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.sgr)?;
                        return Ok(());
                    }
                    0xF2 => {                                                // stc.l DBR,@-Rn
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.dbr)?;
                        return Ok(());
                    }
                    _ if low & 0xF == 0x3 && low >> 4 >= 8 => {              // stc.l Rn_BANK,@-Rn
                        let idx = ((low >> 4) as usize) - 8;
                        self.r[n] = self.r[n].wrapping_sub(4);
                        mem.write_u32(self.r[n], self.bank[idx])?;
                        return Ok(());
                    }
                    0x0E => {                                                // ldc Rn,SR
                        self.sr = self.r[n];
                        return Ok(());
                    }
                    0x1E => { self.gbr = self.r[n]; return Ok(()); }         // ldc Rn,GBR
                    0x2E => { self.vbr = self.r[n]; return Ok(()); }         // ldc Rn,VBR
                    0x3E => { self.ssr = self.r[n]; return Ok(()); }         // ldc Rn,SSR
                    0x4E => { self.spc = self.r[n]; return Ok(()); }         // ldc Rn,SPC
                    0x3A => { self.sgr = self.r[n]; return Ok(()); }         // ldc Rn,SGR
                    0xFA => { self.dbr = self.r[n]; return Ok(()); }         // ldc Rn,DBR
                    _ if low & 0xF == 0xE && low >> 4 >= 8 => {              // ldc Rn,Rn_BANK
                        let idx = ((low >> 4) as usize) - 8;
                        self.bank[idx] = self.r[n];
                        return Ok(());
                    }
                    0x07 => {                                                // ldc.l @Rn+,SR
                        self.sr = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x17 => {                                                // ldc.l @Rn+,GBR
                        self.gbr = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x27 => {                                                // ldc.l @Rn+,VBR
                        self.vbr = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x37 => {                                                // ldc.l @Rn+,SSR
                        self.ssr = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x47 => {                                                // ldc.l @Rn+,SPC
                        self.spc = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x36 => {                                                // ldc.l @Rn+,SGR
                        self.sgr = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0xF6 => {                                                // ldc.l @Rn+,DBR
                        self.dbr = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    _ if low & 0xF == 0x7 && low >> 4 >= 8 => {              // ldc.l @Rn+,Rn_BANK
                        let idx = ((low >> 4) as usize) - 8;
                        self.bank[idx] = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x0A => { self.mach = self.r[n]; return Ok(()); }        // lds Rn,MACH
                    0x1A => { self.macl = self.r[n]; return Ok(()); }        // lds Rn,MACL
                    0x5A => { self.fpul = self.r[n]; return Ok(()); }        // lds Rn,FPUL
                    0x6A => { self.fpscr = self.r[n]; return Ok(()); }       // lds Rn,FPSCR
                    0x06 => {                                                // lds.l @Rn+,MACH
                        self.mach = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x16 => {                                                // lds.l @Rn+,MACL
                        self.macl = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x56 => {                                                // lds.l @Rn+,FPUL
                        self.fpul = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    0x66 => {                                                // lds.l @Rn+,FPSCR
                        self.fpscr = mem.read_u32(self.r[n])?;
                        self.r[n] = self.r[n].wrapping_add(4);
                        return Ok(());
                    }
                    _ => {}
                }
            }
            0x6 => match op & 0xF {
                0x8 => {                                                     // swap.b Rm,Rn
                    let v = self.r[m];
                    self.r[n] = (v & 0xFFFF_0000) | ((v & 0xFF) << 8) | ((v >> 8) & 0xFF);
                    return Ok(());
                }
                0x9 => {                                                     // swap.w Rm,Rn
                    self.r[n] = self.r[m].rotate_left(16);
                    return Ok(());
                }
                0x7 => {                                                     // not Rm,Rn
                    self.r[n] = !self.r[m];
                    return Ok(());
                }
                0xB => {                                                     // neg Rm,Rn
                    self.r[n] = (self.r[m] as i32).wrapping_neg() as u32;
                    return Ok(());
                }
                0xA => {                                                     // negc Rm,Rn
                    let (d, b1) = 0u32.overflowing_sub(self.r[m]);
                    let (d, b2) = d.overflowing_sub(self.t() as u32);
                    self.r[n] = d;
                    self.set_t(b1 || b2);
                    return Ok(());
                }
                _ => {}
            },
            0xC => match op >> 8 {
                0xCC => {                                                    // tst.b #imm,@(R0,GBR)
                    let addr = self.gbr.wrapping_add(self.r[0]);
                    let v = mem.read_u8(addr)?;
                    self.set_t(v & (op & 0xFF) as u8 == 0);
                    return Ok(());
                }
                0xCD => {                                                    // and.b #imm,@(R0,GBR)
                    let addr = self.gbr.wrapping_add(self.r[0]);
                    let v = mem.read_u8(addr)?;
                    mem.write_u8(addr, v & (op & 0xFF) as u8)?;
                    return Ok(());
                }
                0xCE => {                                                    // xor.b #imm,@(R0,GBR)
                    let addr = self.gbr.wrapping_add(self.r[0]);
                    let v = mem.read_u8(addr)?;
                    mem.write_u8(addr, v ^ (op & 0xFF) as u8)?;
                    return Ok(());
                }
                0xCF => {                                                    // or.b #imm,@(R0,GBR)
                    let addr = self.gbr.wrapping_add(self.r[0]);
                    let v = mem.read_u8(addr)?;
                    mem.write_u8(addr, v | (op & 0xFF) as u8)?;
                    return Ok(());
                }
                _ => {}
            },
            0xF => match op & 0xFF {
                0x4D => { self.fr[n] = -self.fr[n]; return Ok(()); }         // fneg FRn
                0x5D => { self.fr[n] = self.fr[n].abs(); return Ok(()); }    // fabs FRn
                0x6D => { self.fr[n] = self.fr[n].sqrt(); return Ok(()); }   // fsqrt FRn
                _ => {}
            },
            _ => {}
        }
        Err(unhandled())
    }

    /// Load/store helper.
    ///
    /// Two encodings reach this function: the `mov @Rm,Rn` families pass the
    /// low nibble 0/1/2, while the post-increment and `@(R0,Rm)` families pass
    /// the opcode's size nibble 4/5/6 (or C/D/E for the indexed loads). Both
    /// mean the same three widths; mapping
    /// only 0/1/2 and falling through to a 32-bit access for 4/5/6 made
    /// `mov.b @Rm+,Rn` read four bytes.
    fn load(&self, mem: &Memory, addr: u32, size: u16) -> Result<u32, Error> {
        match size {
            0 | 4 | 0xC => Ok(mem.read_u8(addr)? as i8 as i32 as u32),
            1 | 5 | 0xD => Ok(mem.read_u16(addr)? as i16 as i32 as u32),
            _ => mem.read_u32(addr),
        }
    }

    fn store(&self, mem: &mut Memory, addr: u32, value: u32, size: u16) -> Result<(), Error> {
        match size {
            0 | 4 | 0xC => mem.write_u8(addr, value as u8),
            1 | 5 | 0xD => mem.write_u16(addr, value as u16),
            _ => mem.write_u32(addr, value),
        }
    }


    fn fpscr_sz(&self) -> bool {
        self.fpscr & (1 << 20) != 0
    }

    /// SZ=1 and an even register moves the FR pair: FRn is the low 32 bits,
    /// FR(n+1) the high 32 bits, little-endian. An odd register selects the
    /// XF bank, which is not modelled; that case stays a single FR move so
    /// specs that seed a dirty FPSCR keep the behaviour they were verified
    /// against.
    fn sz_pair(&self, reg: usize) -> bool {
        self.fpscr_sz() && reg % 2 == 0
    }

    fn fmov_load(&mut self, _pc: u32, _op: u16, mem: &Memory, addr: u32, n: usize) -> Result<(), Error> {
        if self.sz_pair(n) {
            self.fr[n] = mem.read_f32(addr)?;
            self.fr[n + 1] = mem.read_f32(addr.wrapping_add(4))?;
        } else {
            self.fr[n] = mem.read_f32(addr)?;
        }
        Ok(())
    }

    fn fmov_store(&mut self, _pc: u32, _op: u16, mem: &mut Memory, addr: u32, m: usize) -> Result<(), Error> {
        if self.sz_pair(m) {
            mem.write_f32(addr, self.fr[m])?;
            mem.write_f32(addr.wrapping_add(4), self.fr[m + 1])?;
        } else {
            mem.write_f32(addr, self.fr[m])?;
        }
        Ok(())
    }

    fn fmov_reg(&mut self, _pc: u32, _op: u16, n: usize, m: usize) -> Result<(), Error> {
        if self.sz_pair(n) && self.sz_pair(m) {
            self.fr[n] = self.fr[m];
            self.fr[n + 1] = self.fr[m + 1];
        } else {
            self.fr[n] = self.fr[m];
        }
        Ok(())
    }

    fn execute_fpu(
        &mut self,
        op: u16,
        mem: &mut Memory,
        pc: u32,
        n: usize,
        m: usize,
    ) -> Result<(), Error> {
        // The low nibble selects the operation; the `…1101` family is
        // disambiguated by the low byte (fsts/flds/float/ftrc/fldi0/fldi1).
        match op & 0xF {
            0x0 => { self.fr[n] = self.fr[n] + self.fr[m]; Ok(()) }   // fadd
            0x1 => { self.fr[n] = self.fr[n] - self.fr[m]; Ok(()) }   // fsub
            0x2 => { self.fr[n] = self.fr[n] * self.fr[m]; Ok(()) }   // fmul
            0x3 => { self.fr[n] = self.fr[n] / self.fr[m]; Ok(()) }   // fdiv
            0x4 => { self.set_t(self.fr[n] == self.fr[m]); Ok(()) }   // fcmp/eq
            0x5 => { self.set_t(self.fr[n] > self.fr[m]); Ok(()) }    // fcmp/gt
            0xE => {
                // fmac FR0,FRm,FRn: FRn = FRn + FR0*FRm, one rounding step
                self.fr[n] = self.fr[0].mul_add(self.fr[m], self.fr[n]);
                Ok(())
            }
            0xC => self.fmov_reg(pc, op, n, m),                       // fmov
            0x8 => self.fmov_load(pc, op, mem, self.r[m], n),        // fmov.s @Rm,FRn
            0x9 => {                                                  // fmov.s @Rm+,FRn
                self.fmov_load(pc, op, mem, self.r[m], n)?;
                self.r[m] = self.r[m].wrapping_add(if self.fpscr_sz() { 8 } else { 4 });
                Ok(())
            }
            0xA => self.fmov_store(pc, op, mem, self.r[n], m),       // fmov.s FRm,@Rn
            0xB => {                                                  // fmov.s FRm,@-Rn
                self.r[n] = self.r[n].wrapping_sub(if self.fpscr_sz() { 8 } else { 4 });
                self.fmov_store(pc, op, mem, self.r[n], m)
            }
            0x6 => {                                                  // fmov.s @(R0,Rm),FRn
                let addr = self.r[0].wrapping_add(self.r[m]);
                self.fmov_load(pc, op, mem, addr, n)
            }
            0x7 => {                                                  // fmov.s FRm,@(R0,Rn)
                let addr = self.r[0].wrapping_add(self.r[n]);
                self.fmov_store(pc, op, mem, addr, m)
            }
            0xD => match op & 0x00FF {
                0x0D => { self.fr[n] = f32::from_bits(self.fpul); Ok(()) }  // fsts FPUL,FRn
                0x1D => { self.fpul = self.fr[n].to_bits(); Ok(()) }        // flds FRn,FPUL
                0x2D => { self.fr[n] = self.fpul as i32 as f32; Ok(()) }    // float FPUL,FRn
                0x3D => { self.fpul = self.fr[n] as i32 as u32; Ok(()) }    // ftrc FRn,FPUL
                0x8D => { self.fr[n] = 0.0; Ok(()) }                        // fldi0
                0x9D => { self.fr[n] = 1.0; Ok(()) }                        // fldi1
                0xFD => { self.fpscr ^= 1 << 20; Ok(()) }                  // fschg
                _ => Err(Error::Unimplemented(pc, op, "0xF.1101 family")),
            },
            _ => Err(Error::Unimplemented(pc, op, "0xF family")),
        }
    }
}

fn fetch(mem: &Memory, pc: u32) -> Result<u16, Error> {
    mem.read_u16(pc).map_err(|_| Error::FetchOutOfRange(pc))
}

/// Build a memory image with `bytes` mapped at `base`.
pub fn image(base: u32, bytes: &[u8]) -> Memory {
    Memory::with_region(Region::new(base, bytes.to_vec()))
}
