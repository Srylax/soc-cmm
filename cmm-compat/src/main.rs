use std::env::args;

use cmm_compat::from_xlsx;

fn main() -> anyhow::Result<()> {
    let soc_cmm = args().nth(1).expect("You need to provide a path");

    let soc_data = from_xlsx(soc_cmm)?;

    println!("{}", toml::to_string_pretty(&soc_data).unwrap());

    Ok(())
}
