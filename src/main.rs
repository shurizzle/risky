pub(crate) mod libelf;
pub(crate) mod libmem;

fn main() {
    let mut memory = [0u8; 65535];
    let file = std::fs::read("/boot/extlinux/cat.c32").unwrap();
    let elfdata = libelf::load_elf_le(&file).unwrap();
    for sg in elfdata.segments().unwrap().iter() {
        let sg_data = elfdata.segment_data(&sg).unwrap();
        libmem::memw(sg_data, &mut memory, sg.p_paddr as usize).unwrap();
    }
    //...
    // fetch instruction (libmem::memr(4)), increase pc of 4
    // decode instruction
}
