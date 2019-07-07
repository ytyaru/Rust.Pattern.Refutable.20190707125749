/*
 * Rustのパターン（論駁可能性）
 * CreatedAt: 2019-07-07
 */
fn main() {
    let v = Some(0);
    if let Some(x) = v {
        println!("{}", x);
    }
}

