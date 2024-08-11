use std::{collections::HashMap, fs, path::{Path, PathBuf}};

use itertools::Itertools;
use regex::Regex;
use anyhow::{Result,anyhow};
pub fn process_episode(pre_processed: String) -> String {
    let lines = pre_processed.split("\n").collect_vec();
    let mut hm = Vec::<String>::new();
    let mut skip_next: i32 = 0;
    for line in lines.into_iter() {
        if skip_next != 0 {
            skip_next -= 1;
            continue;
        }
        if line.contains("####") && !line.contains("[") {
            let font_change = Regex::new(r"#*?\s(.*?)$").unwrap();

            if let Some(cap) = font_change.captures(line) {
                let capture = cap.get(1).unwrap().as_str();
                let new_line = font_change.replace_all(line, format!("**{}**: ", capture));
                skip_next = 1;
                hm.push(new_line.to_string());
            } else {
                hm.push(line.to_string());
            };
        } else {
            hm.push(line.to_string())
        }
    }
    let output = hm.join("\n");
    output
}


pub fn parse_episode_to_md() -> Result<()> {
    let folder = "./input";
    let output_file = "./output.md";

    let folder_path = Path::new(folder);
    if !folder_path.exists() && !folder_path.is_dir() {
        return Err(anyhow!("No input file"));
    }
    let mut text = String::new();
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
    let mut first = true;
    for (epnum, file_path) in contents.iter().sorted_by_key(|it| it.0) {
        let file_text = fs::read_to_string(&file_path)?;

        let file_regex = Regex::new(r#"---[\s\S]*?---"#)?;
        let regex_ed_file_output = file_regex.replace_all(&file_text, "").to_string();
        let post_processed_string: String = process_episode(regex_ed_file_output);
        let placeholder = if !first {
            format!(
                "\n\n<h1 style=\"page-break-before:always;\"> Episode {}</h1>",
                epnum
            )
        } else {
            first = false;
            format!("\n\n# Episode {}", epnum)
        };
        text.push_str(&placeholder);
        text.push_str(&post_processed_string);
    }
    fs::write(output_file, text.into_bytes())?;
    Ok(())
}
