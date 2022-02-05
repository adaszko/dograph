use fixedbitset::FixedBitSet;

#[derive(Clone)]
pub struct StrictlyUpperTriangularMatrix {
    size: usize,
    matrix: FixedBitSet,
}

// Reference: https://www.intel.com/content/www/us/en/develop/documentation/onemkl-developer-reference-c/top/lapack-routines/matrix-storage-schemes-for-lapack-routines.html
fn get_index_from_row_column(i: usize, j: usize, size: usize) -> usize {
    assert!(i < size, "assertion failed: i < m; i={}, m={}", i, size);
    assert!(j < size, "assertion failed: j < m; j={}, m={}", j, size);
    assert!(i < j, "assertion failed: i < j; i={}, j={}", i, j);
    size * i + j
}

pub struct EdgesIterator<'a> {
    size: usize,
    bitset: &'a FixedBitSet,
    i: usize,
    j: usize,
}

impl<'a> EdgesIterator<'a> {
    pub fn new(size: usize, bitset: &'a FixedBitSet) -> Self {
        Self {
            size,
            bitset,
            i: 0,
            j: 1,
        }
    }
}

impl<'a> Iterator for EdgesIterator<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.i < self.size {
            while self.j < self.size {
                let index = get_index_from_row_column(self.i, self.j, self.size);
                let current_j = self.j;
                self.j += 1;
                if self.bitset[index] {
                    return Some((self.i, current_j));
                }
            }
            self.i += 1;
        }
        None
    }
}

pub struct NeighboursIterator<'a> {
    adjacency_matrix: &'a StrictlyUpperTriangularMatrix,
    left_vertex: usize,
    right_vertex: usize,
}

impl<'a> Iterator for NeighboursIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        while self.right_vertex < self.adjacency_matrix.size() {
            if self
                .adjacency_matrix
                .get(self.left_vertex, self.right_vertex)
            {
                let result = self.right_vertex;
                self.right_vertex += 1;
                return Some(result);
            }
            self.right_vertex += 1;
        }
        None
    }
}

impl StrictlyUpperTriangularMatrix {
    pub fn zeroed(size: usize) -> Self {
        // XXX: The optimal capacity is (size * size - size) / 2
        let capacity = size * size;
        Self {
            size,
            matrix: FixedBitSet::with_capacity(capacity),
        }
    }

    pub fn from_ones(size: usize, ones: &[(usize, usize)]) -> Self {
        let mut result = Self::zeroed(size);
        for (i, j) in ones {
            result.set(*i, *j, true);
        }
        result
    }

    pub fn size(&self) -> usize {
        self.size
    }

    fn index_from_row_column(&self, i: usize, j: usize) -> usize {
        get_index_from_row_column(i, j, self.size())
    }

    pub fn get(&self, i: usize, j: usize) -> bool {
        let index = self.index_from_row_column(i, j);
        self.matrix[index]
    }

    pub fn set(&mut self, i: usize, j: usize, value: bool) -> bool {
        let index = self.index_from_row_column(i, j);
        let current = self.matrix[index];
        self.matrix.set(index, value);
        current
    }

    pub fn iter_ones(&self) -> EdgesIterator {
        EdgesIterator::new(self.size, &self.matrix)
    }

    pub fn iter_neighbours(&self, u: usize) -> NeighboursIterator {
        assert!(u < self.size());
        NeighboursIterator {
            adjacency_matrix: self,
            left_vertex: u,
            right_vertex: u + 1,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_test_3x3_matrix() {
        let mut matrix = StrictlyUpperTriangularMatrix::zeroed(3);
        assert_eq!(matrix.get(0, 1), false);
        let ones: Vec<(usize, usize)> = matrix.iter_ones().collect();
        assert_eq!(ones, vec![]);

        matrix.set(0, 1, true);
        let ones: Vec<(usize, usize)> = matrix.iter_ones().collect();
        assert_eq!(ones, vec![(0, 1)]);
    }
}
