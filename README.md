# swook

_basic slack webhook cli_

Send message to slack channels using the [Slack Incoming Webhook API](https://api.slack.com/messaging/webhooks).

## Installation

You must have a Rust toolchain installed on your system to install this cli. Please reach me if you want to help packaging it!

```
cargo install swook
```

## Usage

````
swook 1.0.0
Send message to slack channels using the Slack Incoming Webhook API

Note: environment variable are read from environment enriched by .env file found in current directory and parents

USAGE:
    swook [OPTIONS] <text>

FLAGS:
    -h, --help       
            Prints help information

    -V, --version    
            Prints version information


OPTIONS:
        --attachment <attachment>                
            The text of an attachement (optional)

        --attachment-color <attachment-color>    
            The color of the attachment (valid only if a text has been specified)

    -c, --channel <channel>
            Channel to publish to, if not present it is read from SWOOK_CHANNEL environment variable
            
            If ommited, the message will be published to the default webhook channel
    -i, --user-icon-url <user-icon-url>          
            The user icon url (optional)

    -n, --username <username>                    
            The username (optional)

    -u, --webhook-url <webhook-url>
            Webhook url, if not present it is read from SWOOK_WEBHOOK_URL environment variable


ARGS:
    <text>    
            The text of the message to publish
````

## License

MIT
