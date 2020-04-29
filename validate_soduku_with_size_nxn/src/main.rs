use itertools::Itertools;

struct Sudoku {
    data: Vec<Vec<u32>>,
}


impl Sudoku {
    fn is_valid_section(r: &Vec<u32>, n: usize) -> bool {
        r.iter().filter(|v| **v >= 1 && **v <= (n as u32)).unique().count() == n
    }

    fn column(&self, c: usize) -> Vec<u32> {
        self.data.iter().map(|r| r[c]).collect_vec()
    }

    fn square(&self, i: usize, side: usize) -> Vec<u32> {
        let mut res: Vec<u32> = Vec::new();
        let row = i / side;
        let col = i % side;
        for r in row*side..(row+1)*side {
            for c in col*side..(col+1)*side {
                res.push(self.data[r][c])
            }
        }
        res
    }

    fn is_valid(&self) -> bool {
        let n = self.data.len();
        let n_sqrt = (n as f32).sqrt();
        if n == 0 || n_sqrt.floor() != n_sqrt {
            return false;
        }

        if self.data.iter().any(|r| r.len() != n) {
            return false;
        }

        // Check rows
        if !self.data.iter().all(|r| Sudoku::is_valid_section(r, n)) {
            return false;
        }

        // Check columns
        if !(0..n).all(|c| Sudoku::is_valid_section(&self.column(c), n)) {
            return false;
        }

        // Check square
        let side = n_sqrt as usize;
        if !(0..n).all(|i| Sudoku::is_valid_section(&self.square(i, side), n)) {
            return false;
        }
        true
    }
}

#[test]
fn good_sudoku() {
    let good_sudoku_1 = Sudoku {
        data: vec![
            vec![7, 8, 4, 1, 5, 9, 3, 2, 6],
            vec![5, 3, 9, 6, 7, 2, 8, 4, 1],
            vec![6, 1, 2, 4, 3, 8, 7, 5, 9],
            vec![9, 2, 8, 7, 1, 5, 4, 6, 3],
            vec![3, 5, 7, 8, 4, 6, 1, 9, 2],
            vec![4, 6, 1, 9, 2, 3, 5, 8, 7],
            vec![8, 7, 6, 3, 9, 4, 2, 1, 5],
            vec![2, 4, 3, 5, 6, 1, 9, 7, 8],
            vec![1, 9, 5, 2, 8, 7, 6, 3, 4]
        ]
    };

    let good_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 4, 2, 3],
            vec![3, 2, 4, 1],
            vec![4, 1, 3, 2],
            vec![2, 3, 1, 4],
        ]
    };
    assert!(good_sudoku_1.is_valid());
    assert!(good_sudoku_2.is_valid());
}

#[test]
fn bad_sudoku() {
    let bad_sudoku_1 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
        ]
    };

    let bad_sudoku_2 = Sudoku {
        data: vec![
            vec![1, 2, 3, 4, 5],
            vec![1, 2, 3, 4],
            vec![1, 2, 3, 4],
            vec![1],
        ]
    };
    assert!(!bad_sudoku_1.is_valid());
    assert!(!bad_sudoku_2.is_valid());
}