use std::{
    collections::HashSet,
    fs::{self, File},
    io::Read,
};

use anyhow::Ok;
use calamine::{Data, Reader, Xlsx, open_workbook};
use roxmltree::Document;

fn main() -> anyhow::Result<()> {
    let mut workbook: Xlsx<_> =
        open_workbook("/home/bsiag.local/kli/Downloads/soc-cmm 2.3.4-basic_BSI.xlsx")?;

    let output = workbook.worksheet_range("_Output")?;

    let mut zip = zip::ZipArchive::new(File::open(
        "/home/bsiag.local/kli/Downloads/soc-cmm 2.3.4-basic_BSI.xlsx",
    )?)?;

    let props = zip
        .file_names()
        .filter(|file| file.starts_with("xl/ctrlProps/"))
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    for ctrl_prop in props {
        let mut ctrl_prop = zip.by_name(&ctrl_prop)?;
        let mut string = String::new();
        string.try_reserve_exact(ctrl_prop.size() as usize)?;
        ctrl_prop.read_to_string(&mut string)?;
        let xml = Document::parse(string.as_str())?;

        let output_link = xml
            .root()
            .first_child()
            .unwrap()
            .attribute("fmlaLink")
            .unwrap();

        let input_link = xml
            .root()
            .first_child()
            .unwrap()
            .attribute("fmlaRange")
            .unwrap();

        let output_row = output_link
            .strip_prefix("_Output!$D$")
            .unwrap()
            .parse::<u32>()?
            - 1;

        let id = output.get_value((output_row, 0)).unwrap();

        let value = output.get_value((output_row, 3)).unwrap();

        println!("{id} -> {} ({value})", input_map(input_link));
    }

    Ok(())
}

fn input_map(input: &str) -> &'static str {
    match input {
        "_Input!$C$13:$C$18" => "Detailed Optional",
        "_Input!$C$13:$C$17" => "Detailed",
        "_Input!$C$3:$C$4" => "Yes/No",
        "_Input!$C$39:$C$43" => "Occurrence",
        "_Input!$C$45:$C$49" => "Satisfaction",
        _ => unreachable!(),
    }
}
