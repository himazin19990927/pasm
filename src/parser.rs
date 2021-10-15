use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub poco);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ast::*, lexer::*, lit::*};

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

    macro_rules! lit_int {
        ($value: expr) => {
            Lit::Int(LitInt {
                digits: $value.to_string(),
            })
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
            Instruction::LD {
                dst: Register::R1,
                src: Parenthesized(Register::R0)
            }
        );

        test_instruction!(
            "LDI r1, #1",
            Instruction::LDI {
                dst: Register::R1,
                src: lit_int!(1),
            }
        );

        test_instruction!(
            "ST r1, (r0)",
            Instruction::ST {
                dst: Parenthesized(Register::R0),
                src: Register::R1,
            }
        );

        test_instruction!(
            "ADD r0, r1",
            Instruction::ADD {
                dst: Register::R0,
                src: Register::R1
            }
        );

        test_instruction!(
            "ADDI r0, #1",
            Instruction::ADDI {
                dst: Register::R0,
                src: lit_int!(1),
            }
        );
    }
}
