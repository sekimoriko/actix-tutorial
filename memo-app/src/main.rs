mod db;

fn main() {
    println!("Hello, world!");
    let connection = db::establish_connection();
}
