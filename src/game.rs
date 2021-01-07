use std::fmt;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Cell {
	Dead = 0,
	Alive = 1,
}

pub struct Universe {
	width: u32,
	height: u32,
	cells: Vec<Cell>,
}

impl Universe {
	pub fn new(width: u32, height: u32) -> Universe {
		Universe {
			width: width,
			height: height,
			cells: (0..width * height)
				.map(|i| if i % 2 == 0 { Cell::Alive } else { Cell::Dead })
				.collect(),
		}
	}

	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
	}
}

impl fmt::Display for Universe {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		for line in self.cells.as_slice().chunks(self.width as usize) {
			for &cell in line {
				let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
				write!(f, "{}", symbol)?;
			}
			write!(f, "\n")?;
		}

		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn get_index() {
		let sut = Universe::new(4, 4);

		assert_eq!(sut.get_index(1, 2), 6);
	}
}
