pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle{ row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        for i in 0..self.row_count as usize {
            rows.push((0..=i).map(|j|
                if j == 0 || j == i {1} else {rows[i-1][j-1]+rows[i-1][j]}).collect());
        }
        rows
    }
}
