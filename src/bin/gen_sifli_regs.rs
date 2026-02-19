//! Generate `src/rfc/sifli/regs.rs` from sifli-pac `bt_rfc.yaml`.
//!
//! Usage:
//! ```sh
//! cargo run --features _gen --bin gen-sifli-regs -- <path-to-bt_rfc.yaml> > src/rfc/sifli/regs.rs
//! rustfmt src/rfc/sifli/regs.rs
//! ```

use std::collections::HashMap;
use std::env;
use std::fmt::Write;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: gen-sifli-regs <path-to-bt_rfc.yaml>");
        std::process::exit(1);
    }

    let content = fs::read_to_string(&args[1]).unwrap_or_else(|e| {
        eprintln!("Failed to read {}: {e}", args[1]);
        std::process::exit(1);
    });

    let lines: Vec<&str> = content.lines().collect();
    let registers = parse_registers(&lines);
    let fieldsets = parse_fieldsets(&lines);
    let output = generate(&registers, &fieldsets);
    print!("{output}");
}

// ---------------------------------------------------------------------------
// Data structures
// ---------------------------------------------------------------------------

struct Register {
    name: String,
    byte_offset: u16,
    fieldset_type: String,
}

struct Field {
    name: String,
    bit_offset: u16,
    bit_size: u16,
}

struct Fieldset {
    description: String,
    fields: Vec<Field>,
}

// ---------------------------------------------------------------------------
// YAML parsing (line-scan, no library needed)
// ---------------------------------------------------------------------------

/// Extract register list from the `block/bt_rfc::BtRfc:` items section.
fn parse_registers(lines: &[&str]) -> Vec<Register> {
    let mut regs = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        // Match "- name: <reg_name>" followed by "byte_offset:" and "fieldset:"
        if let Some(rest) = line.strip_prefix("- name:") {
            let name = rest.trim().to_string();
            if i + 2 < lines.len() {
                let l2 = lines[i + 1].trim();
                let l3 = lines[i + 2].trim();
                if let (Some(off_str), Some(fs_str)) = (
                    l2.strip_prefix("byte_offset:"),
                    l3.strip_prefix("fieldset:"),
                ) {
                    let off_str = off_str.trim();
                    let byte_offset = parse_int(off_str);
                    let fs_str = fs_str.trim();
                    // "bt_rfc::regs::VcoReg1" → "VcoReg1"
                    let fieldset_type = fs_str.rsplit("::").next().unwrap().to_string();
                    regs.push(Register {
                        name,
                        byte_offset,
                        fieldset_type,
                    });
                    i += 3;
                    continue;
                }
            }
        }
        i += 1;
    }
    regs
}

/// Extract all fieldset definitions from `fieldset/bt_rfc::regs::*:` sections.
fn parse_fieldsets(lines: &[&str]) -> HashMap<String, Fieldset> {
    let mut map = HashMap::new();
    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();
        // Match "fieldset/bt_rfc::regs::VcoReg1:"
        if let Some(rest) = line.strip_prefix("fieldset/bt_rfc::regs::") {
            if let Some(type_name) = rest.strip_suffix(':') {
                let type_name = type_name.to_string();
                i += 1;

                // Parse description
                let mut description = String::new();
                while i < lines.len() {
                    let l = lines[i].trim();
                    if let Some(d) = l.strip_prefix("description:") {
                        description = d.trim().trim_matches('"').to_string();
                        i += 1;
                        break;
                    }
                    if l.starts_with("fields:") {
                        break;
                    }
                    i += 1;
                }

                // Parse fields
                let mut fields = Vec::new();
                while i < lines.len() {
                    let l = lines[i].trim();
                    // Stop at next section
                    if (l.starts_with("fieldset/") || l.starts_with("block/")) && l.ends_with(':')
                    {
                        break;
                    }
                    if let Some(rest) = l.strip_prefix("- name:") {
                        let fname = rest.trim().to_string();
                        if i + 2 < lines.len() {
                            let l2 = lines[i + 1].trim();
                            let l3 = lines[i + 2].trim();
                            if let (Some(bo), Some(bs)) = (
                                l2.strip_prefix("bit_offset:"),
                                l3.strip_prefix("bit_size:"),
                            ) {
                                let bit_offset: u16 = bo.trim().parse().unwrap();
                                let bit_size: u16 = bs.trim().parse().unwrap();
                                fields.push(Field {
                                    name: fname,
                                    bit_offset,
                                    bit_size,
                                });
                                i += 3;
                                continue;
                            }
                        }
                    }
                    i += 1;
                }

                map.insert(type_name, Fieldset { description, fields });
                continue;
            }
        }
        i += 1;
    }
    map
}

fn parse_int(s: &str) -> u16 {
    if let Some(hex) = s.strip_prefix("0x").or_else(|| s.strip_prefix("0X")) {
        u16::from_str_radix(hex, 16).unwrap()
    } else {
        s.parse().unwrap()
    }
}

// ---------------------------------------------------------------------------
// Code generation
// ---------------------------------------------------------------------------

fn generate(registers: &[Register], fieldsets: &HashMap<String, Fieldset>) -> String {
    let mut out = String::new();

    // Header
    writeln!(
        out,
        "//! Auto-generated from sifli-pac bt_rfc.yaml. Do not edit manually."
    )
    .unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! Regenerate with:").unwrap();
    writeln!(out, "//! ```sh").unwrap();
    writeln!(
        out,
        "//! cargo run --features _gen --bin gen-sifli-regs -- <path-to-bt_rfc.yaml> > src/rfc/sifli/regs.rs"
    )
    .unwrap();
    writeln!(out, "//! rustfmt src/rfc/sifli/regs.rs").unwrap();
    writeln!(out, "//! ```").unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(out, "//! These constants serve two purposes:").unwrap();
    writeln!(
        out,
        "//! 1. **Register offsets** → used in RFC command `rd()`/`wr()` instructions."
    )
    .unwrap();
    writeln!(
        out,
        "//! 2. **Bit positions** → used in RFC command `or()`/`and()` instructions."
    )
    .unwrap();
    writeln!(out, "//!").unwrap();
    writeln!(
        out,
        "//! For boolean (1-bit) fields, the constant IS the bit number."
    )
    .unwrap();
    writeln!(
        out,
        "//! For multi-bit fields, the constant is the LSB position; individual bits can be"
    )
    .unwrap();
    writeln!(
        out,
        "//! addressed as `FIELD + n` where `n < width`."
    )
    .unwrap();
    writeln!(out).unwrap();

    // offset module
    writeln!(
        out,
        "/// Register byte offsets within the BT_RFC peripheral."
    )
    .unwrap();
    writeln!(out, "///").unwrap();
    writeln!(
        out,
        "/// Used as arguments to [`super::cmd::rd`] and [`super::cmd::wr`]."
    )
    .unwrap();
    writeln!(out, "pub mod offset {{").unwrap();
    for reg in registers {
        let const_name = reg.name.to_ascii_uppercase();
        writeln!(
            out,
            "    pub const {const_name}: u16 = 0x{:02X};",
            reg.byte_offset
        )
        .unwrap();
    }
    writeln!(out, "}}").unwrap();
    writeln!(out).unwrap();

    // Per-register field modules
    for reg in registers {
        let Some(fs) = fieldsets.get(&reg.fieldset_type) else {
            continue;
        };
        if fs.fields.is_empty() {
            continue;
        }

        writeln!(
            out,
            "/// {} (offset 0x{:02X})",
            fs.description, reg.byte_offset
        )
        .unwrap();
        writeln!(out, "pub mod {} {{", reg.name).unwrap();

        for field in &fs.fields {
            let const_name = field.name.to_ascii_uppercase();
            let bit_str = if field.bit_size == 1 { "bit" } else { "bits" };
            writeln!(
                out,
                "    /// {} — {} {} (offset {})",
                field.name, field.bit_size, bit_str, field.bit_offset
            )
            .unwrap();
            writeln!(
                out,
                "    pub const {const_name}: u16 = {};",
                field.bit_offset
            )
            .unwrap();
        }

        writeln!(out, "}}").unwrap();
        writeln!(out).unwrap();
    }

    out
}
