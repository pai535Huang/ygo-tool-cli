use crate::api::search_card;
use colored::Colorize;

pub async fn run(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = search_card(query).await?;
    
    if let Some(card) = result {
        println!("{}", "==== Card Match Found ====".green().bold());
        
        if let Some(cn) = &card.cn_name {
            println!("{}: {}", "CN Name".bold().cyan(), cn.bold());
        }
        if let Some(sc) = &card.sc_name {
            println!("{}: {}", "SC Name".bold().cyan(), sc.bold());
        }
        
        if let Some(jp) = &card.jp_name {
            println!("{}: {}", "JP Name".bold().cyan(), jp);
        }
        if let Some(en) = &card.en_name {
            println!("{}: {}", "EN Name".bold().cyan(), en);
        }
        
        if let Some(text) = &card.text {
            println!("\n{}", "-- Information --".blue().bold());
            if let Some(types) = &text.types {
                println!("{}", types.yellow());
            }
            if let Some(pdesc) = &text.pdesc {
                if !pdesc.is_empty() {
                    println!("\n{}\n{}", "[Pendulum Effect]".magenta().bold(), pdesc);
                }
            }
            if let Some(desc) = &text.desc {
                println!("\n{}\n{}", "[Card Description]".green().bold(), desc);
            }
        }
        println!("{}", "==========================".green().bold());
    } else {
        println!("{} '{}'", "No cards found for query:".red(), query.red().bold());
    }
    
    Ok(())
}
