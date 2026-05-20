use actix_web::{get, App, HttpResponse, HttpServer, Responder};

const PAGE: &str = r#"<!doctype html><html><head><meta charset="utf-8"><title>FlazHost Actix Web App</title><style>*{margin:0;padding:0;box-sizing:border-box}body{font-family:-apple-system,BlinkMacSystemFont,'Segoe UI',Roboto,sans-serif;background:linear-gradient(135deg,#1e293b,#0f172a);min-height:100vh;display:flex;align-items:center;justify-content:center;color:#e2e8f0}.card{background:rgba(255,255,255,.05);border:1px solid rgba(255,255,255,.1);border-radius:16px;padding:3rem 2.5rem;text-align:center;max-width:480px}.card h1{font-size:1.6rem;margin-bottom:1rem}.card p{margin-bottom:.6rem;color:#cbd5e1;line-height:1.6}.meta{margin-top:1.5rem;padding-top:1.2rem;border-top:1px solid rgba(255,255,255,.08);font-size:.8rem;color:#64748b}</style></head><body><div class="card"><h1>Actix Web berjalan di FlazHost</h1><p>Aplikasi Actix Web Anda berhasil di-deploy.</p><p>Edit kode lalu push ke Git untuk redeploy otomatis.</p><div class="meta">Powered by FlazHost Runner</div></div></body></html>"#;

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().content_type("text/html; charset=utf-8").body(PAGE)
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().content_type("application/json").body("{\"status\":\"ok\"}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port: u16 = std::env::var("PORT").unwrap_or_else(|_| "80".into()).parse().unwrap_or(80);
    HttpServer::new(|| App::new().service(home).service(health))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
