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
    let code = match &line {
        Instruction::I(instr) => {
            let c = instr.opcode.id();
            let d = instr.dst.id();
            let x: u16 = match &instr.immediate {
                Lit::Int(imm) => match imm.digits.parse() {
                    Ok(imm) => imm,
                    Err(_) => unimplemented!(),
                },
            };

            (c << 11) | (d << 8) | x
        }
        Instruction::R(instr) => {
            let f = instr.funct.id();
            let d = instr.dst.id();
            let s = instr.src.id();
            (d << 8) | (s << 5) | f
        }
    };

    Code::new(code, line)
}

#[cfg(test)]
pub mod tests {
    use crate::{assembler::encode, code::*, mnemonic::*, register::*};

    #[test]
    fn encode_i() {
        // LDI r1, #1
        let ldi: Instruction = InstructionI {
            opcode: Opcode::LDI,
            dst: Register::R1,
            immediate: 1.into(),
        }
        .into();
        let ldi_code = Code::new(0b01000_001_00000001, ldi.clone());
        assert_eq!(ldi_code, encode(ldi));

        // ADDI r0, #1
        let addi: Instruction = InstructionI {
            opcode: Opcode::ADDI,
            dst: Register::R0,
            immediate: 1.into(),
        }
        .into();
        let addi_code = Code::new(0b01100_000_00000001, addi.clone());
        assert_eq!(addi_code, encode(addi));
    }

    #[test]
    fn encode_r() {
        // LD r1, (r0)
        let ld: Instruction = InstructionR {
            funct: Funct::LD,
            dst: Register::R1,
            src: Register::R0,
        }
        .into();

        let ld_code = Code::new(0b00000_001_000_01001, ld.clone());
        assert_eq!(ld_code, encode(ld));

        // ST r1, (r0)
        let st: Instruction = InstructionR {
            funct: Funct::ST,
            dst: Register::R0,
            src: Register::R1,
        }
        .into();
        let st_code = Code::new(0b00000_000_001_01000, st.clone());
        assert_eq!(st_code, encode(st));

        // ADD r0, r1
        let add: Instruction = InstructionR {
            funct: Funct::ADD,
            dst: Register::R0,
            src: Register::R1,
        }
        .into();
        let add_code = Code::new(0b00000_000_001_00110, add.clone());
        assert_eq!(add_code, encode(add));
    }
}
