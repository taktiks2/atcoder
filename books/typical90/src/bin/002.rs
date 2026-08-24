use proconio::input;

// bit全探索

fn main() {
    input! {
        n: usize,
    };
    if n % 2 == 0 {
        // 左ビットシフトさせることで2のn乗を計算
        for i in 0..2 << (n - 1) {
            let bit = format!("{:0width$b}", i, width = n);
            let mut ttl = 0;
            for c in bit.chars() {
                if c == '0' {
                    ttl += 1;
                } else {
                    ttl -= 1;
                }

                if ttl < 0 {
                    break;
                }
            }
            if ttl == 0 {
                println!("{}", bit.replace('0', "(").replace('1', ")"));
            }
        }
    }
}
