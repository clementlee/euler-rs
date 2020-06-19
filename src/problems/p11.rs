use std::cmp;

const GRID_TEXT: &str = "
08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

#[derive(Debug)]
struct Grid {
    data: Vec<u32>,
    row_length: usize,
}

const NUM: usize = 4;
impl Grid {
    fn get(&self, x: usize, y: usize) -> u32 {
        *self.data.get(x + self.row_length * y).unwrap()
    }
    fn create(grid_text: &str) -> Grid {
        let mut data = Vec::new();
        let mut row_length = None;
        for line in grid_text.trim().split("\n") {
            let mut current_row_length = 0;
            for num in line.split_ascii_whitespace() {
                current_row_length += 1;

                data.push(num.parse().unwrap());
            }

            row_length = match row_length {
                None => Some(current_row_length),
                Some(val) => {
                    assert_eq!(val, current_row_length, "unequal row lengths!");
                    Some(current_row_length)
                }
            }
        }

        Grid {
            data: data,
            row_length: row_length.unwrap(),
        }
    }

    fn get_best(&self, x: usize, y: usize) -> u32 {
        let mut best = 0;

        let can_left = x >= NUM - 1;
        let can_up = y >= NUM - 1;
        let height = self.data.len() / self.row_length;
        let can_down = y + NUM < height;
        if can_left {
            // can go left
            let mut prod = 1;
            for i in 0..NUM {
                prod *= self.get(x - i, y);
            }
            best = cmp::max(prod, best);
        }
        if can_up {
            // can go up
            let mut prod = 1;
            for i in 0..NUM {
                prod *= self.get(x, y - i);
            }
            best = cmp::max(prod, best);
        }
        if can_up && can_left {
            // can go up-left
            let mut prod = 1;
            for i in 0..NUM {
                prod *= self.get(x - i, y - i);
            }
            best = cmp::max(prod, best);
        }
        if can_down && can_left {
            // can go down-left
            let mut prod = 1;
            for i in 0..NUM {
                prod *= self.get(x - i, y + i);
            }
            best = cmp::max(prod, best);
        }

        best
    }
}

pub fn solution() -> u32 {
    let grid = Grid::create(GRID_TEXT);

    let width = grid.row_length;
    let height = grid.data.len() / width;

    let mut best = 0;
    for i in 0..width {
        for j in 0..height {
            best = cmp::max(grid.get_best(i, j), best);
        }
    }

    best
}
