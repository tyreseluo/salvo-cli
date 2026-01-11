use anyhow::Result;
use rust_embed::RustEmbed;
use salvo::prelude::*;

#[derive(RustEmbed)]
#[folder = "assets/visual"]
struct VisualAssets;

#[handler]
async fn index(res: &mut Response) {
    match VisualAssets::get("index.html") {
        Some(file) => {
            let html = String::from_utf8_lossy(file.data.as_ref()).into_owned();
            res.render(Text::Html(html));
        }
        None => {
            res.status_code(StatusCode::INTERNAL_SERVER_ERROR);
            res.render(Text::Plain("index.html not found"));
        }
    }
}

pub async fn run(port: u16) -> Result<()> {
    let addr = format!("127.0.0.1:{port}");
    let url = format!("http://{addr}/");
    let service = Service::new(Router::new().get(index));
    let acceptor = TcpListener::new(addr).bind().await;
    println!("Visual initializer running at {url}");
    open_browser(&url);
    Server::new(acceptor).serve(service).await;
    Ok(())
}

fn open_browser(url: &str) {
    if let Err(err) = try_open_browser(url) {
        eprintln!("Failed to open browser automatically: {err}");
        eprintln!("Open this URL manually: {url}");
    }
}

fn try_open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open").arg(url).spawn()?;
        return Ok(());
    }
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/C", "start", "", url])
            .spawn()?;
        return Ok(());
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        std::process::Command::new("xdg-open").arg(url).spawn()?;
        return Ok(());
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows", unix)))]
    {
        let _ = url;
        Ok(())
    }
}
