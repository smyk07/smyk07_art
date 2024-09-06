use smyk07_art::mix; // <--- importing re-exported function
use smyk07_art::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;

    mix(red, yellow);
}
