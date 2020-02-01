use chrono::Duration;
use chrono::prelude::*;
use std::ops::Add;
use std::collections::HashMap;
use reqwest::Client;
use crate::parse_info::parse_information;

pub async fn extend_information(address: &str, client: &Client, info: &mut UserInformationEditer) -> Result<(), Box<dyn std::error::Error>> {
    let value = client
        .get(address)
        .send().await?
        .text().await?;

    let information = parse_information(value);
    info.information.extend(information);

    Ok(())
}

pub struct UserInformation {
    pub information: HashMap<String, String>
}

pub struct UserInformationEditer {
    pub information: HashMap<String, String>
}

impl UserInformation {
    pub fn new() -> UserInformation {
        let information = HashMap::new();

        UserInformation {
            information,
        }
    }

    pub fn edit(self) -> UserInformationEditer {
        UserInformationEditer {
            information: self.information
        }
    }

    pub fn build_post_params(self) -> Vec<(String, String)> {
        self.information.into_iter().map(|a| a).collect()
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

impl UserInformationEditer {
    pub fn update_quickly(self, month: &str) -> UserInformationEditer {
        match month.parse::<u32>() {
            Ok(i) => {
                let data = i * 50;
                let data = data.to_string();
                self.add_month(month).add_data(&data).limit_speed("100")
            }
            Err(_) => {
                println!("请输入正确的数字");
                self
            }
        }
    }

    pub fn update_money(mut self, money: &str) -> UserInformationEditer {
        match money.parse::<u32>() {
            Ok(_) => *self.information.get_mut("money").unwrap() = money.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn add_money(mut self, money: &str) -> UserInformationEditer {
        let orign: f64 = self.information.get("money").unwrap().parse().unwrap();

        match money.parse::<f64>() {
            Ok(i) => {
                let finnal = (orign + i).to_string();
                *self.information.get_mut("money").unwrap() = finnal;
            }
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn add_month(mut self, month: &str) -> UserInformationEditer {
        match month.parse::<i64>() {
            Ok(i) => {
                let time = Utc.datetime_from_str(&self
                    .information.get("expire_in").unwrap(), "%Y-%m-%d %H:%M:%S")
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
                *self.information.get_mut("expire_in").unwrap() = expire_in;
            }
            Err(_) => println!("请输入正确的数字")
        }
        self
    }


    pub fn update_expire_time(mut self, time: &str) -> UserInformationEditer {
        match Utc.datetime_from_str(time, "%Y-%m-%d %H:%M:%S") {
            Ok(_) => *self.information.get_mut("expire_in").unwrap() = time.to_string(),
            Err(_) => println!("请安装格式正确输入")
        }
        self
    }

    pub fn limit_speed(mut self, speed: &str) -> UserInformationEditer {
        match speed.parse::<u32>() {
            Ok(_) => *self.information.get_mut("node_speedlimit").unwrap() = speed.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn add_data(mut self, data: &str) -> UserInformationEditer {
        let orign: f64 = self.information.get("transfer_enable").unwrap().parse().unwrap();
        match data.parse::<f64>() {
            Ok(i) => {
                let finnal = (orign + i).to_string();
                *self.information.get_mut("transfer_enable").unwrap() = finnal;
            }
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn change_admin(mut self, admin: &str) -> UserInformationEditer {
        match admin.trim().parse::<u32>() {
            Ok(0) => *self.information.get_mut("is_admin").unwrap() = "0".to_string(),
            Ok(1) => *self.information.get_mut("is_admin").unwrap() = "1".to_string(),
            _ => println!("请输入正确的数字")
        }
        self
    }

    pub fn change_class(mut self, class: &str) -> UserInformationEditer {
        match class.parse::<u32>() {
            Ok(_) => *self.information.get_mut("class").unwrap() = class.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn update_class_expire(mut self, time: &str) -> UserInformationEditer {
        match Utc.datetime_from_str(time, "%Y-%m-%d %H:%M:%S") {
            Ok(_) => *self.information.get_mut("class_expire").unwrap() = time.to_string(),
            Err(_) => println!("请安装格式正确输入")
        }
        self
    }

    pub fn change_group(mut self, group: &str) -> UserInformationEditer {
        match group.parse::<u32>() {
            Ok(_) => *self.information.get_mut("group").unwrap() = group.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn enable(mut self, enable: &str) -> UserInformationEditer {
        match enable.trim().parse::<u32>() {
            Ok(0) => *self.information.get_mut("enable").unwrap() = "0".to_string(),
            Ok(1) => *self.information.get_mut("enable").unwrap() = "1".to_string(),
            _ => println!("请输入正确的数字")
        }
        self
    }

    pub fn add_class_month(mut self, month: &str) -> UserInformationEditer {
        match month.parse::<i64>() {
            Ok(i) => {
                let time = Utc.datetime_from_str(&self
                    .information.get("class_expire").unwrap(), "%Y-%m-%d %H:%M:%S")
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
                *self.information.get_mut("class_expire").unwrap() = class_expire;
            }
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn change_node_connector(mut self, ips: &str) -> UserInformationEditer {
        match ips.parse::<u32>() {
            Ok(_) => *self.information.get_mut("node_connector").unwrap() = ips.to_string(),
            Err(_) => println!("请输入正确的数字")
        }
        self
    }

    pub fn update(self) -> UserInformation {
        UserInformation {
            information: self.information
        }
    }
}