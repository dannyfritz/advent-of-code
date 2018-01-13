fn main() {
    let loc = 361527;
    println!("{}", spiral_euclidean_distance(loc));
}

fn spiral_euclidean_distance(loc: u64) -> u64 {
    if loc == 1 {
        return 0
    }
    let mut sqrt: i64 = (loc as f64).sqrt() as i64;
    if sqrt % 2 == 0 {
        sqrt -= 1;
    }
    if sqrt.pow(2) as u64 == loc {
        sqrt -= 2;
    }
    let b = (sqrt.pow(2) + 1) as i64;
    let a = ((loc as i64) - b) as i64;
    let c = (sqrt / 2) as i64;
    let d = ((a % (sqrt + 1)) as i64 - c).abs() as u64;
    let ans = d + ((sqrt as u64 + 1) / 2);
    return ans
}

#[test]
fn it_works_1() {
    assert_eq!(spiral_euclidean_distance(1), 0);
}
#[test]
fn it_works_2() {
    assert_eq!(spiral_euclidean_distance(2), 1);
}
#[test]
fn it_works_9() {
    assert_eq!(spiral_euclidean_distance(9), 2);
}
#[test]
fn it_works_10() {
    assert_eq!(spiral_euclidean_distance(10), 3);
}
#[test]
fn it_works_11() {
    assert_eq!(spiral_euclidean_distance(11), 2);
}
#[test]
fn it_works_12() {
    assert_eq!(spiral_euclidean_distance(12), 3);
}
#[test]
fn it_works_13() {
    assert_eq!(spiral_euclidean_distance(13), 4);
}
#[test]
fn it_works_23() {
    assert_eq!(spiral_euclidean_distance(23), 2);
}
#[test]
fn it_works_1024() {
    assert_eq!(spiral_euclidean_distance(1024), 31);
}
