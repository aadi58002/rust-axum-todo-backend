use data::run;

#[tokio::main]
async fn main() {
    let crearring = run().await;
    println!("{crearring}");
}
