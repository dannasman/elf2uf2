use std::fmt;

type Elf32Addr     = u32;
type Elf32Half     = u16;
type Elf32Off      = u32;
#[allow(dead_code)]
type Elf32Sword    = i32;
type Elf32Word     = u32;


fn slice_to_u16(slice: &[u8]) -> u16 {
    assert_eq!(slice.len(), 2);
    let mut val: u16 = 0;
    val |= slice[0] as u16;
    val |= (slice[1] as u16) << 8;
    val
}

fn slice_to_u32(slice: &[u8]) -> u32 {
    assert_eq!(slice.len(), 4);
    let mut val: u32 = 0;
    val |= slice[0] as u32;
    val |= (slice[1] as u32) << 8;
    val |= (slice[2] as u32) << 16;
    val |= (slice[3] as u32) << 24;
    val
}

pub struct Elf32Ehdr {
    e_ident: [u8; 16],
    e_type: Elf32Half,
    pub e_machine: Elf32Half,
    e_version: Elf32Word,
    e_entry: Elf32Addr,
    e_phoff: Elf32Off,
    e_shoff: Elf32Off,
    e_flags: Elf32Word,
    e_ehsize: Elf32Half,
    e_phentsize: Elf32Half,
    e_phnum: Elf32Half,
    e_shentsize: Elf32Half,
    e_shnum: Elf32Half,
    e_shstrndx: Elf32Half
}

impl fmt::Display for Elf32Ehdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", "-".repeat(70))?;
        let type_str = match self.e_type {
            0       => "ET_NONE",
            1       => "ET_REL",
            2       => "ET_EXEC",
            3       => "ET_DYN",
            4       => "ET_CORE",
            0xff00  => "ET_LOPROC",
            0xffff  => "ET_HIPROC",
            _       => "unknown",
        };
        write!(f, "{0: <32} | {1: <16} | {2: <16}\n", "Type", self.e_type, type_str)?;

        write!(f, "{}\n", "-".repeat(70))?;
        let machine_str = match self.e_machine {
            0       => "EM_NONE",
            1       => "EM_M32",
            2       => "EM_SPARC",
            3       => "EM_386",
            4       => "EM_68K",
            5       => "EM_88K",
            7       => "EM_860",
            8       => "EM_MIPS",
            10      => "EM_MIPS_RS4_BE",
            40      => "EM_ARM",
            243     => "EM_RISCV",
            11..=16 => "RESERVED",
            _       => "unknown",
        };
        write!(f, "{0: <32} | {1: <16} | {2: <16}\n", "Arch", self.e_machine, machine_str)?;

        write!(f, "{}\n", "-".repeat(70))?;
        let version_str = match self.e_version {
            0       => "EV_NONE",
            1       => "EV_CURRENT",
            _       => "unknown",
        };
        write!(f, "{0: <32} | {1: <16} | {2: <16}\n", "Version", self.e_version, version_str)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Entry point", format!("{:#x}", self.e_entry))?;
        
        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Program header offset", self.e_phoff)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section header offset", self.e_shoff)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "ELF header size", self.e_ehsize)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Program header table entry size", self.e_phentsize)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Program header table entry count", self.e_phnum)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section header table entry size", self.e_shentsize)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section header table entry count", self.e_shnum)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section name string table start", self.e_shstrndx)?;

        Ok(())
    }
}

impl Elf32Ehdr {
    pub fn new() -> Elf32Ehdr {
            Elf32Ehdr {
                e_ident: [0; 16],
                e_type: 0,
                e_machine: 0,
                e_version: 0,
                e_entry: 0,
                e_phoff: 0,
                e_shoff: 0,
                e_flags: 0,
                e_ehsize: 0,
                e_phentsize: 0,
                e_phnum: 0,
                e_shentsize: 0,
                e_shnum: 0,
                e_shstrndx: 0
            }
    }
}

struct Elf32Shdr {
    sh_name: Elf32Word,
    sh_type: Elf32Word,
    sh_flags: Elf32Word,
    sh_addr: Elf32Addr,
    sh_offset: Elf32Off,
    sh_size: Elf32Word,
    sh_link: Elf32Word,
    sh_info: Elf32Word,
    sh_addralign: Elf32Word,
    sh_entsize: Elf32Word
}

impl fmt::Display for Elf32Shdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section header name index", self.sh_name)?;

        write!(f, "{}\n", "-".repeat(70))?;
        let type_str = match self.sh_type {
            0                       => "SHT_NULL",
            1                       => "SHT_PROGBITS",
            2                       => "SHT_SYMTAB",
            3                       => "SHT_STRTAB",
            4                       => "SHT_RELA",
            5                       => "SHT_HASH",
            6                       => "SHT_DYNAMIC",
            7                       => "SHT_NOTE",
            8                       => "SHT_NOBITS",
            9                       => "SHT_REL",
            10                      => "SHT_SHLIB",
            11                      => "SHT_DYNSYM",
            0x70000000..=0x7fffffff => "SHT_PROC",
            0x80000000..=0xffffffff => "SHT_USER",
            _       => "unknown",
        };
        write!(f, "{0: <32} | {1: <16} | {2: <16}\n", "Section type", self.sh_type, type_str)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section flags", format!("{:#b}", self.sh_flags))?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section address", format!("{:#x}", self.sh_addr))?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section offset", self.sh_offset)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section size", self.sh_size)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section header table index link", self.sh_link)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section extra information", self.sh_info)?;


        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section address alignment", self.sh_addralign)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Section entry size", self.sh_entsize)?;

        Ok(())
    }
}

impl Elf32Shdr {
    pub fn new() -> Elf32Shdr {
        Elf32Shdr {
            sh_name: 0,
            sh_type: 0,
            sh_flags: 0,
            sh_addr: 0,
            sh_offset: 0,
            sh_size: 0,
            sh_link: 0,
            sh_info: 0,
            sh_addralign: 0,
            sh_entsize: 0
        }
    }
}

#[allow(dead_code)]
struct Elf32Sym {
    st_name: Elf32Word,
    st_value: Elf32Addr,
    st_size: Elf32Word,
    st_other: u8,
    st_shndx: Elf32Half
}

#[allow(dead_code)]
struct Elf32Rel {
    r_offset: Elf32Addr,
    r_info: Elf32Word
}

#[allow(dead_code)]
struct Elf32Rela {
    r_offset: Elf32Addr,
    r_info: Elf32Word,
    r_addend: Elf32Sword,
}

pub struct Elf32Phdr {
    pub p_type: Elf32Word,
    pub p_offset: Elf32Off,
    pub p_vaddr: Elf32Addr,
    pub p_paddr: Elf32Addr,
    pub p_filesz: Elf32Word,
    pub p_memsz: Elf32Word,
    pub p_flags: Elf32Word,
    pub p_align: Elf32Word,
}

impl fmt::Display for Elf32Phdr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", "=".repeat(70))?;
        let type_str = match self.p_type {
            0                       => "PT_NULL",
            1                       => "PT_LOAD",
            2                       => "PT_DYNAMIC",
            3                       => "PT_INTERP",
            4                       => "PT_NOTE",
            5                       => "PT_SHLIB",
            6                       => "PT_PHDR",
            0x70000000..=0x7fffffff => "PT_PROC",
            _       => "unknown",
        };
        write!(f, "{0: <32} | {1: <16} | {2: <16}\n", "Segment type", self.p_type, type_str)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Segment offset", self.p_offset)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Segment virtual address", format!("{:#x}", self.p_vaddr))?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Segment physical address", format!("{:#x}", self.p_paddr))?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Segment file image size", self.p_filesz)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Segment memory image size", self.p_memsz)?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Segment flags", format!("{:#b}", self.p_flags))?;

        write!(f, "{}\n", "-".repeat(70))?;
        write!(f, "{0: <32} | {1: <35}\n", "Segment address alignment", self.p_align)?;

        Ok(())
    }
}

impl Elf32Phdr {
    pub fn new() -> Elf32Phdr {
        Elf32Phdr {
            p_type: 0,
            p_offset: 0,
            p_vaddr: 0,
            p_paddr: 0,
            p_filesz: 0,
            p_memsz: 0,
            p_flags: 0,
            p_align: 0,
        }
    }
}

pub struct Elf32 {
    pub ehdr: Elf32Ehdr,
    pub phdrs: Vec<Elf32Phdr>,
    shdrs: Vec<Elf32Shdr>,
    str_tab: Vec<u8>,
}

impl fmt::Display for Elf32 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.ehdr)?;
        for phdr in &self.phdrs {
            write!(f, "{}", phdr)?;
        }
        for shdr in &self.shdrs {
            write!(f, "{}\n", "=".repeat(70))?;
            let mut i = shdr.sh_name as usize;
            while self.str_tab[i] != 0 {
                write!(f, "{}", self.str_tab[i] as char)?;
                i += 1;
            }
            write!(f, "\n")?;
            write!(f, "{}", shdr)?;
        }
        Ok(())
    }
}

impl Elf32 {
    pub fn new() -> Elf32 {
        Elf32 {
            ehdr: Elf32Ehdr::new(),
            phdrs: Vec::new(),
            shdrs: Vec::new(),
            str_tab: Vec::new(),
        }
    }

    fn parse_ehdr(&mut self, data: &Vec<u8>) {
        let mut i: usize = 0;
        let mut j: usize = 0;

        while j < 16 {
            self.ehdr.e_ident[j] = data[i];
            j += 1;
            i += 1;
        }

        self.ehdr.e_type = slice_to_u16(&data[i..i+2]);
        i += 2;

        self.ehdr.e_machine = slice_to_u16(&data[i..i+2]);
        i += 2;

        self.ehdr.e_version = slice_to_u32(&data[i..i+4]);
        i += 4;

        self.ehdr.e_entry = slice_to_u32(&data[i..i+4]);
        i += 4;

        self.ehdr.e_phoff = slice_to_u32(&data[i..i+4]);
        i += 4;

        self.ehdr.e_shoff = slice_to_u32(&data[i..i+4]);
        i += 4;

        self.ehdr.e_flags = slice_to_u32(&data[i..i+4]);
        i += 4;

        self.ehdr.e_ehsize = slice_to_u16(&data[i..i+2]);
        i += 2;

        self.ehdr.e_phentsize = slice_to_u16(&data[i..i+2]);
        i += 2;

        self.ehdr.e_phnum = slice_to_u16(&data[i..i+2]);
        i += 2;

        self.ehdr.e_shentsize = slice_to_u16(&data[i..i+2]);
        i += 2;

        self.ehdr.e_shnum = slice_to_u16(&data[i..i+2]);
        i += 2;

        self.ehdr.e_shstrndx = slice_to_u16(&data[i..i+2]);
    }

    fn parse_phdrs(&mut self, data: &Vec<u8>) {
        let n = self.ehdr.e_phnum;
        let size = self.ehdr.e_phentsize;

        let mut i: usize = self.ehdr.e_phoff as usize;
        let mut j = 0;
        while j < n*size {
            let mut phdr: Elf32Phdr = Elf32Phdr::new();

            phdr.p_type = slice_to_u32(&data[i..i+4]);
            i += 4;

            phdr.p_offset = slice_to_u32(&data[i..i+4]);
            i += 4;

            phdr.p_vaddr = slice_to_u32(&data[i..i+4]);
            i += 4;

            phdr.p_paddr = slice_to_u32(&data[i..i+4]);
            i += 4;

            phdr.p_filesz = slice_to_u32(&data[i..i+4]);
            i += 4;

            phdr.p_memsz = slice_to_u32(&data[i..i+4]);
            i += 4;

            phdr.p_flags = slice_to_u32(&data[i..i+4]);
            i += 4;

            phdr.p_align = slice_to_u32(&data[i..i+4]);
            i += 4;

            self.phdrs.push(phdr);
            j += size;
        }
    }

    fn parse_shdrs(&mut self, data: &Vec<u8>) {
        let n = self.ehdr.e_shnum;
        let size = self.ehdr.e_shentsize;

        let mut i: usize = self.ehdr.e_shoff as usize;
        let mut j = 0;
        while j < n*size {
            let mut shdr: Elf32Shdr = Elf32Shdr::new();

            shdr.sh_name = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_type = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_flags = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_addr = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_offset = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_size = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_link = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_info = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_addralign = slice_to_u32(&data[i..i+4]);
            i += 4;

            shdr.sh_entsize = slice_to_u32(&data[i..i+4]);
            i += 4;

            if self.ehdr.e_shstrndx as usize == self.shdrs.len() {
                self.parse_str_tab(data, shdr.sh_offset as usize, shdr.sh_size as usize);
            }

            self.shdrs.push(shdr);

            j += size;
        }
    }

    fn parse_str_tab(&mut self, data: &Vec<u8>, offset: usize, size: usize) {
        let mut i = offset;
        while i < offset + size {
            self.str_tab.push(data[i]);
            i += 1;
        }
    }

    pub fn parse_elf(&mut self, data: &Vec<u8>) {
        self.parse_ehdr(data);
        self.parse_phdrs(data);
        self.parse_shdrs(data);
    }
}
