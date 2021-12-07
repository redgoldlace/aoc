use crate::prelude::*;
use std::ops::{Index, IndexMut};

impl<'a> Solution<'a> for Day<4, { Part::One }> {
    type Transformed = (Vec<usize>, Vec<Grid<Cell>>);
    type Result = usize;

    fn transform(input: &'a str) -> Self::Transformed {
        let mut lines = input.lines();
        let mut boards = Vec::new();

        let numbers = lines
            .next()
            .unwrap()
            .split(',')
            .map(|number| number.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        while lines.next().is_some() {
            let items = lines
                .by_ref()
                .take(5)
                .flat_map(|line| {
                    line.split_whitespace()
                        .map(|number| number.parse::<usize>().unwrap())
                })
                .map(Cell::new)
                .collect::<Vec<_>>();

            let grid = Grid::from_raw_parts(5, 5, items.into_boxed_slice());

            boards.push(grid);
        }

        (numbers, boards)
    }

    fn solve((numbers, mut boards): Self::Transformed) -> Self::Result {
        let (last_called, winning_index) = winning_board(&numbers, &mut boards)
            .expect("expected at least one winner. this is bingo. c'mon.");

        let unmarked = boards[winning_index]
            .iter()
            .filter_map(|cell| (!cell.marked).then(|| cell.number))
            .sum::<usize>();

        unmarked * last_called
    }
}

// We run into Borrowing Woes™ here, so we return a (last number, board index) pair instead of the last number
// and a reference to the board in question.
pub fn winning_board(numbers: &[usize], boards: &mut [Grid<Cell>]) -> Option<(usize, usize)> {
    for &number in numbers {
        for (index, board) in boards.iter_mut().enumerate() {
            let position = match board.mark(number) {
                Some(position) => position,
                None => continue,
            };

            if board.check(position) {
                return Some((number, index));
            }
        }
    }

    None
}

pub type Coordinate<T = usize> = (T, T);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Cell {
    pub(crate) number: usize,
    pub(crate) marked: bool,
}

impl Cell {
    pub fn new(number: usize) -> Self {
        Self {
            number,
            marked: false,
        }
    }
}

#[derive(Debug)]
pub struct Grid<T> {
    buffer: Box<[T]>,
    width: usize,
    height: usize,
}

impl Grid<Cell> {
    pub fn check(&self, (x, y): Coordinate) -> bool {
        assert_eq!(self.row(y).count(), self.height());
        assert_eq!(self.row(y).count(), self.height());

        self.row(y).all(|cell| cell.marked) || self.column(x).all(|cell| cell.marked)
    }

    /// Try and mark a number off the board, returning its position if it was marked.
    pub fn mark(&mut self, number: usize) -> Option<Coordinate> {
        self.coordinates()
            .zip(self.into_iter())
            .find_map(|(coordinate, cell)| {
                (cell.number == number).then(|| {
                    cell.marked = true;
                    coordinate
                })
            })
    }
}

impl<T> Grid<T> {
    pub fn new(width: usize, height: usize) -> Self
    where
        T: Default,
    {
        let buffer = std::iter::repeat_with(T::default)
            .take(width * height)
            .collect::<Vec<_>>()
            .into_boxed_slice();

        Self::from_raw_parts(width, height, buffer)
    }

    pub fn new_from(width: usize, height: usize, value: T) -> Self
    where
        T: Clone,
    {
        Self::from_raw_parts(
            width,
            height,
            vec![value; width * height].into_boxed_slice(),
        )
    }

    pub fn from_raw_parts(width: usize, height: usize, buffer: Box<[T]>) -> Self {
        let size = width * height;

        assert!(width * height > 0, "grids may not be zero-sized");
        assert!(
            size == buffer.len(),
            "buffer is incorrectly sized for specified width and height"
        );

        Grid {
            buffer,
            width,
            height,
        }
    }

    pub fn size(&self) -> usize {
        self.width * self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn get<I: GridIndex>(&self, index: I) -> Option<&T> {
        let index = index.as_index(self)?;

        // SAFETY: Index already checked.
        unsafe { Some(self.buffer.get_unchecked(index)) }
    }

    pub fn get_mut(&mut self, index: impl GridIndex) -> Option<&mut T> {
        let index = index.as_index(self)?;

        // SAFETY: Index already checked.
        unsafe { Some(self.buffer.get_unchecked_mut(index)) }
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        IntoIterator::into_iter(self)
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        IntoIterator::into_iter(self)
    }

    pub fn row(&self, index: usize) -> impl Iterator<Item = &T> {
        assert!(index < self.height(), "row out of bounds");

        let width = self.width();
        self.buffer[index * width..].iter().take(width)
    }

    pub fn row_mut(&mut self, index: usize) -> impl Iterator<Item = &mut T> {
        assert!(index < self.height(), "row out of bounds");

        let width = self.width();
        self.buffer[index * width..].iter_mut().take(width)
    }

    pub fn column(&self, index: usize) -> impl Iterator<Item = &T> {
        let width = self.width();
        assert!(index < width, "column out of bounds");

        self.iter().skip(index).step_by(width)
    }

    pub fn column_mut(&mut self, index: usize) -> impl Iterator<Item = &mut T> {
        let width = self.width();
        assert!(index < self.width(), "column out of bounds");

        self.iter_mut().skip(index).step_by(width)
    }

    pub fn coordinates(&self) -> impl Iterator<Item = Coordinate> {
        let width = self.width();

        (0..self.height()).flat_map(move |y| (0..width).map(move |x| (x, y)))
    }
}

impl<'a, T> IntoIterator for &'a Grid<T> {
    type Item = &'a T;

    type IntoIter = <&'a [T] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid<T> {
    type Item = &'a mut T;

    type IntoIter = <&'a mut [T] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.buffer.iter_mut()
    }
}

mod private {
    pub trait Sealed {}
}

use private::Sealed;

impl Sealed for usize {}
impl Sealed for Coordinate {}

/// A type that represents an index into a grid
pub trait GridIndex: Sealed {
    fn as_index<T>(self, grid: &Grid<T>) -> Option<usize>;
    fn as_coordinate<T>(self, grid: &Grid<T>) -> Option<Coordinate>
    where
        Self: Sized,
    {
        let index = self.as_index(grid)?;
        Some((index % grid.width(), index / grid.width()))
    }
}

impl GridIndex for usize {
    fn as_index<T>(self, grid: &Grid<T>) -> Option<usize> {
        (self < grid.size()).then(|| self)
    }
}

impl GridIndex for Coordinate {
    fn as_index<T>(self, grid: &Grid<T>) -> Option<usize> {
        let (x, y) = self;

        if x >= grid.width() || y >= grid.height() {
            None
        } else {
            (y * grid.width() + x).as_index(grid)
        }
    }

    fn as_coordinate<T>(self, grid: &Grid<T>) -> Option<Coordinate> {
        let (x, y) = self;
        (x < grid.width() && y < grid.height()).then(|| self)
    }
}

impl<I, T> Index<I> for Grid<T>
where
    I: GridIndex,
{
    type Output = T;

    fn index(&self, index: I) -> &Self::Output {
        let position = index.as_index(self).expect("grid index out of bounds");
        &self.buffer[position]
    }
}

impl<I, T> IndexMut<I> for Grid<T>
where
    I: GridIndex,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let position = index.as_index(self).expect("grid index out of bounds");
        &mut self.buffer[position]
    }
}
