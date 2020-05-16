pub mod huffman {
	use std::boxed::Box;
	use std::cmp::Ordering;
	use std::collections::*;

	///	Node is a binary tree data structure.
	///	It will be used by huffman compression algorithm
	#[derive(Clone, PartialEq, Eq, Ord, std::fmt::Debug)]
	struct Node {
		letter: char,
		freq: i32,
		left: Option<Box<Node>>,
		right: Option<Box<Node>>,
	}
	impl PartialOrd for Node {
		fn partial_cmp(self: &Node, other: &Node) -> Option<Ordering> {
			let cmp = self.freq.cmp(&other.freq);
			Some(cmp.reverse()) // For min heap
		}
	}
	impl Node {
		/// A convinence function to create a leaf node, i.e a node with no children
		fn new(letter: char, freq: i32) -> Node {
			Node {
				letter,
				freq,
				left: None,
				right: None,
			}
		}
	}

	///
	/// Count the frequency of chars, return a vector of node.
	///
	/// Each node contains the character and corresponding frequency
	/// > Note: Algotithm is based on sorting
	///
	fn freq_count(text: std::str::Chars) -> Vec<Node> {
		let mut freq_vec = Vec::new();
		let mut chars: Vec<char> = text.collect();
		chars.sort();
		let mut freq = 0;
		let mut prev: char = *chars.first().expect("Input cannot be empty");
		for c in chars {
			if c == prev {
				freq += 1;
			} else {
				freq_vec.push(Node::new(prev, freq));
				freq = 1;
				prev = c;
			}
		}
		freq_vec.push(Node::new(prev, freq));
		return freq_vec;
	}

	/// Create huffman encoding using huffman algorithm
	/// ## Input:
	///     Frequency vector: A vector of Nodes containing character frequency
	///     (Use the freq_count function)
	/// ## Output:
	///     Root node of Huffman Tree of type Option<Box<Node>>
	/// # Algorithm
	/// - While priority_queue contains atleast 2 nodes:
	/// 	- Choose two minimum elements and combine them
	/// 	- Insert combined value back to tree
	/// - Return tree
	///
	fn construct_huffman_tree(freq: Vec<Node>) -> Node {
		let mut pq = BinaryHeap::new();
		for node in freq {
			pq.push(node);
		}
		while pq.len() > 1 {
			let (a, b) = (pq.pop().unwrap(), pq.pop().unwrap());
			let new_node = Node {
				letter: '\0',
				freq: a.freq + b.freq,
				left: Option::from(Box::from(a)),
				right: Option::from(Box::from(b)),
			};
			pq.push(new_node);
		}
		pq.pop().unwrap()
	}
	/// Convert huffman tree to a hashmap with key as char and value as encoding
	/// E.g key = 'a', value = '1000'
	fn to_hashmap(node: &Node) -> HashMap<char, String> {
		let mut hm = HashMap::new();
		// Huffman tree is complete binary tree, a node will have either 0 or 2 children, 1 is not possible
		if node.left.is_none() {
			hm.insert(node.letter, "0".to_string());
			return hm;
		}
		fn encode(hm: &mut HashMap<char, String>, node: &Node, encoding: String) {
			if node.left.is_none() {
				hm.insert(node.letter, encoding);
			} else {
				let left_path = String::from(&encoding) + "0";
				let right_path = String::from(&encoding) + "1";
				if let Some(left) = &node.left {
					encode(hm, &left, left_path);
				}
				if let Some(right) = &node.right {
					encode(hm, &right, right_path);
				}
			}
		};
		encode(&mut hm, &node, "".to_string());
		return hm;
	}
	/// Convert huffman node to string of chars using post-order traversal
	fn to_string(huffman_node: &Node) -> String {
		let mut output = String::new();
		fn post_order(node: &Node, output_str: &mut String) {
			if let Some(left) = &node.left {
				post_order(left.as_ref(), output_str);
			}
			if let Some(right) = &node.right {
				post_order(right.as_ref(), output_str);
			}
			output_str.push(node.letter);
		}

		post_order(huffman_node, &mut output);
		return output;
	}
	/// Convert huffman tree to vector of bytes
	///
	/// First element is length of tree
	///
	/// There are only 100 or so printable characters 
	/// based on python's string.printable
	/// So worst case tree size is 2N-1 = 199
	/// So a unsigned char will suffice for length of tree
	///
	/// Following elements are charectars in post-order traversal of tree
	fn embed_tree(huffman_node: &Node) -> Vec<u8> {
		let mut compressed_data = to_string(huffman_node).into_bytes();
		compressed_data.insert(0, compressed_data.len() as u8); // Append length
		return compressed_data;
	}

	/// Simply maps input characters to their corresponding encoding and return as byte array
	///
	/// The first element is padding, (Number of zeroes appended for last encoding), as encoding might not fit into 8 bits
	fn compress_data(text: &String, huffman_node: &Node) -> Vec<u8> {
		let mut byte_stream: Vec<u8> = Vec::new();
		let (mut byte, mut count) = (0, 0);

		let huffman_map = to_hashmap(huffman_node);
		for c in text.chars() {
			let encoding = huffman_map.get(&c).unwrap();
			for e in encoding.bytes() {
				let bit: bool = (e - '0' as u8) != 0;
				byte = byte << 1 | (bit as u8);
				count = (count + 1) % 8;
				if count == 0 {
					byte_stream.push(byte);
					byte = 0;
				}
			}
		}
		if count != 0 {
			let padding: u8 = 8 - count;
			byte <<= padding;
			byte_stream.push(byte);
			byte_stream.insert(0, padding);
		} else {
			byte_stream.insert(0, 0);
		}
		return byte_stream;
	}
	/// Compression using huffman's algorithm
	/// # Data Format
	/// First byte (n): Length of post-order traversal of huffman tree
	///
	/// Following n bytes contain post-order traversal
	///
	/// Padding byte (p): Padding for final byte
	///
	/// All remaining bytes are data
	pub fn compress(text: &String) -> Vec<u8> {
		let frequency = freq_count(text.chars());
		let huffman_tree = construct_huffman_tree(frequency);
		let mut compressed_data = Vec::from(embed_tree(&huffman_tree));
		compressed_data.extend(compress_data(text, &huffman_tree));
		return compressed_data;
	}
	fn construct_tree_from_postorder(postorder: &[u8]) -> Node {
		// parent left right
		// Assuming input does not contain null
		let mut stack = Vec::new();
		for c in postorder {
			if *c == 0 as u8 {
				let (left, right) = (
					stack.pop().expect("Input contains Null byte"),
					stack.pop().expect("Input contains Null byte"),
				);
				stack.push(Node {
					letter: '\0',
					freq: 0,
					left: Option::from(Box::from(right)),
					right: Option::from(Box::from(left)),
				});
			} else {
				stack.push(Node {
					letter: *c as char,
					freq: 0,
					left: None,
					right: None,
				});
			}
		}

		return stack.pop().unwrap();
	}

	fn decompress_data(data: &[u8], tree: &Node) -> String {
		let padding = *data.first().expect("Data empty");
		let data = &data[1..]; // Remove first element which stores number of padded bits
		let mut bit_stream = Vec::new();
		let mut tmp = tree;
		let mut output = String::new();
		for character in data.iter() {
			let mut character = *character;
			for _ in 0..8 {
				let bit: bool = (character >> 7 & 1) != 0;
				character <<= 1;
				bit_stream.push(bit);
			}
		}
		bit_stream.resize(bit_stream.len() - padding as usize, false); // Remove padding bits
		if tree.left.is_none() {
			// Huffman tree is complete binary tree, a node will have either 0 or 2 children, 1 is not possible
			for _ in 0..bit_stream.len() {
				output.push(tree.letter);
			}
			return output;
		}
		for &bit in &bit_stream {
			if tmp.left.is_none() {
				output.push(tmp.letter);
				tmp = tree;
			}
			let right: &Node = tmp.right.as_ref().unwrap().as_ref();
			let left: &Node = tmp.left.as_ref().unwrap().as_ref();
			tmp = if bit { right } else { left };
		}
		if tmp != tree {
			output.push(tmp.letter);
		}
		return output;
	}
	pub fn decompress(data: &Vec<u8>) -> String {
		let post_order_length = *data.first().expect("Data cannot be empty") as usize;
		let post_order = &data[1..=post_order_length];
		let huffman_tree = construct_tree_from_postorder(post_order);
		let data = &data[post_order_length + 1..];
		decompress_data(data, &huffman_tree)
	}
}
