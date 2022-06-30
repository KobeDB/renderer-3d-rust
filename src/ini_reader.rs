use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::ParseError;
use crate::ini_reader::IniValue::Number;

enum IniValue {
    String(String),
    Number(f32),
    Tuple([f32; 3])
}

impl IniValue {
    pub fn from(s: &str) -> Result<IniValue, ()> {
        let result = s.parse::<f32>();
        match result {
            Ok(val) => return Ok(Number(val)),
            Err(_) => {},
        }

        Err(())
    }
}

struct Section {
    values: HashMap<String, IniValue>,
}

pub struct IniConfiguration {
    sections: HashMap<String, Section>,
}

impl IniConfiguration {

    pub fn new(path_to_ini: &str) -> Self {
        let mut sections = HashMap::new();

        let text = fs::read_to_string(path_to_ini)
            .expect(&format!("Failed to open ini file: {}", path_to_ini));

        let mut section_name = String::new();

        for line in text.lines() {
            let line = line.trim();

            if line.is_empty() { continue; }

            if line.starts_with("[") {
                section_name = line.strip_prefix("[").unwrap().strip_suffix("]").unwrap().to_string();
                sections.insert(section_name.clone(), Section{values: HashMap::new()});
            }

            if section_name.is_empty() {
                eprintln!("ini file: {path_to_ini} didn't start with a section!");
            }

            let section = sections.get_mut(&section_name)
                .expect(&format!("no section with name: {section_name}"));

            let split = line.split(":");

            let mut key = None;
            let mut value = None;
            for (i, part) in split.enumerate() {
                if i > 1 { eprintln!("weird split"); }
                if i == 0 { key = Some(part.to_string()); }
                if i == 1 { value = Some(IniValue::from(part).expect("couldn't parse ini value")); }
            }
            section.values.insert(key.expect("no key found"), value.expect("no value found"));
        }

        Self{sections}
    }

}