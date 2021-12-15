pub mod matrix {

    pub struct Matrix<T> {
	cols: usize,
	rows: usize,
	vec: Vec<T>,
    }

    impl<T: Copy> Matrix<T> {

	pub fn from(from: Vec<Vec<T>>) -> Matrix<T> {
	    let mut vec = Vec::new();
	    let cols = from[0].len();
	    let rows = from.len();
	    for row in from {
		vec.extend(row);
	    }
	    Matrix { cols, rows, vec }
	}

	fn is_valid(&self, col: usize, row: usize) -> bool {
	    if col < self.cols &&
		row < self.rows{
		    true
		}
	    else {
		false
	    }
	}

	pub fn get(&self, col: usize, row: usize) -> Option<T> {
	    if self.is_valid(col, row)
	    {
		Some(self.vec[col + row * self.cols])
	    } else {
		None
	    }
	}

	pub fn set(&mut self, val: T, col: usize, row: usize) {
	    if self.is_valid(col, row) {
		self.vec[col + row * self.cols] = val;
	    }
	}
	
	pub fn get_rows(&self) -> usize {
	    self.rows
	}

	pub fn get_cols(&self) -> usize {
	    self.cols
	}

	pub fn get_close_adjs(&self, col: usize, row: usize) -> Option<Vec<(usize, usize)>> {
	    self.get_adjs(col, row, false)
	}

	pub fn get_all_adjs(&self, col: usize, row: usize) -> Option<Vec<(usize, usize)>> {
	    self.get_adjs(col, row, true)
	}

	fn get_adjs(&self, col: usize, row: usize, diagonals: bool) -> Option<Vec<(usize, usize)>> {
	    if self.is_valid(col, row) {
		let mut res = Vec::new();
		let mut adj_positions = vec![(col, row.wrapping_sub(1)),
				     (col + 1, row),
				     (col, row + 1),
				     (col.wrapping_sub(1), row)];
		if diagonals {
		    let mut diagonals = vec![(col.wrapping_sub(1), row.wrapping_sub(1)),
					      (col+1, row.wrapping_sub(1)),
					      (col.wrapping_sub(1), row + 1),
					      (col + 1, row + 1)];
		    adj_positions.append(&mut diagonals);
		}
		for pos in adj_positions {
		    if self.is_valid(pos.0, pos.1) { res.push(pos) };
		}
		return Some(res);
	    } else {
		return None;
	    }
	}
    }
    
    #[cfg(test)]
    mod tests {
	use super::*;

	fn get_matrix() -> Matrix<u8> {
	    let vec = vec![vec![2,1,9,9,9,4,3,2,1,0],
			   vec![3,9,8,7,8,9,4,9,2,1],
			   vec![9,8,5,6,7,8,9,8,9,2],
			   vec![8,7,6,7,8,9,6,7,8,9],
			   vec![9,8,9,9,9,6,5,6,7,8],];
	    Matrix::from(vec)
	}

	#[test]
	fn test_close_adjecency() {
	    let m = get_matrix();
	    assert_eq!(Some(vec![(1, 0) ,(0, 1)]), m.get_close_adjs(0, 0));
	    assert_eq!(Some(vec![(6, 1),
				 (7, 2),
				 (6, 3),
				 (5, 2)]), m.get_close_adjs(6, 2));
	    assert_eq!(Some(vec![(4, 3),
				 (5, 4),
				 (3, 4)]), m.get_close_adjs(4,4)); 
	    assert_eq!(None, m.get_close_adjs(3,5));
	    assert_eq!(None, m.get_close_adjs(10,3));
	}

	#[test]
	fn test_all_adjecency() {
	    let m = get_matrix();
	    assert_eq!(Some(vec![(1, 0) ,(0, 1), (1, 1)]), m.get_all_adjs(0, 0));
	    assert_eq!(Some(vec![(6, 1),
				 (7, 2),
				 (6, 3),
				 (5, 2),
				 (5, 1),
				 (7, 1),
				 (5, 3),
				 (7, 3),]), m.get_all_adjs(6, 2));
	    assert_eq!(Some(vec![(4, 3),
				 (5, 4),
				 (3, 4),
				 (3, 3),
				 (5, 3),]), m.get_all_adjs(4,4)); 
	    assert_eq!(None, m.get_all_adjs(3,5));
	    assert_eq!(None, m.get_all_adjs(10,3));
	}
    }
}
