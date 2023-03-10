use crate::mmu::address_spaces::Addressable;

pub struct JoypadState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub a: bool,
    pub b: bool,
    pub start: bool,
    pub select: bool,
}

pub struct Joypad {
    state: JoypadState,
    p13: u8,
    p12: u8,
    p11: u8,
    p10: u8,
    interrupt: bool,
    direction_selected: bool,
    action_selected: bool,
}

impl Joypad {
    pub fn new() -> Joypad {
        Joypad {
            state: JoypadState {
                up: false,
                down: false,
                left: false,
                right: false,
                a: false,
                b: false,
                start: false,
                select: false,
            },
            p13: 1,
            p12: 1,
            p11: 1,
            p10: 1,
            interrupt: false,
            direction_selected: false,
            action_selected: false,
        }
    }

    pub fn set_state(&mut self, joypad_state: JoypadState) {
        self.state = joypad_state;
        self.compute_value();
    }

    pub fn purge_interrupt(&mut self) -> bool {
        let res: bool = self.interrupt;
        self.interrupt = false;
        res
    }

    fn compute_value(&mut self) {
        if (self.direction_selected && self.state.down)
            || (self.action_selected && self.state.start)
        {
            if self.p13 == 0x1 {
                self.interrupt = true;
            }
            self.p13 = 0x0;
        } else {
            self.p13 = 0x1;
        }

        if (self.direction_selected && self.state.up) || (self.action_selected && self.state.select)
        {
            if self.p12 == 0x1 {
                self.interrupt = true;
            }
            self.p12 = 0x0;
        } else {
            self.p12 = 0x1;
        }

        if (self.direction_selected && self.state.left) || (self.action_selected && self.state.b) {
            if self.p11 == 0x1 {
                self.interrupt = true;
            }
            self.p11 = 0x0;
        } else {
            self.p11 = 0x1;
        }

        if (self.direction_selected && self.state.right) || (self.action_selected && self.state.a) {
            if self.p10 == 0x1 {
                self.interrupt = true;
            }
            self.p10 = 0x0;
        } else {
            self.p10 = 0x1;
        };
    }
}

impl Addressable for Joypad {
    fn write(&mut self, location: u16, byte: u8) {
        if byte == 0x20 || byte == 0x10 {
            self.action_selected = (byte & 0x20) == 0;
            self.direction_selected = (byte & 0x10) == 0;
        } else {
            self.action_selected = false;
            self.direction_selected = false;
        }
        self.compute_value();
    }

    fn read(&self, location: u16) -> u8 {
        let p14: u8 = if self.direction_selected { 0 } else { 1 };
        let p15: u8 = if self.action_selected { 0 } else { 1 };
        (1 << 7)
            | (1 << 6)
            | (p15 << 5)
            | (p14 << 4)
            | (self.p13 << 3)
            | (self.p12 << 2)
            | (self.p11 << 1)
            | self.p10
    }
}
