use replace_with::replace_with;
use std::fmt;

pub trait Monoid {
    const MEMPTY: Self;
    fn mappend(self, other: Self) -> Self;
}

#[derive(Clone, PartialEq, Eq)]
struct Node<T: Monoid + Copy + Eq + fmt::Display + Ord> {
    val: T,
    left: Box<Tree<T>>,
    right: Box<Tree<T>>,
    height: usize,
    size: usize,
    sum: T,
}

#[derive(Clone, PartialEq, Eq)]
enum Tree<T: Monoid + Copy + Eq + fmt::Display + Ord> {
    None(),
    Node(Node<T>),
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> fmt::Display for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tree::None() => {
                write!(f, "")
            }
            Tree::Node(Node {
                val, left, right, ..
            }) => {
                let lft = match &**left {
                    Tree::None() => String::new(),
                    Tree::Node(Node {
                        val: l_val,
                        left: l_left,
                        right: l_right,
                        ..
                    }) => {
                        format!("[ {} {} {} ]", l_left, l_val, l_right)
                    }
                };
                let rht = match &**right {
                    Tree::None() => String::new(),
                    Tree::Node(Node {
                        val: r_val,
                        left: r_left,
                        right: r_right,
                        ..
                    }) => {
                        format!("[ {} {} {} ]", r_left, r_val, r_right)
                    }
                };
                write!(f, "{}", dels(format!("[ {} {} {} ]", lft, val, rht)))
            }
        }
    }
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> fmt::Debug for Tree<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tree::None() => {
                write!(f, "")
            }
            Tree::Node(Node {
                val,
                left,
                right,
                height,
                ..
            }) => {
                let lft = match &**left {
                    Tree::None() => String::new(),
                    Tree::Node(Node {
                        val: l_val,
                        left: l_left,
                        right: l_right,
                        height: l_height,
                        ..
                    }) => {
                        format!("[ {:?} ({},{}) {:?} ]", l_left, l_val, l_height, l_right)
                    }
                };
                let rht = match &**right {
                    Tree::None() => String::new(),
                    Tree::Node(Node {
                        val: r_val,
                        left: r_left,
                        right: r_right,
                        height: r_height,
                        ..
                    }) => {
                        format!("[ {:?} ({},{}) {:?} ]", r_left, r_val, r_height, r_right)
                    }
                };
                write!(
                    f,
                    "{}",
                    dels(format!("[ {} ({},{}) {} ]", lft, val, height, rht))
                )
            }
        }
    }
}

//собираем дерево из верщины и левого и правого поддеревьев
impl<T: Monoid + Copy + Eq + fmt::Display + Ord> From<(T, Tree<T>, Tree<T>)> for Tree<T> {
    fn from((val, left, right): (T, Tree<T>, Tree<T>)) -> Self {
        let mut tree = Tree::Node(Node {
            val,
            sum: T::MEMPTY,
            left: Box::new(left),
            right: Box::new(right),
            height: 0,
            size: 0,
        });
        tree.update();
        tree
    }
}

//собираем дерево по всем параметрам
impl<T: Monoid + Copy + Eq + fmt::Display + Ord> From<(T, Tree<T>, Tree<T>, usize, usize, T)>
    for Tree<T>
{
    fn from((val, left, right, height, size, sum): (T, Tree<T>, Tree<T>, usize, usize, T)) -> Self {
        Tree::Node(Node {
            val,
            left: Box::new(left),
            right: Box::new(right),
            height,
            size,
            sum,
        })
    }
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> From<T> for Tree<T> {
    fn from(val: T) -> Self {
        Tree::Node(Node {
            val,
            left: Box::new(Tree::None()),
            right: Box::new(Tree::None()),
            height: 1,
            size: 1,
            sum: val,
        })
    }
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> From<(Tree<T>, Tree<T>)> for Tree<T> {
    fn from((left, right): (Tree<T>, Tree<T>)) -> Self {
        let mut right = right;
        let vert = right.min();
        right.delete(vert);
        Tree::from((vert, left, right))
    }
}

impl<T: Monoid + Copy + Eq + fmt::Display + Ord> Tree<T> {
    fn destruct(self) -> (T, Tree<T>, Tree<T>) {
        if let Tree::Node(Node {
            val, left, right, ..
        }) = self
        {
            return (val, *left, *right);
        }
        panic!("failed to destruct Tree enum, expected Tree::Node(Node), found Tree::None");
    }
    fn height(&self) -> usize {
        match self {
            Tree::None() => 0usize,
            Tree::Node(Node { height, .. }) => *height,
        }
    }
    fn size(&self) -> usize {
        match self {
            Tree::None() => 0usize,
            Tree::Node(Node { size, .. }) => *size,
        }
    }
    fn sum(&self) -> T {
        match self {
            Tree::None() => T::MEMPTY,
            Tree::Node(Node { sum, .. }) => *sum,
        }
    }
    pub fn mappend_from_to(&self, min: T, max: T) -> T {
        match self {
            Tree::None() => {
                return T::MEMPTY;
            }
            Tree::Node(Node {
                val,
                left,
                right,
                sum,
                ..
            }) => {
                if **left != Tree::None() && left.max() < min {
                    if *val == min {
                        return Monoid::mappend(*val, right.mappend_from_to(min, max));
                    } else {
                        return right.mappend_from_to(min, max);
                    }
                }
                if **right != Tree::None() && max < right.min() {
                    if *val == max {
                        return Monoid::mappend(*val, left.mappend_from_to(min, max));
                    } else {
                        return left.mappend_from_to(min, max);
                    }
                }
                if **left != Tree::None()
                    && **right != Tree::None()
                    && min <= left.min()
                    && right.max() < max
                {
                    return *sum;
                }
                return Monoid::mappend(
                    *val,
                    Monoid::mappend(
                        left.mappend_from_to(min, max),
                        right.mappend_from_to(min, max),
                    ),
                );
            }
        }
    }
    fn balance(&mut self) {
        replace_with(self, || Tree::None(), |self_| self_.rotate());
    }
    fn rotate(self) -> Self {
        if self == Tree::None() {
            return Tree::None();
        }
        let (val, left, right) = self.destruct();
        let mut ans;
        if left.height() > right.height() && left.height() - right.height() > 1 {
            let (d_val, d_left, d_right) = left.destruct();
            if d_left.height() >= d_right.height() {
                let mut right_t = Tree::from((val, d_right, right));
                right_t.update();
                ans = Tree::from((d_val, d_left, right_t));
                ans.update();
                return ans;
            } else {
                let (dd_val, dd_left, dd_right) = d_right.destruct();
                let mut left_t = Tree::from((d_val, d_left, dd_left));
                left_t.update();
                let mut right_t = Tree::from((val, dd_right, right));
                right_t.update();
                ans = Tree::from((dd_val, left_t, right_t));
                ans.update();
                return ans;
            }
        } else if left.height() < right.height() && right.height() - left.height() > 1 {
            let (d_val, d_left, d_right) = right.destruct();
            if d_right.height() >= d_left.height() {
                let mut left_t = Tree::from((val, left, d_left));
                left_t.update();
                ans = Tree::from((d_val, left_t, d_right));
                ans.update();
                return ans;
            } else {
                let (dd_val, dd_left, dd_right) = d_left.destruct();
                let mut left_t = Tree::from((val, left, dd_left));
                left_t.update();
                let mut right_t = Tree::from((d_val, dd_right, d_right));
                right_t.update();
                ans = Tree::from((dd_val, left_t, right_t));
                ans.update();
                return ans;
            }
        }
        let mut ans = Tree::from((val, left, right));
        ans.update();
        ans
    }
    fn insert(&mut self, i: T) {
        replace_with(self, || Tree::None(), |self_| self_.ins(i));
    }
    fn ins(self, i: T) -> Self {
        match self {
            Tree::None() => {
                return Tree::from(i);
            }
            _ => {
                let (val, left, right) = self.destruct();
                if i == val {
                    let mut ans = Tree::from((val, left, right));
                    ans.update();
                    return ans;
                }
                let mut t_left = left;
                let mut t_right = right;
                if i < val {
                    t_left.insert(i);
                } else {
                    t_right.insert(i);
                }
                let mut ans = Tree::from((val, t_left, t_right));
                ans.balance();
                ans.update();
                return ans;
            }
        }
    }
    fn index(&self, i: usize) -> T {
        match self {
            Tree::None() => {
                panic!("wrong index, failed to access");
            }
            Tree::Node(Node {
                val, left, right, ..
            }) => {
                if i == left.size() {
                    return *val;
                }
                if i < left.size() {
                    return left.index(i);
                } else {
                    return right.index(i - left.size() - 1);
                }
            }
        }
    }
    fn min(&self) -> T {
        match self {
            Tree::None() => panic!("tried to find min in empty tree"),
            Tree::Node(Node { val, left, .. }) => {
                if **left == Tree::None() {
                    return *val;
                }
                return left.min();
            }
        }
    }
    fn max(&self) -> T {
        match self {
            Tree::None() => panic!("tried to find max in empty tree"),
            Tree::Node(Node { val, right, .. }) => {
                if **right == Tree::None() {
                    return *val;
                }
                return right.max();
            }
        }
    }
    fn delete(&mut self, i: T) {
        replace_with(self, || Tree::None(), |self_| self_.del(i));
    }
    fn del(self, i: T) -> Self {
        if let Tree::None() = self {
            return self;
        }
        let (val, mut left, mut right) = self.destruct();
        let mut vert = val;
        if val == i {
            if left == Tree::None() && right == Tree::None() {
                return Tree::None();
            }
            if right != Tree::None() {
                vert = right.min();
                right.delete(vert);
            } else {
                vert = left.max();
                left.delete(vert);
            }
        }
        if i < val {
            left.delete(i);
        } else {
            right.delete(i);
        }
        Tree::from((vert, left, right))
    }
    pub fn find(&self, i: T) -> bool {
        match self {
            Tree::None() => false,
            Tree::Node(Node {
                val, left, right, ..
            }) => {
                if i == *val {
                    return true;
                }
                if i > *val {
                    return right.find(i);
                } else {
                    return left.find(i);
                }
            }
        }
    }
    fn update(&mut self) {
        replace_with(self, || Tree::None(), |self_| self_.upd());
    }
    fn upd(self) -> Self {
        if let Tree::None() = self {
            return Tree::None();
        }
        let (val, left, right) = self.destruct();
        let height = left.height().max(right.height()) + 1;
        let size = left.size() + right.size() + 1;
        let sum = Monoid::mappend(val, Monoid::mappend(left.sum(), right.sum()));
        Tree::from((val, left, right, height, size, sum))
    }
    pub fn avl_merge(left: Tree<T>, right: Tree<T>) -> Self {
        if left == Tree::None() && right == Tree::None() {
            return Tree::None();
        }
        if left != Tree::None() {
            let mut lft = left;
            let vert = lft.max();
            lft.delete(vert);
            return Tree::avl_merge_with_root(vert, lft, right);
        }
        let mut righ = right;
        let vert = righ.min();
        righ.delete(vert);
        Tree::avl_merge_with_root(vert, left, righ)
    }
    pub fn avl_merge_with_root(vert: T, left: Tree<T>, right: Tree<T>) -> Self {
        if (left.height() as i32 - right.height() as i32).abs() <= 1 {
            return Tree::from((vert, left, right));
        }
        if left.height() > right.height() {
            let (var, left_t, mut right_t) = left.destruct();
            right_t = Tree::avl_merge_with_root(vert, right_t, right);
            let mut tree = Tree::from((var, left_t, right_t));
            tree.balance();
            tree.update();
            return tree;
        } else {
            let (var, mut left_t, right_t) = right.destruct();
            left_t = Tree::avl_merge_with_root(vert, left, left_t);
            let mut tree = Tree::from((var, left_t, right_t));
            tree.balance();
            tree.update();
            return tree;
        }
    }
    pub fn divide(self, k: T) -> (Self, Self) {
        if self == Tree::None() {
            return (Tree::None(), Tree::None());
        }
        let (vert, left, right) = self.destruct();
        if k < vert {
            let mut right_t = Tree::avl_merge(Tree::from(vert), right);
            let (left_t, right_d) = left.divide(k);
            right_t = Tree::avl_merge(right_d, right_t);
            right_t.balance();
            return (left_t, right_t);
        } else {
            let mut left_t = Tree::avl_merge(left, Tree::from(vert));
            let (left_d, right_t) = right.divide(k);
            left_t = Tree::avl_merge(left_t, left_d);
            left_t.balance();
            return (left_t, right_t);
        }
    }
    pub fn in_order(&self) -> String {
        match self {
            Tree::None() => String::new(),
            Tree::Node(Node {
                val, left, right, ..
            }) => {
                format!("{} {} {}", (*left).in_order(), val, (*right).in_order(),)
            }
        }
    }
    pub fn pre_order(&self) -> String {
        match self {
            Tree::None() => String::new(),
            Tree::Node(Node {
                val, left, right, ..
            }) => {
                format!("{} {} {}", val, (*left).in_order(), (*right).in_order(),)
            }
        }
    }
    pub fn post_order(&self) -> String {
        match self {
            Tree::None() => String::new(),
            Tree::Node(Node {
                val, left, right, ..
            }) => {
                format!("{} {} {}", (*left).in_order(), (*right).in_order(), val)
            }
        }
    }
}

fn dels(string: String) -> String {
    string
        .split_ascii_whitespace()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ")
}
