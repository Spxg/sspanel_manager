use crate::user_info::User;
use crate::logout::logout;
use std::io::stdin;
use crate::website_info::WebsiteData;
use crate::edit_info::{UserInformation, UserInformationEditer};
use crate::{edit_info, parse_info};
use reqwest::Client;

pub struct Menu {
    id: String,
    user: User,
    website: WebsiteData,
    client: Client,
}

impl Menu {
    pub async fn new(website: WebsiteData, client: Client) -> Result<Menu, Box<dyn std::error::Error>> {
        let user = parse_info::parse_user_list(&website.user_address, &client).await?;

        Ok(Menu {
            id: String::default(),
            user,
            website,
            client,
        })
    }

    pub async fn start_loop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.loop_one().await?;
        Ok(())
    }

    async fn loop_one(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            list();

            let mut num = String::new();
            let mut id = String::new();
            stdin().read_line(&mut num).expect("Failed to read line");

            match num.trim().parse::<u32>() {
                Ok(0) => {
                    logout(&self.client, &self.website.logout_address).await?;
                    break;
                }
                Ok(1) => self.user.list(),
                Ok(2) => {
                    println!("请输入ID:");
                    stdin().read_line(&mut id).expect("Failed to read line");
                    if self.user.check(id.trim()) {
                        self.website.insert_id(id.trim());
                        self.id = id.trim().to_string();

                        self.loop_two().await?;
                    }
                }
                Ok(3) => {
                    println!("请输入搜索内容:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    self.user.search(value.trim());
                }
                _ => continue,
            };
        }

        Ok(())
    }

    async fn loop_two(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let mut info = UserInformation::new().edit();
            edit_info::extend_information(&self.website.edit_address, &self.client, &mut info).await?;

            selected();
            let mut num = String::default();
            stdin().read_line(&mut num).expect("Failed to read line");

            let is_update = false;

            match num.trim().parse::<u32>() {
                Ok(0) => break,
                Ok(1) => self.user.detail_info(&self.id),
                Ok(2) => self.loop_three(info, is_update).await?,
                _ => continue,
            }
        }

        Ok(())
    }

    async fn loop_three(&mut self, mut info: UserInformationEditer, mut is_update: bool)
                        -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let mut num = String::default();
            edit();
            stdin().read_line(&mut num).expect("Failed to read line");

            info = match num.trim().parse::<u32>() {
                Ok(0) => break,
                Ok(1) => {
                    println!("请输入续费月数:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.update_quickly(value.trim())
                }
                Ok(2) => {
                    println!("请输入要增加的月数:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.add_month(value.trim())
                }
                Ok(3) => {
                    println!("请输入要增加的流量:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.add_data(value.trim())
                }
                Ok(4) => {
                    println!("请输入要修改的速度:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.limit_speed(value.trim())
                }
                Ok(5) => {
                    println!("请输入要增加的金钱:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.add_money(value.trim())
                }
                Ok(6) => {
                    println!("请输入要重置的金钱:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.update_money(value.trim())
                }
                Ok(7) => {
                    println!("请输入要重置的时间(%Y-%m-%d %H:%M:%S):");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.update_expire_time(value.trim())
                }
                Ok(8) => {
                    println!("请输入要增加的月数:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.add_class_month(value.trim())
                }
                Ok(9) => {
                    println!("请输入要修改的级别:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.change_class(value.trim())
                }
                Ok(10) => {
                    println!("请输入要重置的时间(%Y-%m-%d %H:%M:%S):");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.update_class_expire(value.trim())
                }
                Ok(11) => {
                    println!("请输入要修改的群组:");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.change_group(value.trim())
                }
                Ok(12) => {
                    println!("0禁用, 1启用");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.enable(value.trim())
                }
                Ok(13) => {
                    println!("0取消管理员, 1设置管理员");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.change_admin(value.trim())
                }
                Ok(14) => {
                    println!("请输入要修改的IP连接数(0为不限制)");
                    let mut value = String::new();
                    stdin().read_line(&mut value).expect("Failed to read line");
                    info.change_node_connector(value.trim())
                }
                Ok(15) => {
                    println!("更新中");
                    is_update = true;
                    break;
                }
                _ => continue,
            }
        }
        if is_update {
            info.update().to_edit(&self.website.edit_post_address, &self.client).await?;
            self.user = parse_info::parse_user_list(&self.website.user_address, &self.client).await?;
        }

        Ok(())
    }
}


fn list() {
    println!("以下所有菜单都是按数字0退出");
    println!("1) 列出用户    2) 选择用户");
    println!("3) 搜索用户");
}

fn selected() {
    println!("1) 查看信息    2) 修改信息");
}

fn edit() {
    println!(" 1) 便捷续费   2) 增加时间");
    println!(" 3) 增加流量   4) 修改速度");
    println!(" 5) 增加金钱   6) 重置金钱");
    println!(" 7) 重置时间   8) 增加级别时间");
    println!(" 9) 修改等级  10) 重置等级时间");
    println!("11) 修改群组  12) 用户禁启用");
    println!("13) 修改权限  14) 修改连接IP数");
    println!("15) 完成更改");
}
