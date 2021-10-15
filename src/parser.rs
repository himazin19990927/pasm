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

    macro_rules! test_mnemonic {
        ($input: expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = poco::MnemonicParser::new().parse(lexer).unwrap();

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
        test_mnemonic!(
            "LD r1, (r0)",
            Mnemonic::from(InstructionR {
                funct: Funct::LD,
                dst: Register::R1,
                src: Register::R0,
            })
        );

        test_mnemonic!(
            "LDI r1, #1",
            Mnemonic::from(InstructionI {
                opcode: Opcode::LDI,
                dst: Register::R1,
                immediate: 1.into(),
            })
        );

        test_mnemonic!(
            "LDI r1, #-1",
            Mnemonic::from(InstructionI {
                opcode: Opcode::LDI,
                dst: Register::R1,
                immediate: (-1).into(),
            })
        );

        test_mnemonic!(
            "ST r1, (r0)",
            Mnemonic::from(InstructionR {
                funct: Funct::ST,
                dst: Register::R0,
                src: Register::R1,
            })
        );

        test_mnemonic!(
            "ADD r0, r1",
            Mnemonic::from(InstructionR {
                funct: Funct::ADD,
                dst: Register::R0,
                src: Register::R1,
            })
        );

        test_mnemonic!(
            "ADDI r0, #1",
            Mnemonic::from(InstructionI {
                opcode: Opcode::ADDI,
                dst: Register::R0,
                immediate: 1.into(),
            })
        );

        test_mnemonic!(
            "ADDI r0, #-1",
            Mnemonic::from(InstructionI {
                opcode: Opcode::ADDI,
                dst: Register::R0,
                immediate: (-1).into(),
            })
        );
    }

    #[test]
    fn label() {
        test_mnemonic!(":label", Mnemonic::Label("label".to_string()));
        test_mnemonic!(":label0", Mnemonic::Label("label0".to_string()));
        test_mnemonic!(":end", Mnemonic::Label("end".to_string()));
    }

    #[test]
    fn file() {
        let input1 = r"
:start
LD r1, (r0)
:jump1
LDI r1, #1
:jump2 ST r1, (r0)
ADD r0, r1
ADDI r0, #1
:end
";
        let expected1 = vec![
            Mnemonic::Label("start".to_string()),
            Mnemonic::from(InstructionR {
                funct: Funct::LD,
                dst: Register::R1,
                src: Register::R0,
            }),
            Mnemonic::Label("jump1".to_string()),
            Mnemonic::from(InstructionI {
                opcode: Opcode::LDI,
                dst: Register::R1,
                immediate: 1.into(),
            }),
            Mnemonic::Label("jump2".to_string()),
            Mnemonic::from(InstructionR {
                funct: Funct::ST,
                dst: Register::R0,
                src: Register::R1,
            }),
            Mnemonic::from(InstructionR {
                funct: Funct::ADD,
                dst: Register::R0,
                src: Register::R1,
            }),
            Mnemonic::from(InstructionI {
                opcode: Opcode::ADDI,
                dst: Register::R0,
                immediate: 1.into(),
            }),
            Mnemonic::Label("end".to_string()),
        ];
        test_file!(input1, expected1);
    }
}
