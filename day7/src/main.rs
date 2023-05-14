use std::collections::HashMap;
use std::{fs::read_to_string, str::Lines};
use std::rc::Rc;
use std::cell::RefCell;
use anyhow::*;

type DirRef = Rc<RefCell<Dir>>;

#[derive(Debug)]
struct File {
    name: String,
    size: usize,
}

#[derive(Debug)]
struct Dir {
    name: String,
    parent: Option<DirRef>,
    dirs: HashMap<String, DirRef>,
    files: HashMap<String, File>,
    total_size: usize,
}

impl Dir {
    fn new(name: String, parent: Option<DirRef>) -> Self {
        Dir {
            name: name,
            parent: parent,
            files: HashMap::new(),
            dirs: HashMap::new(),
            total_size: 0,
        }
    }

    fn new_ref(name: &String, parent: Option<DirRef>) -> DirRef {
        let dir = Dir::new(
            name.clone(),
            parent
        );

        Rc::new(RefCell::new(dir))
    }

    fn calculate_size(&mut self) -> usize {
        self.total_size = 0;
        self.total_size += self.files.values().map(|f| { f.size }).sum::<usize>();
        self.total_size += self.dirs.values().map(|d| d.borrow_mut().calculate_size() ).sum::<usize>();
        self.total_size
    }

    fn find_dirs_smaller_than(&self, limit: usize) -> Vec<usize> {
        let mut results = Vec::new();

        if self.total_size <= limit && !self.name.eq("/") {
            results.push(self.total_size);
        }

        for dir in self.dirs.values() {
            let mut subdir_results = dir.borrow().find_dirs_smaller_than(limit);
            results.append(&mut subdir_results);
        }

        results
    }
}

#[derive(Debug)]
struct Parser {
    root: DirRef,
    cwd: DirRef,
}

impl Parser {
    fn new() -> Self {
        let root_dir = Dir::new_ref(&"/".to_string(), None);
        Parser {
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
        let mut cwd = self.cwd.borrow_mut();

        println!("Adding dir {} to {}", dir_name, cwd.name);
        let dir = Dir::new_ref(&dir_name, Some(Rc::clone(&self.cwd)));
        cwd.dirs.insert(dir_name, dir);
        println!("Resulting dirs: {:?}", cwd.dirs.keys());
    }

    fn parse_file(&self, line: &str) {
        let mut file_parts = line.split_whitespace();
        let size: usize = file_parts.next().expect("extract file size").parse().expect("parse file size");
        let name = file_parts.next().expect("extract file name").to_string();
        let file = File { name: name.clone(), size };
        self.cwd.borrow_mut().files.insert(name, file);
    }

    fn cwd_parent(&self) -> DirRef {
        let cwd = self.cwd.borrow();
        Rc::clone(cwd.parent.as_ref().unwrap())
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

        let dir_ref = {
            let old_cwd = self.cwd.borrow();
            println!("Looking for directory {} in {}", dir_name, old_cwd.name);
            Rc::clone(old_cwd.dirs.get(dir_name).expect("Looking directory to cd"))
        };
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

    let mut output_parser = Parser::new();
    for line in lines {
        output_parser.parse_line(line);
    }

    output_parser.calculate_sizes();
    let root = output_parser.root.borrow();
    let results = root.find_dirs_smaller_than(100_000);
    println!("Large dirs: {:?}", results);

    let total: usize = results.iter().sum();
    println!("Total size: {}", total);
}
