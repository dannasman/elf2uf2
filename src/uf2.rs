use std::fmt;

pub struct Uf2Block {
    pub magic_start0: u32,
    pub magic_start1: u32,
    pub flags: u32,
    pub target_addr: u32,
    pub payload_size: u32,
    /*pub block_no: u32,
    pub num_blocks: u32,*/
    pub family_id: u32,
    pub data: [u8; 476],
    pub magic_end: u32
}

impl Uf2Block {
    pub fn new() -> Uf2Block {
        Uf2Block {
            magic_start0: 0,
            magic_start1: 0,
            flags: 0,
            target_addr: 0,
            payload_size: 0,
            /*block_no: 0,
            num_blocks: 0,*/
            family_id: 0,
            data: [0; 476],
            magic_end: 0       
        }
    }
}

impl fmt::Display for Uf2Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", "=".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Magic start 0", format!("{:#x}", self.magic_start0))?;
        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Magic start 1", format!("{:#x}", self.magic_start1))?;
        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Flags", format!("{:#b}", self.flags))?;
        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Target address", format!("{:#x}", self.target_addr))?;
        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Family ID", format!("{:#x}", self.family_id))?;
        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Magic end", format!("{:#x}", self.magic_end))?;
        Ok(())
    }
}

pub struct Uf2 {
    pub blocks: Vec<Uf2Block>
}

impl fmt::Display for Uf2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for block in &self.blocks {
            write!(f, "{}", block)?;
        }
        Ok(())
    }
}

impl Uf2 {
    pub fn new() -> Uf2 {
        Uf2 {
            blocks: Vec::new()
        }
    }

    pub fn write(&self, buf: &mut Vec<u8>) {
        let num_blocks: u32 = self.blocks.len() as u32;
        for (block_no, block) in self.blocks.iter().enumerate() {
            let mut i = 0;
            while i < 32 {
                buf.push(((block.magic_start0 >> i) & 0xff) as u8);
                i += 8;
            }

            i = 0;
            while i < 32 {
                buf.push(((block.magic_start1 >> i) & 0xff) as u8);
                i += 8;
            }

            i = 0;
            while i < 32 {
                buf.push(((block.flags >> i) & 0xff) as u8);
                i += 8;
            }

            i = 0;
            while i < 32 {
                buf.push(((block.target_addr >> i) & 0xff) as u8);
                i += 8;
            }

            i = 0;
            while i < 32 {
                buf.push(((block.payload_size >> i) & 0xff) as u8);
                i += 8;
            }

            i = 0;
            while i < 32 {
                buf.push(((block_no >> i) & 0xff) as u8);
                i += 8;
            }

            i = 0;
            while i < 32 {
                buf.push(((num_blocks >> i) & 0xff) as u8);
                i += 8;
            }

            i = 0;
            while i < 32 {
                buf.push(((block.family_id >> i) & 0xff) as u8);
                i += 8;
            }

            i = 0;
            while i < 476 {
                buf.push(block.data[i]);
                i += 1;
            }

            i = 0;
            while i < 32 {
                buf.push(((block.magic_end >> i) & 0xff) as u8);
                i += 8;
            }
        }
    }
}
