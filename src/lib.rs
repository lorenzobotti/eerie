mod strings;

use std::error::Error;
use std::fs;
use std::fs::File as StdFile;

use std::io::Write;
use std::path::Path;

use strings::which_comes_first;
use subprocess::Exec;
use subprocess::ExitStatus;

use strings::trim_first_line;

const DELIMITER_NEWLINE: &str = "```";
const DELIMITER_NO_NEWLINE: &str = "````";
const SUBTITLE: &str = "## ";

#[derive(Debug, Clone, Copy)]
pub struct File<'a> {
    pub name: &'a str,
    pub content: &'a str,
    pub language: Option<&'a str>,
}

pub struct Files<'a>(pub Vec<File<'a>>);

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

    pub fn create(&self, folder: &Path) -> Result<(), Box<dyn Error>> {
        for parsed_file in &self.0 {
            if ["stdin", "stdout", "stderr", "command", "success"].contains(&parsed_file.name) {
                continue;
            }

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
        Some(self.get("command")?.content.trim())
    }

    pub fn status(&self) -> Option<i32> {
        self.get("status")?.content.trim().parse().ok()
    }

    pub fn run(&self, folder: &Path) -> Result<(), Box<dyn Error>> {
        let mut command_args = self.command().ok_or("can't find command")?.split(" ");
        self.create(folder)?;

        let mut exec = Exec::cmd(command_args.next().unwrap());
        for arg in command_args {
            exec = exec.arg(arg);
        }

        exec = exec.cwd(folder);
        if let Some(stdin) = self.stdin() {
            exec = exec.stdin(stdin);
        }

        let captured = exec.capture()?;

        if let Some(stdout) = self.stdout() {
            let gotten = String::from_utf8(captured.stdout)?;

            if gotten != stdout {
                return Err("stdout doesn't match".into());
            }
        }

        if let Some(stderr) = self.stderr() {
            let gotten = String::from_utf8(captured.stderr)?;

            if &gotten != stderr {
                return Err("stdout doesn't match".into());
            }
        }

        if let Some(expected_status) = self.status() {
            let code = match captured.exit_status {
                ExitStatus::Exited(s) => s as i32,
                ExitStatus::Other(s) => s,
                ExitStatus::Signaled(s) => s as i32,
                ExitStatus::Undetermined => return Err("exit code is undetermined".into()),
            };

            if code != expected_status {
                return Err("wrong exit status".into());
            } else {
                return Ok(());
            }
        }

        if captured.exit_status.success() {
            Ok(())
        } else {
            Err("failing status code".into())
        }
    }
}

impl<'a> File<'a> {
    pub fn from_str(s: &'a str) -> Result<(Self, &'a str), &'static str> {
        let starting_str = s;

        let (delimiter, delimiter_start) =
            which_comes_first(s, DELIMITER_NEWLINE, DELIMITER_NO_NEWLINE)
                .ok_or("can't find beginning delimiter")?;

        let (intro, rest) = s.split_at(delimiter_start);
        let rest = &rest[delimiter.bytes().len()..];
        
        let name = Self::parse_name(intro).ok_or("couldn't parse name")?;
        
        let (content, rest) = rest
            .split_once(delimiter)
            .ok_or("can't find beginning delimiter")?;
        
        let language = Self::parse_language(content).ok_or("can't parse content start")?;
        let mut content = trim_first_line(content).ok_or("can't trim content start")?;

        if delimiter == DELIMITER_NO_NEWLINE && content.ends_with('\n') {
            content = &content[..(content.bytes().len() - 1)];
        }

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
            _ => Some(name),
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
    use std::fs;

    #[test]
    fn execute_test_files() {
        let target_folder = "./temp_test";

        if Path::new(target_folder).is_dir() {
            fs::remove_dir_all(target_folder).unwrap();
        }

        for file in fs::read_dir("./tests").unwrap() {
            let file = file.unwrap();
            let contents = fs::read_to_string(file.path()).unwrap();

            let (files, _) = Files::from_str(&contents).unwrap();
            
            if files.get("command").is_some() {
                fs::create_dir(target_folder).unwrap();
                files.run(Path::new(target_folder)).unwrap();
                fs::remove_dir_all(target_folder).unwrap();
            }
        }
    }

    #[test]
    fn newline() {
        let input_file = include_str!("../tests/newline.eer.md");
        let (files, _) = Files::from_str(input_file).unwrap();

        assert!(!files
            .get("no_new_line.txt")
            .unwrap()
            .content
            .ends_with('\n'));

        assert!(files
            .get("a_new_line.txt")
            .unwrap()
            .content
            .ends_with('\n'));
    }
}
