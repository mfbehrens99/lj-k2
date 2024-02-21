mod frontend;
mod midi;

use frontend::Frontend;

#[tokio::main]
async fn main() {
    tokio::join!(
        run_frontend()
    );
}


async fn run_frontend() {
    
    let mut f = Frontend::new("127.0.0.1:9002").await;
    f.listen().await;
}

// async fn run_midi() {
//     tokio::spawn(async move {
//         let mut f = Frontend::new("127.0.0.1:9002").await;
//         f.listen().await;
//     });
// }


