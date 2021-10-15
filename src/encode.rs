use crate::{
    ast,
    lit::*,
    mnemonic,
    mnemonic::{Funct, InstructionI, InstructionR, Opcode},
};

pub fn encode(line: ast::Instruction) -> Result<mnemonic::Instruction, ()> {
    match line {
        ast::Instruction::LD { dst, src } => Ok(mnemonic::Instruction::R(InstructionR {
            funct: Funct::LD,
            dst: dst,
            src: src.0,
        })),
        ast::Instruction::LDI { dst, src } => match src {
            Lit::Int(lit) => match lit.to_immediate() {
                Ok(immidiate) => Ok(mnemonic::Instruction::I(InstructionI {
                    opcode: Opcode::LDI,
                    dst: dst,
                    immediate: immidiate,
                })),
                Err(_) => Err(()),
            },
        },
        ast::Instruction::ST { dst, src } => Ok(mnemonic::Instruction::R(InstructionR {
            funct: Funct::ST,
            dst: dst.0,
            src: src,
        })),
        ast::Instruction::ADD { dst, src } => Ok(mnemonic::Instruction::R(InstructionR {
            funct: Funct::ADD,
            dst: dst,
            src: src,
        })),
        ast::Instruction::ADDI { dst, src } => match src {
            Lit::Int(lit) => match lit.to_immediate() {
                Ok(immidiate) => Ok(mnemonic::Instruction::I(InstructionI {
                    opcode: Opcode::ADDI,
                    dst: dst,
                    immediate: immidiate,
                })),
                Err(_) => Err(()),
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::register::*;

    #[test]
    fn encode_ld() {
        let lhs = mnemonic::Instruction::R(InstructionR {
            funct: Funct::LD,
            dst: Register::R0,
            src: Register::R1,
        });

        let rhs = ast::Instruction::LD {
            dst: Register::R0,
            src: Parenthesized(Register::R1),
        };

        assert_eq!(Ok(lhs), encode(rhs));
    }

    #[test]
    fn encode_ldi() {
        let lhs = mnemonic::Instruction::I(InstructionI {
            opcode: Opcode::LDI,
            dst: Register::R0,
            immediate: 1,
        });

        let rhs = ast::Instruction::LDI {
            dst: Register::R0,
            src: Lit::Int(LitInt {
                digits: "1".to_string(),
            }),
        };

        assert_eq!(Ok(lhs), encode(rhs));
    }

    #[test]
    fn encode_st() {
        let lhs = mnemonic::Instruction::R(InstructionR {
            funct: Funct::ST,
            dst: Register::R0,
            src: Register::R1,
        });

        let rhs = ast::Instruction::ST {
            dst: Parenthesized(Register::R0),
            src: Register::R1,
        };

        assert_eq!(Ok(lhs), encode(rhs));
    }

    #[test]
    fn encode_add() {
        let lhs = mnemonic::Instruction::R(InstructionR {
            funct: Funct::ADD,
            dst: Register::R0,
            src: Register::R1,
        });

        let rhs = ast::Instruction::ADD {
            dst: Register::R0,
            src: Register::R1,
        };

        assert_eq!(Ok(lhs), encode(rhs));
    }

    #[test]
    fn encode_addi() {
        let lhs = mnemonic::Instruction::I(InstructionI {
            opcode: Opcode::ADDI,
            dst: Register::R0,
            immediate: 1,
        });

        let rhs = ast::Instruction::ADDI {
            dst: Register::R0,
            src: Lit::Int(LitInt {
                digits: "1".to_string(),
            }),
        };

        assert_eq!(Ok(lhs), encode(rhs));
    }
}
