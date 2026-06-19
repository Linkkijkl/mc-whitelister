# Minecraft server whitelister

Serves a website with nice and siple information box, [Dynmap](https://modrinth.com/plugin/dynmap) and a whitelist request form. Players can whitelist themselves by entering a correct passord set by an administrator.

Uses:
- Confusing grief bots with a web ui
- Preventing grief with keeping your server semi-private
- Display server information for new players before they join 

The whitelisting is done trough [Rcon](https://minecraft.wiki/w/RCON). Enable it in your Minecraft server settings.

Our instance is running in [mine.linkkijkl.fi](https://mine.linkkijkl.fi).

### Deploying

Docker Compose is recommended. Example compose.yml:
```yaml:compose.yml
services:
  web:
    build: .
    restart: unless-stopped
    ports:
      - "8080:8080"
    environment:
      - "RCON_URL=your-mc-server-address:25575"
      - "RCON_PASSWORD=your-mc-rcon-password"
      - "WHITELIST_PASSWORD=changeme"
      - "TITLE=My Minecraft server"
      - "MAP_URL=https://minekartta.linkkijkl.fi"
      - "USERNAME_LABEL=Minecraft username:"
      - "PASSWORD_LABEL=Whitelist password:"
      - "SUBMIT_LABEL=Submit"
      - "INFO=
<p><strong>Sample info</strong></p>
<p>This section can contain anything you want.</p>
<p>Maybe some rules:</p>
<ul>
  <li>this and</li>
  <li>that</li>
</ul>"
```

### Development

Use the provided Devcontainer configuration, or install Rust toolchain and Cargo watch.
Then use `cargo run` to compile and run, or `cargo watch -x run` to watch for source changes.

### Useless trivia
- The web framework used by this server, [may_minihttp](https://github.com/Xudong-Huang/may_minihttp), is the fastest web framework available based on the [Techempower leaderboard](https://www.techempower.com/benchmarks/#section=data-r23)
- All static files are compiled inside the built binary preventing even the minute overhead caused by file IO
- There are at least 3 vulnerabilities in the code. Denial of service via resource exhaustion and server crashing, and password mitigation. They naturally can't be disclosed here before someone fixes them, but I encourage you to try finding and exploiting them. Maybe even fix them? Just think of the juicy CV content if you're an aspiring cyber security expert :)
