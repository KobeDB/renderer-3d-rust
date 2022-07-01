use std::collections::HashMap;
use std::fmt::Error;
use std::fs::File;
use std::fs;
use std::io::{self, BufRead};
use std::path::Path;
use std::string::ParseError;
use crate::ini_reader::IniValue::Number;

#[derive(Debug)]
enum IniValue {
    String(String),
    Number(f32),
    Tuple([f32; 3])
}

impl IniValue {
    pub fn from(s: &str) -> Result<IniValue, ()> {

        let s = s.trim();

        if s.starts_with("\"") {
            let result = s.strip_prefix("\"")
                .unwrap()
                .strip_suffix("\"")
                .expect("string literal not properly terminated with a quote")
                .to_string();
            return Ok(IniValue::String(result));
        }

        if s.starts_with("(") {
            let result: Vec<&str> = s.strip_prefix("(")
                .unwrap()
                .strip_suffix(")")
                .expect("tuple literal not properly terminated with a brace")
                .split(",")
                .collect();
            if result.len() != 3 {
                eprintln!("tuple literals must have exactly 3 values");
            }
            return Ok(IniValue::Tuple([
                result[0].trim().parse::<f32>().expect("tuple element must be a number"),
                result[1].trim().parse::<f32>().expect("tuple element must be a number"),
                result[2].trim().parse::<f32>().expect("tuple element must be a number")
            ]))
        }

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

impl Section {
    pub fn as_u32_or_die(&self, key: &str) -> u32 {
        match self.values.get(key).unwrap() {
            Number(val) => { if val.round() == *val { return *val as u32 } else {panic!("")} }
            IniValue::String(_) => { panic!(""); }
            IniValue::Tuple(_) => { panic!(""); }
        }
    }

    pub fn as_f32_or_die(&self, key: &str) -> f32 {
        match self.values.get(key).unwrap() {
            Number(val) => { *val }
            IniValue::String(_) => { panic!(""); }
            IniValue::Tuple(_) => { panic!(""); }
        }
    }

    pub fn as_f32_or_default(&self, key: &str, default: f32) -> f32 {
        match self.values.get(key) {
            Some(result) => {
                match result {
                    Number(val) => { *val }
                    IniValue::String(_) => { default }
                    IniValue::Tuple(_) => { default }
                }
            }
            Err(_) => default
        }
    }
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
                continue;
            }

            if section_name.is_empty() {
                eprintln!("ini file: {path_to_ini} didn't start with a section!");
            }

            let section = sections.get_mut(&section_name)
                .expect(&format!("no section with name: {section_name}"));

            let split = line.split("=");

            let mut key = None;
            let mut value = None;
            for (i, part) in split.enumerate() {
                let part = part.trim();
                if i > 1 { eprintln!("weird split"); }
                if i == 0 { key = Some(part.to_string()); }
                if i == 1 { value = Some(IniValue::from(part).expect("couldn't parse ini value")); }
            }
            section.values.insert(key.expect("no key found"), value.expect("no value found"));
        }
        Self{sections}
    }

    pub fn get_section(&self, section_name: &str) -> Result<&Section, ()> {
        match self.sections.get(section_name) {
            None => { Err(()) },
            Some(sec) => { Ok(sec) },
        }
    }
}

#[test]
fn test_ini_parser() {
    let config = IniConfiguration::new("tori.ini");
    assert!(config.sections.contains_key("General"));

    for section in config.sections.iter() {
        println!("section: {}", section.0);
        for section_keyval in section.1.values.iter() {
            println!("\tkeyval: {{ {}, {:?} }}", section_keyval.0, section_keyval.1);
        }
        println!();
    }

    let figure0_section = config.get_section("Figure0").expect("no section Figure0");
    let scale = figure0_section.as_u32_or_die("scale");
    println!("scale: {scale}");
}