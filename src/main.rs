mod huffman;
fn main() {
	let input = String::from("abcd");
	let c = huffman::compress::compress(&input);
	println!("{:?}", c);
}
