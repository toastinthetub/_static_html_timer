use axum::{response::Html, routing::get, Router};
use chrono::{Datelike, Local, TimeZone, Timelike};
use chrono_tz::Tz;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(home_timer));
    let addr = SocketAddr::from(([0, 0, 0, 0], 4321));

    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Server running at http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("Failed to start server")
}

async fn home_timer() -> Html<String> {
    let pst: Tz = "America/Los_Angeles".parse().unwrap();
    let now = Local::now().with_timezone(&pst);

    let t_d = now.day();
    let t_m = now.month();
    let t_h = now.hour();
    let t_n = now.minute();

    let today = format!("{}/{} @ {}:{}", t_d, t_m, t_h, t_n);

    let target = pst.with_ymd_and_hms(2024, 12, 4, 16, 30, 0).unwrap();

    let duration = target - now;
    let (hours, minutes) = (duration.num_hours(), (duration.num_minutes() % 60).abs());

    let total_days = duration.num_minutes() as f64 / (24.0 * 60.0);
    let formatted_days = format!("{:.3}", total_days);

    Html(format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Time Until Sam's Home</title>
            <style>
                body {{ 
                    font-family: 'Arial', sans-serif; 
                    text-align: center; 
                    background-color: pink;
                    display: flex;
                    justify-content: center;
                    align-items: center;
                    height: 100vh;
                    margin: 0;
                    padding: 20px;
                    box-sizing: border-box;
                }}
                .container {{
                    background-color: white;
                    border-radius: 20px;
                    padding: 30px;
                    box-shadow: 0 10px 25px rgba(0,0,0,0.1);
                    max-width: 400px;
                    width: 100%;
                }}
                h1 {{ 
                    color: #333; 
                    font-size: 2.5rem;
                    margin-bottom: 20px;
                }}
                h2 {{ 
                    color: #FF69B4; 
                    font-size: 3rem;
                    margin: 20px 0;
                }}
                p {{ 
                    color: #666; 
                    font-size: 1.2rem;
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>timer for my madderie shaffer :)</h1>
                <p>Time remaining until Sam gets home:</p>
                <p>(December 4th, 2024 at 4:30 PM PST)</p>
                <h2>{} hours and {} minutes (or exactly {:.5} days)</h2>
                <p>i will see you soon my love</p>
                <p>(for reference, current date/time is {})</p>
            </div>
        </body>
        </html>
        "#,
        hours, minutes, total_days, today,
    ))
}
