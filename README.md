# Arnie
I needed a command line Arnold Schwarzenegger. You need a command line Arnold Schwarzenegger.

[Try the wasm version here!](https://jamesfator.com/arnie/?phrase=Hello%20GitHub!)

## Installing
```bash
# Clone the repo
$ git clone https://github.com/JamesFator/Arnie.git
$ cd Arnie
# Build the binary
$ cargo build --bin cli --release
# Place the binary somewhere in your PATH, I don't know.
$ cp target/release/arnie /usr/local/bin/
```

## WebAssembly
```bash
# Install cargo web
$ cargo install cargo-web
# Build and host the js/wasm
$ cargo web start --bin arnie_js --features web
# Access http://127.0.0.1:8000
# Use the query string to add some spice http://127.0.0.1:8000/?phrase=UH%20OH,%20STINKY!
```


## Features
```bash
# Standard arnie.
$ arnie
     I'LL BE BACK
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

# Rare arnie
$ arnie
                ,,,,,,,,,,---''''---,,,,,,,,,,
      --''''''''          ````][''''          ''''''''--
                           _3'''':.
 _                  .,---------------.
 \\    / _______./  |[__]| o   |J@\"\\__
  \\==o=========:;    |____|[IL__|''''/_/')
     /            `'-,._____===\=_____.,-'
                          \         \     ,
                    \"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"\"
                 GET TO THE CHOPPA

# Get arnie to say something.
$ arnie "Hello world"
     HELLO WORLD
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

# In case you didn't actually want to run the command.
$ arnie --LOL-JK

# Don't know why this is here.
$ arnie "Uh oh, stinky\!"
       .--.  .-'''-.  .--.
      /."".v'.-. .-.`v.""\\
      ||  / / O| | O\ \  ||    UH OH, STINKY!
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
```
