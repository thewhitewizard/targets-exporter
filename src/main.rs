use std::env;
use std::fs;
use warp::Filter;
use dotenv::dotenv;
use tokio::signal;

const TP1_PERCENTAGE: f64 = 0.75;
const TP2_PERCENTAGE: f64 = 0.8;
const TP3_PERCENTAGE: f64 = 0.9;

#[derive(Debug, Clone)]
struct Crypto {
    name: String,
    target: f64,
    tp1: f64,
    tp2: f64,
    tp3: f64,
}

fn read_crypto_file(file_path: &str) -> Vec<Crypto> {
    let content = fs::read_to_string(file_path).expect("Unable to read the file");
    content
        .lines()
        .filter_map(|line| {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let target: f64 = parts[1].parse().ok()?;
                Some(Crypto {
                    name: name.clone(),
                    target,
                    tp1: target * TP1_PERCENTAGE,
                    tp2: target * TP2_PERCENTAGE,
                    tp3: target * TP3_PERCENTAGE,
                })
            } else {
                None
            }
        })
        .collect()
}

fn generate_metrics_output(cryptos: &[Crypto]) -> String {
    let mut metrics_output = String::new();
    for crypto in cryptos {
        metrics_output.push_str(&format!(
            "# HELP cryptocurrency_price_target Target price for {}\n\
             # TYPE cryptocurrency_price_target gauge\n\
             cryptocurrency_price_target{{token=\"{}\"}} {}\n",
            crypto.name, crypto.name, crypto.target
        ));
        metrics_output.push_str(&format!(
            "cryptocurrency_price_target_tp1{{token=\"{}\"}} {}\n\
             cryptocurrency_price_target_tp2{{token=\"{}\"}} {}\n\
             cryptocurrency_price_target_tp3{{token=\"{}\"}} {}\n",
            crypto.name, crypto.tp1,
            crypto.name, crypto.tp2,
            crypto.name, crypto.tp3
        ));
    }
    metrics_output
}

#[tokio::main]
async fn main() {
    dotenv().ok(); 
    let file_path = env::var("TARGETS_FILE").unwrap_or_else(|_| "targets.txt".to_string());
    let cryptos = read_crypto_file(&file_path);


    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C signal");
        println!("Received termination signal, shutting down gracefully. Bye!");
    };

    let metrics_route = warp::path("metrics").map(move || {
        generate_metrics_output(&cryptos)
    });

    println!("Starting server on port 8080...");
    let (_, server) = warp::serve(metrics_route)
        .bind_with_graceful_shutdown(([0, 0, 0, 0], 8080), shutdown_signal);

    server.await;
}
