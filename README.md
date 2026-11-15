# AnimeThemes Discord Bot

## Prerequisites

- Go to [Discord Developer Portal](https://discord.com/developers/applications) and create a application.
- Go to the Bot tab and save the token (reset the token if needed).
- Disable "Public Bot" option and enable the following options: Presence Intent, Server Members Intent and Message Content Intent.
- Replace `<BOT_ID>` with the APPLICATION ID and run this in your browser:

```
https://discord.com/oauth2/authorize?client_id=<BOT_ID>&permissions=8&scope=bot
```

## Running

```
# Set the environment variables
cp .env.example .env
nano .env

# Build and run the Docker image
docker build -t animethemes-discord-rs:local .
docker compose up -d
```