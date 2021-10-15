use crate::{code::*, lit::*, mnemonic::*};

pub fn assemble<I>(input: I) -> Vec<Code>
where
    I: IntoIterator<Item = Instruction>,
{
    let mut result = Vec::new();
    for line in input {
        let code = encode(line);
        result.push(code);
    }

    return result;
}

pub fn encode(line: Instruction) -> Code {
    match line {
        Instruction::I(instr) => {
            let c = instr.opcode.id();
            let d = instr.dst.id();
            let x: u16 = match instr.immediate {
                Lit::Int(imm) => match imm.digits.parse() {
                    Ok(imm) => imm,
                    Err(_) => unimplemented!(),
                },
            };

            let code = (c << 11) | (d << 8) | x;

            Code::new(code, Type::I)
        }
        Instruction::R(instr) => {
            let f = instr.funct.id();
            let d = instr.dst.id();
            let s = instr.src.id();
            let code = (d << 8) | (s << 5) | f;
            Code::new(code, Type::R)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{assembler::encode, code::*, mnemonic::*, register::*};

    #[test]
    fn encode_i() {
        // LDI r1, #1
        let ldi = InstructionI {
            opcode: Opcode::LDI,
            dst: Register::R1,
            immediate: 1.into(),
        }
        .into();
        let ldi_code = Code::new(0b01000_001_00000001, Type::I);
        assert_eq!(ldi_code, encode(ldi));

        // ADDI r0, #1
        let addi = InstructionI {
            opcode: Opcode::ADDI,
            dst: Register::R0,
            immediate: 1.into(),
        }
        .into();
        let addi_code = Code::new(0b01100_000_00000001, Type::I);
        assert_eq!(addi_code, encode(addi));
    }

    #[test]
    fn encode_r() {
        // LD r1, (r0)
        let ld = InstructionR {
            funct: Funct::LD,
            dst: Register::R1,
            src: Register::R0,
        }
        .into();
        let ld_code = Code::new(0b00000_001_000_01001, Type::R);
        assert_eq!(ld_code, encode(ld));

        // ST r1, (r0)
        let st = InstructionR {
            funct: Funct::ST,
            dst: Register::R0,
            src: Register::R1,
        }
        .into();
        let st_code = Code::new(0b00000_000_001_01000, Type::R);
        assert_eq!(st_code, encode(st));

        // ADD r0, r1
        let add = InstructionR {
            funct: Funct::ADD,
            dst: Register::R0,
            src: Register::R1,
        }
        .into();
        let add_code = Code::new(0b00000_000_001_00110, Type::R);
        assert_eq!(add_code, encode(add));
    }
}
