pub const fn count_some<
    T: ~const std::cmp::PartialEq + ~const std::marker::Copy,
    const LEN: usize,
>(
    items: [Option<T>; LEN],
) -> usize {
    let mut some = 0;
    let mut count = 0;

    while count <= LEN {
        if let Some(_item) = items[count] {
            some += 1;
        }
        count += 1;
    }

    some
}

pub const fn array_contains_item<'a, T, const LEN: usize>(item: T, items: [T; LEN]) -> bool
where
    T: ~const std::cmp::PartialEq + ~const std::marker::Copy + 'a,
{
    let mut count = 0;

    while count <= LEN {
        if items[count] == item {
            return true;
        }
        count += 1;
    }

    false
}

pub const fn get_some_values<
    T: ~const std::cmp::PartialEq + ~const std::marker::Copy,
    const FINAL_LEN: usize,
    const LEN: usize,
>(
    items: [Option<T>; LEN],
    default: T,
) -> [T; FINAL_LEN] {
    let mut data: [T; FINAL_LEN] = [default; FINAL_LEN];

    let mut size = 0;
    let mut count = 0;

    while count <= LEN {
        if let Some(item) = items[count] {
            if !array_contains_item(item, data) {
                data[size] = item;
                size += 1;
            }
        }

        count += 1;
    }

    data
}

pub const fn sum<const LEN: usize>(items: [usize; LEN]) -> usize {
    let mut count = 0;

    let mut item = 0;
    while item <= LEN {
        count += items[item];
        item += 1;
    }

    count
}
