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

    macro_rules! test_instruction {
        ($input: expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = poco::InstructionParser::new().parse(lexer).unwrap();

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
        test_instruction!(
            "LD r1, (r0)",
            Instruction::from(InstructionR {
                funct: Funct::LD,
                dst: Register::R1,
                src: Register::R0,
            })
        );

        test_instruction!(
            "LDI r1, #1",
            Instruction::from(InstructionI {
                opcode: Opcode::LDI,
                dst: Register::R1,
                immediate: 1.into(),
            })
        );

        test_instruction!(
            "ST r1, (r0)",
            Instruction::from(InstructionR {
                funct: Funct::ST,
                dst: Register::R0,
                src: Register::R1,
            })
        );

        test_instruction!(
            "ADD r0, r1",
            Instruction::from(InstructionR {
                funct: Funct::ADD,
                dst: Register::R0,
                src: Register::R1,
            })
        );

        test_instruction!(
            "ADDI r0, #1",
            Instruction::from(InstructionI {
                opcode: Opcode::ADDI,
                dst: Register::R0,
                immediate: 1.into(),
            })
        );
    }

    #[test]
    fn file() {
        let input1 = r"
LD r1, (r0)
LDI r1, #1
ST r1, (r0)
ADD r0, r1
ADDI r0, #1
";
        let expected1 = vec![
            Instruction::from(InstructionR {
                funct: Funct::LD,
                dst: Register::R1,
                src: Register::R0,
            }),
            Instruction::from(InstructionI {
                opcode: Opcode::LDI,
                dst: Register::R1,
                immediate: 1.into(),
            }),
            Instruction::from(InstructionR {
                funct: Funct::ST,
                dst: Register::R0,
                src: Register::R1,
            }),
            Instruction::from(InstructionR {
                funct: Funct::ADD,
                dst: Register::R0,
                src: Register::R1,
            }),
            Instruction::from(InstructionI {
                opcode: Opcode::ADDI,
                dst: Register::R0,
                immediate: 1.into(),
            }),
        ];
        test_file!(input1, expected1);
    }
}
