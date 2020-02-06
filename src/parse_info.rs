use regex::Regex;
use reqwest::Client;
use crate::user_info::{User, Records};

lazy_static! {
    static ref RE: Regex = Regex::new("<div class=\"form-group form-group-label\">(.*?)</div>").unwrap();
    static ref RE_ONE: Regex = Regex::new("<input class=\"form-control maxwidth-edit\".*?id=\"(?P<id>.*?)\".*?value=\"(?P<value>.*?)\".*?>")
        .unwrap();
    static ref RE_TWO: Regex = Regex::new("<div class=\"checkbox switch\">.*?value=\"(?P<value>.*?)\".*?id=\"(?P<id>.*?)\".*?type")
        .unwrap();
    static ref RE_THREE: Regex = Regex::new("<select.*?id=\"(?P<id>.*?)\".*?value=\"(?P<value>.*?)\" selected")
        .unwrap();
    static ref RE_FOUR: Regex = Regex::new("<textarea.*?id=\"(?P<id>.*?)\".*?>(?P<value>.*?)</textarea>")
        .unwrap();
}

pub fn parse_information(orign: String) -> Vec<(String, String)> {
    let mut x = orign.replace("\n", "_");
    x = x.replace(char::is_control, "")
        .replace("type=\"password\"", "value=\"\"")
        .replace("class=\"access-hide\"", "value=\"0\"")
        .replace("checked value=\"0\"", "value=\"1\"");

    let mut info_one = Vec::new();
    let mut info_two = Vec::new();
    let mut info_three = Vec::new();
    let mut info_four = Vec::new();

    for i in RE.captures_iter(&x)
    {
        info_one.extend(RE_ONE.captures_iter(i.get(1).unwrap().as_str())
            .map(|i| (i.name("id").unwrap().as_str().to_string()
                      , i.name("value").unwrap().as_str().to_string()))
            .collect::<Vec<(String, String)>>());

        info_two.extend(RE_TWO.captures_iter(i.get(1).unwrap().as_str())
            .map(|i| (i.name("id").unwrap().as_str().to_string()
                      , i.name("value").unwrap().as_str().to_string()))
            .collect::<Vec<(String, String)>>());

        info_three.extend(RE_THREE.captures_iter(i.get(1).unwrap().as_str())
            .map(|i| (i.name("id").unwrap().as_str().to_string()
                      , i.name("value").unwrap().as_str().to_string()))
            .collect::<Vec<(String, String)>>());

        info_four.extend(RE_FOUR.captures_iter(i.get(1).unwrap().as_str())
            .map(|i| (i.name("id").unwrap().as_str().to_string()
                      , i.name("value").unwrap().as_str().to_string().replace("_", "\n")))
            .collect::<Vec<(String, String)>>());
    }

    info_one.extend(info_two);
    info_one.extend(info_three);
    info_one.extend(info_four);
    info_one
}

pub async fn parse_user_list(address: &String, client: &Client) -> Result<User, Box<dyn std::error::Error>> {
    let user = address.as_str().to_string();
    let post_address = user + "/ajax?columns%5B0%5D%5Bda\
    ta%5D=op&columns%5B0%5D%5Bname%5D=&columns%5B0%5D%5Bsearchable%5D=true&columns%5B0%5D%5Borderabl\
    e%5D=false&columns%5B0%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B0%5D%5Bsearch%5D%5Bregex%5D=false&co\
    lumns%5B1%5D%5Bdata%5D=id&columns%5B1%5D%5Bname%5D=&columns%5B1%5D%5Bsearchable%5D=true&columns%\
    5B1%5D%5Borderable%5D=true&columns%5B1%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B1%5D%5Bsearch%5D%5Br\
    egex%5D=false&columns%5B2%5D%5Bdata%5D=user_name&columns%5B2%5D%5Bname%5D=&columns%5B2%5D%5Bsear\
    chable%5D=true&columns%5B2%5D%5Borderable%5D=true&columns%5B2%5D%5Bsearch%5D%5Bvalue%5D=&columns\
    %5B2%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B3%5D%5Bdata%5D=remark&columns%5B3%5D%5Bname%5D=&c\
    olumns%5B3%5D%5Bsearchable%5D=true&columns%5B3%5D%5Borderable%5D=true&columns%5B3%5D%5Bsearch%5D\
    %5Bvalue%5D=&columns%5B3%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B4%5D%5Bdata%5D=email&columns%\
    5B4%5D%5Bname%5D=&columns%5B4%5D%5Bsearchable%5D=true&columns%5B4%5D%5Borderable%5D=true&columns\
    %5B4%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B4%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B5%5D%5Bdat\
    a%5D=money&columns%5B5%5D%5Bname%5D=&columns%5B5%5D%5Bsearchable%5D=true&columns%5B5%5D%5Bordera\
    ble%5D=true&columns%5B5%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B5%5D%5Bsearch%5D%5Bregex%5D=false&c\
    olumns%5B6%5D%5Bdata%5D=im_type&columns%5B6%5D%5Bname%5D=&columns%5B6%5D%5Bsearchable%5D=true&co\
    lumns%5B6%5D%5Borderable%5D=true&columns%5B6%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B6%5D%5Bsearch%\
    5D%5Bregex%5D=false&columns%5B7%5D%5Bdata%5D=im_value&columns%5B7%5D%5Bname%5D=&columns%5B7%5D%5\
    Bsearchable%5D=true&columns%5B7%5D%5Borderable%5D=true&columns%5B7%5D%5Bsearch%5D%5Bvalue%5D=&co\
    lumns%5B7%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B8%5D%5Bdata%5D=node_group&columns%5B8%5D%5Bn\
    ame%5D=&columns%5B8%5D%5Bsearchable%5D=true&columns%5B8%5D%5Borderable%5D=true&columns%5B8%5D%5B\
    search%5D%5Bvalue%5D=&columns%5B8%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B9%5D%5Bdata%5D=expir\
    e_in&columns%5B9%5D%5Bname%5D=&columns%5B9%5D%5Bsearchable%5D=true&columns%5B9%5D%5Borderable%5D\
    =true&columns%5B9%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B9%5D%5Bsearch%5D%5Bregex%5D=false&columns\
    %5B10%5D%5Bdata%5D=class&columns%5B10%5D%5Bname%5D=&columns%5B10%5D%5Bsearchable%5D=true&columns\
    %5B10%5D%5Borderable%5D=true&columns%5B10%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B10%5D%5Bsearch%\
    %5Bregex%5D=false&columns%5B11%5D%5Bdata%5D=class_expire&columns%5B11%5D%5Bname%5D=&columns%5B11\
    %5D%5Bsearchable%5D=true&columns%5B11%5D%5Borderable%5D=true&columns%5B11%5D%5Bsearch%5D%5Bvalue\
    %5D=&columns%5B11%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B12%5D%5Bdata%5D=passwd&columns%5B12%\
    5D%5Bname%5D=&columns%5B12%5D%5Bsearchable%5D=true&columns%5B12%5D%5Borderable%5D=true&columns%5\
    B12%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B12%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B13%5D%5Bda\
    ta%5D=port&columns%5B13%5D%5Bname%5D=&columns%5B13%5D%5Bsearchable%5D=true&columns%5B13%5D%5Bord\
    erable%5D=true&columns%5B13%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B13%5D%5Bsearch%5D%5Bregex%5D=fa\
    lse&columns%5B14%5D%5Bdata%5D=method&columns%5B14%5D%5Bname%5D=&columns%5B14%5D%5Bsearchable%5D=\
    true&columns%5B14%5D%5Borderable%5D=true&columns%5B14%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B14%5D\
    %5Bsearch%5D%5Bregex%5D=false&columns%5B15%5D%5Bdata%5D=protocol&columns%5B15%5D%5Bname%5D=&colu\
    mns%5B15%5D%5Bsearchable%5D=true&columns%5B15%5D%5Borderable%5D=true&columns%5B15%5D%5Bsearch%5D\
    %5Bvalue%5D=&columns%5B15%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B16%5D%5Bdata%5D=obfs&columns\
    %5B16%5D%5Bname%5D=&columns%5B16%5D%5Bsearchable%5D=true&columns%5B16%5D%5Borderable%5D=true&col\
    umns%5B16%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B16%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B17%5\
    D%5Bdata%5D=online_ip_count&columns%5B17%5D%5Bname%5D=&columns%5B17%5D%5Bsearchable%5D=true&colu\
    mns%5B17%5D%5Borderable%5D=false&columns%5B17%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B17%5D%5Bsearc\
    h%5D%5Bregex%5D=false&columns%5B18%5D%5Bdata%5D=last_ss_time&columns%5B18%5D%5Bname%5D=&columns%\
    5B18%5D%5Bsearchable%5D=true&columns%5B18%5D%5Borderable%5D=false&columns%5B18%5D%5Bsearch%5D%5B\
    value%5D=&columns%5B18%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B19%5D%5Bdata%5D=used_traffic&co\
    lumns%5B19%5D%5Bname%5D=&columns%5B19%5D%5Bsearchable%5D=true&columns%5B19%5D%5Borderable%5D=tru\
    e&columns%5B19%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B19%5D%5Bsearch%5D%5Bregex%5D=false&columns%5\
    B20%5D%5Bdata%5D=enable_traffic&columns%5B20%5D%5Bname%5D=&columns%5B20%5D%5Bsearchable%5D=true&\
    columns%5B20%5D%5Borderable%5D=true&columns%5B20%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B20%5D%5Bse\
    arch%5D%5Bregex%5D=false&columns%5B21%5D%5Bdata%5D=last_checkin_time&columns%5B21%5D%5Bname%5D=&\
    %5B21%5D%5Bsearchable%5D=true&columns%5B21%5D%5Borderable%5D=false&columns%5B21%5D%5Bsearch%5D%5\
    Bvalue%5D=&columns%5B21%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B22%5D%5Bdata%5D=today_traffic&\
    columns%5B22%5D%5Bname%5D=&columns%5B22%5D%5Bsearchable%5D=true&columns%5B22%5D%5Borderable%5D=t\
    rue&columns%5B22%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B22%5D%5Bsearch%5D%5Bregex%5D=false&columns\
    %5B23%5D%5Bdata%5D=enable&columns%5B23%5D%5Bname%5D=&columns%5B23%5D%5Bsearchable%5D=true&column\
    s%5B23%5D%5Borderable%5D=true&columns%5B23%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B23%5D%5Bsearch%5\
    D%5Bregex%5D=false&columns%5B24%5D%5Bdata%5D=reg_date&columns%5B24%5D%5Bname%5D=&columns%5B24%5D\
    %5Bsearchable%5D=true&columns%5B24%5D%5Borderable%5D=true&columns%5B24%5D%5Bsearch%5D%5Bvalue%5D\
    =&columns%5B24%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B25%5D%5Bdata%5D=reg_ip&columns%5B25%5D%\
    5Bname%5D=&columns%5B25%5D%5Bsearchable%5D=true&columns%5B25%5D%5Borderable%5D=true&columns%5B25\
    %5D%5Bsearch%5D%5Bvalue%5D=&columns%5B25%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B26%5D%5Bdata%\
    5D=auto_reset_day&columns%5B26%5D%5Bname%5D=&columns%5B26%5D%5Bsearchable%5D=true&columns%5B26%5\
    D%5Borderable%5D=true&columns%5B26%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B26%5D%5Bsearch%5D%5Brege\
    x%5D=false&columns%5B27%5D%5Bdata%5D=auto_reset_bandwidth&columns%5B27%5D%5Bname%5D=&columns%5B2\
    7%5D%5Bsearchable%5D=true&columns%5B27%5D%5Borderable%5D=true&columns%5B27%5D%5Bsearch%5D%5Bvalu\
    e%5D=&columns%5B27%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B28%5D%5Bdata%5D=ref_by&columns%5B28\
    %5D%5Bname%5D=&columns%5B28%5D%5Bsearchable%5D=true&columns%5B28%5D%5Borderable%5D=true&columns%\
    %5D%5Bsearch%5D%5Bvalue%5D=&columns%5B28%5D%5Bsearch%5D%5Bregex%5D=false&columns%5B29%5D%5Bdata%\
    5D=ref_by_user_name&columns%5B29%5D%5Bname%5D=&columns%5B29%5D%5Bsearchable%5D=true&columns%5B29\
    %5D%5Borderable%5D=false&columns%5B29%5D%5Bsearch%5D%5Bvalue%5D=&columns%5B29%5D%5Bsearch%5D%5Br\
    egex%5D=false&columns%5B30%5D%5Bdata%5D=top_up&columns%5B30%5D%5Bname%5D=&columns%5B30%5D%5Bsear\
    chable%5D=true&columns%5B30%5D%5Borderable%5D=false&columns%5B30%5D%5Bsearch%5D%5Bvalue%5D=&colu\
    mns%5B30%5D%5Bsearch%5D%5Bregex%5D=false&order%5B0%5D%5Bcolumn%5D=1&order%5B0%5D%5Bdir%5D=asc&st\
    art=0&search%5Bvalue%5D=&search%5Bregex%5D=false";

    let post = client.post(&post_address)
        .form(&[("draw", "1"), ("length", "1")])
        .send().await?
        .text().await?
        .replace("recordsTotal", "records_total");
    let records: Records = serde_json::from_str(&post)?;

    let post = client.post(&post_address)
        .form(&[("draw", "1"), ("length", &records.records_total.to_string())])
        .send().await?
        .text().await?;
    let user: User = serde_json::from_str(&post)?;

    Ok(user)
}