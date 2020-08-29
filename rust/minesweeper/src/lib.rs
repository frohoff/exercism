use std::ops::Range;

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    fn get_range(i: usize, m: usize) -> Range<usize> {
        (if i > 0 { i - 1 } else { 0 })..(if i < m - 1 { i + 2 } else { m })
    }
    let mut res: Vec<Vec<char>> = minefield.iter().map(|&s| s.chars().collect::<Vec<_>>()).collect();
    out(&res);
    let m = res.len();
    let n = if m > 0 { res[0].len() } else { 0 };
    for i in 0..m {
        for j in 0..n {
            println!("[{}, {}] = '{}' ", i, j, res[i][j]);
            if res[i][j] == '*' {
                println!("{:?} {:?}", get_range(i, m), get_range(j, n));
                for ii in get_range(i, m) {
                    for jj in get_range(j, n) {
                        let c = res[ii][jj];
                        // println!("  [{}, {}] = '{}' ", ii, jj, c);
                        res[ii][jj] = match (c, c.to_digit(10)) {
                            (' ', _) => '1',
                            (_, Some(num)) => (num + 1).to_string().chars().nth(0).unwrap(),
                            _ => c
                        };
                        // println!(" >[{}, {}] = '{}' ", ii, jj, res[ii][jj]);                        
                    }
                }
            }
        }
    }
    out(&res);
    res.iter().map(|v| v.iter().collect::<String>()).collect::<Vec<_>>()
}

fn out(v: &Vec<Vec<char>>) {
    let m = v.len();
    let n = if m > 0 { v[0].len() } else { 0 };
    for i in 0..m {
        print!("|");
        for j in 0..n {
            print!("{}", v[i][j]);
        }
        println!("|");
    }
}