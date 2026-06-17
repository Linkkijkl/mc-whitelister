# Minecraft server whitelister

Serves a website with nice and siple information box, [Dynmap](https://modrinth.com/plugin/dynmap) and a whitelist request form. Players can whitelist themselves by entering a correct passord, which is set by an environment vairable.

Uses:
- Preventing grief bots with a web ui
- Preventing real grief with keeping your server semi-private
- Show server information for new players before they join 

The whitelisting is done trough [Rcon](https://minecraft.wiki/w/RCON). Enable it in your Minecraft server settings.

### Running

Docker Compose is recommended. Example docker compose file:
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