extern crate local_ip;

fn main() {
    println!("{}", local_ip::get().expect("Could not find local IP address.").to_string());
}
