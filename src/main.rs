use rust_compression::compress::huffman;
fn main() {
	let input = String::from("abcd");
	let c = huffman::compress(&input);
	println!("{:?}", c);
}
