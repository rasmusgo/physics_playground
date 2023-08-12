mod sparse_pga;

fn main() {
    let p = sparse_pga::Point::default();
    let m = sparse_pga::Line::default();
    let r = p + m;
    println!("{r:?}");
}
