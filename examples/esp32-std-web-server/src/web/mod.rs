use anyhow::Error;
use esp_idf_svc::http::server::{Configuration, EspHttpServer};
use esp_idf_hal::io::Write;
use esp_idf_svc::http::Method;

pub fn new() -> Result<Box<EspHttpServer<'static>>, Error> {
  let mut server = EspHttpServer::new(&Configuration::default())?;

  // http://<sta ip>/ handler
  let handler_result = server.fn_handler("/", Method::Get, |request| {
    let html = index_html();
    let mut response = request.into_ok_response()?;
    response.write_all(html.as_bytes())?;
    Ok(())
  });

  if handler_result.is_err() {
    return Err(Error::msg("Cannot add handler"));
  }

  Ok(Box::new(server))
}

fn templated(content: impl AsRef<str>) -> String {
    format!(
        r#"
<!DOCTYPE html>
<html>
    <head>
        <meta charset="utf-8">
        <title>esp-rs web server</title>
    </head>
    <body>
        {}
    </body>
</html>
"#,
        content.as_ref()
    )
}

fn index_html() -> String {
    templated("Hello from ESP32!")
}
