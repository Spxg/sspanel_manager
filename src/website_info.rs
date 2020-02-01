extern crate serde_json;
extern crate serde;
use serde::{Deserialize, Serialize};

pub struct WebsiteData {
    pub website_address: String,
    pub login_address: String,
    pub user_address: String,
    pub logout_address: String,
    pub edit_address: String,
    pub edit_post_address: String,
}

#[derive(Serialize, Deserialize)]
pub struct WebsiteDataBuilder {
    website_address: String,
}

impl WebsiteData {
    pub fn new() -> WebsiteDataBuilder {
        WebsiteDataBuilder {
            website_address: String::default(),
        }
    }

    pub fn insert_id(&mut self, id: &str) {
        self.edit_address = self.edit_address.replace("_", id);
        self.edit_post_address = self.edit_post_address.replace("_", id);
    }
}

impl WebsiteDataBuilder {
    pub fn load_address(self, address: &str) -> WebsiteDataBuilder {
        WebsiteDataBuilder {
            website_address: address.to_string(),
        }
    }

    pub fn build(self) -> WebsiteData {
        let website_address = self.website_address.as_str();

        let login =  website_address.to_string() + "/auth/login";
        let user = website_address.to_string() + "/admin/user";
        let logout  =  website_address.to_string() + "/auth/logout";
        let edit = user.to_string() + "/_/edit";
        let edit_post = user.to_string() + "/_";

        WebsiteData {
            website_address: website_address.to_string(),
            login_address: login,
            user_address: user,
            logout_address: logout,
            edit_address: edit,
            edit_post_address: edit_post,
        }
    }
}