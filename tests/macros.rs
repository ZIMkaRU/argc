#[macro_export]
macro_rules! snapshot {
    (
        $source:expr,
        $args:expr,
    ) => {
        let cli = argc::Cli::new($source);
        let (stdout, stderr) = match cli.run($args).unwrap() {
            Ok(stdout) => (stdout, String::new()),
            Err(stderr) => (String::new(), stderr),
        };

        let args = $args.join(" ");
        let output = format!(
            r###"RUN
{}

STDOUT
{}

STDERR
{}
"###,
            args, stdout, stderr
        );
        insta::assert_snapshot!(output);
    };
}

#[macro_export]
macro_rules! plain {
    (
        $source:expr,
        $args:expr,
        $(stdout: $stdout:expr,)?
        $(stderr: $stderr:expr,)?
    ) => {
        let cli = argc::Cli::new($source);
        let result = match cli.run($args).unwrap()  {
            Ok(stdout) => (stdout, String::new()),
            Err(stderr) => (String::new(), stderr),
        };
        $({
            assert_eq!(result.0.as_str(), $stdout);
        })?
        $({
            assert_eq!(result.1.as_str(), $stderr);
        })?
    };
}

#[macro_export]
macro_rules! fatal {
    (
        $source:expr,
        $args:expr,
        $err:expr
    ) => {
        let cli = argc::Cli::new($source);
        let err = cli.run($args).unwrap_err();
        assert_eq!(err.to_string().as_str(), $err);
    };
}

#[macro_export]
macro_rules! snapshot_compgen {
    (
        $source:expr,
        $args:expr
    ) => {
        let cli = argc::Cli::new($source);
        let (stdout, stderr) = match cli.compgen($args) {
            Ok(stdout) => (stdout.join(" "), String::new()),
            Err(stderr) => (String::new(), stderr.to_string()),
        };

        let args = $args.join(" ");
        let output = format!(
            r###"RUN
{}

STDOUT
{}

STDERR
{}
"###,
            args, stdout, stderr
        );
        insta::assert_snapshot!(output);
    };
}
