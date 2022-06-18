fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Failed to read line.");
    let n = s.trim().parse::<usize>().expect("Failed to convert int.");
    s.clear();

    std::io::stdin().read_line(&mut s).unwrap();
    let arr = s.trim().split(" ").map(|x| x.parse::<i32>().expect("failed to convert int."))
    .collect::<Vec<i32>>();

    let mut memo: Vec<i32> = Vec::new();

    for i in 0..n {
        if i == 0 {
            memo.push(1);
            continue;
        }
        let mut max_val = 0;
        let mut max_cnt = 0;
        for j in 0..i {
            if max_cnt < memo[j] && arr[j] < arr[i] {
                max_val = arr[j];
                max_cnt = memo[j];
            }
            else if max_cnt == memo[j] && arr[j] < max_val {
                max_val = arr[j];
            }
        }
        memo.push(max_cnt + 1);
    }

    let mut ans: usize = 0;
    for i in 0..n {
        if memo[ans] < memo[i] {
            ans = i;
        }
    }
    println!("{}", memo[ans]);
}