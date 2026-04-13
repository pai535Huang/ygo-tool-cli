use crate::api::{download_image, search_card};
use viuer::{Config, print};
use viuer::{get_kitty_support, KittySupport};

pub async fn run(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = search_card(query).await?;
    
    if let Some(card) = result {
        println!("Downloading image for {} (ID: {})...", card.cn_name.as_deref().unwrap_or("Unknown").to_string(), card.id);
        
        let kitty = get_kitty_support() != KittySupport::None;
        
        let (url, image_bytes) = download_image(card.id).await?;
        
        if kitty {
            let img = image::load_from_memory(&image_bytes)?;
            
            let mut conf = Config::default();
            conf.absolute_offset = false;
            conf.restore_cursor = false;
            conf.transparent = true;
            // 调整宽度和高度的比例。终端字符通常高是宽的2倍左右。
            // 设定高度为16行，宽度适当放大（如24列）来补偿终端由于字符比例导致的被挤压感
            conf.width = Some(26);
            conf.height = Some(16);
            conf.use_kitty = kitty;
            conf.use_iterm = false;
            
            print(&img, &conf)?;
        } else {
            // Unsupported terminal, output the URL instead of using block drawing
            println!("Advanced terminal image rendering (Kitty/Sixel) is not supported in this terminal.");
            println!("Image URL: {}", url);
        }
    } else {
        println!("No cards found for query: '{}'", query);
    }
    
    Ok(())
}
