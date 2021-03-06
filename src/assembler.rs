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
    fn encode_b_addr() {
        let items = vec![
            Item::label("l0".into()),                               //      :l0
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 0    NOP
            Item::instr_b(OpcodeB::BEZ, Register::R0, "l0".into()), // 1    BEZ r0, l0  (1 -> 0 => -1)
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 2    NOP
            Item::label("l1".into()),                               //      :l1
            Item::instr_b(OpcodeB::BEZ, Register::R0, "l1".into()), // 3    BEZ r0, l1 // (3 -> 3 => 0)
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 4    NOP
            Item::instr_b(OpcodeB::BEZ, Register::R0, "l2".into()), // 5    BEZ r0, l2 // (5 -> 6 => 1)
            Item::label("l2".into()),                               //      :l2
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 6    NOP
        ];

        let (_, table) = convert(items);

        {
            // 1    BEZ r0, l0  (1 -> 0 => -1)
            let m = Mnemonic::instr_b(OpcodeB::BEZ, Register::R0, "l0".into());
            let c = Code::new(0b10000_000_11111111, m.clone());
            assert_eq!(c, encode(m, &table, 1));
        }

        {
            // 3    BEZ r0, l1 // (3 -> 3 => 0)
            let m = Mnemonic::instr_b(OpcodeB::BEZ, Register::R0, "l1".into());
            let c = Code::new(0b10000_000_00000000, m.clone());
            assert_eq!(c, encode(m, &table, 3));
        }

        {
            // 5    BEZ r0, l2 // (5 -> 6 => 1)
            let m = Mnemonic::instr_b(OpcodeB::BEZ, Register::R0, "l2".into());
            let c = Code::new(0b10000_000_00000001, m.clone());
            assert_eq!(c, encode(m, &table, 5));
        }
    }

    #[test]
    fn encode_b_instr() {
        let table = &{
            let mut t = HashMap::new();
            t.insert("label".into(), 0);
            t
        };

        {
            // BEZ r0, label
            let m = Mnemonic::instr_b(OpcodeB::BEZ, Register::R0, "label".into());
            let c = Code::new(0b10000_000_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // BNZ r0, label
            let m = Mnemonic::instr_b(OpcodeB::BNZ, Register::R0, "label".into());
            let c = Code::new(0b10001_000_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // BPL r0, label
            let m = Mnemonic::instr_b(OpcodeB::BPL, Register::R0, "label".into());
            let c = Code::new(0b10010_000_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // BMI r0, label
            let m = Mnemonic::instr_b(OpcodeB::BMI, Register::R0, "label".into());
            let c = Code::new(0b10011_000_00000000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }
    }

    #[test]
    fn encode_j_jmp() {
        let items = vec![
            Item::label("l0".into()),                               //      :l0
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 0    NOP
            Item::instr_j(OpcodeJ::JMP, "l0".into()),               // 1    JMP l0  (1 -> 0 => -1)
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 2    NOP
            Item::label("l1".into()),                               //      :l1
            Item::instr_j(OpcodeJ::JMP, "l1".into()),               // 3    JMP l1 // (3 -> 3 => 0)
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 4    NOP
            Item::instr_j(OpcodeJ::JMP, "l2".into()),               // 5    JMP l2 // (5 -> 6 => 1)
            Item::label("l2".into()),                               //      :l2
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 6    NOP
        ];

        let (_, table) = convert(items);

        {
            // 1    JMP l0 // (1 -> 0 => -1)
            let m = Mnemonic::instr_j(OpcodeJ::JMP, "l0".into());
            let c = Code::new(0b10100_11111111111, m.clone());
            assert_eq!(c, encode(m, &table, 1));
        }

        {
            // 3    JMP l1 // (3 -> 3 => 0)
            let m = Mnemonic::instr_j(OpcodeJ::JMP, "l1".into());
            let c = Code::new(0b10100_00000000000, m.clone());
            assert_eq!(c, encode(m, &table, 3));
        }

        {
            // 5    JMP l2 // (5 -> 6 => 1)
            let m = Mnemonic::instr_j(OpcodeJ::JMP, "l2".into());
            let c = Code::new(0b10100_00000000001, m.clone());
            assert_eq!(c, encode(m, &table, 5));
        }
    }

    #[test]
    fn encode_j_jal() {
        let items = vec![
            Item::label("l0".into()),                               //      :l0
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 0    NOP
            Item::instr_j(OpcodeJ::JAL, "l0".into()),               // 1    JAL l0  (1 -> 0 => -1)
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 2    NOP
            Item::label("l1".into()),                               //      :l1
            Item::instr_j(OpcodeJ::JAL, "l1".into()),               // 3    JAL l1 // (3 -> 3 => 0)
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 4    NOP
            Item::instr_j(OpcodeJ::JAL, "l2".into()),               // 5    JAL l2 // (5 -> 6 => 1)
            Item::label("l2".into()),                               //      :l2
            Item::instr_r(FunctR::NOP, Register::R0, Register::R0), // 6    NOP
        ];

        let (_, table) = convert(items);

        {
            // 1    JAL l0 // (1 -> 0 => -1)
            let m = Mnemonic::instr_j(OpcodeJ::JAL, "l0".into());
            let c = Code::new(0b10101_11111111111, m.clone());
            assert_eq!(c, encode(m, &table, 1));
        }

        {
            // 3    JMP l1 // (3 -> 3 => 0)
            let m = Mnemonic::instr_j(OpcodeJ::JAL, "l1".into());
            let c = Code::new(0b10101_00000000000, m.clone());
            assert_eq!(c, encode(m, &table, 3));
        }

        {
            // 5    JAL l2 // (5 -> 6 => 1)
            let m = Mnemonic::instr_j(OpcodeJ::JAL, "l2".into());
            let c = Code::new(0b10101_00000000001, m.clone());
            assert_eq!(c, encode(m, &table, 5));
        }
    }

    #[test]
    fn encode_jr() {
        let table = &HashMap::new();
        {
            // JR r0
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R0);
            let c = Code::new(0b00000_000_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // JALR r0
            let m = Mnemonic::instr_jr(FunctJR::JALR, Register::R0);
            let c = Code::new(0b00000_000_000_11000, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }
    }

    #[test]
    fn encode_jr_register() {
        let table = &HashMap::new();
        {
            // JR r0
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R0);
            let c = Code::new(0b00000_000_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // JR r1
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R1);
            let c = Code::new(0b00000_001_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // JR r2
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R2);
            let c = Code::new(0b00000_010_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // JR r3
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R3);
            let c = Code::new(0b00000_011_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // JR r4
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R4);
            let c = Code::new(0b00000_100_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // JR r5
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R5);
            let c = Code::new(0b00000_101_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // JR r6
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R6);
            let c = Code::new(0b00000_110_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }

        {
            // JR r7
            let m = Mnemonic::instr_jr(FunctJR::JR, Register::R7);
            let c = Code::new(0b00000_111_000_01010, m.clone());
            assert_eq!(c, encode(m, table, 0));
        }
    }
}
