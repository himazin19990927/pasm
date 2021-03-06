use crate::mnemonic::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Code {
    code: u16,
    instruction: Mnemonic,
}

impl Code {
    pub fn new(code: u16, instruction: Mnemonic) -> Self {
        Code {
            code: code,
            instruction: instruction,
        }
    }

    pub fn get_code(&self) -> u16 {
        self.code
    }

    pub fn get_line(&self, underscore: bool, mnemonic: bool) -> String {
        let code = if underscore {
            match self.instruction {
                Mnemonic::R(_) => {
                    let (d, s, f) = self.split_as_r_instr();
                    format!("{:05b}_{:03b}_{:03b}_{:05b}", 0, d, s, f)
                }
                Mnemonic::I(_) => {
                    let (c, d, x) = self.split_as_i_instr();
                    format!("{:05b}_{:03b}_{:08b}", c, d, x)
                }
                Mnemonic::B(_) => {
                    let (c, s, x) = self.split_as_i_instr();
                    format!("{:05b}_{:03b}_{:08b}", c, s, x)
                }
                Mnemonic::J(_) => {
                    let (c, x) = self.split_as_j_instr();
                    format!("{:05b}_{:011b}", c, x)
                }
                Mnemonic::JR(_) => {
                    let (d, _, f) = self.split_as_r_instr();
                    format!("{:05b}_{:03b}_{:03b}_{:05b}", 0, d, 0, f)
                }
            }
        } else {
            format!("{:016b}", self.code)
        };

        let comment = if mnemonic {
            format!("// {}", self.instruction)
        } else {
            "".to_string()
        };

        if underscore {
            format!("{:<20} {}", code, comment)
        } else {
            format!("{:<16} {}", code, comment)
        }
    }

    fn split_as_r_instr(&self) -> (u16, u16, u16) {
        let d = (0b00000_111_000_00000 & self.code) >> 8;
        let s = (0b00000_000_111_00000 & self.code) >> 5;
        let f = 0b00000_000_000_11111 & self.code;

        (d, s, f)
    }

    fn split_as_i_instr(&self) -> (u16, u16, u16) {
        let c = (0b11111_000_00000000 & self.code) >> 11;
        let d = (0b00000_111_00000000 & self.code) >> 8;
        let x = 0b00000_000_11111111 & self.code;

        (c, d, x)
    }

    fn split_as_j_instr(&self) -> (u16, u16) {
        let c = (0b11111_00000000000 & self.code) >> 11;
        let x = 0b00000_11111111111 & self.code;

        (c, x)
    }
}
