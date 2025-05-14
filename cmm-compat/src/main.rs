use std::{collections::HashMap, env::args, fs::File, io::Read};

use anyhow::Ok;
use calamine::{Data, DataType, Reader, ToCellDeserializer, Xlsx, open_workbook};
use cmm_core::{
    CID, Control,
    answer::{Answer, Detailed, DetailedOptional, Occurence, Satisfaction},
};
use roxmltree::Document;

fn main() -> anyhow::Result<()> {
    let soc_cmm = args().nth(1).expect("File Path requried");
    let mut workbook: Xlsx<_> = open_workbook(&soc_cmm)?;

    let output = workbook.worksheet_range("_Output")?;
    let guidance = workbook.worksheet_range("_Guidance")?;

    let mut output_map = list_output(&output);
    map_form_controls(&mut output_map, &output, soc_cmm)?;
    let controls = map_answers(output_map, &guidance);

    Ok(())
}

fn input_map(input: &str, value: usize) -> Answer {
    match input {
        "_Input!$C$13:$C$18" => {
            Answer::DetailedOptional(DetailedOptional::from_repr(value).unwrap_or_default())
        }
        "_Input!$C$13:$C$17" => Answer::Detailed(Detailed::from_repr(value).unwrap_or_default()),
        "_Input!$C$3:$C$4" => Answer::Bool(value > 1),
        "_Input!$C$39:$C$43" => Answer::Occurence(Occurence::from_repr(value).unwrap_or_default()),
        "_Input!$C$45:$C$49" => {
            Answer::Satisfaction(Satisfaction::from_repr(value).unwrap_or_default())
        }
        _ => unreachable!(),
    }
}

fn map_answers(
    mut answers: HashMap<CID, Answer>,
    guidance_range: &calamine::Range<Data>,
) -> HashMap<CID, Control> {
    let mut controls = HashMap::new();
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
        let Some(answer) = answers.remove(&cid) else {
            println!("Skipping unknown cid {}", cid);
            continue;
        };
        let control = Control::new(answer, guides);
        assert!(controls.insert(cid, control).is_none());
    }
    controls
}

fn map_form_controls(
    controls: &mut HashMap<CID, Answer>,
    output_ragne: &calamine::Range<Data>,
    path: String,
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

        let id = output_ragne.get_value((output_row, 0)).unwrap();

        let value = output_ragne
            .get_value((output_row, 3))
            .unwrap()
            .as_i64()
            .unwrap();

        let entry = controls.get_mut(&id.to_string()).unwrap();

        if matches!(entry, Answer::Any(_)) {
            *entry = input_map(input_link, value as usize);
        } else {
            println!(
                "Skipped {} with value of {}, existing: {:?}",
                id, value, entry
            )
        }
    }
    Ok(())
}

fn list_output(output_ragne: &calamine::Range<Data>) -> HashMap<CID, Answer> {
    output_ragne
        .rows()
        .filter(|row| {
            let Some(id) = row[0].as_string() else {
                return false;
            };
            let mut chars = id.chars().skip(1);
            chars.next().unwrap().is_whitespace()
                && chars.next().unwrap().is_ascii_digit()
                && row[13].as_string() != Some("NIST MAPPING".to_owned())
        })
        .map(|row| {
            (
                row[0].as_string().unwrap(),
                if ToCellDeserializer::is_empty(&row[3]) {
                    Answer::None
                } else {
                    Answer::Any(row[3].as_string().unwrap())
                },
            )
        })
        .collect()
}
