
#[tokio::main]
async fn main() {
let world = world();
    hello();
    world.await;
}
fn hello() {
    println!("hello");
}

async fn world() {
    println!("world");
}

fn world2() -> impl Future<Output=()> {
async {
println!("world2");
}
}