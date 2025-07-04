use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use anyhow::Ok;
use calamine::{Data, DataType, Reader, ToCellDeserializer, Xlsx, open_workbook};
use cmm_core::{
    CID, CMM,
    answer::{Answer, Detailed, DetailedOptional, Occurence, Satisfaction},
    control::Control,
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
    nist_compat(&mut controls);
    // Service.2.17.35 has no guidance
    extend_generic_guidances(&mut controls);

    Ok(CMM::from_map(controls, aspects(&output)).unwrap())
}

fn nist_compat(controls: &mut HashMap<CID, Control>) {
    let compat = Vec::from([
        ("S 4.15.30", Answer::DetailedOptional(DetailedOptional::No)),
        ("S 4.15.31", Answer::DetailedOptional(DetailedOptional::No)),
        ("S 2.17.33", Answer::DetailedOptional(DetailedOptional::No)),
        ("S 2.17.34", Answer::DetailedOptional(DetailedOptional::No)),
        ("S 2.17.35", Answer::DetailedOptional(DetailedOptional::No)),
        ("S 2.17.36", Answer::DetailedOptional(DetailedOptional::No)),
        ("S 6.15.20", Answer::DetailedOptional(DetailedOptional::No)),
        ("M 2.2.5", Answer::Detailed(Detailed::No)),
        ("M 2.4.5", Answer::Detailed(Detailed::No)),
        ("M 2.4.9", Answer::Detailed(Detailed::No)),
        ("M 3.11.1", Answer::Detailed(Detailed::No)),
        ("M 3.11.2", Answer::Detailed(Detailed::No)),
        ("M 3.11.3", Answer::Detailed(Detailed::No)),
        ("B 4.6", Answer::Detailed(Detailed::No)),
        ("P 2.2.14", Answer::Any(String::new())),
    ]);

    for (cid, answer) in compat {
        let control = controls.get_mut(cid).expect("Compat CID not in controls");
        control.set_answer(answer);
        control.set_nist_only(cid != "P 2.2.14");
    }
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

        let mut comments: HashMap<CID, String> = range
            .rows()
            .skip_while(|row| row[1].to_string() != "Comments and/or Remarks")
            .skip(1)
            .filter(|row| row[11].is_string() && row[13].is_string())
            .map(|row| {
                (
                    format!("{} {}", domain, row[11].to_string()),
                    row[13].to_string(),
                )
            })
            .collect();

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
                let cid = format!(
                    "{} {}",
                    domain,
                    row[1].to_string().split_whitespace().next().unwrap()
                );

                (
                    cid.clone(),
                    Control::new(
                        row[2].to_string(),
                        row[15].as_string(),
                        Answer::Title,
                        comments.remove(&cid),
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

fn aspects(output_range: &calamine::Range<Data>) -> HashMap<String, String> {
    output_range
        .rows()
        .filter(|row| {
            row[0].to_string().contains("-")
                && row[0]
                    .to_string()
                    .chars()
                    .filter(|char| char.is_ascii_digit())
                    .count()
                    == 1
                && ToCellDeserializer::is_empty(&row[3])
        })
        .flat_map(|row| row[0].as_string())
        .map(|aspect| {
            let (aid, title) = aspect.split_once("-").unwrap();
            (aid.replace(" ", ""), title.trim().to_owned())
        })
        .collect()
}

fn extend_generic_guidances(controls: &mut HashMap<CID, Control>) {
    let monitoring_capabilities = HashSet::from([
        "S 1.16.12",
        "S 1.16.13",
        "S 1.16.14",
        "S 1.16.15",
        "S 1.16.16",
        "S 1.16.17",
        "S 1.16.18",
        "S 1.16.19",
        "S 1.16.20",
        "S 1.16.21",
        "S 1.16.22",
        "S 1.16.23",
        "S 1.16.24",
        "S 1.16.25",
        "S 1.16.26",
        "S 1.16.27",
    ]);
    for (cid, control) in controls.iter_mut() {
        if !control.answer().is_capability() || !control.guidances().is_empty() {
            continue;
        }
        if monitoring_capabilities.contains(cid.as_str()) {
            control.set_guidances(vec![
                "Not in place".to_owned(),
                "Log sources connected, basic monitoring".to_owned(),
                "Specific use cases defined and operationalised".to_owned(),
                "Use cases, playbooks and procedures defined and implemented".to_owned(),
                "Fully implemented, performance measured and improved".to_owned(),
                "Not required for SOC operations".to_owned(),
            ]);
        } else {
            control.set_guidances(vec![
                "Not in place".to_owned(),
                "Partially implemented, incomplete".to_owned(),
                "Averagely implemented, partially documented".to_owned(),
                "Mostly implemented, documented and approved".to_owned(),
                "Fully implemented, documented, approved, actively improved".to_owned(),
                "Not required for SOC operations".to_owned(),
            ]);
        }
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

        let cid = guidance_cid_map(cid, row);

        let Some(control) = controls.get_mut(&cid) else {
            eprintln!("Skipping unknown cid {} - probably an old relic", cid);
            continue;
        };
        control.set_guidances(guides);
    }
}
fn guidance_cid_map(cid: CID, row: u32) -> CID {
    HashMap::from([
        (("M 4.1", 829), "M 4.1.1"),
        (("M 4.2", 835), "M 4.1.2"),
        (("M 4.3", 841), "M 4.1.3"),
        (("M 4.4", 847), "M 4.1.4"),
        (("M 4.5", 853), "M 4.1.5"),
        (("M 4.6", 859), "M 4.1.6"),
        (("M 4.7", 865), "M 4.1.7"),
        (("M 4.8", 871), "M 4.1.8"),
        (("M 4.9", 877), "M 4.1.9"),
        (("M 4.10", 883), "M 4.1.10"),
        (("M 4.11", 889), "M 4.1.11"),
        (("S 3.12", 1844), "S 3.13"),
        (("S 3.13", 1850), "S 3.14"),
        (("S 3.14", 1856), "S 3.15"),
        (("T 5.4.1", 1512), "T 4.4.1"),
        (("T 5.4.2", 1518), "T 4.4.2"),
        (("T 5.4.3", 1524), "T 4.4.3"),
        (("T 5.4.4", 1530), "T 4.4.4"),
        (("T 5.4.5", 1536), "T 4.4.5"),
    ])
    .get(&(cid.as_str(), row))
    .map(|cid| cid.to_string())
    .unwrap_or(cid)
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

        let Some(entry) = controls.get_mut(&id.to_string()) else {
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
