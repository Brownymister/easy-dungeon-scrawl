#[derive(Debug)]
pub struct Fight {
    pub enteties: Vec<crate::Entity>,
}

impl Fight {
    pub fn new(entity: crate::Entity) -> Fight {
        return Fight {
            enteties: vec![entity],
        };
    }
}
