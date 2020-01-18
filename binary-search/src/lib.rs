pub fn find<T>(iter: impl AsRef<[T]>, key: T) -> Option<usize>
where
    T: PartialOrd + Copy + Clone + ?Sized,
{
    let mut left = 0i32;
    let array: Vec<T> = Vec::from(iter.as_ref());
    let mut right = array.len() as i32 - 1;
    while left <= right {
        let mid = (left + right) / 2;
        let value = array[mid as usize];
        if value == key {
            return Some(mid as usize);
        } else if value < key {
            left = mid + 1;
        } else if value > key {
            right = mid - 1;
        }
    }
    None
}
