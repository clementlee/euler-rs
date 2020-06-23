const SPIRAL_SIZE: usize = 1001;

fn is_sqrt(x: u32) -> bool {
    let y = x as f64;
    let y = y.sqrt();
    let y = y as u32;

    y * y == x
}

pub fn solution() -> usize {
    assert_eq!(SPIRAL_SIZE % 2, 1);
    let mut grid = vec![0; SPIRAL_SIZE * SPIRAL_SIZE];

    let midpoint = SPIRAL_SIZE / 2;
    let mut x = midpoint;
    let mut y = midpoint;

    let mut n = 1;
    grid[x + y * SPIRAL_SIZE] = n;
    let mut layer = 1;
    while n <= SPIRAL_SIZE * SPIRAL_SIZE {
        if is_sqrt(n as u32) && n % 2 == 1 {
            x += 1;
            layer += 1;
        } else {
            let valid_points = vec![(x, y + 1), (x - 1, y), (x, y - 1), (x + 1, y)];
            let next_points: Vec<&(usize, usize)> = valid_points
                .iter()
                .filter(|&(potx, poty)| {
                    potx >= &0 && poty >= &0 && potx < &SPIRAL_SIZE && poty < &SPIRAL_SIZE
                })
                .filter(|&(potx, poty)| grid[potx + SPIRAL_SIZE * poty] == 0)
                .filter(|(potx, poty)| {
                    let distx = *potx as i32 - midpoint as i32;
                    let disty = *poty as i32 - midpoint as i32;

                    distx.abs() < layer && disty.abs() < layer
                })
                .collect();
            let next_point = next_points[0];
            x = next_point.0;
            y = next_point.1;
        }

        n += 1;
        grid[x + y * SPIRAL_SIZE] = n;
    }

    let diag1: usize = (0..SPIRAL_SIZE).map(|x| grid[x + x * SPIRAL_SIZE]).sum();
    let diag2: usize = (0..SPIRAL_SIZE)
        .map(|x| grid[(SPIRAL_SIZE - 1 - x) + x * SPIRAL_SIZE])
        .sum();

    diag1 + diag2 - 1
}
