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
			cells: Vec::with_capacity((width * height) as usize),
		}
	}

	fn get_index(&self, row: u32, column: u32) -> usize {
		(row * self.width + column) as usize
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
