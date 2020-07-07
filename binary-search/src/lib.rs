use std::cmp::PartialOrd;

pub fn find<T, U>(array: U, key: T) -> Option<usize>
where
    T: PartialOrd,
    U: AsRef<[T]>,
{
    let array = array.as_ref();
    if array.is_empty() {
        return None;
    }
    let mut min = 0 as usize;
    let mut max = array.len() - 1;
    let mut pointer;
    while min <= max {
        pointer = min + (max - min) / 2;
        if array[pointer] < key {
            min = pointer + 1;
        } else if array[pointer] > key {
            if pointer == 0 {
                break;
            }
            max = pointer - 1;
        } else {
            return Some(pointer);
        }
    }

    None
}
