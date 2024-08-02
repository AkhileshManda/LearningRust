use std::{fs::File, io::copy};

use error_chain::error_chain;
use tempfile::Builder;

error_chain! {
     foreign_links {
         Io(std::io::Error);
         HttpRequest(reqwest::Error);
     }
}

#[tokio::main]
async fn main() -> Result<()> {
    let image_url = "https://www.rust-lang.org/logos/rust-logo-512x512.png";
    let res = reqwest::get(image_url).await?;
    println!("res is : {:?}", res);

    let temp_dir = Builder::new().prefix("example").tempdir()?;

    let mut dest = {
        let file_name = res
            .url()
            .path_segments()
            .and_then(|segment| segment.last())
            .and_then(|name| if name.is_empty() { None } else { Some(name) })
            .unwrap_or("tmp.bin");

        println!("File name is : {:?}", file_name);

        let file_name = temp_dir.path().join(file_name);
        println!("will be located under: '{:?}'", file_name);
        File::create(file_name)?
    };

    println!("Destinations is : {:?}", dest);

    let content = res.text().await?;
    copy(&mut content.as_bytes(), &mut dest)?;

    Ok(())
}
