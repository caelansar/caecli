use crate::{process_jwt_sign, process_jwt_verify, CmdExector};

use chrono::{Duration, Utc};
use clap::Parser;
use enum_dispatch::enum_dispatch;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum JwtSubCommand {
    #[command(about = "sign")]
    Sign(SignOpts),
    #[command(about = "verify")]
    Verify(VerifyOpts),
}

#[derive(Debug, Parser)]
pub struct SignOpts {
    // subject
    #[arg(long)]
    pub sub: String,
    // audience
    #[arg(long)]
    pub aud: String,
    // expiration time
    #[arg(long, value_parser = parse_exp)]
    pub exp: usize,
}

#[derive(Debug, Parser)]
pub struct VerifyOpts {
    #[arg(long, short)]
    pub token: String,
}

impl CmdExector for SignOpts {
    async fn execute(self) -> anyhow::Result<()> {
        println!("{} {} {}", self.sub, self.aud, self.exp);
        let token = process_jwt_sign(self.sub, self.aud, self.exp)?;
        println!("{token}");
        Ok(())
    }
}

impl CmdExector for VerifyOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let data = process_jwt_verify(&self.token);
        if data.is_ok() {
            println!("✓ Signature verified");
        } else {
            println!("⚠ Signature not verified");
        }
        Ok(())
    }
}

fn parse_exp(exp: &str) -> Result<usize, &'static str> {
    if let Some(days_str) = exp.strip_suffix('d') {
        if let Ok(days) = days_str.parse::<i64>() {
            let now = Utc::now();
            let time = now + Duration::days(days);
            return Ok(time.timestamp() as usize);
        }
    }
    Err("invalid duration format")
}
