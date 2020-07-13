use std::{env, process::Command};

pub fn run_cmd<I>(cmd: String, env_file: String, args: I) -> eyre::Result<()>
where
    I: Iterator<Item = <std::env::Args as Iterator>::Item>,
{
    #[cfg(unix)]
    use std::os::unix::process::CommandExt;

    use eyre::Report;

    set_env_from_file(env_file);

    #[cfg(windows)]
    let result = Command::new(cmd)
        .args(args)
        .spawn()
        .map(|_| ())
        .wrap_err("failed to spawn process");

    // returning Err here since `exec` won't return on success
    #[cfg(unix)]
    let result = Err(Report::new(Command::new(cmd).args(args).exec()));

    result
}

fn set_env_from_file(env_file: String) {
    for line in env_file.lines() {
        let split = line.split('=');
        let split: Vec<&str> = split.collect();
        if split.len() != 2 {
            break;
        }
        env::set_var(split[0], split[1]);
    }
}
