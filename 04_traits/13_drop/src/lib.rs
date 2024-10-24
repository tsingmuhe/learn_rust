struct DropBomb {
    status: bool,
}

impl DropBomb {
    fn new() -> Self {
        Self { status: true }
    }

    fn defuse(&mut self) {
        self.status = false;
    }
}

impl Drop for DropBomb {
    fn drop(&mut self) {
        if self.status {
            panic!("Boom!");
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
