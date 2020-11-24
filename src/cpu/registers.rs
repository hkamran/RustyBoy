pub struct Registers {
    a: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    f: u8,
    h: u8,
    l: u8,
}

impl Registers {
    
    pub fn new() -> Self {
        return Registers {
            a: 0,
            b: 0,
            c: 0,
            d: 0,
            e: 0,
            f: 0,
            h: 0,
            l: 0
        }
    }

    pub fn get_a(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_a(&mut self, value: u8) -> () {
        self.a = value;
    }

    pub fn get_b(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_b(&mut self, value: u8) -> () {
        self.a = value;
    }

    pub fn get_c(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_c(&mut self, value: u8) -> () {
        self.a = value;
    }

    pub fn get_d(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_d(&mut self, value: u8) -> () {
        self.a = value;
    }

    pub fn get_e(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_e(&mut self, value: u8) -> () {
        self.a = value;
    }

    pub fn get_f(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_f(&mut self, value: u8) -> () {
        self.a = value;
    }

    pub fn get_h(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_h(&mut self, value: u8) -> () {
        self.a = value;
    }

    pub fn get_l(&mut self) -> u8 {
        return self.a;
    }

    pub fn set_l(&mut self, value: u8) -> () {
        self.a = value;
    }

    pub fn get_af(&mut self) -> u16 {
        return (self.a as u16) << 8
            | self.f as u16;
    }

    pub fn set_af(&mut self, value: u16) {
        self.a = ((value & 0xFF00) >> 8) as u8;
        self.f = (value & 0xFF) as u8;
    }

    pub fn get_bc(&mut self) -> u16 {
        return (self.b as u16) << 8
            | self.c as u16;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.b = ((value & 0xFF00) >> 8) as u8;
        self.c = (value & 0xFF) as u8;
    }

    pub fn get_de(&mut self) -> u16 {
        return (self.d as u16) << 8
            | self.e as u16;
    }

    pub fn set_de(&mut self, value: u16) {
        self.d = ((value & 0xFF00) >> 8) as u8;
        self.e = (value & 0xFF) as u8;
    }

    pub fn get_hl(&mut self) -> u16 {
        return (self.h as u16) << 8
            | self.l as u16;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.h = ((value & 0xFF00) >> 8) as u8;
        self.l = (value & 0xFF) as u8;
    }

    pub fn set_f_carry(&mut self, value: bool) {
        if value == true {
            self.f |= 0x10;
        } else {
            self.f &= 0xEF;
        }
    }

    pub fn set_f_substract(&mut self, value: bool) {
        if value == true {
            self.f |= 0x40;
        } else {
            self.f &= 0xBF;
        }
    }

    pub fn set_f_half_carry(&mut self, value: bool) {
        if value == true {
            self.f |= 0x20;
        } else {
            self.f &= 0xDF;
        }
    }

    pub fn set_f_zero(&mut self, value: bool) {
        if value == true {
            self.f |= 0x80;
        } else {
            self.f &= 0x7F;
        }
    }

    pub fn get_f_carry(&mut self) -> bool {
        return self.f & 0x10 > 0;
    }

    pub fn get_f_substract(&mut self) -> bool {
        return self.f & 0x40 > 0;
    }

    pub fn get_f_half_carry(&mut self) -> bool {
        return self.f & 0x20 > 0;
    }

    pub fn get_f_zero(&mut self) -> bool {
        return self.f & 0x80 > 0;
    }
}