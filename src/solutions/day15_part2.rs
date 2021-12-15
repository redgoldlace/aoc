aoc!(day = 15, part = 2);

use super::{day15::djikstra, day4::Grid};

#[transform]
fn transform(input: _) -> Grid<usize> {
    let view = input
        .lines()
        .flat_map(|line| {
            line.trim()
                .chars()
                .map(|number| number.to_digit(10).unwrap() as usize)
        })
        .collect::<Vec<_>>()
        .into_boxed_slice();

    let view = Grid::from_raw_parts(100, 100, view);
    let mut buffer = Grid::new_from_copy(500, 500, 0);

    for (x, y) in cartesian(5) {
        for (index_x, index_y) in cartesian(100) {
            let item = view[(index_x, index_y)];
            let position = (x * 100 + index_x, y * 100 + index_y);

            // This is C-tier code and I hate it.
            buffer[position] = (item + (x + y) - 1) % 9 + 1;
        }
    }

    buffer
}

#[solve]
fn solve(input: _) -> usize {
    djikstra(&input, (0, 0), (499, 499)).unwrap()
}

pub fn cartesian(n: usize) -> impl Iterator<Item = (usize, usize)> {
    (0..n).flat_map(move |y| (0..n).map(move |x| (x, y)))
}

impl<T> Grid<T> {
    pub fn new_from_copy(width: usize, height: usize, value: T) -> Self
    where
        T: Copy,
    {
        let size = width * height;

        assert!(size < isize::MAX as usize, "size overflows");

        let mut buffer = Vec::<T>::with_capacity(size);
        let ptr = buffer.as_mut_ptr();

        unsafe {
            for i in 0..size {
                std::ptr::write(ptr.offset(i as isize), value);
            }

            buffer.set_len(size);
        }

        Self::from_raw_parts(width, height, buffer.into_boxed_slice())
    }
}
