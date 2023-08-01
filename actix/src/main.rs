use actix_web::{web, App, HttpResponse, HttpServer};
use tokio_postgres::Row;
use tokio_postgres::NoTls;
use actix_files::NamedFile;

const DATABASE_URL: &str = "postgres://postgres:1234567@localhost:5432/demo"; // Replace with your database URL

// Define a struct to represent the data stored in the database
#[derive(Debug, serde::Serialize)]
struct DatabaseRecord {
    src_ip: Option<String>,
    dst_ip: Option<String>,
    src_port: i32,
    dst_port: i32,
    seq_number: i32,
    ack_number: i32,
    flags: i32,
    window_size: i32,
}

async fn get_data_from_db() -> Result<Vec<DatabaseRecord>, actix_web::Error> {
    let (client, connection) = tokio_postgres::connect(DATABASE_URL, NoTls)
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    tokio::spawn(connection);

    let rows = client
        .query("SELECT * FROM sniffer", &[])
        .await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e))?;

    let mut result = Vec::new();

    for row in rows {
        let src_ip: Option<String> = row.get("src_ip");
        let dst_ip: Option<String> = row.get("dst_ip");
        let src_port: i32 = row.get("src_port");
        let dst_port: i32 = row.get("dst_port");
        let seq_number: i32 = row.get("seq_number");
        let ack_number: i32 = row.get("ack_number");
        let flags: i32 = row.get("flags");
        let window_size: i32 = row.get("window_size");

        result.push(DatabaseRecord {
            src_ip,
            dst_ip,
            src_port,
            dst_port,
            seq_number,
            ack_number,
            flags,
            window_size,
        });
    }

    Ok(result)
}

async fn index() -> HttpResponse {
    // Fetch data from the database
    let data = match get_data_from_db().await {
        Ok(data) => data,
        Err(_) => return HttpResponse::InternalServerError().finish(),
    };

    // Render the data as JSON and return it as the API response
    HttpResponse::Ok()
        .content_type("application/json")
        .json(data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/json", web::get().to(index))
            .route("/", web::get().to(webapp)) // Add this line to use the webapp function
    })
    .bind("127.0.0.1:2000")?
    .run()
    .await
}

async fn webapp() -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("static/tarun.html")?)
}