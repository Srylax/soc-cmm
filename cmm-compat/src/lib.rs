use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::Ok;
use calamine::{Data, DataType, Reader, ToCellDeserializer, Xlsx, open_workbook};
use cmm_core::{
    answer::{Answer, Detailed, DetailedOptional, Occurence, Satisfaction},
    cid::CID,
    control::Control,
    data::SOCData,
};

use roxmltree::Document;

pub fn from_xlsx<P: AsRef<Path>>(path: P) -> anyhow::Result<SOCData> {
    let mut workbook: Xlsx<_> = open_workbook(&path)?;
    let mut controls = comments(&mut workbook)?;

    let output = workbook.worksheet_range("_Output")?;

    extend_answer_from_output(&output, &mut controls);
    extend_answer_from_form_controls(&mut controls, &output, path)?;
    nist_compat(&mut controls);

    // let controls = controls
    //     .into_iter()
    //     .filter(|(_cid, control)| !control.is_default())
    //     .collect();

    Ok(SOCData::from_map(controls))
}

fn nist_compat(controls: &mut HashMap<CID, Control>) {
    let compat = Vec::from([
        (
            "Services.4.15.30",
            Answer::DetailedOptional(DetailedOptional::No),
        ),
        (
            "Services.4.15.31",
            Answer::DetailedOptional(DetailedOptional::No),
        ),
        (
            "Services.2.17.33",
            Answer::DetailedOptional(DetailedOptional::No),
        ),
        (
            "Services.2.17.34",
            Answer::DetailedOptional(DetailedOptional::No),
        ),
        (
            "Services.2.17.35",
            Answer::DetailedOptional(DetailedOptional::No),
        ),
        (
            "Services.2.17.36",
            Answer::DetailedOptional(DetailedOptional::No),
        ),
        (
            "Services.6.15.20",
            Answer::DetailedOptional(DetailedOptional::No),
        ),
        ("Process.2.2.5", Answer::Detailed(Detailed::No)),
        ("Process.2.4.5", Answer::Detailed(Detailed::No)),
        ("Process.2.4.9", Answer::Detailed(Detailed::No)),
        ("Process.3.11.1", Answer::Detailed(Detailed::No)),
        ("Process.3.11.2", Answer::Detailed(Detailed::No)),
        ("Process.3.11.3", Answer::Detailed(Detailed::No)),
        ("Business.4.6", Answer::Detailed(Detailed::No)),
        ("People.2.2.14", Answer::Any(String::new())),
    ]);

    for (cid, answer) in compat {
        let control = controls
            .get_mut(&cid.parse().unwrap())
            .expect("Compat CID not in controls");
        control.set_answer(answer);
        control.set_nist_only(cid != "People.2.2.14");
    }
}

// Create all Controls from the sheets including comments - without answer type
fn comments(workbook: &mut Xlsx<BufReader<File>>) -> anyhow::Result<HashMap<CID, Control>> {
    let sheets = vec![
        ("Business - BSD", "Business"),
        ("Business - CST", "Business"),
        ("Business - CHT", "Business"),
        ("Business - GOV", "Business"),
        ("Business - PRV", "Business"),
        ("People - EMP", "People"),
        ("People - R&H", "People"),
        ("People - PEM", "People"),
        ("People - KNM", "People"),
        ("People - T&E", "People"),
        ("Process - MGT", "Process"),
        ("Process - O&F", "Process"),
        ("Process - RPT", "Process"),
        ("Process - UCM", "Process"),
        ("Process - DTE", "Process"),
        ("Technology - SIM", "Technology"),
        ("Technology - NDR", "Technology"),
        ("Technology - EDR", "Technology"),
        ("Technology - A&O", "Technology"),
        ("Services - SCM", "Services"),
        ("Services - SIM", "Services"),
        ("Services - A&F", "Services"),
        ("Services - THR", "Services"),
        ("Services - HNT", "Services"),
        ("Services - VUL", "Services"),
        ("Services - LOG", "Services"),
    ];

    let mut cids = HashMap::new();

    for (sheet, domain) in sheets {
        let range = workbook.worksheet_range(sheet)?;

        let mut comments: HashMap<CID, String> = range
            .rows()
            .skip_while(|row| row[1] != "Comments and/or Remarks")
            .skip(1)
            .filter(|row| row[11].is_string() && row[13].is_string())
            .map(|row| {
                (
                    format!("{}.{}", domain, row[11]).parse().unwrap(),
                    row[13].to_string(),
                )
            })
            .collect();

        let controls = range
            .rows()
            .skip(9)
            .take_while(|row| row[1] != "Comments and/or Remarks")
            .filter(|row| {
                row[1]
                    .to_string()
                    .chars()
                    .next()
                    .map(|char| char.is_ascii_digit())
                    .unwrap_or(false)
            })
            .map(|row| {
                let cid: CID = format!(
                    "{}.{}",
                    domain,
                    row[1].to_string().split_whitespace().next().unwrap()
                )
                .parse()
                .unwrap();

                (cid, Control::new(Answer::Title, comments.remove(&cid)))
            });
        cids.extend(controls);
    }
    Ok(cids)
}

fn extend_answer_from_output(
    output_ragne: &calamine::Range<Data>,
    controls: &mut HashMap<CID, Control>,
) {
    let cids = output_ragne.rows().filter(|row| {
        let Some(id) = row[0].as_string() else {
            return false;
        };
        let mut chars = id.chars().skip(1);
        chars.next().unwrap().is_whitespace()
            && chars.next().unwrap().is_ascii_digit()
            && row[13].as_string() != Some("NIST MAPPING".to_owned())
    });

    for row in cids {
        let Some(control) = controls
            .get_mut(&to_cid(&row[0].to_string()).expect(&format!("CID not found {}", row[0])))
        else {
            eprintln!("Output contains unlisted CID: {}", row[0].to_string());
            continue;
        };
        let answer = if ToCellDeserializer::is_empty(&row[3]) {
            Answer::Title
        } else {
            Answer::Any(row[3].as_string().unwrap())
        };
        control.set_answer(answer);
    }
}

fn extend_answer_from_form_controls<P: AsRef<Path>>(
    controls: &mut HashMap<CID, Control>,
    output_range: &calamine::Range<Data>,
    path: P,
) -> anyhow::Result<()> {
    let mut zip = zip::ZipArchive::new(File::open(path)?)?;

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

        let id = output_range.get_value((output_row, 0)).unwrap();

        let Some(value) = output_range.get_value((output_row, 3)).unwrap().as_i64() else {
            eprintln!(
                "Control maps to outdated control (this is probably wanted): {} row: {}",
                id, output_row
            );
            continue;
        };

        let Some(entry) = controls.get_mut(&to_cid(&id.to_string()).unwrap()) else {
            eprintln!("Output contains unlisted CID: {}", id);
            continue;
        };

        let _type = output_range.get_value((output_row, 2)).unwrap().to_string();

        if matches!(entry.answer(), Answer::Any(_)) {
            entry.set_answer(input_map(input_link, value as usize, &_type));
        } else {
            eprintln!(
                "Skipped {} with value of {}, existing: {:?}",
                id, value, entry
            )
        }
    }
    Ok(())
}

fn input_map(input: &str, value: usize, _type: &str) -> Answer {
    match input {
        "_Input!$C$13:$C$18" => {
            assert_eq!(_type, "C");
            Answer::DetailedOptional(DetailedOptional::from_repr(value).unwrap_or_default())
        }
        "_Input!$C$13:$C$17" => {
            assert_eq!(_type, "M");
            Answer::Detailed(Detailed::from_repr(value).unwrap_or_default())
        }
        "_Input!$C$3:$C$4" => Answer::Bool(value > 1),
        "_Input!$C$39:$C$43" => {
            assert_eq!(_type, "M");
            Answer::Occurence(Occurence::from_repr(value).unwrap_or_default())
        }
        "_Input!$C$45:$C$49" => {
            assert_eq!(_type, "M");
            Answer::Satisfaction(Satisfaction::from_repr(value).unwrap_or_default())
        }
        _ => unreachable!(),
    }
}

fn to_cid(str: &str) -> anyhow::Result<CID> {
    let id = str
        .replace("P ", "People.")
        .replace("M ", "Process.")
        .replace("B ", "Business.")
        .replace("T ", "Technology.")
        .replace("S ", "Services.")
        .replace(" - Scope", "")
        .replace(" - Security incident Management", "")
        .replace(" - Security Analysis", "");
    Ok(id.parse()?)
}
