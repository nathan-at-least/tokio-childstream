mod options;

use self::options::Options;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = Options::parse()?;
    todo!("{:#?}", opts.subcommands);
}
