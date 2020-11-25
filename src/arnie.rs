use oorandom::Rand32;

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
        _.-\/       \/-._
      .'   (\`.___.'/)   `.
     /      \`.___.'/      `.
    /        `.___.'         \
    |     |             \     \
    |     |   .      .  |\     \
    |     |             | \     \
     \     \            |  \     \
      \     \           |.' `.    \
       `.    \         .'     `.   \
  _.._   `.   `-. ___ /        /`.  `.
'    "-._|`\    `.__)       .'   /    `.
|         `-.\     \/      .'   / /\  )|\.
 \          _/ / /|/     .'    (_/ / / | \)
  `._      (__/_/-/   ..'         (_/| |\_)
     ``--._____.-(     `.            `-'
                  `--.   `.
                    (_/\ \\\
                       /_///
"#;
const LE_MONKE_PHRASE: &str = "UH OH, STINKY!";
const LOL_JK: &str = "--LOL-JK";

pub fn get_arnie(phrase: String, seed: u64) -> String {
    let mut phrase: &str = &phrase.to_ascii_uppercase();
    let output: &str;
    let mut rng = Rand32::new(seed);
    if phrase == LOL_JK {
        return "".to_string();
    } else if phrase == LE_MONKE_PHRASE {
        output = LE_MONKE_ASCII;
    } else if rng.rand_float() < 0.01 {
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
    output.replace("{}", phrase)
}
