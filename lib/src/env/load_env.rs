use dotenvy::dotenv;
pub fn load_system_properties() {
    // loading the .env file from the current project root folder. client..etc..
    dotenv().ok();
}
