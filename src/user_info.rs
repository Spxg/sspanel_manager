extern crate serde_json;
extern crate serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Information {
    pub id: u32,
    pub user_name: String,
    pub email: String,
    pub enable_traffic: f64,
    pub used_traffic: f64,
    pub money: String,
    pub expire_in: String,
    pub class: u32,
    pub class_expire: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub data: Vec<Information>,
}

#[derive(Serialize, Deserialize)]
pub struct Records {
    pub records_total: u32,
}

impl User {
    pub fn list(&self) {
        println!("id   username   email");
        for i in &self.data {
            println!("{}   {}   {}", i.id, i.user_name, i.email);
        }
        println!();
    }

    pub fn search(&self, value: &str) {
        println!("id   username   email");
        for i in &self.data {
            if i.user_name.contains(value) || i.email.contains(value) {
                println!("{}   {}   {}", i.id, i.user_name, i.email);
            }
        }
        println!();
    }

    pub fn check(&self, id: &str) -> bool {
        match id.parse::<u32>() {
            Ok(u) => {
                for i in &self.data {
                    if i.id == u {
                        println!("你选择的用户是");
                        println!("id: {}", i.id);
                        println!("username: {}", i.user_name);
                        println!("email: {}", i.email);
                        println!();
                        return true;
                    }
                }
                println!("用户不存在\n");
            }
            Err(_) => println!("id输入不正确\n")
        }
        false
    }

    pub fn detail_info(&self, id: &str) {
        for i in &self.data {
            if i.id == id.parse::<u32>().unwrap() {
                println!("{:#?}", i);
                break;
            }
        }
        println!();
    }
}
