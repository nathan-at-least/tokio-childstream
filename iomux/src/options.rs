use clap::Parser;
use tokio::process::Command;

#[derive(Debug)]
pub struct Options {
    pub subcommands: Vec<Command>,
}

/// Multiplex the stdout/err of multiple child processes
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct RawOptions {
    /// Sequences of subcommand args, separated by `--`
    #[clap(trailing_var_arg = true)]
    subcommands: Vec<String>,
}

impl Options {
    pub fn parse() -> anyhow::Result<Self> {
        use clap::CommandFactory;

        let ropts = RawOptions::parse();
        match parse_subcommands(ropts.subcommands.iter().map(String::as_ref)) {
            Ok(subcommands) => Ok(Options { subcommands }),
            Err(e) => {
                eprintln!("{}", RawOptions::command().render_help().ansi());
                Err(e)
            }
        }
    }
}

fn parse_subcommands<'a, I>(args: I) -> anyhow::Result<Vec<Command>>
where
    I: IntoIterator<Item = &'a str>,
{
    use anyhow::anyhow;

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
            if arg == "--" {
                return Err(anyhow!("encountered `--` when expecting command name"));
            }
            cmdslot = Some(Command::new(arg));
        }
    }
    if let Some(cmd) = cmdslot {
        commands.push(cmd);
        Ok(commands)
    } else {
        Err(if commands.is_empty() {
            anyhow!("no command given")
        } else {
            anyhow!("trailing `--` disallowed")
        })
    }
}

#[cfg(test)]
mod tests;
