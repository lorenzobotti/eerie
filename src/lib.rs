mod strings;

use std::error::Error;
use std::fs;
use std::fs::File as StdFile;
use std::io::Write;
use std::path::Path;

use strings::trim_first_line;
use strings::trim_start;

const DELIMITER: &str = "```";
const SUBTITLE: &str = "## ";
const QUOTE: char = '"';

#[derive(Debug, Clone, Copy)]
pub struct File<'a> {
    pub name: &'a str,
    pub content: &'a str,
    pub language: Option<&'a str>,
}

pub struct Files<'a> (pub Vec<File<'a>>);

impl<'a> Files<'a> {
    pub fn from_str(input: &'a str) -> Result<(Self, &'a str), &'static str> {
        let mut files = Vec::new();
        let mut rest = input;
    
        loop {
            match File::from_str(rest) {
                Ok((file, left)) => {
                    files.push(file);
                    rest = left;
                }
                Err(err) => {
                    if files.is_empty() {
                        return Err(err);
                    } else {
                        return Ok((Self(files), rest));
                    }
                }
            }
        }
    }

    pub fn get(&self, name: &str) -> Option<File<'a>> {
        Some(*self.0.iter().find(|file| file.name == name)?)
    }

    pub fn create(&self, folder: &Path) -> Result<(), Box<dyn Error>>{
        for parsed_file in &self.0 {
            let path = folder.join(Path::new(parsed_file.name));
            let folder_name = path.parent().unwrap();

            fs::create_dir_all(folder_name)?;
            let mut file = StdFile::create(path)?;
            file.write_all(parsed_file.content.as_bytes())?;
        }

        Ok(())
    }

    pub fn stdout(&self) -> Option<&'a str> {
        Some(self.get("stdout")?.content)
    }

    pub fn stdin(&self) -> Option<&'a str> {
        Some(self.get("stdin")?.content)
    }
    
    pub fn stderr(&self) -> Option<&'a str> {
        Some(self.get("stderr")?.content)
    }

    pub fn command(&self) -> Option<&'a str> {
        Some(self.get("command")?.content)
    }
}

impl<'a> File<'a> {
    pub fn from_str(s: &'a str) -> Result<(Self, &'a str), &'static str> {
        let starting_str = s;

        let (intro, rest) = s
            .split_once(DELIMITER)
            .ok_or("can't find beginning delimiter")?;

        let name = Self::parse_name(intro).ok_or("couldn't parse name")?;
        let (content, rest) = rest
            .split_once(DELIMITER)
            .ok_or("can't find beginning delimiter")?;

        let language = Self::parse_language(content).ok_or("can't parse content start")?;
        let content = trim_first_line(content).ok_or("can't trim content start")?;

        let parsed_len = starting_str.bytes().len() - rest.bytes().len();

        Ok((
            Self {
                name: name,
                content: content,
                language: language,
            },
            &starting_str[parsed_len..],
        ))
    }

    fn parse_name(input: &'a str) -> Option<&'a str> {
        let name = input.split(SUBTITLE).nth(1)?.split('\n').next()?.trim();
        match name.len() {
            0 => None,
            _ => Some(name)
        }
    }
    
    fn parse_language(input: &'a str) -> Option<Option<&'a str>> {
        let line = input.lines().next()?;
        let lang = line.trim();
        match lang.len() {
            0 => Some(None),
            _ => Some(Some(lang)),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_start() {
        let inputs = [
            ("hamburger.go", "hamburger.go"),
            (r#""johnny boy""#, "johnny boy"),
            ("stdout", "stdout"),
        ];

        for (input, expected) in inputs {
            let got = File::parse_name(input).unwrap();
            assert_eq!(expected, got);
        }
    }
}
