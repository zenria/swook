use clap::Parser;
use slack_hook3::{AttachmentBuilder, PayloadBuilder, Slack};

/// Send message to slack channels using the Slack Incoming Webhook API
///
/// Note: environment variable are read from environment enriched by .env file found in current directory and parents
#[derive(Parser)]
#[command(version)]
struct Opt {
    /// Webhook url, if not present it is read from SWOOK_WEBHOOK_URL environment variable
    #[arg(long = "webhook-url", short = 'u', env = "SWOOK_WEBHOOK_URL")]
    webhook_url: String,
    /// Channel to publish to, if not present it is read from SWOOK_CHANNEL environment variable
    ///
    /// If ommited, the message will be published to the default webhook channel
    #[arg(long = "channel", short = 'c', env = "SWOOK_CHANNEL")]
    channel: Option<String>,
    /// The username (optional)
    #[arg(long = "username", short = 'n')]
    username: Option<String>,
    /// The user icon url (optional)
    #[arg(long = "user-icon-url", short = 'i')]
    user_icon_url: Option<String>,
    /// The text of an attachement (optional)
    #[arg(long = "attachment")]
    attachment: Vec<String>,
    /// The color of the attachment (valid only if a text has been specified)
    #[arg(long = "attachment-color")]
    attachment_color: Option<String>,
    /// The text of the message to publish
    text: String,
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let opt: Opt = Opt::parse();

    let payload = PayloadBuilder::new().text(opt.text.clone());
    // change default webhook channel
    let payload = match &opt.channel {
        Some(channel) => payload.channel(channel),
        None => payload,
    };
    // change username if needed
    let payload = match &opt.username {
        Some(username) => payload.username(username),
        None => payload,
    };
    // change icon url is needed
    let payload = match &opt.user_icon_url {
        Some(icon_url) => payload.icon_url(icon_url),
        None => payload,
    };
    // add attachmement if any

    let payload = payload.attachments(opt.attachment.iter().map(|attachment|{
        let attachment = AttachmentBuilder::new(attachment.clone()).text(attachment.clone());
            let attachment = match &opt.attachment_color {
                Some(color) => attachment.color(color.clone()),
                None => attachment,
            };
            match attachment.build() {
                Ok(a) => a,
                Err(e) => {
                    eprintln!("Unable to parse color: {}", e);
                    eprintln!("Slack color can be either \"good\", \"warning\", \"danger\" or a web hexadecimal color, like \"#fecc00\" or \"#ccc\"");
                    std::process::exit(1);
                }
    }}).collect()).build().unwrap();

    // finally send to slack
    match Slack::new(opt.webhook_url) {
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
