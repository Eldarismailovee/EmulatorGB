pub struct Cartridge {
    rom: Vec<u8>,
}

impl Cartridge {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self { rom: bytes }
    }

    pub fn rom(&self) -> &[u8] {
        &self.rom
    }

    pub fn len(&self) -> usize {
        self.rom.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rom.is_empty()
    }
}