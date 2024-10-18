pub fn min<T: PartialEq + PartialOrd>(left: T, right: T) -> T
{
    if left <= right {
        left
    } else {
        right
    }
}