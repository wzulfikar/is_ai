mod is_ai;
use is_ai::is_ai;

fn main() {
    let test_strings = [
      "Ah, I see the issue", // AI
      "ah, i see", // AI
      "Hello world", // Not AI
    ];
    for s in test_strings {
      println!("{} → {}", s, if is_ai(s) {"AI"} else {"Not AI"});
    }
}
