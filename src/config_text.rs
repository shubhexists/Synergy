const CONFIG_TEXT: &str =
    "# Edit this configuration file accroding to your needs to run the executable.
# The configuration file is in YAML format.
";
const DATABASE_DESCRIPTION: &str = "Current Supported Databases are : Mongodb";
const URI_DESCRIPTION: &str = "# Add your database connection string here ";
const AUTH_HEADER_DESCRIPTION: &str =
    "Add your auth header here if you want to use it. None by default.";
const API_KEY_DESCRIPTION: &str = "# Add your api key here";
const DATABASE_DESC: &str = "# Choose the database you want to connect to";

pub fn get_config_file_text() -> String {
    let text: String = format!("{CONFIG_TEXT}\n\n");
    let text: String = format!("{text}{DATABASE_DESC} \n");
    let text: String = format!("{text}database: \"\" # {DATABASE_DESCRIPTION}\n");
    let text: String = format!("{text}uri: \"\" {URI_DESCRIPTION} \n\n");
    let text: String = format!("{text}auth_header: \"\"");
    let text: String = format!("{text} # {AUTH_HEADER_DESCRIPTION} \n\n");
    let text: String = format!("{text}api_key: \"\" {API_KEY_DESCRIPTION} \n");
    text
}
