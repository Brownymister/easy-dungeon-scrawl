use rand::prelude::*;

#[derive(Debug, Clone)]
pub struct Fight {
    /// all entities acting in the fight
    pub entities: Vec<crate::Entity>,
    /// order in which the entities in the fight can act
    pub iniative: Vec<isize>,
    round: isize,
}

impl Fight {
    pub fn new(entities: Vec<crate::Entity>) -> Fight {
        return Fight {
            entities,
            round: 0,
            iniative: vec![],
        };
    }

    pub fn ini(&mut self) {
        let mut inis = vec![];
        for _ in self.entities.iter() {
            let mut rng = rand::thread_rng();
            inis.push(rng.gen_range(0, 12));
        }
        self.iniative = inis;
        log::info!("{:?}", self.iniative);
    }

    pub fn start(&mut self) -> () {
        self.round += 1;
    }
}
