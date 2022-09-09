use super::template;
use bytes::Bytes;
use mail_builder::MessageBuilder;
use rusoto_core::{Region, RusotoError};
use rusoto_ses::{RawMessage, SendRawEmailError, SendRawEmailRequest, Ses, SesClient};

pub async fn send_confirmation_email(
    to: String,
    code: i16,
) -> Result<(), RusotoError<SendRawEmailError>> {
    let sesclient = SesClient::new(Region::EuWest3);
    let data_email = MessageBuilder::new()
        .from(("The squareview team", "john@jenoh.dev"))
        .to(to.clone())
        .subject("Hello, world!")
        .html_body(template::confirmation_email(code))
        .write_to_string()
        .unwrap();

    match sesclient
        .send_raw_email(SendRawEmailRequest {
            raw_message: RawMessage {
                data: Bytes::from(base64::encode(data_email.to_string())),
            },
            destinations: Some(vec![to]),
            source: Some("john@jenoh.dev".to_string()),
            ..SendRawEmailRequest::default()
        })
        .await
    {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
