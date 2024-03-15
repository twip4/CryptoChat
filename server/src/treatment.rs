pub mod treatment {
    use crate::Client;
    pub fn analyse(mut message: String, user: Client) -> (String, bool) {
        if message.starts_with("!") {
            return match message.trim_end_matches('\n').as_ref() {
                "!help" => ("Tiens de l'aide, sale batard.".to_string(), true),
                "!quit" => ("KO".to_string(), true),
                _ => ("Commande inconnue.".to_string(), true),
            };
        }

        message = format!("{}: {}", user.pseudo, message.trim_end_matches('\n'));
        (message, false)
    }
}
