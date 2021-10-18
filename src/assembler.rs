use crate::{code::*, mnemonic::*};
use std::collections::HashMap;

pub fn convert<I>(input: I) -> (Vec<Mnemonic>, HashMap<String, i16>)
where
    I: IntoIterator<Item = Item>,
{
    let mut mnemonics = Vec::new();
    let mut table = HashMap::new();

    let mut current_index = 0;
    for line in input {
        match line {
            Item::Label(label) => {
                table.insert(label, current_index);
            }
            Item::Mnemonic(m) => {
                current_index += 1;
                mnemonics.push(m);
            }
        }
    }

    return (mnemonics, table);
}

pub fn assemble<I>(input: I, table: &HashMap<String, i16>) -> Vec<Code>
where
    I: IntoIterator<Item = Mnemonic>,
{
    let mut result = Vec::new();
    for (addr, line) in input.into_iter().enumerate() {
        let code = encode(line, table, addr as i16);
        result.push(code);
    }

    return result;
}

pub fn encode(line: Mnemonic, table: &HashMap<String, i16>, current_addr: i16) -> Code {
    let code = match &line {
        Mnemonic::R(instr) => {
            let f = instr.funct.id();
            let d = instr.dst.id();
            let s = instr.src.id();
            (d << 8) | (s << 5) | f
        }
        Mnemonic::I(instr) => {
            let c = instr.opcode.id();
            let d = instr.dst.id();
            let x = instr.immediate as u16 & 0b00000_000_11111111;

            (c << 11) | (d << 8) | x
        }
        Mnemonic::B(instr) => {
            let c = instr.opcode.id();
            let s = instr.src.id();

            let dst_addr = table[&instr.label];
            let addr = dst_addr - current_addr;

            (c << 11) | (s << 8) | addr as u16 & (0b00000_000_11111111)
        }
        Mnemonic::J(instr) => {
            let c = instr.opcode.id();

            let dst_addr = table[&instr.label];
            let addr = dst_addr - current_addr;

            (c << 11) | addr as u16 & 0b00000_11111111111
        }
        Mnemonic::JR(_) => todo!(),
    };

    Code::new(code, line)
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use crate::{
        assembler::{convert, encode},
        code::*,
        mnemonic::*,
        register::*,
    };

    #[test]
    fn generate_table() {
        {
            let items = vec![
                Item::label("init".into()),
                Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 0
                Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 1
                Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 2
                Item::label("loop".into()),
                Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 3
                Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 4
                Item::label("end".into()),
                Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 5
            ];

            let (_, table) = convert(items);
            assert_eq!(table["init".into()], 0);
            assert_eq!(table["loop".into()], 3);
            assert_eq!(table["end".into()], 5);
        }
    }

    #[test]
    fn encode_i() {
        let table = &HashMap::new();

        // LDI r1, #1
        let ldi = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, 1);
        let ldi_code = Code::new(0b01000_001_00000001, ldi.clone());
        assert_eq!(ldi_code, encode(ldi, table, 0));

        // LDI r1, #-1
        let ldi = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, -1);
        let ldi_code = Code::new(0b01000_001_11111111, ldi.clone());
        assert_eq!(ldi_code, encode(ldi, table, 0));

        // ADDI r0, #120
        let addi = Mnemonic::instr_i(OpcodeI::ADDI, Register::R0, 120);
        let addi_code = Code::new(0b01100_000_01111000, addi.clone());
        assert_eq!(addi_code, encode(addi, table, 0));

        // ADDI r0, #-120
        let addi = Mnemonic::instr_i(OpcodeI::ADDI, Register::R0, -120);
        let addi_code = Code::new(0b01100_000_10001000, addi.clone());
        assert_eq!(addi_code, encode(addi, table, 0));
    }

    #[test]
    fn encode_r() {
        let table = &HashMap::new();

        // LD r1, (r0)
        let ld = Mnemonic::instr_r(FunctR::LD, Register::R1, Register::R0);
        let ld_code = Code::new(0b00000_001_000_01001, ld.clone());
        assert_eq!(ld_code, encode(ld, table, 0));

        // ST r1, (r0)
        let st = Mnemonic::instr_r(FunctR::ST, Register::R0, Register::R1);
        let st_code = Code::new(0b00000_000_001_01000, st.clone());
        assert_eq!(st_code, encode(st, table, 0));

        // ADD r0, r1
        let add = Mnemonic::instr_r(FunctR::ADD, Register::R0, Register::R1);
        let add_code = Code::new(0b00000_000_001_00110, add.clone());
        assert_eq!(add_code, encode(add, table, 0));
    }

    #[test]
    fn encode_j() {
        let items = vec![
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 0
            Item::instr_j(OpcodeJ::JMP, "label".into()),            // 1
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 2
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 3
            Item::label("label".into()),                            // :label
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 4
            Item::label("loop".into()),                             // :loop
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 5
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 6
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 7
            Item::instr_j(OpcodeJ::JMP, "loop".into()),             // 8
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 9
            Item::label("end".into()),                              // :end
            Item::instr_j(OpcodeJ::JMP, "end".into()),              // 10
        ];

        let (_, table) = convert(items);

        // JMP label (1 -> 4 => +3)
        let jmp_label = Mnemonic::instr_j(OpcodeJ::JMP, "label".into());
        let jmp_label_code = Code::new(0b10100_00000000011, jmp_label.clone());
        assert_eq!(jmp_label_code, encode(jmp_label, &table, 1));

        // JMP loop (8 -> 5 => -3)
        let jmp_loop = Mnemonic::instr_j(OpcodeJ::JMP, "loop".into());
        let jmp_loop_code = Code::new(0b10100_11111111101, jmp_loop.clone());
        assert_eq!(jmp_loop_code, encode(jmp_loop, &table, 8));

        // JMP end (10 -> 10 => 0)
        let jmp_end = Mnemonic::instr_j(OpcodeJ::JMP, "end".into());
        let jmp_end_code = Code::new(0b10100_00000000000, jmp_end.clone());
        assert_eq!(jmp_end_code, encode(jmp_end, &table, 10));
    }

    #[test]
    fn encode_b() {
        let items = vec![
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 0
            Item::instr_b(OpcodeB::BEZ, Register::R0, "label".into()), // 1
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 2
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 3
            Item::label("label".into()),                            // :label
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 4
            Item::label("loop".into()),                             // :loop
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 5
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 6
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 7
            Item::instr_b(OpcodeB::BEZ, Register::R0, "loop".into()), // 8
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1), // 9
            Item::label("end".into()),                              // :end
            Item::instr_b(OpcodeB::BEZ, Register::R0, "ebd".into()), // 10
        ];

        let (_, table) = convert(items);

        // JMP label (1 -> 4 => +3)
        let bez_label = Mnemonic::instr_b(OpcodeB::BEZ, Register::R0, "label".into());
        let bez_label_code = Code::new(0b10000_000_00000011, bez_label.clone());
        assert_eq!(bez_label_code, encode(bez_label, &table, 1));

        // JMP loop (8 -> 5 => -3)
        let bez_loop = Mnemonic::instr_b(OpcodeB::BEZ, Register::R0, "loop".into());
        let bez_loop_code = Code::new(0b10000_000_11111101, bez_loop.clone());
        assert_eq!(bez_loop_code, encode(bez_loop, &table, 8));

        // JMP end (10 -> 10 => 0)
        let bez_end = Mnemonic::instr_b(OpcodeB::BEZ, Register::R0, "end".into());
        let bez_end_code = Code::new(0b10000_000_00000000, bez_end.clone());
        assert_eq!(bez_end_code, encode(bez_end, &table, 10));
    }
}
