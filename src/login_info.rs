use reqwest::Client;

extern crate serde_json;
extern crate serde;

use serde::{Deserialize, Serialize};
use std::process::exit;
use crate::website_info::{WebsiteDataBuilder, WebsiteData};
use std::io::stdin;

pub struct LoginInformation {
    email: String,
    passwd: String,
    code_enable: bool,
    code: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginInformationBuilder {
    email: String,
    passwd: String,
    code_enable: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Json {
    address: WebsiteDataBuilder,
    admin_info: LoginInformationBuilder,
}

impl LoginInformation {
    pub fn new() -> LoginInformationBuilder {
        LoginInformationBuilder {
            email: String::default(),
            passwd: String::default(),
            code_enable: false,
        }
    }

    pub fn edit_code(mut self, code: &str) -> Self {
        self.code_enable = true;
        self.code = code.to_string();
        self
    }

    pub fn build_post_params(&self) -> Vec<(&str, &String)> {
        let mut params = vec![];
        params.push(("email", &self.email));
        params.push(("passwd", &self.passwd));
        params.push(("code", &self.code));
        params
    }

    pub async fn to_login(&self, address: &str, client: &Client) -> Result<bool, Box<dyn std::error::Error>> {
        let message = client.post(address)
            .form(&self.build_post_params())
            .send().await?
            .text().await?;

        let success = if message.contains("\"ret\":1") {
            println!("登录成功\n");
            true
        } else {
            println!("登录失败， 请检查登录信息");
            false
        };

        if !success {
            exit(0);
        }

        Ok(success)
    }
}

impl LoginInformationBuilder {
    pub fn email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    pub fn passwd(mut self, passwd: &str) -> Self {
        self.passwd = passwd.to_string();
        self
    }

    pub fn code_enable(mut self, code_enable: bool) -> Self {
        self.code_enable = code_enable;
        self
    }

    pub fn build_login_information(self, code: &str) -> LoginInformation {
        LoginInformation {
            email: self.email,
            passwd: self.passwd,
            code_enable: self.code_enable,
            code: code.to_string(),
        }
    }
}


impl Json {
    pub fn new(address: &WebsiteData, admin_info: &LoginInformation) -> Json {
        Json
        {
            address: WebsiteData::new().load_address(&address.website_address),
            admin_info: LoginInformation::new()
                .email(&admin_info.email)
                .passwd(&admin_info.passwd)
                .code_enable(admin_info.code_enable),
        }
    }

    pub fn parse(self) -> (WebsiteData, LoginInformationBuilder) {
        (self.address.build(), self.admin_info)
    }
}

pub fn get_login_info() -> Result<(WebsiteData, LoginInformation, bool), Box<dyn std::error::Error>> {
    let (website, user, have_json) = match std::fs::read_to_string("config.json") {
        Ok(i) => {
            println!("找到配置文件，开始登录");
            let json: Json = serde_json::from_str(&i)?;
            let (website, user_builder) = json.parse();
            let check = user_builder.code_enable;

            let mut user = user_builder.build_login_information("");

            if check {
                loop {
                    println!("请输入二次验证码");
                    let mut code = String::new();
                    stdin().read_line(&mut code).unwrap();

                    if let Err(_) = code.trim().parse::<u32>() {
                        println!("请重新设置");
                        continue;
                    }

                    user = user.edit_code(code.trim());
                    break;
                }
            }

            (website, user, true)
        }
        _ => {
            println!("未找到配置文件");
            println!("请输入网址(http/https)");
            let mut address = String::new();
            stdin().read_line(&mut address).unwrap();
            let website = WebsiteData::new().load_address(address.trim()).build();

            println!("请输入邮箱");
            let mut email = String::new();
            stdin().read_line(&mut email).unwrap();

            println!("请输入密码");
            let mut passwd = String::new();
            stdin().read_line(&mut passwd).unwrap();

            let mut user = LoginInformation::new()
                .email(email.trim())
                .passwd(passwd.trim())
                .code_enable(false)
                .build_login_information("");

            loop {
                println!("是否设置了两步验证(0未设置, 1设置了)");
                let mut enable = String::new();
                stdin().read_line(&mut enable).unwrap();

                match enable.trim().parse() {
                    Ok(0) => break,
                    Ok(1) => {
                        println!("请输入二次验证码");
                        let mut code = String::new();
                        stdin().read_line(&mut code).unwrap();

                        if let Err(_) = code.trim().parse::<u32>() {
                            println!("请重新设置");
                            continue;
                        }
                        user = user.edit_code(code.trim());
                        break;
                    }
                    _ => continue
                };
            }

            (website, user, false)
        }
    };

    Ok((website, user, have_json))
}