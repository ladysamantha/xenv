use std::{env, fs, path::Path};

use xenv::run_cmd;

fn main() -> eyre::Result<()> {
    let mut args = env::args().skip(1);
    let cmd = args.next().ok_or(eyre::eyre!("missing cmd"))?;
    let env_path = env::var("XENV_PATH").unwrap_or(".xenv".into());
    let env_path = Path::new(&env_path);
    let env_file = if env_path.exists() {
        fs::read_to_string(env_path)?
    } else {
        "".into()
    };

    run_cmd(cmd, env_file, args)
}
