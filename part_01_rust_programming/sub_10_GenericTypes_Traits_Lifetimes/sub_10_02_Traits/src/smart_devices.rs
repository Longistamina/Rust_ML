// The 3 traits
pub trait Switchable {
    fn switch_on(&mut self);
    fn switch_off(&mut self);
}

pub trait Dimmable {
    fn dim(&mut self);
    fn brighten(&mut self);
}

pub trait Network {
    fn connect(&mut self);
    fn disconnect(&mut self);
}

// The 3 devices (types)

pub struct SmartBulb {
    pub switch: bool,
    pub dim: bool,
}

pub struct SmartThermostat {
    pub switch: bool,
    pub wifi: bool,
}

pub struct StandardToaster {
    pub switch: bool
}

// impl trait Switchable

impl Switchable for SmartBulb {
    fn switch_on(&mut self) {
        self.switch = true;
    }
    fn switch_off(&mut self) {
        self.switch = false;
    }
}

impl Switchable for SmartThermostat {
    fn switch_on(&mut self) {
        self.switch = true;
    }
    fn switch_off(&mut self) {
        self.switch = false;
    }
}

impl Switchable for StandardToaster {
    fn switch_on(&mut self) {
        self.switch = true;
    }
    fn switch_off(&mut self) {
        self.switch = false;
    }
}

// impl trait Dimmable

impl Dimmable for SmartBulb {
    fn dim(&mut self) {
        self.dim = true;
    }

    fn brighten(&mut self) {
        self.dim = false;
    }
}

// impl trait NetWork

impl Network for SmartThermostat {
    fn connect(&mut self) {
        self.wifi = true;
    }

    fn disconnect(&mut self) {
        self.wifi = false;
    }
}
