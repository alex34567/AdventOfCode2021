use std::cell::Cell;
use std::cmp::max;
use std::fmt::{self, Display, Formatter};
use std::rc::{Rc, Weak};

static INPUT: &str = include_str!("input/Day18.txt");

#[derive(Clone)]
enum ShellNum {
    Regular(u8),
    Pair([Rc<Cell<ShellNumNode>>; 2]),
}

impl Display for ShellNum {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            ShellNum::Regular(x) => write!(f, "{}", x),
            ShellNum::Pair(pair) => {
                let left = ShellNumNode::get(&pair[0]);
                let right = ShellNumNode::get(&pair[1]);
                write!(f, "[{},{}]", left, right)
            }
        }
    }
}

#[derive(Clone)]
struct ShellNumNode {
    value: ShellNum,
    left_child: bool,
    parent: Weak<Cell<ShellNumNode>>,
}

impl Display for ShellNumNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.value)
    }
}

impl ShellNumNode {
    fn get(this: &Cell<Self>) -> Self {
        let orig = this.replace(ShellNumNode {
            value: ShellNum::Regular(0),
            left_child: false,
            parent: Weak::new(),
        });
        let ret = orig.clone();
        this.set(orig);
        ret
    }

    fn add_left(this: &Cell<Self>, value: u8) {
        let mut this_val = ShellNumNode::get(this);
        match &mut this_val.value {
            ShellNum::Regular(x) => {
                *x += value;
                this.set(this_val);
            }
            ShellNum::Pair(pair) => ShellNumNode::add_left(&pair[0], value),
        }
    }

    fn add_left_right(this: &Cell<Self>, value: u8) {
        let mut this_val = ShellNumNode::get(this);
        match &mut this_val.value {
            ShellNum::Regular(x) => {
                *x += value;
                this.set(this_val);
            }
            ShellNum::Pair(pair) => ShellNumNode::add_right(&pair[0], value),
        }
    }

    fn add_right(this: &Cell<Self>, value: u8) {
        let mut this_val = ShellNumNode::get(this);
        match &mut this_val.value {
            ShellNum::Regular(x) => {
                *x += value;
                this.set(this_val);
            }
            ShellNum::Pair(pair) => ShellNumNode::add_right(&pair[1], value),
        }
    }

    fn add_right_left(this: &Cell<Self>, value: u8) {
        let mut this_val = ShellNumNode::get(this);
        match &mut this_val.value {
            ShellNum::Regular(x) => {
                *x += value;
                this.set(this_val);
            }
            ShellNum::Pair(pair) => ShellNumNode::add_left(&pair[1], value),
        }
    }

    fn explode(this: Rc<Cell<Self>>, depth: i32) -> bool {
        let mut this_val = ShellNumNode::get(&this);
        match this_val.value {
            ShellNum::Regular(_) => false,
            ShellNum::Pair(pair) => {
                if depth == 4 {
                    let mut explode_left = Some(match ShellNumNode::get(&pair[0]).value {
                        ShellNum::Regular(x) => x,
                        _ => {
                            panic!()
                        }
                    });
                    let mut explode_right = Some(match ShellNumNode::get(&pair[1]).value {
                        ShellNum::Regular(x) => x,
                        _ => {
                            panic!()
                        }
                    });
                    let mut curr_node = this_val.parent.upgrade();
                    let mut is_left_side = this_val.left_child;
                    while let Some(node) = curr_node {
                        let node_val = ShellNumNode::get(&node);
                        if is_left_side {
                            if let Some(explode) = explode_right {
                                ShellNumNode::add_right_left(&node, explode);
                                explode_right = None;
                            }
                        } else if let Some(explode) = explode_left {
                                ShellNumNode::add_left_right(&node, explode);
                                explode_left = None;
                            }
                        if explode_left.is_none() && explode_right.is_none() {
                            break;
                        }
                        is_left_side = node_val.left_child;
                        curr_node = node_val.parent.upgrade();
                    }
                    this_val.value = ShellNum::Regular(0);
                    this.set(this_val);
                    return true;
                }
                ShellNumNode::explode(pair[0].clone(), depth + 1)
                    || ShellNumNode::explode(pair[1].clone(), depth + 1)
            }
        }
    }

    fn split(this: Rc<Cell<Self>>) -> bool {
        let mut this_val = ShellNumNode::get(&this);
        match this_val.value {
            ShellNum::Regular(x) => {
                if x > 9 {
                    let left_side = x / 2;
                    let right_side = left_side + (x & 1);
                    let left_node = Rc::new(Cell::new(ShellNumNode {
                        value: ShellNum::Regular(left_side),
                        left_child: true,
                        parent: Rc::downgrade(&this),
                    }));

                    let right_node = Rc::new(Cell::new(ShellNumNode {
                        value: ShellNum::Regular(right_side),
                        left_child: false,
                        parent: Rc::downgrade(&this),
                    }));

                    this_val.value = ShellNum::Pair([left_node, right_node]);
                    this.set(this_val);
                    true
                } else {
                    false
                }
            }
            ShellNum::Pair(pair) => {
                ShellNumNode::split(pair[0].clone()) || ShellNumNode::split(pair[1].clone())
            }
        }
    }

    fn magnitude(this: &Cell<Self>) -> u64 {
        let this_val = ShellNumNode::get(this);
        match this_val.value {
            ShellNum::Regular(x) => x as u64,
            ShellNum::Pair(pair) => {
                ShellNumNode::magnitude(&pair[0]) * 3 + ShellNumNode::magnitude(&pair[1]) * 2
            }
        }
    }

    fn add(mut this: Rc<Cell<Self>>, rhs: &str) -> Rc<Cell<ShellNumNode>> {
        let new_parent = Rc::new(Cell::new(ShellNumNode {
            value: ShellNum::Regular(0),
            left_child: true,
            parent: Weak::new(),
        }));

        let left = this;
        let mut left_value = ShellNumNode::get(&left);
        left_value.left_child = true;
        left_value.parent = Rc::downgrade(&new_parent);
        left.set(left_value);
        let right = parse_input(&mut rhs.chars(), Rc::downgrade(&new_parent), false);
        new_parent.set(ShellNumNode {
            value: ShellNum::Pair([left, right]),
            left_child: true,
            parent: Weak::new(),
        });
        this = new_parent;

        let mut action_done = true;
        while action_done {
            action_done = false;

            let mut needs_exploding = true;
            while needs_exploding {
                needs_exploding = ShellNumNode::explode(this.clone(), 0);
                action_done = action_done || needs_exploding;
            }

            action_done = ShellNumNode::split(this.clone()) || action_done;
        }
        this
    }
}

fn parse_input(
    chars: &mut impl Iterator<Item = char>,
    parent: Weak<Cell<ShellNumNode>>,
    left_child: bool,
) -> Rc<Cell<ShellNumNode>> {
    let action = chars.next().unwrap();
    if action == '[' {
        let new_parent = Rc::new(Cell::new(ShellNumNode {
            value: ShellNum::Regular(0),
            left_child,
            parent: parent.clone(),
        }));

        let left = parse_input(chars, Rc::downgrade(&new_parent), true);
        assert_eq!(chars.next(), Some(','));
        let right = parse_input(chars, Rc::downgrade(&new_parent), false);
        assert_eq!(chars.next(), Some(']'));

        new_parent.set(ShellNumNode {
            value: ShellNum::Pair([left, right]),
            left_child,
            parent,
        });
        new_parent
    } else {
        let mut string = [0; 4];
        let num = action.encode_utf8(&mut string).parse().unwrap();
        Rc::new(Cell::new(ShellNumNode {
            value: ShellNum::Regular(num),
            left_child,
            parent,
        }))
    }
}

pub fn day18() {
    let lines = INPUT.trim().split('\n').collect::<Vec<_>>();
    let mut res = parse_input(&mut lines[0].chars(), Weak::new(), true);

    for line in lines.iter().skip(1) {
        res = ShellNumNode::add(res, line)
    }

    println!("Part1: {}", ShellNumNode::magnitude(&res));

    let mut part2 = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            let num = parse_input(&mut lines[i].chars(), Weak::new(), true);
            let res = ShellNumNode::add(num, lines[j]);

            part2 = max(part2, ShellNumNode::magnitude(&res));
        }
    }

    println!("Part2: {}", part2);
}
