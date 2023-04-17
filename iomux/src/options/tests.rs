use test_case::test_case;

fn ok(s: &str) -> Result<String, String> {
    Ok(s.to_string())
}

fn err(s: &str) -> Result<String, String> {
    Err(s.to_string())
}

#[test_case([] => err("no command given"))]
#[test_case(["foo"] => ok(r#""foo""#))]
#[test_case(["--"] => err("encountered `--` when expecting command name"))]
#[test_case(["foo", "--"] => err("trailing `--` disallowed"))]
#[test_case(["foo", "bar", "--", "quz", "wux"] => ok(r#""foo" "bar" -- "quz" "wux""#))]
#[test_case(["foo", "--", "--", "bar"] => err("encountered `--` when expecting command name"))]
fn parse_subcommands<const K: usize>(args: [&str; K]) -> Result<String, String> {
    super::parse_subcommands(args.into_iter())
        .map(|cmds| {
            cmds.into_iter()
                .map(|cmd| format!("{:?}", cmd.as_std()))
                .collect::<Vec<String>>()
                .join(" -- ")
        })
        .map_err(|e| format!("{:#}", e))
}
