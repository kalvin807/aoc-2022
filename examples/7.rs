use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

struct TrieNode {
    children: HashMap<String, TrieNode>,
    name: String,
    is_file: bool,
    _size: i32,
}

impl TrieNode {
    fn new(name: String, is_file: bool, size: i32) -> Self {
        TrieNode {
            children: HashMap::new(),
            name,
            is_file,
            _size: if is_file { size } else { -1 },
        }
    }

    fn size(&self) -> i32 {
        if self.is_file {
            self._size
        } else {
            let mut size = 0;
            for (_, child) in self.children.iter() {
                size += child.size(); // Recursive call
            }
            size
        }
    }
}

/*
    Trie Implementation
*/

struct Trie {
    root: TrieNode,
    path: Vec<String>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {
    fn new() -> Self {
        Trie {
            root: TrieNode::new("/".to_string(), false, 0), // root dir
            path: vec![],
        }
    }

    fn insert_file(&mut self, filename: String, size: i32) {
        let mut current_node = &mut self.root;

        for p in &self.path {
            let next_node = current_node
                .children
                .entry(p.to_string())
                .or_insert_with(|| TrieNode::new(p.clone(), false, 0));
            current_node = next_node;
        }
        // Insert file node
        current_node
            .children
            .entry(filename.clone())
            .or_insert_with(|| TrieNode::new(filename.clone(), true, size));
    }

    // recursive iter all node, get size if it is dir, push to result
    fn get_all_dirs_size(&self) -> Vec<(String, i32)> {
        let mut result = vec![];
        let mut path = vec![];
        Self::get_all_dirs_size_helper(&self.root, &mut path, &mut result);
        result
    }

    fn get_all_dirs_size_helper(
        node: &TrieNode,
        path: &mut Vec<String>,
        result: &mut Vec<(String, i32)>,
    ) {
        if !node.is_file {
            let mut size = 0;
            for (key, child) in node.children.iter() {
                path.push(key.to_string());
                Self::get_all_dirs_size_helper(child, path, result);
                size += child.size();
                path.pop();
            }
            result.push((path.join("/"), size));
        }
    }

    fn pwd(&self) -> String {
        format!("/{}", self.path.join("/"))
    }

    fn cd_into(&mut self, dir: &str) {
        self.path.push(dir.to_string());
    }

    fn cd_root(&mut self) {
        self.path = vec![];
    }

    fn cd_back(&mut self) {
        self.path.pop();
    }

    fn root_size(&self) -> i32 {
        self.root.size()
    }

    fn find_dir_with_closest_size(&self, target_size: &i32) -> (String, i32) {
        let all_size = self.get_all_dirs_size();
        all_size
            .iter()
            .filter(|(_, size)| *size >= *target_size)
            .min_by(|a, b| a.1.cmp(&b.1))
            .unwrap()
            .clone()
    }
}

fn runner(path: &str) -> (i32, i32) {
    println!("reading file: {}", path);
    let file = File::open(path).unwrap();
    let lines = BufReader::new(&file).lines().flatten();

    let mut fs = Trie::new();

    for ln in lines {
        let mut input = ln.split_whitespace();
        // Cmd start with "$"
        let first = input.next().unwrap();
        if first == "$" {
            let cmd = input.next().unwrap();
            match cmd {
                "cd" => {
                    let arg = input.next().unwrap();
                    match arg {
                        "/" => {
                            fs.cd_root();
                        }
                        ".." => {
                            fs.cd_back();
                        }
                        dir => fs.cd_into(dir),
                    }
                    println!("{}", fs.pwd());
                }
                "ls" => {
                    println!("ls",);
                }
                _ => {
                    println!("Unknown cmd: {}", cmd);
                }
            }
        } else if first != "dir" {
            let filename = input.next().unwrap();
            fs.insert_file(filename.to_string(), first.parse().unwrap());
        }
    }

    let fd = fs.get_all_dirs_size();

    fd.iter().for_each(|(dir, size)| {
        println!("{}: {}", dir, size);
    });

    let part_1 = fd
        .iter()
        .filter(|(_, size)| *size <= 100000)
        .map(|(_, size)| size)
        .sum::<i32>();

    let remaining_space = 70000000 - fs.root_size();
    let need_space = 30000000 - remaining_space;

    let (dir_path, dir_size) = fs.find_dir_with_closest_size(&need_space);
    println!("{}: {}", dir_path, dir_size);

    (part_1, dir_size)
}
fn main() {
    let path = "inputs/actual/7";
    println!("Ans: {:?}", runner(path));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let path = "inputs/test/7";
        let res = super::runner(path);
        assert_eq!(res, (95437, 24933642));
    }
}
