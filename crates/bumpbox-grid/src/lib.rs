#![cfg_attr(not(any(test, feature = "std")), no_std)]

use bumpbox_core::{Aabb, Aabb3, Fx32, Vec2, Vec3};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GridError {
    InvalidLayout,
    InvalidCellSize,
    OutOfBounds,
    CellOverflow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Cell<const MAX_PER_CELL: usize> {
    items: [Option<u32>; MAX_PER_CELL],
}

impl<const MAX_PER_CELL: usize> Cell<MAX_PER_CELL> {
    fn empty() -> Self {
        Self { items: core::array::from_fn(|_| None) }
    }

    fn clear(&mut self) {
        let mut index = 0usize;
        while index < MAX_PER_CELL {
            self.items[index] = None;
            index += 1;
        }
    }

    fn push(&mut self, item_id: u32) -> Result<(), GridError> {
        let mut index = 0usize;
        while index < MAX_PER_CELL {
            if self.items[index] == Some(item_id) {
                return Ok(());
            }
            if self.items[index].is_none() {
                self.items[index] = Some(item_id);
                return Ok(());
            }
            index += 1;
        }
        Err(GridError::CellOverflow)
    }
}

pub struct UniformGrid<const CELL_COUNT: usize, const MAX_PER_CELL: usize> {
    origin: Vec2,
    cell_size: Fx32,
    width: usize,
    height: usize,
    cells: [Cell<MAX_PER_CELL>; CELL_COUNT],
}

pub struct UniformGrid3<const CELL_COUNT: usize, const MAX_PER_CELL: usize> {
    origin: Vec3,
    cell_size: Fx32,
    width: usize,
    height: usize,
    depth: usize,
    cells: [Cell<MAX_PER_CELL>; CELL_COUNT],
}

impl<const CELL_COUNT: usize, const MAX_PER_CELL: usize> UniformGrid<CELL_COUNT, MAX_PER_CELL> {
    pub fn new(
        width: usize,
        height: usize,
        origin: Vec2,
        cell_size: Fx32,
    ) -> Result<Self, GridError> {
        if width.checked_mul(height) != Some(CELL_COUNT) {
            return Err(GridError::InvalidLayout);
        }
        if cell_size <= Fx32::ZERO {
            return Err(GridError::InvalidCellSize);
        }
        Ok(Self {
            origin,
            cell_size,
            width,
            height,
            cells: core::array::from_fn(|_| Cell::empty()),
        })
    }

    pub fn clear(&mut self) {
        let mut index = 0usize;
        while index < CELL_COUNT {
            self.cells[index].clear();
            index += 1;
        }
    }

    pub fn insert(&mut self, item_id: u32, bounds: &Aabb) -> Result<(), GridError> {
        let (x0, x1) = self.x_range(bounds)?;
        let (y0, y1) = self.y_range(bounds)?;

        let mut y = y0;
        while y <= y1 {
            let mut x = x0;
            while x <= x1 {
                let cell_index = self.cell_index(x as usize, y as usize);
                self.cells[cell_index].push(item_id)?;
                x += 1;
            }
            y += 1;
        }

        Ok(())
    }

    pub fn query_aabb(&self, bounds: &Aabb, out: &mut [u32]) -> Result<usize, GridError> {
        let (x0, x1) = self.x_range(bounds)?;
        let (y0, y1) = self.y_range(bounds)?;

        let mut len = 0usize;
        let mut y = y0;
        while y <= y1 {
            let mut x = x0;
            while x <= x1 {
                let cell = &self.cells[self.cell_index(x as usize, y as usize)];
                let mut item_index = 0usize;
                while item_index < MAX_PER_CELL {
                    if let Some(item_id) = cell.items[item_index] {
                        if !contains(&out[..len], item_id) && len < out.len() {
                            out[len] = item_id;
                            len += 1;
                        }
                    }
                    item_index += 1;
                }
                x += 1;
            }
            y += 1;
        }

        insertion_sort(&mut out[..len]);
        Ok(len)
    }

    fn x_range(&self, bounds: &Aabb) -> Result<(i32, i32), GridError> {
        let min = self.cell_coord(bounds.min.x, self.origin.x, self.width)?;
        let max = self.cell_coord(
            max_exclusive_to_inclusive(bounds.min.x, bounds.max.x),
            self.origin.x,
            self.width,
        )?;
        Ok((min, max))
    }

    fn y_range(&self, bounds: &Aabb) -> Result<(i32, i32), GridError> {
        let min = self.cell_coord(bounds.min.y, self.origin.y, self.height)?;
        let max = self.cell_coord(
            max_exclusive_to_inclusive(bounds.min.y, bounds.max.y),
            self.origin.y,
            self.height,
        )?;
        Ok((min, max))
    }

    fn cell_coord(
        &self,
        value: Fx32,
        origin_axis: Fx32,
        axis_extent: usize,
    ) -> Result<i32, GridError> {
        let shifted = value - origin_axis;
        let coord = floor_div_i32(shifted.raw(), self.cell_size.raw());
        if coord < 0 || coord >= axis_extent as i32 {
            return Err(GridError::OutOfBounds);
        }
        Ok(coord)
    }

    fn cell_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

impl<const CELL_COUNT: usize, const MAX_PER_CELL: usize> UniformGrid3<CELL_COUNT, MAX_PER_CELL> {
    pub fn new(
        width: usize,
        height: usize,
        depth: usize,
        origin: Vec3,
        cell_size: Fx32,
    ) -> Result<Self, GridError> {
        if width.checked_mul(height).and_then(|area| area.checked_mul(depth)) != Some(CELL_COUNT) {
            return Err(GridError::InvalidLayout);
        }
        if cell_size <= Fx32::ZERO {
            return Err(GridError::InvalidCellSize);
        }
        Ok(Self {
            origin,
            cell_size,
            width,
            height,
            depth,
            cells: core::array::from_fn(|_| Cell::empty()),
        })
    }

    pub fn clear(&mut self) {
        let mut index = 0usize;
        while index < CELL_COUNT {
            self.cells[index].clear();
            index += 1;
        }
    }

    pub fn insert(&mut self, item_id: u32, bounds: &Aabb3) -> Result<(), GridError> {
        let (x0, x1) = self.x_range(bounds)?;
        let (y0, y1) = self.y_range(bounds)?;
        let (z0, z1) = self.z_range(bounds)?;

        let mut z = z0;
        while z <= z1 {
            let mut y = y0;
            while y <= y1 {
                let mut x = x0;
                while x <= x1 {
                    let cell_index = self.cell_index(x as usize, y as usize, z as usize);
                    self.cells[cell_index].push(item_id)?;
                    x += 1;
                }
                y += 1;
            }
            z += 1;
        }

        Ok(())
    }

    pub fn query_aabb(&self, bounds: &Aabb3, out: &mut [u32]) -> Result<usize, GridError> {
        let (x0, x1) = self.x_range(bounds)?;
        let (y0, y1) = self.y_range(bounds)?;
        let (z0, z1) = self.z_range(bounds)?;

        let mut len = 0usize;
        let mut z = z0;
        while z <= z1 {
            let mut y = y0;
            while y <= y1 {
                let mut x = x0;
                while x <= x1 {
                    let cell = &self.cells[self.cell_index(x as usize, y as usize, z as usize)];
                    let mut item_index = 0usize;
                    while item_index < MAX_PER_CELL {
                        if let Some(item_id) = cell.items[item_index] {
                            if !contains(&out[..len], item_id) && len < out.len() {
                                out[len] = item_id;
                                len += 1;
                            }
                        }
                        item_index += 1;
                    }
                    x += 1;
                }
                y += 1;
            }
            z += 1;
        }

        insertion_sort(&mut out[..len]);
        Ok(len)
    }

    fn x_range(&self, bounds: &Aabb3) -> Result<(i32, i32), GridError> {
        let min = self.cell_coord(bounds.min.x, self.origin.x, self.width)?;
        let max = self.cell_coord(
            max_exclusive_to_inclusive(bounds.min.x, bounds.max.x),
            self.origin.x,
            self.width,
        )?;
        Ok((min, max))
    }

    fn y_range(&self, bounds: &Aabb3) -> Result<(i32, i32), GridError> {
        let min = self.cell_coord(bounds.min.y, self.origin.y, self.height)?;
        let max = self.cell_coord(
            max_exclusive_to_inclusive(bounds.min.y, bounds.max.y),
            self.origin.y,
            self.height,
        )?;
        Ok((min, max))
    }

    fn z_range(&self, bounds: &Aabb3) -> Result<(i32, i32), GridError> {
        let min = self.cell_coord(bounds.min.z, self.origin.z, self.depth)?;
        let max = self.cell_coord(
            max_exclusive_to_inclusive(bounds.min.z, bounds.max.z),
            self.origin.z,
            self.depth,
        )?;
        Ok((min, max))
    }

    fn cell_coord(
        &self,
        value: Fx32,
        origin_axis: Fx32,
        axis_extent: usize,
    ) -> Result<i32, GridError> {
        let shifted = value - origin_axis;
        let coord = floor_div_i32(shifted.raw(), self.cell_size.raw());
        if coord < 0 || coord >= axis_extent as i32 {
            return Err(GridError::OutOfBounds);
        }
        Ok(coord)
    }

    fn cell_index(&self, x: usize, y: usize, z: usize) -> usize {
        z * (self.width * self.height) + y * self.width + x
    }
}

fn max_exclusive_to_inclusive(min: Fx32, max: Fx32) -> Fx32 {
    if max > min {
        Fx32::from_raw(max.raw().saturating_sub(1))
    } else {
        min
    }
}

fn floor_div_i32(lhs: i32, rhs: i32) -> i32 {
    let quotient = lhs / rhs;
    let remainder = lhs % rhs;
    if remainder != 0 && ((remainder > 0) != (rhs > 0)) {
        quotient - 1
    } else {
        quotient
    }
}

fn contains(slice: &[u32], value: u32) -> bool {
    let mut index = 0usize;
    while index < slice.len() {
        if slice[index] == value {
            return true;
        }
        index += 1;
    }
    false
}

fn insertion_sort(slice: &mut [u32]) {
    let mut index = 1usize;
    while index < slice.len() {
        let value = slice[index];
        let mut inner = index;
        while inner > 0 && slice[inner - 1] > value {
            slice[inner] = slice[inner - 1];
            inner -= 1;
        }
        slice[inner] = value;
        index += 1;
    }
}
