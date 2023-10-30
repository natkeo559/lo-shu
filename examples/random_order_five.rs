use lo_shu::{Construction, O5};

fn main() {
    let mut p = Construction::<O5>::identity();

    let mut checked = 0;
    loop {
        if let Some(valid) = p.shuffle().check_n_s() {
            println!("{}", valid.square);
            break
        } else {
            checked += 1;
            println!("{}", checked);
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        }
    }
}