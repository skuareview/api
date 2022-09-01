// use super::template;
// use bytes::Bytes;
// use mail_builder::MessageBuilder;
// use rusoto_ses::{Ses, SesClient};

// pub async fn send_alert_email(to: String) -> bool {
//     println!("{}", template::alert_template());
//     let sesclient = SesClient::new(rusoto_core::Region::EuWest3);
//     let data_email = MessageBuilder::new()
//         .from(("John Doe", "john@jenoh.dev"))
//         .to(to.clone())
//         .subject("Hello, world!")
//         .html_body(template::alert_template())
//         .write_to_string()
//         .unwrap();

//     let result = sesclient
//         .send_raw_email(rusoto_ses::SendRawEmailRequest {
//             raw_message: rusoto_ses::RawMessage {
//                 data: Bytes::from(base64::encode(data_email.to_string())),
//             },
//             destinations: Some(vec![to]),
//             source: Some("john@jenoh.dev".to_string()),
//             ..rusoto_ses::SendRawEmailRequest::default()
//         })
//         .await;
//     if result.is_err() {
//         eprintln!("Couldn't send email : {:?}", result);
//         return false;
//     }
//     return true;
// }
