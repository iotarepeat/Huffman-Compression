pub mod compress;
extern crate clap;
use clap::{App, Arg, ArgGroup};
use compress::huffman;
use std::fs;
fn main() {
	let matches = App::new("Simple compression using rust")
		.arg(
			Arg::with_name("compress")
				.short("c")
				.long("compress")
				.value_name("FILE")
				.takes_value(true),
		)
		.arg(
			Arg::with_name("decompress")
				.short("d")
				.long("decompress")
				.value_name("FILE")
				.takes_value(true),
		)
		.group(
			ArgGroup::with_name("action")
				.args(&["compress", "decompress"])
				.required(true),
		)
		.get_matches();

	if let Some(file) = matches.value_of("compress") {
		let error_msg = "Error reading file: ".to_string() + file;
		let data = fs::read_to_string(file).expect(&error_msg);
		let compressed_data = huffman::compress(&data);
		let output_file = file.to_string() + ".cmp";
		let error_msg = "Error writing file: ".to_string() + &output_file;
		fs::write(output_file, compressed_data).expect(&error_msg);
	}
	if let Some(file) = matches.value_of("decompress") {
		let error_msg = "Error reading file: ".to_string() + file;
		let data = fs::read(file).expect(&error_msg);
		let compressed_data = huffman::decompress(&data);
		let output_file = &file[0..file.len() - 4];
		let error_msg = "Error writing file: ".to_string() + &output_file;
		fs::write(output_file, compressed_data).expect(&error_msg);
	}
}
