use actix_files::Files;
use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use my_site::configuration::get_configuration;
//use my_site::startup::run;
use tera::{Context, Tera};

#[derive(serde::Serialize)]
struct SocialMedia {
    name: String,
    url: String,
}

#[get("/")]
async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let mut ctx = Context::new();
    ctx.insert("name", "Francis");
    ctx.insert(
        "highlights",
        &vec![
            "Senior Linux Engineer at Tower Research Capital",
            "Linux Administrator at RBC",
            "Graduated from and former starting pitcher at Drew University",
        ],
    );
    ctx.insert(
        "linkedin_url",
        "https://www.linkedin.com/in/francis-rossi-a8b30a89/",
    );
    ctx.insert(
        "social_medias",
        &vec![
            SocialMedia {
                name: String::from("LinkedIn"),
                url: String::from("https://www.linkedin.com/in/francis-rossi-a8b30a89/"),
            },
            SocialMedia {
                name: String::from("GitHub"),
                url: String::from("https://github.com/prey169"),
            },
        ],
    );

    let rendered = tmpl
        .render("index.html", &ctx)
        .expect("Failed to render template");
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").expect("Failed to initialize Tera");

    let configuration = get_configuration().expect("Failed to read configuration.");
    let address = format!(
        "{}:{}",
        configuration.application.host, configuration.application.port
    );

    println!("Starting server at http://{}", address);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(index)
        //            .service(Files::new("/static", "./static").show_files_listing())
    })
    .bind(address)?
    .run()
    .await
}
