use crate::io::read_file_lines;
use crate::problem::Problem;
use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

pub struct DayTen {}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    val: usize,
    pos: [usize; 2],
    is_leaf: bool,
    children: Vec<Rc<RefCell<Node>>>,
    // parent: Option<[usize; 2]>,
}

impl Node {
    pub fn new() -> Self {
        Node {
            val: 0,
            pos: [0, 0],
            is_leaf: false,
            children: vec![],
            // parent: None,
        }
    }

    pub fn build_tree(&mut self, content: &Vec<String>) {
        if self.val == 9 {
            self.is_leaf = true;
            return;
        }
        for dy in [-1, 0, 1] {
            for dx in [-1, 0, 1] {
                if (dy == 0 && dx == 0)
                    || (dy == 1 && dx == 1)
                    || (dy == -1 && dx == -1)
                    || (dy == -1 && dx == 1)
                    || (dy == 1 && dx == -1)
                {
                    continue;
                }
                let x = self.pos[0] as isize + dx;
                let y = self.pos[1] as isize + dy;
                if x >= 0 && x < content[0].len() as isize && y >= 0 && y < content.len() as isize {
                    let c = content
                        .iter()
                        .skip(y as usize)
                        .nth(0)
                        .unwrap()
                        .chars()
                        .skip(x as usize)
                        .nth(0)
                        .unwrap();
                    let val = c.to_digit(10).unwrap_or(100) as usize;
                    if val != self.val + 1 {
                        continue;
                    }
                    let mut child = Node::new();
                    child.pos = [x as usize, y as usize];
                    child.val = val;
                    child.build_tree(content);
                    self.children.push(Rc::new(RefCell::new(child)));
                }
            }
        }
    }

    pub fn walk_one(&self, history: &mut HashSet<[usize; 2]>) {
        for mut child in &self.children {
            if history.contains(&child.borrow().pos) {
                continue;
            } else if child.borrow().is_leaf {
                history.insert(child.borrow().pos);
                continue;
            } else {
                child.borrow().walk_one(history);
            }
        }
    }

    pub fn walk_tuah(&self) -> usize {
        let mut sum = 0;
        for mut child in &self.children {
            if child.borrow().is_leaf {
                sum += 1;
            } else {
                sum += child.borrow().walk_tuah();
            }
        }
        sum
    }
}
impl Problem for DayTen {
    fn part_one(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        // get all roots
        let mut roots = vec![];
        for (y, line) in content.iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '0' {
                    let mut root = Rc::new(RefCell::new(Node::new()));
                    root.borrow_mut().pos = [x, y];
                    roots.push(root);
                }
            }
        }
        let mut sum = 0;
        for root in roots.iter_mut() {
            root.borrow_mut().build_tree(&content);
            let mut history = HashSet::new();
            root.borrow_mut().walk_one(&mut history);
            sum += history.len();
        }

        format!("{}", sum)
    }

    fn part_two(&self, input: &Path) -> String {
        let content = read_file_lines(input);
        // get all roots
        let mut roots = vec![];
        for (y, line) in content.iter().enumerate() {
            for (x, c) in line.char_indices() {
                if c == '0' {
                    let mut root = Rc::new(RefCell::new(Node::new()));
                    root.borrow_mut().pos = [x, y];
                    roots.push(root);
                }
            }
        }
        let mut sum = 0;
        for root in roots.iter_mut() {
            root.borrow_mut().build_tree(&content);
            sum += root.borrow_mut().walk_tuah();
        }

        format!("{}", sum)
    }
}
