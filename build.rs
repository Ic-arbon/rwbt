fn main() {
    println!("cargo::rerun-if-changed=data/RW_DM_CORE_REG.csv");
    println!("cargo::rerun-if-changed=data/RW_BT_CORE_REG.csv");
    println!("cargo::rerun-if-changed=data/RW_BLE_CORE_REG.csv");

    #[cfg(not(feature = "prebuild"))]
    gen::run();
}

#[cfg(not(feature = "prebuild"))]
mod gen {
    use std::env;
    use std::fmt::Write;
    use std::fs;
    use std::path::Path;

    pub fn run() {
        let out_dir = env::var("OUT_DIR").unwrap();
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let data_dir = Path::new(&manifest_dir).join("data");

        let mut output = String::new();
        writeln!(output, "// Auto-generated from RivieraWaves YODA CSV. Do not edit.").unwrap();
        writeln!(output).unwrap();

        generate_block(
            &mut output,
            &data_dir.join("RW_DM_CORE_REG.csv"),
            "Dm",
            "DM core",
        );
        generate_block(
            &mut output,
            &data_dir.join("RW_BT_CORE_REG.csv"),
            "Bt",
            "BT core",
        );
        generate_block(
            &mut output,
            &data_dir.join("RW_BLE_CORE_REG.csv"),
            "Ble",
            "BLE core",
        );

        fs::write(Path::new(&out_dir).join("mac_generated.rs"), output).unwrap();
    }

    // --- CSV parsing ---

    fn parse_csv_line(line: &str) -> Vec<String> {
        let mut fields = Vec::new();
        let mut current = String::new();
        let mut in_quotes = false;

        for ch in line.chars() {
            match ch {
                '"' => in_quotes = !in_quotes,
                ',' if !in_quotes => {
                    fields.push(std::mem::take(&mut current));
                }
                _ => current.push(ch),
            }
        }
        fields.push(current);
        fields
    }

    struct Field {
        name: String,
        width: u32,
        offset: u32,
        access: String,
    }

    struct Register {
        name: String,
        addr: u32,
        fields: Vec<Field>,
        is_array: bool,
        array_count: usize,
        array_stride: u32,
    }

    fn parse_registers(csv_path: &Path) -> Vec<Register> {
        let content = fs::read_to_string(csv_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {}", csv_path.display(), e));

        let mut lines = content.lines();
        lines.next(); // skip header

        // Group rows by register name, preserving insertion order
        let mut reg_order: Vec<String> = Vec::new();
        let mut reg_rows: std::collections::HashMap<String, Vec<Vec<String>>> =
            std::collections::HashMap::new();

        for line in lines {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let fields = parse_csv_line(line);
            if fields.len() < 12 {
                continue;
            }
            let rname = fields[0].clone();
            if !reg_rows.contains_key(&rname) {
                reg_order.push(rname.clone());
            }
            reg_rows.entry(rname).or_default().push(fields);
        }

        let mut registers = Vec::new();

        for rname in &reg_order {
            let rows = &reg_rows[rname];
            let first = &rows[0];

            let addr = u32::from_str_radix(&first[3], 16).unwrap();
            let (is_array, array_count, array_stride) = parse_array_info(&first[11]);

            // Collect public fields only
            let fields: Vec<Field> = rows
                .iter()
                .filter(|r| r[10] != "Private")
                .map(|r| Field {
                    name: r[1].clone(),
                    width: r[2].parse().unwrap(),
                    offset: r[5].parse().unwrap(),
                    access: r[6].clone(),
                })
                .collect();

            // Clean register name (remove [n] suffix for array registers)
            let clean_name = rname.split('[').next().unwrap().to_string();

            registers.push(Register {
                name: clean_name,
                addr,
                fields,
                is_array,
                array_count,
                array_stride,
            });
        }

        registers
    }

    /// Parse RArray field: "True;n;start;end;step;regsize" or "False;..."
    fn parse_array_info(s: &str) -> (bool, usize, u32) {
        let s = s.trim_matches('"');
        let parts: Vec<&str> = s.split(';').collect();
        if parts[0] == "True" {
            let start: usize = parts[2].parse().unwrap();
            let end: usize = parts[3].parse().unwrap();
            let step: usize = parts[4].parse().unwrap();
            let regsize: u32 = parts[5].parse().unwrap();
            let count = (end - start) / step + 1;
            (true, count, regsize)
        } else {
            (false, 1, 4)
        }
    }

    // --- Name conversion ---

    fn to_snake_case(s: &str) -> String {
        s.to_ascii_lowercase()
    }

    fn to_pascal_case(s: &str) -> String {
        s.split('_')
            .filter(|p| !p.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(c) => {
                        let mut s = c.to_uppercase().to_string();
                        s.extend(chars.map(|c| c.to_ascii_lowercase()));
                        s
                    }
                }
            })
            .collect()
    }

    // --- Field type helpers ---

    fn field_rust_type(width: u32) -> &'static str {
        match width {
            1 => "bool",
            2..=8 => "u8",
            9..=16 => "u16",
            17..=32 => "u32",
            _ => panic!("Unsupported field width: {width}"),
        }
    }

    fn field_mask(width: u32) -> u32 {
        if width >= 32 {
            0xFFFF_FFFF
        } else {
            (1u32 << width) - 1
        }
    }

    /// Determine register-level access from public fields.
    fn register_access(fields: &[Field]) -> &'static str {
        let has_read = fields.iter().any(|f| f.access.contains('R'));
        let has_write = fields.iter().any(|f| f.access.contains('W'));
        match (has_read, has_write) {
            (true, true) => "RW",
            (true, false) => "R",
            (false, true) => "W",
            (false, false) => "RW",
        }
    }

    // --- Code generation ---

    fn generate_block(output: &mut String, csv_path: &Path, block_name: &str, doc: &str) {
        let registers = parse_registers(csv_path);
        let mod_name = block_name.to_ascii_lowercase();

        writeln!(output, "pub mod {mod_name} {{").unwrap();
        writeln!(output, "    #[allow(unused_imports)]").unwrap();
        writeln!(output, "    use crate::common::{{Reg, R, W, RW}};").unwrap();
        writeln!(output).unwrap();

        // Block struct
        writeln!(output, "    /// {doc} block.").unwrap();
        writeln!(output, "    #[derive(Copy, Clone, Eq, PartialEq)]").unwrap();
        writeln!(output, "    pub struct {block_name} {{").unwrap();
        writeln!(output, "        ptr: *mut u8,").unwrap();
        writeln!(output, "    }}").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "    unsafe impl Send for {block_name} {{}}").unwrap();
        writeln!(output, "    unsafe impl Sync for {block_name} {{}}").unwrap();
        writeln!(output).unwrap();

        // Block impl
        writeln!(output, "    impl {block_name} {{").unwrap();
        writeln!(output, "        #[inline(always)]").unwrap();
        writeln!(
            output,
            "        pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {{"
        )
        .unwrap();
        writeln!(output, "            Self {{ ptr: ptr as *mut u8 }}").unwrap();
        writeln!(output, "        }}").unwrap();
        writeln!(output).unwrap();
        writeln!(output, "        #[inline(always)]").unwrap();
        writeln!(
            output,
            "        pub const fn as_ptr(&self) -> *mut () {{"
        )
        .unwrap();
        writeln!(output, "            self.ptr as *mut ()").unwrap();
        writeln!(output, "        }}").unwrap();

        for reg in &registers {
            if reg.fields.is_empty() {
                continue;
            }

            let method_name = to_snake_case(&reg.name);
            let type_name = to_pascal_case(&reg.name);
            let access = register_access(&reg.fields);

            writeln!(output).unwrap();

            if reg.is_array {
                writeln!(
                    output,
                    "        /// {} (0x{:02X}) — {} entries, stride {}",
                    reg.name, reg.addr, reg.array_count, reg.array_stride
                )
                .unwrap();
                writeln!(output, "        #[inline(always)]").unwrap();
                writeln!(
                    output,
                    "        pub const fn {method_name}(self, n: usize) -> Reg<regs::{type_name}, {access}> {{"
                )
                .unwrap();
                writeln!(output, "            assert!(n < {});", reg.array_count).unwrap();
                writeln!(
                    output,
                    "            unsafe {{ Reg::from_ptr(self.ptr.add(0x{:02X}usize + n * {}usize) as _) }}",
                    reg.addr, reg.array_stride
                )
                .unwrap();
                writeln!(output, "        }}").unwrap();
            } else {
                writeln!(output, "        /// {} (0x{:02X})", reg.name, reg.addr).unwrap();
                writeln!(output, "        #[inline(always)]").unwrap();
                writeln!(
                    output,
                    "        pub const fn {method_name}(self) -> Reg<regs::{type_name}, {access}> {{"
                )
                .unwrap();
                writeln!(
                    output,
                    "            unsafe {{ Reg::from_ptr(self.ptr.add(0x{:02X}usize) as _) }}",
                    reg.addr
                )
                .unwrap();
                writeln!(output, "        }}").unwrap();
            }
        }

        writeln!(output, "    }}").unwrap();
        writeln!(output).unwrap();

        // Regs module
        writeln!(output, "    pub mod regs {{").unwrap();

        for reg in &registers {
            if reg.fields.is_empty() {
                continue;
            }

            let type_name = to_pascal_case(&reg.name);

            writeln!(output).unwrap();
            writeln!(output, "        /// {}", reg.name).unwrap();
            writeln!(output, "        #[repr(transparent)]").unwrap();
            writeln!(output, "        #[derive(Copy, Clone, Eq, PartialEq)]").unwrap();
            writeln!(output, "        pub struct {type_name}(pub u32);").unwrap();
            writeln!(output).unwrap();
            writeln!(output, "        impl Default for {type_name} {{").unwrap();
            writeln!(output, "            fn default() -> Self {{ Self(0) }}").unwrap();
            writeln!(output, "        }}").unwrap();
            writeln!(output).unwrap();
            writeln!(output, "        impl {type_name} {{").unwrap();

            for field in &reg.fields {
                let fname = to_snake_case(&field.name);
                let ftype = field_rust_type(field.width);
                let mask = field_mask(field.width);
                let offset = field.offset;

                // Getter
                if field.width == 1 {
                    writeln!(output).unwrap();
                    writeln!(
                        output,
                        "            /// {name} — 1 bit (offset {offset})",
                        name = field.name
                    )
                    .unwrap();
                    writeln!(output, "            #[inline(always)]").unwrap();
                    writeln!(
                        output,
                        "            pub const fn {fname}(&self) -> bool {{"
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "                (self.0 >> {offset}) & 0x1 != 0"
                    )
                    .unwrap();
                    writeln!(output, "            }}").unwrap();
                } else if field.width == 32 && offset == 0 {
                    writeln!(output).unwrap();
                    writeln!(
                        output,
                        "            /// {name} — {width} bits (offset {offset})",
                        name = field.name,
                        width = field.width
                    )
                    .unwrap();
                    writeln!(output, "            #[inline(always)]").unwrap();
                    writeln!(
                        output,
                        "            pub const fn {fname}(&self) -> {ftype} {{"
                    )
                    .unwrap();
                    writeln!(output, "                self.0").unwrap();
                    writeln!(output, "            }}").unwrap();
                } else {
                    writeln!(output).unwrap();
                    writeln!(
                        output,
                        "            /// {name} — {width} bits (offset {offset})",
                        name = field.name,
                        width = field.width
                    )
                    .unwrap();
                    writeln!(output, "            #[inline(always)]").unwrap();
                    writeln!(
                        output,
                        "            pub const fn {fname}(&self) -> {ftype} {{"
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "                ((self.0 >> {offset}) & 0x{mask:X}) as {ftype}"
                    )
                    .unwrap();
                    writeln!(output, "            }}").unwrap();
                }

                // Setter
                if field.width == 1 {
                    writeln!(output).unwrap();
                    writeln!(output, "            #[inline(always)]").unwrap();
                    writeln!(
                        output,
                        "            pub fn set_{fname}(&mut self, val: bool) {{"
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "                self.0 = (self.0 & !(0x1 << {offset})) | ((val as u32) << {offset});"
                    )
                    .unwrap();
                    writeln!(output, "            }}").unwrap();
                } else if field.width == 32 && offset == 0 {
                    writeln!(output).unwrap();
                    writeln!(output, "            #[inline(always)]").unwrap();
                    writeln!(
                        output,
                        "            pub fn set_{fname}(&mut self, val: {ftype}) {{"
                    )
                    .unwrap();
                    writeln!(output, "                self.0 = val;").unwrap();
                    writeln!(output, "            }}").unwrap();
                } else {
                    writeln!(output).unwrap();
                    writeln!(output, "            #[inline(always)]").unwrap();
                    writeln!(
                        output,
                        "            pub fn set_{fname}(&mut self, val: {ftype}) {{"
                    )
                    .unwrap();
                    writeln!(
                        output,
                        "                self.0 = (self.0 & !(0x{mask:X} << {offset})) | ((val as u32 & 0x{mask:X}) << {offset});"
                    )
                    .unwrap();
                    writeln!(output, "            }}").unwrap();
                }
            }

            writeln!(output, "        }}").unwrap();
        }

        writeln!(output, "    }}").unwrap(); // close regs module
        writeln!(output, "}}").unwrap(); // close block module
        writeln!(output).unwrap();
    }
}
