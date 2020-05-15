pub mod compress {
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
		let mut prev: char = *chars.first().unwrap();
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
	fn construct_huffman_tree(freq: Vec<Node>) -> Box<Node> {
		let mut pq = BinaryHeap::new();
		for node in freq {
			pq.push(Option::from(Box::from(node)));
		}
		while pq.len() > 1 {
			let (a, b) = (pq.pop().flatten().unwrap(), pq.pop().flatten().unwrap());
			let new_node = Node {
				letter: '\0',
				freq: a.freq + b.freq,
				left: Option::from(a),
				right: Option::from(b),
			};
			pq.push(Option::from(Box::from(new_node)));
		}
		pq.pop().flatten().unwrap()
	}
	/// Convert huffman tree to a hashmap with key as char and value as encoding
	/// E.g key = 'a', value = '1000'
	fn to_hashmap(node: &Node) -> HashMap<char, String> {
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
		let mut hm = HashMap::new();
		encode(&mut hm, &node, String::new());
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
	/// First byte (n): Length of pre-order traversal of huffman tree
	///
	/// Following n bytes contain pre-order traversal
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
}
