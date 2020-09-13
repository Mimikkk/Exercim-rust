fn is_safe(x: i32, y: i32, size: u32) -> bool {
    size as i32 > x && x >= 0 && size as i32 > y && y >= 0
}

fn is_empty(x: i32, y: i32, board: &Vec<Vec<u32>>) -> bool {
    board[x as usize][y as usize] == 0
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut result: Vec<Vec<u32>> = vec![vec![0 ; size as usize] ; size as usize];
    let mut x = 0;
    let mut y = -1;

    let mut value: u32 = 1;
    let directions: [(i32, i32);4] = [(0,1),(1,0),(0,-1),(-1,0)];
    for (m, n) in (0..size.pow(2) as usize).map(|i| directions[i%4]) {
        while is_safe(x+m, y+n, size) && is_empty(x+m,y+n, result.as_ref()) {
            x += m;
            y += n;
            result[x as usize][y as usize] = value;
            value += 1;
        }
    }

    result
}
