use serde_derive::Deserialize;
#[derive(Deserialize, Debug)]
pub struct ProfileBridge {
    pub username: String,
    pub password: String,
    pub email: String,
    pub fullname: String,
    pub phone_number: String,
    pub profile_picture_url: String,
}
#[derive(Deserialize, Debug)]
pub struct LoginRequestBridge {
    pub email: String,
    pub password: String,
    pub url: String,
    pub ipv4: String,
    pub user_agen: String,
}
