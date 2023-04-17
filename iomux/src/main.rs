use clap::Parser;
use tokio::process::Command;

/// Multiplex the stdout/err of multiple child processes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Options {
    /// Sequences of subcommand args, separated by `--`
    #[clap(trailing_var_arg = true)]
    subcommands: Vec<String>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts = Options::parse();
    let _commands = parse_subcommands(opts.subcommands.iter().map(String::as_ref))?;

    Ok(())
}

fn parse_subcommands<'a, I>(args: I) -> anyhow::Result<Vec<Command>>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut commands: Vec<Command> = vec![];
    let mut cmdslot = None;

    for arg in args {
        if let Some(mut cmd) = cmdslot.take() {
            if arg == "--" {
                commands.push(cmd);
            } else {
                cmd.arg(arg);
                cmdslot = Some(cmd);
            }
        } else {
            cmdslot = Some(Command::new(arg));
        }
    }
    let cmd = cmdslot.ok_or_else(|| anyhow::anyhow!("trailing `--` not allowed"))?;
    commands.push(cmd);
    Ok(commands)
}

#[cfg(test)]
mod tests;
