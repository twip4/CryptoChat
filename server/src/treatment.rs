pub mod treatment {
    use crate::Client;
    pub fn analyse(mut message: String, user: Client) -> String{
        message = format!("{} : {}", user.pseudo, message.trim_end_matches('\n'));
        return message;
    }
}