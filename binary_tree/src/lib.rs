mod btree;

#[cfg(test)]

mod tests {
	use super::btree::BTree;

    #[test]
		fn create_tree() {
			let mut my_tree = BTree::new();
			my_tree.insert("abc")
		}
}
