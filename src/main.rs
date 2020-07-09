use std::{
    collections::HashMap,
    env,
    fs,
    path::Path,
    process::Command,
};

use eyre::WrapErr;

fn main() -> eyre::Result<()> {
    let mut args = env::args().skip(1);
    let cmd = args.next().ok_or_else(|| eyre::eyre!("missing first argument"))?;
    let env_path = env::var("XENV_PATH")
        .unwrap_or_else(|_| ".xenv".into());
    let env_path = Path::new(&env_path);
    let env_file = if env_path.exists() {
        fs::read_to_string(env_path)?
    } else {
        "".into()
    };
    let envs: HashMap<&str, &str> =
            env_file
            .lines()
            .map(|line| {
                let mut it = line.split('=');
                (it.next().expect("missing first part"), it.next().expect("missing second part"))   
            })
            .filter(|(a, _)| a.len() > 0)
            .collect();
    Command::new(cmd)
        .envs(envs)
        .args(args)
        .spawn()
        .map(|_| ())
        .wrap_err_with(|| eyre::eyre!("failed to spawn process"))
}