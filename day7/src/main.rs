use std::collections::HashMap;
use std::{fs::read_to_string, str::Lines};
use std::rc::Rc;
use std::cell::RefCell;
use anyhow::Context;

type DirRef = Rc<RefCell<Dir>>;
type DirSize = (String, usize);

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
}

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
        match line.chars().next() {
            Some('$') => self.parse_command(line),
            Some('d') => self.parse_dir(line),
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

    fn calculate_total_sizes(&self) {
        self.root.borrow_mut().calculate_size();
    }

    fn total_size(&self) -> usize {
        self.root.borrow().total_size
    }

}

struct DirIterator {
    dirs_to_walk: Vec<DirRef>
}

impl IntoIterator for &Filesystem {
    type Item = DirSize;
    type IntoIter = DirIterator;

    fn into_iter(self) -> Self::IntoIter {
        DirIterator { dirs_to_walk: [self.root.clone()].to_vec() }
    }
}

impl Iterator for DirIterator {
    type Item = DirSize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.dirs_to_walk.pop() {
            None => None,
            Some(dir) => {
                self.dirs_to_walk.extend(dir.borrow().dirs.values().map(|r| Rc::clone(r)));
                Some((dir.borrow().name.clone(), dir.borrow().total_size))
            }
        }
    }
}

const TOTAL_DISK_SIZE: usize = 70000000;
const SPACE_NEEDED: usize = 30000000;

fn main() {
    // If first argument is "real", use the real input file
    // Otherwise, use the test input file
    let input_file = if std::env::args().nth(1).unwrap_or(String::default()).eq("real") {
        "real-input.txt"
    } else {
        "demo-input.txt"
    };
    println!("Using input file: {}", input_file);

    let input: String = read_to_string(input_file).context("failed to read the data file").unwrap();
    let lines: Lines = input.lines();

    let mut fs = Filesystem::new();
    lines.for_each(|l| fs.parse_line(l));

    fs.calculate_total_sizes();

    let sub100k_dirs = fs.into_iter().filter(|d| d.1 < 100_000);
    let total_sub100k: usize = sub100k_dirs.map(|d| d.1).sum();
    println!("Total sub-100k dirs size: {}", total_sub100k);

    let total_used = fs.total_size();
    let unused_space = TOTAL_DISK_SIZE - total_used;
    let space_to_free = SPACE_NEEDED - unused_space;

    let mut dir_sizes: Vec<DirSize> = fs.into_iter().collect();
    dir_sizes.sort_by(|(_, size1), (_, size2)| size1.cmp(size2));
    for (dir, size) in dir_sizes {
        if size >= space_to_free {
            println!("Largest candidate: {} size {}", dir, size);
            break;
        }
    }
}
