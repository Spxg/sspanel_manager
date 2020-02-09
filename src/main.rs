mod login_info;
mod edit_info;
mod parse;
mod website_info;
mod user_info;
mod menu;
mod logout;

#[macro_use] extern crate lazy_static;
use crate::menu::Menu;
use std::io::{stdin, Write};
use crate::login_info::Json;
use std::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder().cookie_store(true).build().unwrap();

    let (website, user, have_json) = login_info::get_login_info()?;

    if user.to_login(&website.login_address, &client).await? && !have_json {
        loop {
            println!("是否保存登录信息，下次无需手动登录？(0取消， 1确定)");
            let mut num = String::new();
            stdin().read_line(&mut num).expect("Failed to read line");

            match num.trim().parse::<i32>() {
                Ok(0) => break,
                Ok(1) => {
                    let mut file = File::create("login_info.json")
                        .expect("Failed to create file");
                    let json = Json::new(&website, &user);
                    let value = serde_json::to_string_pretty(&json)?;

                    file.write_all(value.as_bytes())?;
                    break;
                }
                _ => continue
            }
        }
    }

    Menu::new(website, client).await?.start_loop().await?;

    Ok(())
}

