use rust_compression::compress::huffman;
fn main() {
	let input = String::from("2150");
	let c = huffman::compress(&input);
	let d = huffman::decompress(&c);
	println!("{}", d);
}
