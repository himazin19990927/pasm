use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub poco);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::*;
    use crate::lexer::*;

    macro_rules! test_register {
        ($input: expr, $expected: expr) => {
            let lexer = Lexer::new($input);
            let result = poco::RegisterParser::new().parse(lexer).unwrap();

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
}
