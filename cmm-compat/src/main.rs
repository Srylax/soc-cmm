use std::env::args;

use cmm_compat::from_xlsx;

fn main() -> anyhow::Result<()> {
    let soc_cmm = args().nth(1).unwrap_or("../soc-cmm-2.3.4.xlsx".to_owned());

    let cmm = from_xlsx(soc_cmm)?;

    // let aspect = cmm.aspects(&Domain::Technology).unwrap().get(1).unwrap();
    // let mut keys = aspect
    //     .controls()
    //     .iter()
    //     .map(|(cid, control)| {
    //         format!(
    //             "{} {} {:?}",
    //             cid,
    //             control.answer().capability_in_scope(),
    //             control.answer()
    //         )
    //     })
    //     .collect::<Vec<_>>();
    // keys.sort();

    // println!("{:#?}", keys);
    // println!("M {:#?}", aspect.maturity_factor());
    // println!("M {:#?}", aspect.maturity_total_score());
    // println!("M {:#?}", aspect.maturity_max_score());
    // println!("M {:#?}", aspect.maturity_final_score());

    // println!("C {:#?}", aspect.capability_factor());
    // println!("C {:#?}", aspect.capability_total_score());
    // println!("C {:#?}", aspect.capability_max_score());
    // println!("C {:#?}", aspect.capability_final_score());

    // println!("{}", toml::to_string(&cmm.to_simple()).unwrap());
    Ok(())
}
