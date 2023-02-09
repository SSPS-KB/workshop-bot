# Workshop bot
This repository contains the source code of SSPŠ KB discord bot.

## Deploying
This project is automatically built and deployed with docker, so you can simply run it with
```shell
docker run ghcr.io/ssps-kb/workshop-bot
```
if the pull and start was successful, you'll get `There is no Config.toml in current directory nor in /etc/workshop-bot`
error. See [configuration](#configuration)

### Tags
Here is a list of tags you can find in the registry, and it's description.

| Tag     | Description                                    |
|---------|------------------------------------------------|
| latest  | The latest stable (tagged) release             |
| x.x.x   | The specific version (stable) ie. 0.3.0        |
| edge    | Bleeding edge version, contents of main branch |
| sha-xxx | Specific commit. ie. adbfca5 = sha-adbfca5     |

## Configuration
Most of this project's configuration is self-explanatory if you ever set up a discord bot.
The project looks in current directory for `Config.toml` and if there is no configuration it checks `/etc/workshop-bot`.
If running in docker, you'll want to either add a volume to `/app/Config.toml` or even better add a whole folder to
`/etc/workshop-bot`.

Instead of using names, the bot is using IDs to find channels.

### Discord token (discord_token)
This is the only required value, it is used to authenticate with discord. NEVER SHARE IT WITH ANYONE.
To get it create a bot on https://discord.com/developers. If you don't know how to do that, search "How to create a
discord bot", it has been explained over thousand times.

### Workshop invite channel (workshop_invite_channel)
This configuration option is basically useless. It's used to create and log an invite at the bot's startup.  
It's only used for the invite to look more professional.

### Guilds config
SOON<sup>TM</sup>

## License
<sup>
Licensed under either the <a href="LICENSE-APACHE">Apache License, Version 2.0</a> or
<a href="LICENSE-MIT">MIT license</a> at your option.
</sup>
<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this repository by you, shall be dual licensed as above, without any additional terms or conditions. 
</sub>
