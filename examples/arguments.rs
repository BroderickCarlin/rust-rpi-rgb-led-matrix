/// Simple use of matrix arguments
use clap;
use rpi_led_matrix::args;

fn main() {
    let mut app = clap::App::new("Argument Example").about("shows basic usage of matrix arguments");
    args::add_matrix_args(&mut app);
    let matches = app.get_matches();
    println!("{:#?}", matches);
}
