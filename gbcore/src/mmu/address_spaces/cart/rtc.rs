use std::time::{Duration, SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct Rtc {
    s: u8,
    m: u8,
    h: u8,
    dl: u16,

    latched_s: u8,
    latched_m: u8,
    latched_h: u8,
    latched_dl: u8,
    latched_dh: u8,

    last_update: SystemTime,
    now: SystemTime,
    timer_halt: bool,
    day_carry: bool,

    latch_state: bool,
}

impl Rtc {
    pub fn new() -> Rtc {
        Rtc {
            s: 0,
            m: 0,
            h: 0,
            dl: 0,
            latched_s: 0,
            latched_m: 0,
            latched_h: 0,
            latched_dl: 0,
            latched_dh: 0,
            last_update: UNIX_EPOCH,
            now: UNIX_EPOCH,
            timer_halt: false,
            day_carry: false,
            latch_state: false,
        }
    }

    pub fn update_timer(&mut self) {
        if !self.timer_halt {
            let elapsed: u64 = self.now
                .duration_since(self.last_update)
                .expect("Time went backwards")
                .as_secs();
            if elapsed > 0 {
                self.inc_s(elapsed);
                self.last_update = self.now;
            }
        }
    }

    pub fn update_now(&mut self, elapsed_secs: u64) {
        self.now = UNIX_EPOCH + Duration::from_secs(elapsed_secs);
    }

    fn inc_s(&mut self, inc: u64) {
        let minutes: u64 = (inc + (self.s as u64)) / 60;
        self.s = (inc + (self.s as u64) - (minutes * 60)) as u8;
        self.inc_m(minutes);
    }

    fn inc_m(&mut self, inc: u64) {
        let hours: u64 = (inc + (self.m as u64)) / 60;
        self.m = (inc + (self.m as u64) - (hours * 60)) as u8;
        self.inc_h(hours);
    }

    fn inc_h(&mut self, inc: u64) {
        let days: u64 = (inc + (self.h as u64)) / 24;
        self.h = (inc + (self.h as u64) - (days * 24)) as u8;
        self.inc_d(days);
    }

    fn inc_d(&mut self, inc: u64) {
        self.dl += inc as u16;
        if self.dl > 0x1FF {
            self.dl %= 0x1FF;
            self.day_carry = true;
        }
    }

    pub fn update_latch_state(&mut self, value: u8) {
        if !self.latch_state && value == 0 {
            self.latch_state = true;
        } else if self.latch_state && value == 1 {
            self.latch_state = false;
            self.latch_registers();
        }
    }

    pub fn latch_registers(&mut self) {
        self.update_timer();
        self.latched_s = 0b11000000 | self.s;
        self.latched_m = 0b11000000 | self.m;
        self.latched_h = 0b11100000 | self.h;
        self.latched_dl = (self.dl & 0xFF) as u8;
        self.latched_dh = ((self.dl & 0x100) >> 8) as u8;
        self.latched_dh |= if self.timer_halt { 1 << 6 } else { 0 };
        self.latched_dh |= if self.day_carry { 1 << 7 } else { 0 };
    }

    pub fn read(&self, add: u8) -> u8 {
        match add {
            0x08 => self.latched_s,
            0x09 => self.latched_m,
            0x0A => self.latched_h,
            0x0B => self.latched_dl,
            0x0C => self.latched_dh,
            _ => 0,
        }
    }

    pub fn write(&mut self, add: u8, value: u8) {
        match add {
            0x08 => self.write_s(value),
            0x09 => self.write_m(value),
            0x0A => self.write_h(value),
            0x0B => self.write_dl(value),
            0x0C => self.write_dh(value),
            0x0D => self.update_latch_state(value),
            _ => {}
        }
    }

    fn write_s(&mut self, value: u8) {
        self.update_timer();
        let new: u8 = value & 0b00111111;
        self.s = new;
        self.latched_s = 0b11000000 | self.s;
    }

    fn write_m(&mut self, value: u8) {
        self.update_timer();
        let new: u8 = value & 0b00111111;
        self.m = new;
        self.latched_m = 0b11000000 | self.m;
    }

    fn write_h(&mut self, value: u8) {
        self.update_timer();
        let new: u8 = value & 0b00011111;
        self.h = new;
        self.latched_h = 0b11100000 | self.h;
    }

    fn write_dl(&mut self, value: u8) {
        self.update_timer();
        self.dl = value as u16;
        self.latched_dl = value;
    }

    fn write_dh(&mut self, value: u8) {
        self.update_timer();
        self.latched_dh = value & 0b11000001;
        self.timer_halt = (self.latched_dh & 0x40) != 0;
        self.day_carry = (self.latched_dh & 0x60) != 0;
        if (self.latched_dh & 0x1) != 0 {
            self.dl |= 0x100;
        } else {
            self.dl &= 0x0FF;
        }
    }

    pub fn deserialize(data: &Vec<u8>) -> Rtc {
        let dl: u16 = ((data[4] as u16) << 8) | (data[3] as u16);
        let unix_elapsed: u64 = ((data[17] as u64) << 56)
            | ((data[16] as u64) << 48)
            | ((data[15] as u64) << 40)
            | ((data[14] as u64) << 32)
            | ((data[13] as u64) << 24)
            | ((data[12] as u64) << 16)
            | ((data[11] as u64) << 8)
            | data[10] as u64;
        let last_update: SystemTime = UNIX_EPOCH + Duration::from_secs(unix_elapsed);
        let timer_halt: bool = data[18] == 1;
        let day_carry: bool = data[19] == 1;
        let latch_state: bool = data[20] == 1;

        Rtc {
            s: data[0],
            m: data[1],
            h: data[2],
            dl: dl,

            latched_s: data[5],
            latched_m: data[6],
            latched_h: data[7],
            latched_dl: data[8],
            latched_dh: data[9],
            last_update: last_update,
            now: last_update,
            timer_halt: timer_halt,
            day_carry: day_carry,
            latch_state: latch_state,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8> = vec![0; 21];
        data[0] = self.s;
        data[1] = self.m;
        data[2] = self.h;
        data[3] = (self.dl & 0x00FF) as u8;
        data[4] = ((self.dl & 0xFF00) >> 8) as u8;

        data[5] = self.latched_s;
        data[6] = self.latched_m;
        data[7] = self.latched_h;
        data[8] = self.latched_dl;
        data[9] = self.latched_dh;

        let unix_elapsed: u64 = self
            .last_update
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        data[10] = (unix_elapsed & 0x00000000000000FF) as u8;
        data[11] = ((unix_elapsed & 0x000000000000FF00) >> 8) as u8;
        data[12] = ((unix_elapsed & 0x0000000000FF0000) >> 16) as u8;
        data[13] = ((unix_elapsed & 0x00000000FF000000) >> 24) as u8;
        data[14] = ((unix_elapsed & 0x000000FF00000000) >> 32) as u8;
        data[15] = ((unix_elapsed & 0x0000FF0000000000) >> 40) as u8;
        data[16] = ((unix_elapsed & 0x00FF000000000000) >> 48) as u8;
        data[17] = ((unix_elapsed & 0xFF00000000000000) >> 56) as u8;

        data[18] = if self.timer_halt { 0x1 } else { 0x0 };

        data[19] = if self.day_carry { 0x1 } else { 0x0 };

        data[20] = if self.latch_state { 0x1 } else { 0x0 };

        data
    }
}
