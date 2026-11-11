//! Run an SH-4 image (or a slice of one) and print architectural state.
//!
//! The oracle exists to answer one question per function: does the Rust
//! rewrite compute the same registers and memory? This tiny CLI makes that
//! scriptable from the spec runner and usable by hand.

use std::path::PathBuf;
use std::process::ExitCode;

use sh4_oracle::{image, Cpu, Stop, LOAD_BASE};

fn parse_hex(s: &str) -> Result<u32, String> {
    let t = s.trim();
    let t = t.strip_prefix("0x").or_else(|| t.strip_prefix("0X")).unwrap_or(t);
    u32::from_str_radix(t, 16).map_err(|e| format!("bad hex {s:?}: {e}"))
}

fn usage() -> ExitCode {
    eprintln!(
        "usage: sh4-oracle <image> [--base 0x8C010000] [--entry 0x...] \
[--steps N] [--set rN=0xVALUE | --set fN=FLOAT] [--dump-args]"
    );
    ExitCode::from(2)
}

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next().filter(|a| a != "-h" && a != "--help") else {
        return usage();
    };
    let path = PathBuf::from(path);

    let mut base = LOAD_BASE;
    let mut entry = LOAD_BASE;
    let mut steps = 1_000_000usize;
    let mut sets: Vec<String> = Vec::new();

    while let Some(flag) = args.next() {
        let mut take = |name: &str| -> Result<String, ExitCode> {
            args.next().ok_or_else(|| {
                eprintln!("{name} needs a value");
                ExitCode::from(2)
            })
        };
        let r = match flag.as_str() {
            "--base" => take("--base").and_then(|v| parse_hex(&v).map(|v| base = v).map_err(|_| ExitCode::from(2))),
            "--entry" => take("--entry").and_then(|v| parse_hex(&v).map(|v| entry = v).map_err(|_| ExitCode::from(2))),
            "--steps" => take("--steps").and_then(|v| {
                v.parse::<usize>()
                    .map(|v| steps = v)
                    .map_err(|_| ExitCode::from(2))
            }),
            "--set" => take("--set").map(|v| sets.push(v)),
            other => {
                eprintln!("unknown flag {other}");
                return usage();
            }
        };
        if let Err(code) = r {
            return code;
        }
    }

    let bytes = match std::fs::read(&path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("cannot read {}: {e}", path.display());
            return ExitCode::from(1);
        }
    };

    let mut mem = image(base, &bytes);
    let mut cpu = Cpu::new();
    cpu.pc = entry;

    for spec in sets {
        let (reg, value) = match spec.split_once('=') {
            Some(p) => p,
            None => {
                eprintln!("--set expects rN=VALUE or fN=VALUE, got {spec:?}");
                return ExitCode::from(2);
            }
        };
        let (kind, idx) = reg.split_at(1);
        let Ok(idx) = idx.parse::<usize>() else {
            eprintln!("bad register {reg:?}");
            return ExitCode::from(2);
        };
        if idx > 15 {
            eprintln!("register index out of range: {reg:?}");
            return ExitCode::from(2);
        }
        match kind {
            "r" => match parse_hex(value) {
                Ok(v) => cpu.r[idx] = v,
                Err(e) => {
                    eprintln!("{e}");
                    return ExitCode::from(2);
                }
            },
            "f" => match value.parse::<f32>() {
                Ok(v) => cpu.fr[idx] = v,
                Err(e) => {
                    eprintln!("bad float {value:?}: {e}");
                    return ExitCode::from(2);
                }
            },
            _ => {
                eprintln!("register must start with r or f: {reg:?}");
                return ExitCode::from(2);
            }
        }
    }

    let stop = cpu.run(&mut mem, steps);
    println!("image      {} ({} bytes) at 0x{base:08x}", path.display(), bytes.len());
    println!("entry      0x{entry:08x}");
    match &stop {
        Ok(Stop::Halt) => println!("stop       halt (rts -> HALT_PC)"),
        Ok(Stop::Steps(n)) => println!("stop       step limit {n}"),
        Err(e) => println!("error      {e}"),
    }
    println!("steps      {}", cpu.steps);
    for row in 0..4 {
        let mut line = String::new();
        for col in 0..4 {
            let i = row * 4 + col;
            line.push_str(&format!("r{i:<2}=0x{:08x}  ", cpu.r[i]));
        }
        println!("{line}");
    }
    println!(
        "pr=0x{:08x} sr=0x{:08x} t={} pc=0x{:08x}",
        cpu.pr,
        cpu.sr,
        cpu.t(),
        cpu.pc
    );
    for row in 0..2 {
        let mut line = String::new();
        for col in 0..4 {
            let i = row * 4 + col;
            line.push_str(&format!("fr{i:<2}={:<12}  ", cpu.fr[i]));
        }
        println!("{line}");
    }
    println!("writes      {}", mem.writes.len());

    match stop {
        Ok(Stop::Halt) => ExitCode::SUCCESS,
        Ok(Stop::Steps(_)) => ExitCode::from(3),
        Err(_) => ExitCode::from(1),
    }
}
