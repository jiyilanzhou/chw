
// 在"..\65learn_oo1\Cargo.toml"配置" workspace "
use getavg;

fn main() {
    let mut a = getavg::AvgCollect::new();

    a.add(1);
    println!("average = {}", a.average());

    a.add(2);
    println!("average = {}", a.average());

    a.add(3);
    println!("average = {}", a.average());

    a.remove();
    println!("average = {}", a.average());

}