use std::{
    collections::HashMap,
    fs::{self, File},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use indicatif::ProgressIterator;
use itertools::Itertools;
use polars::df;
use polars::{io::SerWriter, prelude::CsvWriter};
use regex::Regex;
struct Episode(i32, String, String);

pub fn parse_episode() -> Result<()> {
    let folder = "./input";

    let folder_path = Path::new(folder);
    if !folder_path.exists() && !folder_path.is_dir() {
        return Err(anyhow!("No input file"));
    }
    let mut episodes = Vec::<i32>::new();
    let mut character = Vec::<String>::new();
    let mut text = Vec::<String>::new();
    let mut contents = HashMap::<i32, PathBuf>::new();
    for file in fs::read_dir(folder)? {
        let file_path = file?.path();
        let title_regex = Regex::new(r#"([^-]+?)\."#)?;
        let title_text = title_regex.captures(file_path.to_str().unwrap()).unwrap();

        let epnum = title_text.iter().collect_vec()[1]
            .unwrap()
            .as_str()
            .parse::<i32>()?;

        contents.insert(epnum, file_path);
    }
    for (epnum, file_path) in contents.iter().sorted_by_key(|it| it.0).progress() {
        let file_text = fs::read_to_string(&file_path)?;

        let file_regex = Regex::new(r#"---[\s\S]*?---"#)?;
        let regex_ed_file_output = file_regex.replace_all(&file_text, "").to_string();
        let entries = parse_episode_to_df(epnum, regex_ed_file_output)?;
        for ep in entries.into_iter() {
            episodes.push(ep.0);
            character.push(ep.1);
            text.push(ep.2);
        }
    }
    let mut df = df!("episode"=>episodes,"character"=>character,"text"=>text)?;
    let mut file = File::create("./output.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();
    Ok(())
}

fn parse_episode_to_df(epnum: &i32, episode_text: String) -> Result<Vec<Episode>> {
    let mut hm = Vec::<Episode>::new();
    let lines = episode_text.split("\n").collect_vec();
    // let mut skip_next: i32 = 0;
    let mut current_character: String = String::new();
    let mut current_character_lines: String = String::new();
    for line in lines.into_iter() {
        // if skip_next != 0 {
        //     skip_next -= 1;
        //     continue;
        // }
        if line.contains("####") && !line.contains("[") {
            let font_change = Regex::new(r"#*?\s(.*?)$").unwrap();

            if let Some(cap) = font_change.captures(line) {
                let capture = cap.get(1).unwrap().as_str();
                // skip_next = 1;
                hm.push(Episode(
                    epnum.to_owned(),
                    current_character.clone(),
                    current_character_lines.clone(),
                ));
                current_character = capture.to_string();
                current_character_lines = String::new();
            }
        } else if !line.is_empty() && !current_character.is_empty() && !line.contains("#") {
            if !current_character_lines.is_empty() {
                current_character_lines.push_str("\n");
            }
            current_character_lines.push_str(line);
        }
    }
    if !current_character.is_empty() && !current_character_lines.is_empty() {
        hm.push(Episode(
            epnum.to_owned(),
            current_character.to_string(),
            current_character_lines.clone(),
        ));
    }
    Ok(hm)
}
