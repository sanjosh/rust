use std::cmp::Ordering;

#[allow(dead_code)]

#[derive(Debug)]
pub enum BTree<T: Ord> {
	Leaf {
		v: T,
		l: Box<BTree<T>>,
		r: Box<BTree<T>>,
	},
	Empty,
}

impl<T: Ord> BTree<T> {
	pub fn new() -> BTree<T> {
		BTree::Empty
	}

	pub fn insert(&mut self, nv: T) {
		match self {
			&mut BTree::Leaf { ref v, ref mut l, ref mut r } => {
				match nv.cmp(v) {
					Ordering::Less => r.insert(nv),
					Ordering::Greater => l.insert(nv),
					_  => return
				}
			},
			&mut BTree::Empty => {
				*self = BTree::Leaf { v: nv, l: Box::new(BTree::Empty), r: Box::new(BTree::Empty) }
			},
		};
	}

	pub fn is_empty(&self) -> bool {
		match self {
			&BTree::Leaf { .. } => false,
			&BTree::Empty => true,
		}
	}

	pub fn find(&self, fv: T) -> bool {
		match self {
			&BTree::Leaf { ref v, ref l, ref r } => {
				match fv.cmp(v) {
					Ordering::Less => r.find(fv),
					Ordering::Greater => l.find(fv),
					_  => true
				}
			},
			&BTree::Empty => false,
		}
	}
}
