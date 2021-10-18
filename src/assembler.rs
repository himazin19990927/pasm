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
        Mnemonic::JR(instr) => {
            let f = instr.funct.id();
            let d = instr.dst.id();
            (d << 8) | f
        }
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
    fn encode_r() {
        let table = &HashMap::new();

        {
            // NOP
            let m = Mnemonic::instr_r(FunctR::NOP, Register::R0, Register::R0);
            let c = Code::new(0b00000_000_000_00000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // MV r0, r1
            let m = Mnemonic::instr_r(FunctR::MV, Register::R0, Register::R1);
            let c = Code::new(0b00000_000_001_00001, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // AND r0, r1
            let m = Mnemonic::instr_r(FunctR::AND, Register::R0, Register::R1);
            let c = Code::new(0b00000_000_001_00010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // OR r0, r1
            let m = Mnemonic::instr_r(FunctR::OR, Register::R0, Register::R1);
            let c = Code::new(0b00000_000_001_00011, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // SL r0
            let m = Mnemonic::instr_r(FunctR::SL, Register::R1, Register::R0);
            let c = Code::new(0b00000_001_000_00100, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // SR r0
            let m = Mnemonic::instr_r(FunctR::SR, Register::R1, Register::R0);
            let c = Code::new(0b00000_001_000_00101, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // ADD r0, r1
            let m = Mnemonic::instr_r(FunctR::ADD, Register::R0, Register::R1);
            let c = Code::new(0b00000_000_001_00110, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // SUB r0, r1
            let m = Mnemonic::instr_r(FunctR::SUB, Register::R0, Register::R1);
            let c = Code::new(0b00000_000_001_00111, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // ST r1, (r2)
            let m = Mnemonic::instr_r(FunctR::ST, Register::R1, Register::R2);
            let c = Code::new(0b00000_001_010_01000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LD r1, (r2)
            let m = Mnemonic::instr_r(FunctR::LD, Register::R1, Register::R2);
            let c = Code::new(0b00000_001_010_01001, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }
    }

    #[test]
    fn encode_i() {
        let table = &HashMap::new();

        {
            // LDI r1, #2
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, 2);
            let c = Code::new(0b01000_001_00000010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDIU r1, #2
            let m = Mnemonic::instr_i(OpcodeI::LDIU, Register::R1, 2);
            let c = Code::new(0b01001_001_00000010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // ADDI r1, #2
            let m = Mnemonic::instr_i(OpcodeI::ADDI, Register::R1, 2);
            let c = Code::new(0b01100_001_00000010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // ADDIU r1, #2
            let m = Mnemonic::instr_i(OpcodeI::ADDIU, Register::R1, 2);
            let c = Code::new(0b01101_001_00000010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDHI r1, #2
            let m = Mnemonic::instr_i(OpcodeI::LDHI, Register::R1, 2);
            let c = Code::new(0b01010_001_00000010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }
    }

    #[test]
    fn encode_i_immediate() {
        let table = &HashMap::new();

        {
            // LDI r1, #-127
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, -127);
            let c = Code::new(0b01000_001_10000001, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r1, #-1
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, -1);
            let c = Code::new(0b01000_001_11111111, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r1, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, 0);
            let c = Code::new(0b01000_001_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r1, #1
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, 1);
            let c = Code::new(0b01000_001_00000001, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r1, #127
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, 127);
            let c = Code::new(0b01000_001_01111111, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }
    }

    #[test]
    fn encode_i_register() {
        let table = &HashMap::new();

        {
            // LDI r0, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R0, 0);
            let c = Code::new(0b01000_000_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r1, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R1, 0);
            let c = Code::new(0b01000_001_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r2, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R2, 0);
            let c = Code::new(0b01000_010_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r3, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R3, 0);
            let c = Code::new(0b01000_011_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r4, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R4, 0);
            let c = Code::new(0b01000_100_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r5, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R5, 0);
            let c = Code::new(0b01000_101_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r6, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R6, 0);
            let c = Code::new(0b01000_110_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // LDI r7, #0
            let m = Mnemonic::instr_i(OpcodeI::LDI, Register::R7, 0);
            let c = Code::new(0b01000_111_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }
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

    #[test]
    fn encode_jr() {
        let table = &HashMap::new();

        // JR r0
        let jr = Mnemonic::instr_jr(FunctJR::JR, Register::R0);
        let jr_code = Code::new(0b00000_000_000_01010, jr.clone());
        assert_eq!(jr_code, encode(jr, table, 0));
    }
}
