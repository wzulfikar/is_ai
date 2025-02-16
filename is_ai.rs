pub fn is_ai(text: &str) -> bool {
  text.to_lowercase().starts_with("ah, i see")
}
