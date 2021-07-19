use clap::App;

fn main() {
    let _matches = App::new("Holium")
        .bin_name("holium")
        .version("1.0.0-alpha")
        .author("Polyphene <contact@polyphene.io>")
        .about("Enjoy the power of the Holium Framework.")
        .get_matches();
}