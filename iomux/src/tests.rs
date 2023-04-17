use test_case::test_case;

#[test_case(["foo"] => Ok(r#""foo""#.to_string()))]
fn parse_subcommands<const K: usize>(args: [&str; K]) -> Result<String, String> {
    crate::parse_subcommands(args.into_iter())
        .map(|cmds| {
            cmds.into_iter()
                .map(|cmd| format!("{:?}", cmd.as_std()))
                .collect::<Vec<String>>()
                .join(" -- ")
        })
        .map_err(|e| format!("{:#}", e))
}
