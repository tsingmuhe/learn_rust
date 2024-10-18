pub struct WrappingU32 {
    value: u32,
}

impl From<u32> for WrappingU32 {
    fn from(value: u32) -> WrappingU32 {
        WrappingU32 { value }
    }
}

fn example() {
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}