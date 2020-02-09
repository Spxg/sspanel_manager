extern crate serde_json;
extern crate serde;

use serde::{Deserialize, Serialize};
use chrono::Duration;
use chrono::prelude::*;
use std::ops::Add;
use std::collections::HashMap;
use reqwest::Client;
use crate::parse::parse_info;

pub async fn extend_info(address: &str, client: &Client, info: &mut UserInfoEditer) -> Result<(), Box<dyn std::error::Error>> {
    let value = client
        .get(address)
        .send().await?
        .text().await?;

    let information = parse_info(value);
    info.info.extend(information);
    Ok(())
}

pub struct UserInfo {
    pub info: HashMap<String, String>
}

pub struct UserInfoEditer {
    pub info: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct UpdateQuicklyInfo {
    data: String,
    speed_limit: String,
}

pub struct UpdateQuicklyInfoBuilder {
    data: String,
    speed_limit: String,
}

impl UserInfo {
    pub fn new() -> UserInfo {
        let info = HashMap::new();

        UserInfo {
            info,
        }
    }

    pub fn edit(self) -> UserInfoEditer {
        UserInfoEditer {
            info: self.info
        }
    }

    pub fn build_post_params(self) -> Vec<(String, String)> {
        self.info.into_iter().map(|a| a).collect()
    }

    pub async fn to_edit(self, address: &String, client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        let message = client.put(address)
            .form(&self.build_post_params())
            .send().await?
            .text().await?;

        if message.contains("\"ret\":1") {
            println!("修改成功\n");
        } else {
            println!("修改失败\n");
        };

        Ok(())
    }
}

impl UserInfoEditer {
    pub fn update_quickly(self, month: &str, info: &UpdateQuicklyInfo) -> UserInfoEditer {
        match month.parse::<u32>() {
            Ok(i) => {
                let data = f64::from(i) * info.data.parse::<f64>().unwrap();
                let data = data.to_string();
                self.add_month(month).add_data(&data).limit_speed(&info.speed_limit)
            }
            Err(_) => {
                println!("请输入正确的数字");
                self
            }
        }
    }

    pub fn update_money(mut self, money: &str) -> UserInfoEditer {
        match money.parse::<u32>() {
            Ok(_) => *self.info.get_mut("money").unwrap() = money.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn add_money(mut self, money: &str) -> UserInfoEditer {
        let orign: f64 = self.info.get("money").unwrap().parse().unwrap();

        match money.parse::<f64>() {
            Ok(i) => {
                let finnal = (orign + i).to_string();
                *self.info.get_mut("money").unwrap() = finnal;
            }
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn add_month(mut self, month: &str) -> UserInfoEditer {
        match month.parse::<i64>() {
            Ok(i) => {
                let time = Utc.datetime_from_str(&self
                    .info.get("expire_in").unwrap(), "%Y-%m-%d %H:%M:%S")
                    .unwrap();

                let expire_in = if time > Utc::now() {
                    time.add(Duration::days(i * 30))
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                } else {
                    Utc::now().add(Duration::days(i * 30))
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                };
                *self.info.get_mut("expire_in").unwrap() = expire_in;
            }
            Err(_) => println!("请输入正确的数字")
        }
        self
    }


    pub fn update_expire_time(mut self, time: &str) -> UserInfoEditer {
        match Utc.datetime_from_str(time, "%Y-%m-%d %H:%M:%S") {
            Ok(_) => *self.info.get_mut("expire_in").unwrap() = time.to_string(),
            Err(_) => println!("请安装格式正确输入")
        }
        self
    }

    pub fn limit_speed(mut self, speed: &str) -> UserInfoEditer {
        match speed.parse::<u32>() {
            Ok(_) => *self.info.get_mut("node_speedlimit").unwrap() = speed.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn add_data(mut self, data: &str) -> UserInfoEditer {
        let orign: f64 = self.info.get("transfer_enable").unwrap().parse().unwrap();
        match data.parse::<f64>() {
            Ok(i) => {
                let finnal = (orign + i).to_string();
                *self.info.get_mut("transfer_enable").unwrap() = finnal;
            }
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn change_admin(mut self, admin: &str) -> UserInfoEditer {
        match admin.trim().parse::<u32>() {
            Ok(0) => *self.info.get_mut("is_admin").unwrap() = "0".to_string(),
            Ok(1) => *self.info.get_mut("is_admin").unwrap() = "1".to_string(),
            _ => println!("请输入正确的数字")
        }
        self
    }

    pub fn change_class(mut self, class: &str) -> UserInfoEditer {
        match class.parse::<u32>() {
            Ok(_) => *self.info.get_mut("class").unwrap() = class.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn update_class_expire(mut self, time: &str) -> UserInfoEditer {
        match Utc.datetime_from_str(time, "%Y-%m-%d %H:%M:%S") {
            Ok(_) => *self.info.get_mut("class_expire").unwrap() = time.to_string(),
            Err(_) => println!("请安装格式正确输入")
        }
        self
    }

    pub fn change_group(mut self, group: &str) -> UserInfoEditer {
        match group.parse::<u32>() {
            Ok(_) => *self.info.get_mut("group").unwrap() = group.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn enable(mut self, enable: &str) -> UserInfoEditer {
        match enable.trim().parse::<u32>() {
            Ok(0) => *self.info.get_mut("enable").unwrap() = "0".to_string(),
            Ok(1) => *self.info.get_mut("enable").unwrap() = "1".to_string(),
            _ => println!("请输入正确的数字")
        }
        self
    }

    pub fn add_class_month(mut self, month: &str) -> UserInfoEditer {
        match month.parse::<i64>() {
            Ok(i) => {
                let time = Utc.datetime_from_str(&self
                    .info.get("class_expire").unwrap(), "%Y-%m-%d %H:%M:%S")
                    .unwrap();

                let class_expire = if time > Utc::now() {
                    time.add(Duration::days(i * 30))
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                } else {
                    Utc::now().add(Duration::days(i * 30))
                        .format("%Y-%m-%d %H:%M:%S")
                        .to_string()
                };
                *self.info.get_mut("class_expire").unwrap() = class_expire;
            }
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn change_node_connector(mut self, ips: &str) -> UserInfoEditer {
        match ips.parse::<u32>() {
            Ok(_) => *self.info.get_mut("node_connector").unwrap() = ips.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn update(self) -> UserInfo {
        UserInfo {
            info: self.info
        }
    }
}

impl UpdateQuicklyInfo {
    pub fn new() -> UpdateQuicklyInfoBuilder {
        UpdateQuicklyInfoBuilder {
            data: String::default(),
            speed_limit: String::default(),
        }
    }
}

impl UpdateQuicklyInfoBuilder {
    pub fn data(self, data: &str) -> UpdateQuicklyInfoBuilder {
        UpdateQuicklyInfoBuilder {
            data: data.to_string(),
            ..self
        }
    }

    pub fn speed_limit(self, speed_limit: &str) -> UpdateQuicklyInfoBuilder {
        UpdateQuicklyInfoBuilder {
            speed_limit: speed_limit.to_string(),
            ..self
        }
    }

    pub fn build(self) -> UpdateQuicklyInfo {
        UpdateQuicklyInfo {
            data: self.data,
            speed_limit: self.speed_limit,
        }
    }
}