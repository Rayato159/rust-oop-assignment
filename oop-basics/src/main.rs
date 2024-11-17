trait Weapon {
    fn attack(&self);
}

struct Sword;

impl Weapon for Sword {
    fn attack(&self) {
        println!("Sword attack");
    }
}

struct Staff;

impl Weapon for Staff {
    fn attack(&self) {
        println!("Staff attack");
    }
}

struct Warrior {
    health: u8,
    strength: u8,
    intelligent: u8,
    weapon: Box<dyn Weapon>,
}

impl Warrior {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 10,
            intelligent: 0,
            weapon: Box::new(Sword),
        }
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }

        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

struct Mage {
    health: u8,
    strength: u8,
    intelligent: u8,
    weapon: Box<dyn Weapon>,
}

impl Mage {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 0,
            intelligent: 10,
            weapon: Box::new(Staff),
        }
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }

        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

struct Healer {
    health: u8,
    strength: u8,
    intelligent: u8,
    weapon: Box<dyn Weapon>,
}

impl Healer {
    fn new() -> Self {
        Self {
            health: 100,
            strength: 5,
            intelligent: 5,
            weapon: Box::new(Staff),
        }
    }

    fn health_increase(&mut self, value: u8) {
        if self.health + value > 100 {
            self.health = 100;
            return;
        }

        self.health += value;
    }

    fn health_decrease(&mut self, value: u8) {
        self.health = self.health.saturating_sub(value);
    }
}

fn special_attack(weapon: Box<dyn Weapon>) {
    weapon.attack();
}

fn main() {
    let mut warrior = Warrior::new();
    let mut mage = Mage::new();
    let mut healer = Healer::new();

    warrior.health_decrease(10);
    mage.health_decrease(10);
    healer.health_decrease(10);

    println!("Warrior health: {}", warrior.health);
    println!("Mage health: {}", mage.health);
    println!("Healer health: {}", healer.health);

    warrior.health_increase(10);
    mage.health_increase(10);
    healer.health_increase(10);

    println!("Warrior health: {}", warrior.health);
    println!("Mage health: {}", mage.health);
    println!("Healer health: {}", healer.health);

    special_attack(warrior.weapon);
    special_attack(mage.weapon);
    special_attack(healer.weapon);
}
