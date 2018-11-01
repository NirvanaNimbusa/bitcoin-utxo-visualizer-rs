use self::memmap::Mmap;
use std::env;
use std::fs::File;

extern crate memmap;

mod blk;

struct CheckSequential {
    last_block: u32,
    num_changes: u32,
}

impl CheckSequential {
    fn new() -> CheckSequential {
        return CheckSequential {
            last_block: 0_u32.wrapping_sub(1),
            num_changes: 0,
        };
    }

    fn print(&self) {
        println!(
            "last block: {}, num changes: {}",
            self.last_block, self.num_changes
        );
    }
}

impl blk::BlockCallback for CheckSequential {
    fn begin_block(&mut self, block_height: u32) {
        if block_height != self.last_block.wrapping_add(1) {
            println!(
                "begin_block: expected block {} but got {}",
                self.last_block + 1,
                block_height
            );
        }
        self.last_block = block_height;
    }

    fn change(&mut self, _block_height: u32, _amount: i64, _is_same_as_previous_change: bool) {
        self.num_changes += 1;
    }
    fn end_block(&mut self, block_height: u32) {
        if block_height != self.last_block {
            println!(
                "begin_block: expected block {} but got {}",
                self.last_block, block_height
            );
        }
    }
}

fn main() -> std::io::Result<()> {
    // get filename as first argument
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    // process each byte
    let mut callback = CheckSequential::new();

    // mmap file into an u8 array
    let data = File::open(filename)?;
    let data = unsafe { Mmap::map(&data)? };
    let data = data.as_ref();

    match blk::parse(&mut data.iter(), &mut callback) {
        None => {
            println!("Something bad happened");
        }
        Some(()) => {
            println!("parsing successfull!");
        }
    }

    callback.print();
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::prelude::*;
    use std::io::BufReader;

    #[test]
    fn read_with_bufreader() -> std::io::Result<()> {
        let f = File::open("data/all.blk")?;
        let f = BufReader::new(f);

        let mut hash: u32 = 0;
        for b in f.bytes() {
            hash = hash.wrapping_add(b.unwrap() as u32);
        }

        assert_eq!(2127919936, hash);

        Ok(())
    }
}
