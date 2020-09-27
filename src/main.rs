use rand::Rng;
use std::env;

const ARNIE_ASCII: &str = r#"
     {}
                       ____
                     <((((((\\\
                     /      . }}\
                     ;--..--._|}}
  (\                 '--/\--'  )
   \\                | '-'  :'|
    \\               . -==- .-|
     \\               \.__.'   \--._
     [\\          __.--|       //  _/'--.
     \ \\       .'-._ ('-----'/ __/      \
      \ \\     /   __>|      | '--.       |
       \ \\   |   \   |     /    /       /
        \ '\ /     \  |     |  _/       /
         \  \       \ |     | /        /
          \  \      \        /
"#;
const ARNIE_PHRASE: &str = "I'LL BE BACK";

const ARNIE_RARE_ASCII: &str = r#"
                ,,,,,,,,,,---''''---,,,,,,,,,,
      --''''''''          ````][''''          ''''''''--
                           _3'''':.
 _                  .,---------------.
 \\    / _______./  |[__]| o   |J@\"\\__
  \\==o=========:;    |____|[IL__|''''/_/')
     /            `'-,._____===\=_____.,-'
                          \         \     ,
                    \"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"
                 {}
"#;
const ARNIE_RARE_PHRASE: &str = "GET TO THE CHOPPA";

const LE_MONKE_ASCII: &str = r#"
       .--.  .-'''-.  .--.
      /."".v'.-. .-.`v.""\\
      ||  / / O| | O\ \  ||    {}
      \\_/| \__| |__/ |\_//
       `-'\  .-n-n-.  /`-'
        .-\/       \/-.
      .'   (\`.___.'/)   `.
     /      \`.___.'/      `.
    /        `.___.'         \
    |     |             \     \
    |     |   .      .  |\     \
    |     |             | \     \
     \     \            |  \     \
      \     \           |.' `.    \
       .    \         .'     .   \
  ..   .   `-. ___ /        /.  `.
'    "-._|`\    .__)       .'  /    .
|         `-.\     \/      .'   / /\  )|\.
 \          _/ / /|/     .'    (_/ / / | \)
  `._      (_/_/-/   ..'         (_/| |\)
     ``--._____.-(     `.            `-'
                  --.   .
                    (_/\ \\\
                       /_///
"#;
const LE_MONKE_PHRASE: &str = "UH OH, STINKY!";
const LOL_JK: &str = "--LOL-JK";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let mut phrase: &str = &args.join(" ").to_ascii_uppercase();
    let output: &str;
    let mut rng = rand::thread_rng();
    if phrase == LOL_JK {
        return;
    } else if phrase == LE_MONKE_PHRASE {
        output = LE_MONKE_ASCII;
    } else if rng.gen_range(1, 100) == 1 {
        output = ARNIE_RARE_ASCII;
        if phrase.is_empty() {
            phrase = ARNIE_RARE_PHRASE;
        }
    } else {
        output = ARNIE_ASCII;
        if phrase.is_empty() {
            phrase = ARNIE_PHRASE;
        }
    }
    println!("{}", output.replace("{}", phrase));
}
