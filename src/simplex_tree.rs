use crate::point::Point;
use hashbrown::HashMap;
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};

#[derive(PartialEq, Eq, PartialOrd, Debug)]
struct Node {
    value: usize,
    //depth: u64,  //optional for now: use will depend on whether not having it could cause mistakes
    children: BTreeSet<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
}

type LevelList<'a> = Vec<&'a Node>;

//holds a map from levels to a map from labels to corresponding levellists
struct LevelListMap<'a> {
    matrix: HashMap<u64, HashMap<u64, LevelList<'a>>>,
}

impl<'a> LevelListMap<'a> {
    //returns a map from depth levels j to all LevelLists(j, label)
    fn levels(&self, label: u64) -> Option<&HashMap<u64, LevelList<'a>>> {
        self.matrix.get(&label)
    }
    //returns a mutable map from depth levels j to all LevelLists L(j, label) (or None)
    fn mut_levels(&mut self, label: u64) -> Option<&mut HashMap<u64, LevelList<'a>>> {
        self.matrix.get_mut(&label)
    }
    //returns a reference to the LevelList L(level, label) (or None)
    fn level_list(&self, label: u64, level: u64) -> Option<&LevelList<'a>> {
        if let Some(levels) = self.matrix.get(&label) {
            levels.get(&level)
        } else {
            None
        }
    }
    //returns a mutable reference to the LevelList L(level, label) (or None)
    fn mut_level_list(&mut self, label: u64, level: u64) -> Option<&mut LevelList<'a>> {
        if let Some(levels) = self.matrix.get_mut(&label) {
            levels.get_mut(&level)
        } else {
            None
        }
    }

    //we insert a given NodeSet in the LevelList L(level, label)
    fn insert_node_set(&mut self, node: &'a Node, label: u64, level: u64) -> bool {
        let at_least_one_level: bool;
        if let Some(levels) = self.mut_levels(label) {
            at_least_one_level = true;
            if let Some(level_list) = levels.get_mut(&level) {
                if level_list.contains(&node) {
                    return false;
                } else {
                    level_list.push(node);
                    return true;
                }
            } else {
                let mut level_list = LevelList::with_capacity(1);
                level_list.push(node);
                levels.insert(level, level_list);
                return true;
            }
        } else {
            at_least_one_level = false;
        }
        if !at_least_one_level {
            let mut levels: HashMap<u64, LevelList<'a>> = HashMap::new();
            let mut level_list = LevelList::with_capacity(1);
            level_list.push(node);
            levels.insert(level, level_list);
            self.matrix.insert(label, levels);
            return true;
        }
        panic!("Code should never reach here");
    }

    //removes a certain NodeSet (possibly) stored in L(level, label)
    fn remove_node_set(&mut self, node_set: &'a Node, label: u64, level: u64) -> bool {
        if let Some(level_list) = self.mut_level_list(label, level) {
            if let Some(pos) = level_list.iter().position(|x| *x == node_set) {
                level_list.remove(pos);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

#[derive(Debug)]
pub struct SimplexTree {
    pub points: Vec<Point>,
    top_nodes: Vec<Node>, //set the parent node to ROOT
}

//constructors
//the SimplexTree originally only holds all points as 0-simplices
impl SimplexTree {
    fn new(p: Vec<Point>) -> Self {
        let mut p = p;
        p.sort();
        let nodes: Vec<Node> = p
            .iter()
            .enumerate()
            .map(|(i, _x)| Node {
                value: i,
                children: BTreeSet::new(),
                parent: None,
            })
            .collect();
        SimplexTree {
            points: p,
            top_nodes: nodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_creation() {
        let points = vec![
            Point {
                coordinates: vec![1.0, 1.0, 1.0],
            },
            Point {
                coordinates: vec![0.0, 5.0, 10.0],
            },
            Point {
                coordinates: vec![2.0, -1.0, 7.0],
            },
        ];
        let tree = SimplexTree::new(points);
        println!("{:?}", tree);
        assert_eq!(
            tree.points,
            vec![
                Point {
                    coordinates: vec![0.0, 5.0, 10.0],
                },
                Point {
                    coordinates: vec![1.0, 1.0, 1.0],
                },
                Point {
                    coordinates: vec![2.0, -1.0, 7.0],
                }
            ]
        );
    }
}
