use crate::msg::Msg;
use crate::tconnection::Tconnection;
use crate::user::User;
use serde::{Serialize, Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub struct Info {
    pub user: User,
    pub channels: Vec<String>,
}
#[derive(Debug, PartialEq)]
pub struct Winner {
    pub name: String,
    pub points: u32,
    pub channel: String,
}

impl Winner {
    pub fn new(name: &str, points: u32, channel: &str) -> Winner {
        Winner {
            name: name.to_string(),
            points: points,
            channel: channel.to_string()
        }
    }
}

pub async fn add_points(connnection: &mut Tconnection,x: Option<Winner>){
    if x.is_some() {
        let x = x.unwrap();
        connnection.send_channel_msg(x.channel ,&format!("!+ {} {}",x.name, x.points)).await.unwrap();
    }
}

pub fn get_winner_prize(msg: &Msg) -> Option<Winner> {

    let string_arr = msg.msg.as_ref().unwrap();
    if let Some(x) = check_tanat(&string_arr, &msg.channel) {
        return Some(x);
    }
    // have to check bingo before tip 
    if let Some(x) = check_bingo(&string_arr, &msg.channel) {
        return Some(x);
    }
    if let Some(x) = check_los(&string_arr, &msg.channel) {
        return Some(x);
    }
    if let Some(x) = check_tip(&string_arr, &msg.channel) {
        return Some(x);
    }
    /*if let Some(x) = check_test(&string_arr) {
        return Some(x);
    }*/
    None
}

fn check_tanat(x: &Vec<String>, channel: &str) -> Option<Winner> {
    if x.contains(&"Taannaat".to_string()) && x.contains(&"Clap".to_string()) && x.contains(&"Wow,".to_string()) {
        return Some(Winner::new("Taannaat",x[15].parse().unwrap() , channel));
    }
    if x.contains(&"Taannaat".to_string()) && x.contains(&"třetí".to_string()) && x.contains(&"zase".to_string()){
        return Some(Winner::new("Taannaat",3000 , channel));
    }
    if x.contains(&"Taannaat".to_string()) && x.contains(&"druhý".to_string()) && x.contains(&"bonus".to_string()){
        return Some(Winner::new("Taannaat",5000 , channel));
    }
    if x.contains(&"bingo".to_string()) && x.contains(&"napoprvé".to_string()) && x.contains(&"Taannaat".to_string()) {
        return Some(Winner::new("Taannaat",9000 , channel));
    }
    None
}

fn check_los(x: &Vec<String>, channel: &str) -> Option<Winner> {
    if x.contains(&"vyhrál/a".to_string()) {
        if x.contains(&"200".to_string()){
            return Some(Winner::new(&x[0],200,channel));
        }
        if x.contains(&"50".to_string()){
            return Some(Winner::new(&x[0],50,channel));
        }
        if x.contains(&"1000".to_string()){
            return Some(Winner::new(&x[0],1000,channel));
        }
        if x.contains(&"5000".to_string()){
            return Some(Winner::new(&x[0],5000,channel));
        }
    }
    None            
}

fn check_tip(x: &Vec<String>, channel: &str) -> Option<Winner> {
    if x.contains(&"vedle,".to_string()) {
        if x.len() > 31 && x[31] == "5" {
            return Some(Winner::new(&x[0],200,channel));
        }
        if x.len() > 28 && x[28] == "4"{
            return Some(Winner::new(&x[0],300,channel));
        }
            //hotovo
        if x.len() > 28 && x[28] == "3" {
            return Some(Winner::new(&x[0],400,channel));
        }
        if x.len() > 28 && x[28] == "2" {
            return Some(Winner::new(&x[0],500,channel));
        }
                //hotovo
        if x.len() > 30 && x[30] == "1"  {
            return Some(Winner::new(&x[0],800,channel));
        }
    }
    if x.contains(&"BULLSEYE!!!".to_string()) {
        return Some(Winner::new(&x[0],3000,channel));
    }
    None
}

fn check_bingo(x: &Vec<String>, channel: &str) -> Option<Winner> {
    if x.contains(&"uhodl/a".to_string()){
        if x.len() > 19 && !x.contains(&"BULLSEYE!!!".to_string()) {
            return Some(Winner::new(&x[0],x[19].parse().unwrap(),channel));
        }
        if x.contains(&"1.".to_string()) {
            return Some(Winner::new(&x[0],9000,channel));
        }
        if x.contains(&"2.".to_string()) {
            return Some(Winner::new(&x[0],5000,channel));
        }
        if x.contains(&"3.".to_string()) {
            return Some(Winner::new(&x[0],3000,channel));
        }
    }
    None
}
#[cfg(test)]
mod tests {
    use crate::contest::{check_bingo, check_tip};

    use super::{ check_tanat, Winner, check_los};

    #[test]
    fn tanat_basic_bingo() {
        let test_string = "ResidentSleeper Clap Wow, Taannaat vyhrál další bingo s číslem 469, jaké překvapení... připiš si těch 1500 mikoinů a třeba se jimi udav PepeSpit Další bingo začne až za 6 hodin FeelsLateMan ";
        let test: Vec<String> = test_string.split_ascii_whitespace().map(|s| s.to_string()).collect();
        let x = check_tanat(&test, "random").unwrap();
        let y = Winner::new("Taannaat", 1500, "random");
        assert_eq!(x, y);

    }

    #[test]
    fn los_50() {
        let test_string = "name si koupil/a los: Setřel/a jsi všechna políčka a vyhrál/a jsi usmolených 50 mikoinů! O výhru se přihlaš u moderátorů!. Další los můžeš koupit za hodinu! FeelsLateMan A pokud pěkně nepoprosíš a nepoděkuješ, tak nedostaneš nic peepoWeird ";
        let test_vec :Vec<String> = test_string.split_ascii_whitespace().map(|s| s.to_owned()).collect();
        let x = check_los(&test_vec, "random").unwrap();
        let y = Winner::new("name",50 ,"random");
        assert_eq!(x,y);
    }
    #[test]
    fn los_200() {
        let test_string = "name si koupil/a los: Setřel/a jsi všechna políčka a vyhrál/a jsi usmolených 200 mikoinů! O výhru se přihlaš u moderátorů!. Další los můžeš koupit za hodinu! FeelsLateMan";
        let test_vec :Vec<String> = test_string.split_ascii_whitespace().map(|s| s.to_owned()).collect();
        let x = check_los(&test_vec, "random").unwrap();
        let y = Winner::new("name",200 ,"random");
        assert_eq!(x,y);
    }
    #[test]
    fn generic_bingo() {
        let test_string = "name Tipnul/a sis číslo 164 a naprosto neuvěřitelně jsi uhodl/a hledané číslo! Pog Dej vědět modům, aby ti připsali 1250 bodů, užívej si této výhry, kterou do soutěže vložili ostatní uživatelé (a jako vítěz máš ještě malý bonus) peepoComfy Další bingo začne až za 6 hodin FeelsLateMan";
        let test_vec :Vec<String> = test_string.split_ascii_whitespace().map(|s| s.to_owned()).collect();
        let x = check_bingo(&test_vec, "random").unwrap();
        let y = Winner::new("name",1250 ,"random");
        assert_eq!(x,y);
    }
    #[test]
    fn tip_miss_one() {
        let test_string = "name Tipnul/a sis číslo 56, které sice nebylo přesně to, které jsi měl/a uhodnout, ale bylo jen o kousíček vedle Hype Hledané číslo bylo 55, ty jsi byl pouze o 1 vedle, takže si napiš modům o krásných 800 bodů PETTHEMOD Další tip můžeš zase až za hodinu";
        let test_vec :Vec<String> = test_string.split_ascii_whitespace().map(|s| s.to_owned()).collect();
        let x = check_tip(&test_vec, "random").unwrap();
        let y = Winner::new("name",800,"random");
        assert_eq!(x,y);
    }
    #[test]
    fn bullseye() {
        let test_string = "Taannaat Tipnul/a sis číslo 44 - BULLSEYE!!! Je to neskutečné, ale uhodl/a jsi hledané číslo Pog Hledané číslo bylo 44, napiš si modům o nádherných 3000 bodů PETTHEMOD zbMoney Další tip můžeš zase až za hodinu ";
        let test_vec :Vec<String> = test_string.split_ascii_whitespace().map(|s| s.to_owned()).collect();
        let x = check_tip(&test_vec, "mik_vr").unwrap();
        let y = Winner::new("Taannaat",3000 ,"mik_vr");
        assert_eq!(x,y);
    }
    #[test]
    fn tip_miss_three() {
        let test_string = "semenotyvemeno Tipnul/a sis číslo 23, které sice nebylo přesně to, které jsi měl/a uhodnout, ale dostal/a jsi se do tolerance. Hledané číslo bylo 20, ty jsi byl o 3 vedle, takže si napiš modům o 400 bodů PETTHEMOD Další tip můžeš zase až za hodinu";
        let test_vec :Vec<String> = test_string.split_ascii_whitespace().map(|s| s.to_owned()).collect();
        let x = check_tip(&test_vec, "mik_vr").unwrap();
        let y = Winner::new("semenotyvemeno",400 ,"mik_vr");
        assert_eq!(x,y);
    }
    #[test]
    fn tip_miss_four() {
        let test_string = "Taannaat Tipnul/a sis číslo 88, které sice nebylo přesně to, které jsi měl/a uhodnout, ale dostal/a jsi se do tolerance. Hledané číslo bylo 84, ty jsi byl o 4 vedle, takže si napiš modům o 300 bodů PETTHEMOD Další tip můžeš zase až za hodinu";
        let test_vec :Vec<String> = test_string.split_ascii_whitespace().map(|s| s.to_owned()).collect();
        let x = check_tip(&test_vec, "mik_vr").unwrap();
        let y = Winner::new("Taannaat",300 ,"mik_vr");
        assert_eq!(x,y);
    }
}