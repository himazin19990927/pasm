use crate::{code::*, mnemonic::*};

pub fn encode(line: Instruction) -> Code {
    match line {
        Instruction::I(_) => {
            todo!()
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
