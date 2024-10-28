pub trait HasLife {
    fn take_damage(&mut self, damage: isize);
    fn weaken(&mut self, weaken_strength: isize);
}

pub trait CanAttack {
    fn attack(&mut self, target: &mut dyn HasLife);
}

#[derive(Debug)]
pub struct Troll {
    pub health: isize,
    pub regen: isize,
    pub attack: isize,
}

impl Troll {
    pub fn default() -> Self {
        Self {
            health: 200,
            regen: 10,
            attack: 20,
        }
    }
}

impl HasLife for Troll {
    fn take_damage(&mut self, damage: isize) {
        self.health -= damage;
    }

    fn weaken(&mut self, weaken_strength: isize) {
        self.attack -= weaken_strength;
        self.regen -= weaken_strength;
    }
}

impl CanAttack for Troll {
    fn attack(&mut self, target: &mut dyn HasLife) {
        target.take_damage(self.attack);
    }
}

pub struct Giant {
    pub health: isize,
    pub attack: isize,
}

impl HasLife for Giant {
    fn take_damage(&mut self, damage: isize) {
        self.health -= damage;
    }

    fn weaken(&mut self, weaken_strength: isize) {
        self.attack -= weaken_strength;
    }
}

impl CanAttack for Giant {
    fn attack(&mut self, target: &mut dyn HasLife) {
        target.take_damage(self.attack);
    }
}
