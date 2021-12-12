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

    pub fn get(&self, col: usize, row: usize) -> Option<T> {
	if col < self.cols &&
	    row < self.rows
	{
	    Some(self.vec[col + row * self.cols])
	} else {
	    None
	}
    }

    // for skipping checks we use the checks found in the get method
    // we assume that the matrix is smaller then usize
    // out of bound requests return empty vector
    fn get_adjs(&self, col: usize, row: usize) -> Vec<T> {
	let mut res = Vec::new();
	if let None = self.get(col, row) {
	    return res;
	}
	let adj_positions = [(col, row.wrapping_sub(1)),
			     (col + 1, row),
			     (col, row + 1),
			     (col.wrapping_sub(1), row)];
	for pos in adj_positions {
	    if let Some(t) = self.get(pos.0, pos.1) {
		res.push(t)
	    }
	}
	res
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
    fn test_adjecency() {
	let m = get_matrix();
	assert_eq!(vec![1,3], m.get_adjs(0, 0));
	assert_eq!(vec![4,8,6,8], m.get_adjs(6, 2));
	assert_eq!(vec![8,6,9], m.get_adjs(4,4)); 
	assert_eq!(Vec::<u8>::new(), m.get_adjs(3,5));
	assert_eq!(Vec::<u8>::new(), m.get_adjs(10,3));
    }
}
