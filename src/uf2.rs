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
        writeln!(f, "{}", "=".repeat(70))?;
        writeln!(f, "{0: <32} | {1: <35}", "Magic start 0", format!("{:#x}", self.magic_start0))?;
        writeln!(f, "{}", "-".repeat(70))?;
        writeln!(f, "{0: <32} | {1: <35}", "Magic start 1", format!("{:#x}", self.magic_start1))?;
        writeln!(f, "{}", "-".repeat(70))?;
        writeln!(f, "{0: <32} | {1: <35}", "Flags", format!("{:#b}", self.flags))?;
        writeln!(f, "{}", "-".repeat(70))?;
        writeln!(f, "{0: <32} | {1: <35}", "Target address", format!("{:#x}", self.target_addr))?;
        writeln!(f, "{}", "-".repeat(70))?;
        writeln!(f, "{0: <32} | {1: <35}", "Family ID", format!("{:#x}", self.family_id))?;
        writeln!(f, "{}", "-".repeat(70))?;
        writeln!(f, "{0: <32} | {1: <35}", "Magic end", format!("{:#x}", self.magic_end))?;
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
        for (i, block) in self.blocks.iter().enumerate() {
            assert!(i <= u32::MAX as usize);
            let block_no = i as u32;
            buf.extend_from_slice(&block.magic_start0.to_le_bytes());
            buf.extend_from_slice(&block.magic_start1.to_le_bytes());
            buf.extend_from_slice(&block.flags.to_le_bytes());
            buf.extend_from_slice(&block.target_addr.to_le_bytes());
            buf.extend_from_slice(&block.payload_size.to_le_bytes());
            buf.extend_from_slice(&block_no.to_le_bytes());
            buf.extend_from_slice(&num_blocks.to_le_bytes());
            buf.extend_from_slice(&block.family_id.to_le_bytes());
            let mut j = 0;
            while j < 476 {
                buf.push(block.data[j]);
                j += 1;
            }
            buf.extend_from_slice(&block.magic_end.to_le_bytes());
        }
    }
}
