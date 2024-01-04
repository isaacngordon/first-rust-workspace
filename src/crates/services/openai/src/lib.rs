pub mod chat;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chat_can_prompt() {
        let res = chat::prompt_sync(
            String::from("MARCO!"),
            vec![]   
        );

        assert_eq!(res.is_ok(), true);
    }
}
