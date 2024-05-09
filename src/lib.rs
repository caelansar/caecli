mod cmd;
mod process;

pub use cmd::{
    Base64DecodeOpts, Base64EncodeOpts, Base64SubCommand, CsvOpts, GenPassOpts, HttpServeOpts,
    HttpSubCommand, KeyGenerateOpts, Opts, SubCommand, TextSignOpts, TextSubCommand,
    TextVerifyOpts, TimeOpts,
};
use enum_dispatch::enum_dispatch;
pub use process::*;

#[enum_dispatch]
#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
