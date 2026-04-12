use crate::api::{search_card, search_cards};
use colored::Colorize;
use std::process::{Command, Stdio};

pub async fn run_fzf() -> Result<(), Box<dyn std::error::Error>> {
    let current_exe = std::env::current_exe()?;
    let exe_path = current_exe.to_string_lossy();

    let mut cmd = Command::new("fzf");
    cmd.args(&[
        "--height",
        "40%",
        "--reverse",
        "--disabled",
        "--delimiter",
        "\t",
        "--with-nth",
        "2",
        "--preview",
        &format!("{} fzf-preview {{1}}", exe_path),
        "--bind",
        &format!("start:reload({} fzf-list \"\"),change:reload({} fzf-list {{q}} || true)", exe_path, exe_path),
    ]);
    cmd.stdin(Stdio::inherit());
    cmd.stderr(Stdio::inherit());
    cmd.stdout(Stdio::piped());

    let child = cmd.spawn()?;
    let output = child.wait_with_output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let selected = stdout.trim();
        if !selected.is_empty() {
            let parts: Vec<&str> = selected.split('\t').collect();
            if parts.len() >= 2 {
                crate::cmd_search::run(parts[1]).await?;
            }
        }
    }

    Ok(())
}

pub async fn run_list(query: &str) -> Result<(), Box<dyn std::error::Error>> {
    let results = search_cards(query).await?;
    for card in results {
        let name = card.cn_name.clone().unwrap_or_else(|| card.id.to_string());
        println!("{}\t{}", card.id, name);
    }
    Ok(())
}

pub async fn run_preview(id_str: &str) -> Result<(), Box<dyn std::error::Error>> {
    let result = search_card(id_str).await?;
    
    if let Some(card) = result {
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
    }
    
    Ok(())
}
