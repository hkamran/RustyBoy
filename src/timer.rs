
pub struct Timer {
    div:  u8,
    tima: u8,
    tma:  u8,
    tac:  u32,
    enabled: bool,
    divider_counter: u32,
    timer_counter: u32,
    pub interrupt: u8,
}

// https://www.coranac.com/tonc/text/timers.htm#sec-intro
impl Timer {

    pub fn new() -> Timer {
        Timer {
            div: 0,
            tima: 0,
            tma: 0,
            tac: 0,
            enabled: true,
            divider_counter: 0,
            timer_counter: 0,
            interrupt: 0,
        }
    }

    pub fn reset(&mut self) {
        self.div = 0;
        self.tima = 0;
        self.tma = 0;
        self.tac = 0;
        self.enabled = true;
        self.divider_counter = 0;
        self.timer_counter = 0;
        self.interrupt = 0;
    }

    pub fn read_byte(&self, address: u16) -> u8 {
        match address {
            0xFF04 => self.div,
            0xFF05 => self.tima,
            0xFF06 => self.tma,
            0xFF07 => {
                (if self.enabled { 0x4 } else { 0 }) |
                    (match self.tac { 16 => 1, 64 => 2, 256 => 3, _ => 0 })
            }
            _ => panic!("{:4X}", address),
        }
    }

    pub fn write_byte(&mut self, address: u16, value: u8) {
        match address {
            0xFF04 => { self.div = 0; },
            0xFF05 => { self.tima = value; },
            0xFF06 => { self.tma = value; },
            0xFF07 => {
                self.enabled = value & 0x4 != 0;
                self.tac = match value & 0x3 { 1 => 16, 2 => 64, 3 => 256, _ => 1024 };
            },
            _ => panic!("{:4X}", address),
        };
    }

    // https://www.coranac.com/tonc/text/timers.htm#sec-intro
    // https://hacktix.github.io/GBEDG/timers/
    pub fn execute_ticks(&mut self, ticks: u32) {
        self.divider_counter += ticks;

        if self.enabled {
            self.timer_counter += ticks;
        }

        self.execute_tick();
    }

    pub fn execute_tick(&mut self) {
        while self.divider_counter >= 256 {
            self.div = self.div.wrapping_add(1);
            self.divider_counter = 0;
        }

        if self.enabled {
            while self.timer_counter >= self.tac {
                self.tima = self.tima.wrapping_add(1);
                if self.tima == 0 {
                    self.tima = self.tma;
                    self.interrupt |= 0x04;
                }
                self.timer_counter = self.tac;
            }
        }
    }
}
