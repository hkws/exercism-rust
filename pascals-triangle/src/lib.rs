pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = vec![];
        if row_count > 0 {
            rows.push(vec![1]);
            // iは1-indexedな行番号であり行の長さ
            for i in 2..=row_count as usize {
                let mut row = vec![];
                // jは0-indexedな列の位置
                for j in 0..i as usize {
                    if j == 0 || j == i - 1 {
                        row.push(1);
                    } else {
                        row.push(rows[i - 2][j - 1] + rows[i - 2][j]);
                    }
                }
                println!("{:?}", row);
                rows.push(row);
            }
        };
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
