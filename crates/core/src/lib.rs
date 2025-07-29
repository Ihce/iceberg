use capstone::{BuildsCapstone, Capstone, Insn, arch::arm::ArchMode};
use memmap2::Mmap;
use serde::Serialize;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

#[derive(Serialize)]
pub struct Row {
    pub off: u64,
    pub size: u8,
    pub bytes: String,
    pub text: String,
    pub ok: bool,
}

/// Dump one‑ISA superset disassembly to `out.jsonl`.
pub fn scan<P: AsRef<Path>>(bin: P, out: P) -> std::io::Result<()> {
    let f = File::open(&bin)?;
    let map = unsafe { Mmap::map(&f)? };

    let cs = Capstone::new()
        .arm()
        .mode(ArchMode::Arm) // pick your ISA/mode
        .build()
        .unwrap();

    let mut w = BufWriter::new(File::create(out)?);

    let mut off = 0usize;
    while off < map.len() {
        let slice = &map[off..];
        if let Ok(insns) = cs.disasm_all(slice, off as u64) {
            if let Some(i) = insns.iter().next() {
                write_row(&mut w, i, true)?;
                off += i.bytes().len();
                continue;
            }
        }
        // invalid byte
        write_row_invalid(&mut w, off as u64, slice[0])?;
        off += 1;
    }
    Ok(())
}

fn write_row(w: &mut impl Write, i: &Insn, ok: bool) -> std::io::Result<()> {
    serde_json::to_writer(
        w,
        &Row {
            off: i.address(),
            size: i.bytes().len() as u8,
            bytes: hex::encode(i.bytes()),
            text: format!(
                "{} {}",
                i.mnemonic().unwrap_or(""),
                i.op_str().unwrap_or("")
            ),
            ok,
        },
    )?;
    w.write_all(b"\n")
}

fn write_row_invalid(w: &mut impl Write, off: u64, b: u8) -> std::io::Result<()> {
    serde_json::to_writer(
        w,
        &Row {
            off,
            size: 1,
            bytes: format!("{:02x}", b),
            text: ".db".into(),
            ok: false,
        },
    )?;
    w.write_all(b"\n")
}
