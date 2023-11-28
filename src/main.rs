pub(crate) mod decode;
pub(crate) mod elf;
pub(crate) mod error;
pub(crate) mod instructions;
pub(crate) mod isa;
pub(crate) mod mem;
pub(crate) mod num;
pub(crate) mod ops;
pub(crate) mod registers;

fn main() {
    let mut memory = [0u8; 262140];
    let mut regs = registers::Registers::default();
    let file = std::fs::read(
        "/home/andreatedeschi/Public/tests/riscv/litmus-tests-riscv/elf-tests/basic/build/loop2-O0",
    )
    .unwrap();
    let elfdata = elf::load_elf_le(&file).unwrap();
    let mut program_counter: u32 = elfdata.ehdr.e_entry as u32;
    for sg in elfdata.segments().unwrap().iter() {
        let sg_data = elfdata.segment_data(&sg).unwrap();
        println!("{}, {}", sg.p_paddr, sg.p_memsz);
        mem::memw(sg_data, &mut memory, sg.p_paddr as usize).unwrap();
    }
    //...
    loop {
        // fetch instruction (libmem::memr(4)), increase pc of 4
        let ins = u32::from_le_bytes(mem::memr32(&memory, program_counter as usize).unwrap());
        // decode and execute instruction
        <u32 as isa::Isa>::execute(ins, &mut regs, &mut program_counter, &mut memory);
        // increment the program counter
    }
}
