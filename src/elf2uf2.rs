use crate::elf::*;
use crate::uf2::*;

const HAS_FAMILY_ID: u32    = 0x00002000;
const MAGIC_START0: u32     = 0x0a324655;
const MAGIC_START1: u32     = 0x9e5d5157;
const MAGIC_END: u32        = 0x0ab16f30;
const PAYLOAD_SIZE: u32     = 256;

pub struct Elf2Uf2 {
    elf: Elf32,
    pub uf2: Uf2
}

impl Elf2Uf2 {
    pub fn new() -> Elf2Uf2 {
        Elf2Uf2 {
            elf: Elf32::new(),
            uf2: Uf2::new()
        }
    }

    pub fn convert(&mut self, data: &Vec<u8>, buf: &mut Vec<u8>) {
        self.elf.parse_elf(data);

        let family_id: u32 = match self.elf.ehdr.e_machine {
            40      => 0xe48bff59, /* ARM */
            243     => 0xe48bff5a, /* RISCV */
            _       => 0xe48bff58, /* generic catch-all data */
        };

        for phdr in &self.elf.phdrs {
            if phdr.p_type != 1 {
                continue;
            }

            let file_size: u32 = u32::min(phdr.p_memsz, phdr.p_filesz);
            if file_size == 0 {
                continue;
            }

            let n: usize = ((file_size + PAYLOAD_SIZE - 1) / PAYLOAD_SIZE) as usize;
            let i: usize = phdr.p_offset as usize;
            let mut j: usize = 0;
            let mut start_addr: u32 = phdr.p_paddr & !0xff;
            let mut k: usize = (phdr.p_paddr - start_addr) as usize;
            while j < n {
                let mut block = Uf2Block::new();
                block.magic_start0 = MAGIC_START0;
                block.magic_start1 = MAGIC_START1;
                block.flags |= HAS_FAMILY_ID;
                block.target_addr = start_addr;
                block.payload_size = PAYLOAD_SIZE;
                block.family_id = family_id;
                while k < PAYLOAD_SIZE as usize {
                    if j*(PAYLOAD_SIZE as usize)+k < (file_size as usize) {
                        block.data[k] = data[i+j*(PAYLOAD_SIZE as usize)+k];
                    }
                    k += 1;
                }
                block.magic_end = MAGIC_END;
                start_addr += PAYLOAD_SIZE;
                j += 1;
                k = 0;
                self.uf2.blocks.push(block);
            }
        }

        self.uf2.write(buf);
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::elf2uf2::Elf2Uf2;

    #[test]
    fn test_elf2uf2() {
        let steps: Vec<(&str, &str)> = vec![
            ("test/test_arm.elf", "test/correct_arm.uf2"),
            ("test/test_riscv.elf", "test/correct_riscv.uf2"),
        ];

        for (elf, uf2) in steps {
            let data: Vec<u8> = fs::read(elf)
                            .expect("Should be a elf file given as input");
            let correct: Vec<u8> = fs::read(uf2)
                            .expect("Should be a uf2 file given as input");
            let mut buf = Vec::<u8>::new();
            let mut elf2uf2 = Elf2Uf2::new();
            elf2uf2.convert(&data, &mut buf);

            assert_eq!(buf.len(), correct.len());
            for (b1, b2) in buf.iter().zip(correct.iter()) {
                assert_eq!(b1, b2);
            }
        }
    }
}
