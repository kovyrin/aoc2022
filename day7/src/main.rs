use std::collections::HashMap;
use std::{fs::read_to_string, str::Lines};
use std::rc::Rc;
use std::cell::RefCell;
use anyhow::*;

type DirRef = Rc<RefCell<Dir>>;

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Option<DirRef>,
    dirs: HashMap<String, DirRef>,
    files_size: usize,
    total_size: usize,
}

impl Dir {
    fn new(name: &str, parent: Option<DirRef>) -> Self {
        let full_name: String = match &parent {
            None => name.to_string(),
            Some(parent) => format!("{}/{}", parent.borrow().name, name)
        };

        Dir {
            name: full_name,
            parent: parent,
            dirs: HashMap::new(),
            files_size: 0,
            total_size: 0,
        }
    }

    fn new_ref(name: &str, parent: Option<DirRef>) -> DirRef {
        let dir = Dir::new(name, parent);
        Rc::new(RefCell::new(dir))
    }

    fn calculate_size(&mut self) -> usize {
        self.total_size = self.files_size;
        self.total_size += self.dirs.values().map(|d| d.borrow_mut().calculate_size() ).sum::<usize>();
        self.total_size
    }

    fn find_dirs_smaller_than(&self, limit: usize) -> HashMap<String,usize> {
        let mut results = HashMap::new();

        if self.total_size <= limit {
            results.insert(self.name.clone(), self.total_size);
        }

        for dir in self.dirs.values() {
            let subdir_results = dir.borrow().find_dirs_smaller_than(limit);
            results.extend(subdir_results);
        }

        results
    }
}

#[derive(Debug)]
struct Filesystem {
    root: DirRef,
    cwd: DirRef,
}

impl Filesystem {
    fn new() -> Self {
        let root_dir = Dir::new_ref("", None);
        Filesystem {
            root: Rc::clone(&root_dir),
            cwd: root_dir
        }
    }

    fn parse_line(&mut self, line: &str) {
        if line.is_empty(){
            return;
        }

        match line.chars().next().unwrap() {
            '$' => self.parse_command(line),
            'd' => self.parse_dir(line),
            _ => self.parse_file(line),
        }
    }

    fn parse_command(&mut self, line: &str) {
        if line.starts_with("$ ls") {
            // nothing to do here
        } else if line.starts_with("$ cd") {
            let dir_name = &line[5..];
            self.cd(dir_name);
        }
    }

    fn parse_dir(&mut self, line: &str) {
        let dir_name = line[4..].to_string();
        let dir = Dir::new_ref(&dir_name, Some(Rc::clone(&self.cwd)));
        self.cwd.borrow_mut().dirs.insert(dir_name, dir);
    }

    fn parse_file(&self, line: &str) {
        let mut file_parts = line.split_whitespace();
        let size: usize = file_parts.next().expect("extract file size").parse().expect("parse file size");
        self.cwd.borrow_mut().files_size += size;
    }

    fn cwd_parent(&self) -> DirRef {
        let cwd = self.cwd.borrow();
        cwd.parent.as_ref().unwrap().clone()
    }

    fn cd(&mut self, dir_name: &str) {
        if dir_name.eq("/") {
            self.cwd = Rc::clone(&self.root);
            return
        }

        if dir_name.eq("..") {
            self.cwd = self.cwd_parent();
            return
        }

        let dir_ref = Rc::clone(self.cwd.borrow().dirs.get(dir_name).expect("Looking directory to cd"));
        self.cwd = dir_ref;
    }

    fn calculate_sizes(&self) {
        self.root.borrow_mut().calculate_size();
    }
}

fn main() {
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let args: Vec<String> = std::env::args().collect();
    let input_file: &str;
    if args.len() > 1 && args[1] == "real" {
        input_file = "real-input.txt";
    } else {
        input_file = "demo-input.txt";
    }
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let lines: Lines = input.lines();

    let mut output_parser = Filesystem::new();
    for line in lines {
        output_parser.parse_line(line);
    }

    output_parser.calculate_sizes();
    let root = output_parser.root.borrow();
    let results = root.find_dirs_smaller_than(100_000);
    println!("Large dirs: {:?}", results);

    let total: usize = results.values().sum();
    println!("Total size: {}", total);
}
