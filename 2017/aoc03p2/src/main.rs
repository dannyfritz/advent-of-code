use std::collections::HashMap;

fn main() {
    // println!("{}", u128::max_value());
    let mut cache = HashMap::new();
    let mut x = 0;
    loop {
        let result = euclidean_spiral_neighbor_sum(x, &mut cache);
        println!("{} -> {}", x, result);
        x += 1;
        if result > 361527 {
            break;
        }
    }
}

fn euclidean_spiral(x: i64, y: i64) -> i64 {
    if x == 0 && y == 0 {
        return 1
    }
    let max = x.abs().max(y.abs());
    let sqrt = max * 2 + 1;
    let half_sqrt = sqrt / 2;
    let mut ans = sqrt.pow(2);
    if -y == max {
        ans -= half_sqrt - x;
    } else if x == max {
        ans -= (sqrt - 1) * 3;
        ans -= half_sqrt - y;
    } else if y == max {
        ans -= (sqrt - 1) * 2;
        ans -= half_sqrt + x;
    } else if -x == max {
        ans -= sqrt - 1;
        ans -= half_sqrt + y;
    } 
    return ans
}

fn euclidean_coords(value: i64) -> (i64, i64) {
    let mut x = 0;
    let mut y = 0;
    if value == 1 {
        return (x, y)
    }
    let mut sqrt = ((value) as f64).sqrt().ceil() as i64;
    sqrt = if sqrt % 2 == 0 { sqrt + 1 } else { sqrt };
    let half_sqrt = (sqrt / 2) as i64;
    let cursor = sqrt.pow(2);
    if cursor - (sqrt - 1) * 1 < value {
        y = -half_sqrt;
        x = value - (cursor - (sqrt - 1) * 1) - half_sqrt;
    } else if cursor - (sqrt - 1) * 2 < value {
        x = -half_sqrt;
        y = (cursor - (sqrt - 1) * 2) + half_sqrt - value;
    } else if cursor - (sqrt - 1) * 3 < value {
        y = half_sqrt;
        x = (cursor - (sqrt - 1) * 3) + half_sqrt - value;
    } else if cursor - (sqrt - 1) * 4 < value {
        x = half_sqrt;
        y = value - (cursor - (sqrt - 1) * 4) - half_sqrt;
    }
    return (x, y)
}

fn euclidean_spiral_neighbor_sum(value: i64, cache: &mut HashMap<i64, u64>) -> u64 {
    if cache.contains_key(&value) {
        match cache.get(&value) {
            Some(&sum) => { return sum }
            None => {}
        }
    }
    if value == 1 {
        cache.insert(1, 1);
        return 1
    }
    let (x, y) = euclidean_coords(value);
    let mut ans = 0;
    for x2 in x-1..x+2 {
        for y2 in y-1..y+2 {
            if euclidean_spiral(x2, y2) < value {
                ans += euclidean_spiral_neighbor_sum(euclidean_spiral(x2, y2), cache);
            }
        }
    }
    cache.insert(value, ans);
    return ans;
}

#[test]
fn euclidean_coords_test() {
    assert_eq!(euclidean_coords(1), (0, 0));
    assert_eq!(euclidean_coords(2), (1, 0));
    assert_eq!(euclidean_coords(3), (1, 1));
    assert_eq!(euclidean_coords(4), (0, 1));
    assert_eq!(euclidean_coords(5), (-1, 1));
    assert_eq!(euclidean_coords(6), (-1, 0));
    assert_eq!(euclidean_coords(7), (-1, -1));
    assert_eq!(euclidean_coords(8), (0, -1));
    assert_eq!(euclidean_coords(9), (1, -1));
    assert_eq!(euclidean_coords(10), (2, -1));
    assert_eq!(euclidean_coords(11), (2, 0));
    assert_eq!(euclidean_coords(12), (2, 1));
    assert_eq!(euclidean_coords(13), (2, 2));
    assert_eq!(euclidean_coords(14), (1, 2));
    assert_eq!(euclidean_coords(15), (0, 2));
    assert_eq!(euclidean_coords(16), (-1, 2));
    assert_eq!(euclidean_coords(17), (-2, 2));
    assert_eq!(euclidean_coords(18), (-2, 1));
    assert_eq!(euclidean_coords(19), (-2, 0));
    assert_eq!(euclidean_coords(20), (-2, -1));
    assert_eq!(euclidean_coords(21), (-2, -2));
    assert_eq!(euclidean_coords(22), (-1, -2));
    assert_eq!(euclidean_coords(23), (0, -2));
    assert_eq!(euclidean_coords(24), (1, -2));
    assert_eq!(euclidean_coords(25), (2, -2));
    assert_eq!(euclidean_coords(26), (3, -2));
    assert_eq!(euclidean_coords(27), (3, -1));
}

#[test]
fn euclidean_spiral_test_1() {
    assert_eq!(euclidean_spiral(0, 0), 1);
}
#[test]
fn euclidean_spiral_test_2() {
    assert_eq!(euclidean_spiral( 1, 0), 2);
    assert_eq!(euclidean_spiral( 1, 1), 3);
    assert_eq!(euclidean_spiral( 0, 1), 4);
    assert_eq!(euclidean_spiral(-1, 1), 5);
    assert_eq!(euclidean_spiral(-1, 0), 6);
    assert_eq!(euclidean_spiral(-1,-1), 7);
    assert_eq!(euclidean_spiral( 0,-1), 8);
    assert_eq!(euclidean_spiral( 1,-1), 9);
}
#[test]
fn euclidean_spiral_test_3() {
    assert_eq!(euclidean_spiral( 2,-1), 10);
    assert_eq!(euclidean_spiral( 2, 0), 11);
    assert_eq!(euclidean_spiral( 2, 1), 12);
    assert_eq!(euclidean_spiral( 2, 2), 13);
    assert_eq!(euclidean_spiral( 1, 2), 14);
    assert_eq!(euclidean_spiral( 0, 2), 15);
    assert_eq!(euclidean_spiral(-1, 2), 16);
    assert_eq!(euclidean_spiral(-2, 2), 17);
    assert_eq!(euclidean_spiral(-2, 1), 18);
    assert_eq!(euclidean_spiral(-2, 0), 19);
    assert_eq!(euclidean_spiral(-2,-1), 20);
    assert_eq!(euclidean_spiral(-2,-2), 21);
    assert_eq!(euclidean_spiral(-1,-2), 22);
    assert_eq!(euclidean_spiral( 0,-2), 23);
    assert_eq!(euclidean_spiral( 1,-2), 24);
    assert_eq!(euclidean_spiral( 2,-2), 25);
}
#[test]
fn euclidean_spiral_4() {
    assert_eq!(euclidean_spiral(3, -2), 26);
}
#[test]
fn euclidean_spiral_neighbor_sum_test() {
    let mut sum_cache = HashMap::new();
    assert_eq!(euclidean_spiral_neighbor_sum(1, &mut sum_cache), 1);
    assert_eq!(euclidean_spiral_neighbor_sum(2, &mut sum_cache), 1);
    assert_eq!(euclidean_spiral_neighbor_sum(3, &mut sum_cache), 2);
    assert_eq!(euclidean_spiral_neighbor_sum(4, &mut sum_cache), 4);
    assert_eq!(euclidean_spiral_neighbor_sum(5, &mut sum_cache), 5);
    assert_eq!(euclidean_spiral_neighbor_sum(6, &mut sum_cache), 10);
    assert_eq!(euclidean_spiral_neighbor_sum(7, &mut sum_cache), 11);
    assert_eq!(euclidean_spiral_neighbor_sum(8, &mut sum_cache), 23);
    assert_eq!(euclidean_spiral_neighbor_sum(9, &mut sum_cache), 25);
    assert_eq!(euclidean_spiral_neighbor_sum(10, &mut sum_cache), 26);
    assert_eq!(euclidean_spiral_neighbor_sum(11, &mut sum_cache), 54);
    assert_eq!(euclidean_spiral_neighbor_sum(12, &mut sum_cache), 57);
    assert_eq!(euclidean_spiral_neighbor_sum(13, &mut sum_cache), 59);
    assert_eq!(euclidean_spiral_neighbor_sum(14, &mut sum_cache), 122);
    assert_eq!(euclidean_spiral_neighbor_sum(15, &mut sum_cache), 133);
    assert_eq!(euclidean_spiral_neighbor_sum(16, &mut sum_cache), 142);
    assert_eq!(euclidean_spiral_neighbor_sum(17, &mut sum_cache), 147);
    assert_eq!(euclidean_spiral_neighbor_sum(18, &mut sum_cache), 304);
    assert_eq!(euclidean_spiral_neighbor_sum(19, &mut sum_cache), 330);
    assert_eq!(euclidean_spiral_neighbor_sum(20, &mut sum_cache), 351);
    assert_eq!(euclidean_spiral_neighbor_sum(21, &mut sum_cache), 362);
    assert_eq!(euclidean_spiral_neighbor_sum(22, &mut sum_cache), 747);
    assert_eq!(euclidean_spiral_neighbor_sum(23, &mut sum_cache), 806);
}