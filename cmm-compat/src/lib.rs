use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::Ok;
use calamine::{Data, DataType, Reader, ToCellDeserializer, Xlsx, open_workbook};
use cmm_core::{
    CID, CMM, Control,
    answer::{Answer, Detailed, DetailedOptional, Occurence, Satisfaction},
};
use roxmltree::Document;

pub fn from_xlsx<P: AsRef<Path>>(path: P) -> anyhow::Result<cmm_core::CMM> {
    let mut workbook: Xlsx<_> = open_workbook(&path)?;
    let mut controls = question_remarks(&mut workbook)?;

    let output = workbook.worksheet_range("_Output")?;
    let guidance = workbook.worksheet_range("_Guidance")?;

    extend_answer_from_output(&output, &mut controls);
    extend_answer_from_form_controls(&mut controls, &output, path)?;
    extend_control_from_guidance(&mut controls, &guidance);

    Ok(CMM::from_map(controls).unwrap())
}

fn question_remarks(workbook: &mut Xlsx<BufReader<File>>) -> anyhow::Result<HashMap<CID, Control>> {
    let sheets = vec![
        ("Business - BSD", "B"),
        ("Business - CST", "B"),
        ("Business - CHT", "B"),
        ("Business - GOV", "B"),
        ("Business - PRV", "B"),
        ("People - EMP", "P"),
        ("People - R&H", "P"),
        ("People - PEM", "P"),
        ("People - KNM", "P"),
        ("People - T&E", "P"),
        ("Process - MGT", "M"),
        ("Process - O&F", "M"),
        ("Process - RPT", "M"),
        ("Process - UCM", "M"),
        ("Process - DTE", "M"),
        ("Technology - SIM", "T"),
        ("Technology - NDR", "T"),
        ("Technology - EDR", "T"),
        ("Technology - A&O", "T"),
        ("Services - SCM", "S"),
        ("Services - SIM", "S"),
        ("Services - A&F", "S"),
        ("Services - THR", "S"),
        ("Services - HNT", "S"),
        ("Services - VUL", "S"),
        ("Services - LOG", "S"),
    ];

    let mut cids = HashMap::new();

    for (sheet, domain) in sheets {
        let range = workbook.worksheet_range(sheet)?;
        let iter = range
            .rows()
            .skip(9)
            .take_while(|row| row[1].to_string() != "Comments and/or Remarks")
            .filter(|row| {
                row[1]
                    .to_string()
                    .chars()
                    .next()
                    .map(|char| char.is_ascii_digit())
                    .unwrap_or(false)
            })
            .map(|row| {
                (
                    format!("{} {}", domain, row[1]),
                    Control::new(
                        row[2].to_string(),
                        row[15].as_string(),
                        Answer::None,
                        Vec::new(),
                    ),
                )
            });
        cids.extend(iter);
    }
    Ok(cids)
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

fn extend_control_from_guidance(
    controls: &mut HashMap<CID, Control>,
    guidance_range: &calamine::Range<Data>,
) {
    for row in 0..=guidance_range.end().unwrap().0 {
        let is_starting_guidance = guidance_range
            .get_value((row, 1))
            .and_then(|data| data.as_i64());
        if is_starting_guidance.is_none() || is_starting_guidance.unwrap() != 0 {
            continue;
        }
        let mut guides = Vec::new();
        for guide in 1..=5 {
            let Some(guidance_index) = guidance_range.get_value((row + guide, 1)).unwrap().as_i64()
            else {
                continue;
            };
            if guidance_index as u32 != guide {
                continue;
            }
            let guidance = guidance_range
                .get_value((row + guide, 2))
                .unwrap()
                .to_string();
            guides.push(guidance);
        }
        let cid = guidance_range.get_value((row, 0)).unwrap().to_string();
        // let control = Control::new(answer, guides);
        let Some(control) = controls.get_mut(&cid) else {
            println!("Skipping unknown cid {}", cid);
            continue;
        };
        control.set_guidances(guides);
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
            println!(
                "Control maps to outdated control (this is probably wanted): {} row: {}",
                id, output_row
            );
            continue;
        };

        let Some(entry) = controls.get_mut(&id.to_string()) else {
            println!("Output contains unlisted CID: {}", id);
            continue;
        };

        let _type = output_range.get_value((output_row, 2)).unwrap().to_string();

        if matches!(entry.answer(), Answer::Any(_)) {
            entry.set_answer(input_map(input_link, value as usize, &_type));
        } else {
            println!(
                "Skipped {} with value of {}, existing: {:?}",
                id, value, entry
            )
        }
    }
    Ok(())
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
        let Some(control) = controls.get_mut(&row[0].to_string()) else {
            println!("Output contains unlisted CID: {}", row[0].to_string());
            continue;
        };
        let answer = if ToCellDeserializer::is_empty(&row[3]) {
            Answer::None
        } else {
            Answer::Any(row[3].as_string().unwrap())
        };
        control.set_answer(answer);
    }
}
