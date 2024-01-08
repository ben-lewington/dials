pub struct MyFlags(u8);

impl MyFlags {
    pub const FLAG_0: u8 = 0b00000001;
    pub const FLAG_1: u8 = 0b00000010;
    pub const FLAG_2: u8 = 0b00000100;
    pub const FLAG_3: u8 = 0b00001000;
    pub const FLAG_4: u8 = 0b01110000;
    pub const FLAG_5: u8 = 0b10000000;
    pub const FLAG_4_START: u8 = 4;
    pub const FLAG_4_SIZE: u8 = 3;

    pub fn flag_0(&self) -> bool {
        self.0 & 1 == 1
    }

    pub fn flag_1(&self) -> bool {
        (self.0 >> 1) & 1 == 1
    }

    pub fn flag_2(&self) -> bool {
        (self.0 >> 2) & 1 == 1
    }

    pub fn flag_3(&self) -> bool {
        (self.0 >> 3) & 1 == 1
    }

    pub fn flag_4(&self) -> u8 {
        // self.0      0b|xxx||||
        // self.0 >> 4 0b0000|xxx
        // 255 >> 5    0b00000111
        //          |= 0b00000xxx
        (self.0 >> Self::FLAG_4_START) | (255 >> (8 - Self::FLAG_4_SIZE))
    }

    pub fn flag_5(&self) -> bool {
        (self.0 >> 7) & 1 == 1
    }

    pub fn set_flag_0(&mut self) {
        // self.0 0bzyxwvuts
        // FLAG_0 0b00000001
        //      | 0bzyxwvut1
        self.0 |= Self::FLAG_0;
    }

    pub fn unset_flag_0(&mut self) {
        self.0 &= Self::FLAG_0;
    }

    pub fn toggle_flag_0(&mut self) {
        self.0 ^= Self::FLAG_0;
    }

    pub fn set_flag_4(&mut self, mut value: u8) {
        if value >= 2_u8.pow(Self::FLAG_4_SIZE as u32) {
            value = value % 2_u8.pow(Self::FLAG_4_SIZE as u32);
        }
        // value 0b00000xxx
        // mask  0b10001111
        let mask = Self::FLAG_0 | Self::FLAG_1 | Self::FLAG_2 | Self::FLAG_3 | Self::FLAG_5;

        (*self).0 = (self.0 & mask) | (value << Self::FLAG_4_START)
    }
}

