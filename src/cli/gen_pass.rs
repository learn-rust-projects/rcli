use std::rc::Rc;

use super::prelude::*;

#[derive(Parser, Debug)]
pub struct GenPassOpts {
    #[arg(short, long, help = "Password length", default_value_t = 16)]
    pub length: u8,
    #[arg(long, help = "Password has upper case", default_value_t = true)]
    pub upper_case: bool,
    #[arg(long, help = "Password has lower case", default_value_t = true)]
    pub lower_case: bool,
    #[arg(long, help = "Password has number", default_value_t = true)]
    pub number: bool,
    #[arg(long, help = "Password has symbol", default_value_t = true)]
    pub symbol: bool,
}

impl CmdExc for GenPassOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let result = crate::gen_pass(
            self.length,
            self.upper_case,
            self.lower_case,
            self.number,
            self.symbol,
        );
        let strength = zxcvbn::zxcvbn(&result?, &[]);
        eprintln!("Password score: {:?}", strength.score());
        let rc = Rc::new(1);
        println!("{}", rc);

        Ok(())
    }
}
