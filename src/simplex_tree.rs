use crate::point::Point;
use hashbrown::HashMap;
use std::collections::BTreeSet;

#[derive(PartialEq, Eq, Hash)]
struct Node<'a> {
    value: u64,
    children: NodeSet<'a>,
}

#[derive(PartialEq, Eq, Hash)]
struct NodeSet<'a> {
    values: BTreeSet<Node<'a>>,
    parent: &'a Node<'a>,
}

type LevelList<'a> = Vec<&'a NodeSet<'a>>;

//holds a map from levels to a map from labels to corresponding levellists
struct LevelListMap<'a> {
    matrix: HashMap<u64, HashMap<u64, LevelList<'a>>>,
}

impl<'a> LevelListMap<'a> {
    //returns a map from depth levels j to all LevelLists L(j, label) (or None)
    fn levels(&mut self, label: u64) -> Option<&mut HashMap<u64, LevelList<'a>>> {
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

    //we insert a given NodeSet in the LevelList L(level, label)
    fn insert_node_set(&mut self, node_set: &'a NodeSet<'a>, label: u64, level: u64) -> bool {
        let mut levels: &mut HashMap<u64, LevelList<'a>> = &mut HashMap::new();
        if let Some(lvl) = self.levels(label) {
            levels = lvl;
            if let Some(level_list) = levels.get_mut(&level) {
                if level_list.contains(&node_set) {
                    false
                } else {
                    level_list.push(node_set);
                    true
                }
            } else {
                let mut level_list = LevelList::with_capacity(1);
                level_list.push(node_set);
                levels.insert(level, level_list);
                true
            }
        } else {
            self.matrix.insert(label, *levels);
            let mut level_list = LevelList::with_capacity(1);
            level_list.push(node_set);
            levels.insert(level, level_list);
            true
        }
    }

    //removes a certain NodeSet (possibly) stored in L(level, label)
    fn remove_node_set(&mut self, node_set: &'a NodeSet<'a>, label: u64, level: u64) -> bool {
        if let Some(level_list) = self.level_list(label, level) {
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

/*pub struct SimplexTree {
    top_nodes: Vec<Node>, //set the parent node to ROOT
}*/

//constructors
