struct Order {
    price: u32,
    quantity: u32,
}

impl Order {
    fn is_available(&self) -> bool {
        if self.price > 0 && self.quantity > 0 {
            true
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Order;

    #[test]
    fn test_is_available() {
        let order = Order {
            price: 100,
            quantity: 10,
        };

        assert!(order.is_available());
    }

    #[test]
    fn test_is_not_available() {
        let order = Order {
            price: 100,
            quantity: 0,
        };

        assert!(!order.is_available());
    }
}
