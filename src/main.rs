use std::fs;
use std::env;

mod elf;
mod uf2;
mod elf2uf2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let infile_name: &str = &args[1];
    let outfile_name: &str = &args[2];
    let data: Vec<u8> = fs::read(infile_name)
                        .expect("Should be a read file given as input");
    let mut buf = Vec::<u8>::new();
    let mut elf2uf2 = elf2uf2::Elf2Uf2::new();
    elf2uf2.convert(&data, &mut buf);

    fs::write(outfile_name, buf).unwrap();
}
