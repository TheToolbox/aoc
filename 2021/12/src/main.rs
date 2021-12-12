use core::panic;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Missing argument");
    }

    let data = fs::read_to_string(&args[2])
        .unwrap();
    
    let system = CaveSystem::new(&data);
    let revisit = args[1] == "part2";
    let mut result = count_traversals(&system, 
                                    &Cave::new(String::from("start")),
                                    !revisit,
                                    String::from(""));
    result.sort();
    result.dedup();
    
    //println!("{:?}",result);
    println!("{:?}",result.len());
}

#[derive(Clone)]
struct CaveSystem {
    caves: HashMap<Cave, HashSet<Cave>>
}

impl CaveSystem {
    fn new(lines: &str) -> CaveSystem {
        let mut caves: HashMap<Cave, HashSet<Cave>> = HashMap::new();
        for line in lines.lines() {
            let (root, leaf) = line.split_once('-').unwrap();
            let cave1 = Cave::new(String::from(root));
            let cave2 = Cave::new(String::from(leaf));

            match caves.get_mut(&cave1) {
                Some(value) => { value.insert(cave2.clone()); },
                None => {
                    let mut connection = HashSet::new();
                    connection.insert(cave2.clone());
                    caves.insert(cave1.clone(), connection);
                },
            }
            match caves.get_mut(&cave2) {
                Some(value) => { value.insert(cave1.clone()); },
                None => {
                    let mut connection = HashSet::new();
                    connection.insert(cave1.clone());
                    caves.insert(cave2.clone(), connection);
                },
            }
        }

        //println!("{:?}",caves);

        CaveSystem {
            caves
        }

    }


}

fn count_traversals(caves: &CaveSystem, from: &Cave, revisited: bool, path: String) -> Vec<String> {
    if from.is_end() { return vec![path + "->end"]; }
    let mut caves = caves.clone();
    if from.big {
        let x = caves.caves.get(from);
        match x {
            Some(nexts) => {
                nexts.iter()
                    .filter(|next| !next.is_start())
                    .map(|next| count_traversals(&caves,next, revisited, path.clone() + "->" + &from.name))
                    .flatten()
                    .collect()
            },
            None => vec![],
        }
    } else {
        let revisit = if !revisited {
            match caves.caves.get(from) {
                Some(nexts) => {
                    nexts.clone().iter()
                        .filter(|next| !next.is_start())
                        .map(|next| count_traversals(&caves, next, true, path.clone() + "->" + &from.name))
                        .flatten()
                        .collect()
                },
                None => vec![],    
            }
        } else {
            vec![]
        };
        
        [
            revisit,
            match caves.caves.remove(from) {
                Some(nexts) => {
                    nexts.iter()
                        .filter(|next| !next.is_start())
                        .map(|next| count_traversals(&caves, next, revisited, path.clone() + "->" + &from.name))
                        .flatten()
                        .collect()
                },
                None => vec![],    
            }
        ].iter().flatten().cloned().collect()
    }
    
}


#[derive(PartialEq, Eq, Hash, Clone)]
struct Cave {
    big: bool,
    name: String,
}

impl Cave {
    fn new(name: String) -> Cave {
        Cave {
            big: name.to_uppercase() == name,
            name
        }
    }

    fn is_end(&self) -> bool {
        self.name == "end"
    }
    
    fn is_start(&self) -> bool {
        self.name == "start"
    }
}

use std::fmt;
impl fmt::Debug for Cave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f,"{}", self.name)
    }
}