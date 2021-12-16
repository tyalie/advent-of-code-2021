/// Allocate 500x500 field in the dma buffer,
/// that isn't used otherwise. This gives me
/// 500kb free memory to use and is checked at compile
/// time (I think) for errors.
///
#[link_section = ".dmabuffers"]
static mut FIELD: [[u16; 500]; 500] = [[u16::MAX; 500]; 500];

pub struct CostField {
    field: &'static mut [[u16; 500]; 500],
}


impl CostField {
    pub fn new(size: &(u16, u16)) -> Self {
        let memory: &'static mut [[u16; 500]; 500] = unsafe { &mut FIELD };
        assert!(size.0 as usize <= memory.len() && size.1 as usize <= memory[0].len());
        Self::clear_mem(memory);

        CostField {
            field: memory
        }
    }

    fn clear_mem(mem: &mut [[u16; 500]; 500]) {
        for row in mem.iter_mut() {
            for v in row.iter_mut() {
                *v = u16::MAX;
            }
        }
    }

    pub fn get(&self, x: u16, y: u16) -> &u16 {
        &self.field[y as usize][x as usize]
    }

    pub fn get_mut(&mut self, x: u16, y: u16) -> &mut u16 {
        &mut self.field[y as usize][x as usize]
    }

}
