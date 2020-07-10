use std::fs::File;
use std::io::Read;

use byteorder::{ByteOrder, LittleEndian};

use vm::vm::VM;

const DEFAULT_RUNTIME_STACK_SIZE: usize = 512;

fn main() {
    let mut file = File::open("_fixtures/a.out".clone()).unwrap();

    let mut buf = [0; 4];
    let len = file.metadata().unwrap().len();
    let mut l = 0;

    let mut program: Vec<u32> = Vec::new();

    while l < len {
        file.read_exact(&mut buf).unwrap();
        let int = LittleEndian::read_u32(&buf[..]);
        program.push(int);
        l += 4;
    }

    let mut vm = VM::new(DEFAULT_RUNTIME_STACK_SIZE);
    vm.load_program(&program);
    vm.run();
}
