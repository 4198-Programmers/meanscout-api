use axum::response::IntoResponse;
use std::io::Read;
use std::fs::File;
use crate::settings;

pub async fn logs_get() -> impl IntoResponse {
    let config = settings::Settings::new().unwrap();
    let mut file = File::open(config.logs_dir).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let contents = contents.lines().map(|line| 
        format!("<div>{}</div>", line)
        .replace("DEBUG", "<span style=\"color: #7289da;\">DEBUG</span>")
        .replace("INFO", "<span style=\"color: yellow;\">INFO</span>")
        .replace("ERROR", "<span style=\"color: red;\">ERROR</span>")
        .replace("SUCCESS", "<span style=\"color: green;\">SUCCESS</span>")

    ).collect::<String>();
    
    let contents = format!(
    r#"
    <html>
        <body style="background-color: #1e1e2e; color: #cdd6f4">
            {}
        </body>
    </html>
    "#,
    contents);
    axum::response::Html(contents)
}
