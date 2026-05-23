/// Took algs from there:
/// https://github.com/theonlypwner/crc32
use anyhow::{self, Result};
use clap::Parser;

mod reverse_crc;
use reverse_crc::{Crc32, Crc32Reverse};

const POLYNOMIAL: u32 = 0xedb88320;

#[derive(Parser, Debug)]
struct Cli {
    file: std::path::PathBuf,
    outfile: std::path::PathBuf,
    target_checksum: u32,
    #[clap(short, long, default_value_t = 4)]
    patch_size: usize,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut data = std::fs::read(cli.file)?;

    let crc_custom = Crc32::new(POLYNOMIAL);
    let crc_reverse = Crc32Reverse::new(POLYNOMIAL);

    let egg_index = find_subslice_index(&data, b"AAAAAAAA").expect("egg not found!");

    let before = crc_custom.calc(&data[..egg_index], 0);
    let after_hashset =
        crc_reverse.rewind(&data[egg_index + cli.patch_size..], cli.target_checksum);
    let after = after_hashset.into_iter().next().expect("no after found");

    let new_bytes = crc_reverse.find_reverse(after, before);

    println!("Found {} potential collisions", new_bytes.len());
    println!("{:x?}", &new_bytes);

    let new_bytes = new_bytes
        .into_iter()
        .find(|b| b.len() == cli.patch_size)
        .expect(&format!("No {} bytes collision found!", cli.patch_size));

    data[egg_index..egg_index + cli.patch_size].copy_from_slice(&new_bytes);

    // Validate the collision
    let after_checksum = crc_custom.calc(&data, 0);
    if after_checksum != cli.target_checksum {
        panic!("Collision found but checksum mismatch");
    }

    // for i in 0..u64::MAX {
    //     if i % 0x10000 == 0 {
    //         println!("{:x}", i);
    //     }

    //     let bytes = i.to_le_bytes();
    //     data[egg_index..egg_index + 8].copy_from_slice(&bytes);
    //     let checksum = crc.checksum(&data);
    //     if checksum == cli.target_checksum {
    //         break;
    //     }
    // }

    println!("Collision found and validated!");
    std::fs::write(&cli.outfile, &data)?;
    Ok(())
}

fn find_subslice_index<T: PartialEq>(vector: &[T], target: &[T]) -> Option<usize> {
    vector
        .windows(target.len()) // Create an iterator over windows of the target slice's length
        .position(|window| window == target) // Find the first window that matches the target slice
}
