use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub poco);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{lexer::*, mnemonic::*, register::*};

    macro_rules! test_register {
        ($input: expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = poco::RegisterParser::new().parse(lexer).unwrap();

            assert_eq!($expected, result);
        };
    }

    macro_rules! test_item {
        ($input: expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = poco::ItemParser::new().parse(lexer).unwrap();

            assert_eq!($expected, result);
        };
    }

    macro_rules! test_file {
        ($input:expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = poco::FileParser::new().parse(lexer).unwrap();

            assert_eq!($expected, result);
        };
    }

    #[test]
    fn register() {
        test_register!("r0", Register::R0);
        test_register!("r1", Register::R1);
        test_register!("r2", Register::R2);
        test_register!("r3", Register::R3);
        test_register!("r4", Register::R4);
        test_register!("r5", Register::R5);
        test_register!("r6", Register::R6);
        test_register!("r7", Register::R7);
    }

    #[test]
    fn instruction() {
        test_item!(
            "LD r1, (r0)",
            Item::instr_r(FunctR::LD, Register::R1, Register::R0)
        );
        test_item!("LDI r1, #1", Item::instr_i(OpcodeI::LDI, Register::R1, 1));
        test_item!("LDI r1, #-1", Item::instr_i(OpcodeI::LDI, Register::R1, -1));
        test_item!(
            "ST r1, (r0)",
            Item::instr_r(FunctR::ST, Register::R0, Register::R1)
        );
        test_item!(
            "ADD r0, r1",
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1)
        );
        test_item!("ADDI r0, #1", Item::instr_i(OpcodeI::ADDI, Register::R0, 1));
        test_item!(
            "ADDI r0, #-1",
            Item::instr_i(OpcodeI::ADDI, Register::R0, -1)
        );
    }

    #[test]
    fn instruction_j() {
        test_item!("JMP label", Item::instr_j(OpcodeJ::JMP, "label".into()));
    }

    #[test]
    fn instruction_b() {
        test_item!(
            "BEZ r0, label",
            Item::instr_b(OpcodeB::BEZ, Register::R0, "label".into())
        );
    }

    #[test]
    fn instruction_jr() {
        test_item!("JR r0", Item::instr_jr(OpcodeJR::JR, Register::R0));
    }

    #[test]
    fn label() {
        test_item!(":label", Item::Label("label".to_string()));
        test_item!(":label0", Item::Label("label0".to_string()));
        test_item!(":end", Item::Label("end".to_string()));
    }

    #[test]
    fn file() {
        let input1 = r"
:start
LD r1, (r0)
BEZ r0, jump1
:jump1
LDI r1, #1
:jump2 ST r1, (r0)
ADD r0, r1
ADDI r0, #1
JMP end
:end
";
        let expected1 = vec![
            Item::label("start".to_string()),
            Item::instr_r(FunctR::LD, Register::R1, Register::R0),
            Item::instr_b(OpcodeB::BEZ, Register::R0, "jump1".into()),
            Item::label("jump1".to_string()),
            Item::instr_i(OpcodeI::LDI, Register::R1, 1.into()),
            Item::label("jump2".to_string()),
            Item::instr_r(FunctR::ST, Register::R0, Register::R1),
            Item::instr_r(FunctR::ADD, Register::R0, Register::R1),
            Item::instr_i(OpcodeI::ADDI, Register::R0, 1.into()),
            Item::instr_j(OpcodeJ::JMP, "end".into()),
            Item::label("end".to_string()),
        ];
        test_file!(input1, expected1);
    }
}
