use serde::Deserialize;
use slack_hook3::{AttachmentBuilder, PayloadBuilder, Slack};
use structopt::StructOpt;

/// Send message to slack channels using the Slack Incoming Webhook API
///
/// Note: environment variable are read from environment enriched by .env file found in current directory and parents
#[derive(StructOpt)]
struct Opt {
    /// Webhook url, if not present it is read from SWOOK_WEBHOOK_URL environment variable
    #[structopt(long = "webhook-url", short = "u")]
    webhook_url: Option<String>,
    /// Channel to publish to, if not present it is read from SWOOK_CHANNEL environment variable
    ///
    /// If ommited, the message will be published to the default webhook channel
    #[structopt(long = "channel", short = "c")]
    channel: Option<String>,
    /// The username (optional)
    #[structopt(long = "username", short = "n")]
    username: Option<String>,
    /// The user icon url (optional)
    #[structopt(long = "user-icon-url", short = "i")]
    user_icon_url: Option<String>,
    /// The text of an attachement (optional)
    #[structopt(long = "attachment")]
    attachment: Option<String>,
    /// The color of the attachment (valid only if a text has been specified)
    #[structopt(long = "attachment-color")]
    attachment_color: Option<String>,
    /// The text of the message to publish
    text: String,
}

#[derive(Deserialize)]
struct Env {
    webhook_url: Option<String>,
    channel: Option<String>,
}

#[tokio::main]
async fn main() {
    let _ = dotenv::dotenv();
    let env: Option<Env> = envy::prefixed("SWOOK_").from_env().ok();
    let opt: Opt = Opt::from_args();

    let webhook_url = opt
        .webhook_url
        .as_ref()
        .or(env.as_ref().map(|e| e.webhook_url.as_ref()).flatten())
        .unwrap_or_else(|| {
            eprintln!("No webhook url provided!");
            std::process::exit(1);
        })
        .as_str();
    let channel = opt
        .channel
        .as_ref()
        .or(env.as_ref().map(|e| e.channel.as_ref()).flatten())
        .map(String::as_str);

    let payload = PayloadBuilder::new().text(opt.text);
    // change default webhook channel
    let payload = match channel {
        Some(channel) => payload.channel(channel),
        None => payload,
    };
    // change username if needed
    let payload = match opt.username {
        Some(username) => payload.username(username),
        None => payload,
    };
    // change icon url is needed
    let payload = match opt.user_icon_url {
        Some(icon_url) => payload.icon_url(&icon_url),
        None => payload,
    };
    // add attachmement if any
    let payload = match opt.attachment {
        Some(attachment) => {
            let attachment = AttachmentBuilder::new(attachment.clone()).text(attachment);
            let attachment = match opt.attachment_color {
                Some(color) => attachment.color(color),
                None => attachment,
            };
            let attachment = match attachment.build() {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Unable to parse color: {}", e);
                    eprintln!("Slack color can be either \"good\", \"warning\", \"danger\" or a web hexadecimal color, like \"#fecc00\" or \"#ccc\"");
                    std::process::exit(1);
                }
            };
            payload.attachments(vec![attachment])
        }
        None => payload,
    }
    .build()
    .unwrap();
    // finally send to slack
    match Slack::new(webhook_url) {
        Ok(slack) => {
            if let Err(e) = slack.send(&payload).await {
                eprintln!("Unable to notify slack channel: {}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("Unable to notify slack channel: {}", e);
            std::process::exit(1);
        }
    }
}
