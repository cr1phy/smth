use api::start;

fn main() {
    let result = start();
    if let Some(err) = result.err() {
        println!("Something went wrong! {}", err)
    }
}
