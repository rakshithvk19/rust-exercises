// TODO: implement a so-called "Drop bomb": a type that panics when dropped
//  unless a certain operation has been performed on it.
//  You can see the expected API in the tests below.

pub struct DropBomb {
    defuse: bool,
}

impl DropBomb {
    pub fn new() -> Self {
        Self { defuse: false }
    }

    pub fn defuse(&mut self) {
        if self.defuse == false {
            self.defuse = true;
        }
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.defuse == false {
            panic!()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn test_drop_bomb() {
        let bomb = DropBomb::new();
        // The bomb should panic when dropped
    }

    #[test]
    fn test_defused_drop_bomb() {
        let mut bomb = DropBomb::new();
        bomb.defuse();
        // The bomb should not panic when dropped
        // since it has been defused
    }
}
