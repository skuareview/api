use self::model::{InsertableMetric, Metric};
use super::DbPool;
use actix_web::{post, web, Error, HttpResponse};
// use lettre::Message;
use base64::{decode, encode};
use bytes::Bytes;
use mail_builder::MessageBuilder;
// use lettre::MessageBuilder;
// use lettre_email::EmailBuilder;
use rusoto_ses::{Body, RawMessage, SendEmailRequest, Ses, SesClient};
pub mod model;

/// Inserts new user with name defined in form.
#[post("/metrics")]
pub async fn add_metrics(
    pool: web::Data<DbPool>,
    form: web::Json<InsertableMetric>,
) -> Result<HttpResponse, Error> {
    // use web::block to offload blocking Diesel code without blocking server thread
    let metric = web::block(move || {
        let conn = pool.get()?;
        Metric::insert_new_metric(&form, &conn)
    })
    .await?
    .map_err(actix_web::error::ErrorInternalServerError)?;

    // let ses_client = SesClient::new(rusoto_core::Region::EuWest3);

    // let from = "Hello World <contact@jenoh.dev>";
    // let to = "Théo <memint.contact@gmail.com>";
    let subject = "Hello World";
    let body = "<h1>Hello World</h1>".to_string();
    let sesclient = SesClient::new(rusoto_core::Region::EuWest3);
    // let recipient = Mailbox::from_str("memint.contact@gmail.com");
    let to = format!("{}", "service-ses@jenoh.dev");
    let ok = to.clone();
    println!("{:?}", vec![ok]);
    // let result = sesclient
    //     .send_email(rusoto_ses::SendEmailRequest {
    //         destination: rusoto_ses::Destination {
    //             to_addresses: Some(vec![to]),
    //             ..rusoto_ses::Destination::default()
    //         },
    //         message: rusoto_ses::Message {
    //             subject: rusoto_ses::Content {
    //                 data: subject.to_string(),
    //                 ..rusoto_ses::Content::default()
    //             },
    //             body: rusoto_ses::Body {
    //                 text: Some(rusoto_ses::Content {
    //                     data: body,
    //                     ..rusoto_ses::Content::default()
    //                 }),
    //                 ..rusoto_ses::Body::default()
    //             },
    //         },
    //         source: "hello@jenoh.dev".to_string(),
    //         ..rusoto_ses::SendEmailRequest::default()
    //     })
    //     .await;
    let eml = MessageBuilder::new()
        .from(("John Doe", "john@jenoh.dev"))
        .to("service-ses@jenoh.dev")
        .subject("Hello, world!")
        // .text_body("Message contents go here.")
        .html_body("<h1>HTML body with</h1>")
        // .binary_attachment("image/png", "image.png", [1, 2, 3, 4].as_ref())
        .write_to_string()
        .unwrap();

    let result = sesclient
        .send_raw_email(rusoto_ses::SendRawEmailRequest {
            raw_message: rusoto_ses::RawMessage {
                data: Bytes::from(base64::encode(eml.to_string())),
            },
            destinations: Some(vec!["service-ses@jenoh.dev".to_string()]),
            //     to_addresses: Some(vec![to]),
            //     ..rusoto_ses::Destination::default()
            // },
            // message: rusoto_ses::Message {
            //     subject: rusoto_ses::Content {
            //         data: subject.to_string(),
            //         ..rusoto_ses::Content::default()
            //     },
            //     body: rusoto_ses::Body {
            //         text: Some(rusoto_ses::Content {
            //             data: body,
            //             ..rusoto_ses::Content::default()
            //         }),
            //         ..rusoto_ses::Body::default()
            //     },
            // },
            source: Some("hello@jenoh.dev".to_string()),
            ..rusoto_ses::SendRawEmailRequest::default()
        })
        .await;
    if result.is_err() {
        eprintln!("Couldn't send email to : {:?}", result);
    }
    // send_email_ses(&ses_client, from, to, subject, body).await?;
    Ok(HttpResponse::Created().json(metric))
}

async fn send_email_ses(
    ses_client: &SesClient,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // let email = Message::new("fdslfs");
    let sesclient = SesClient::new(rusoto_core::Region::EuWest3);
    let to = format!("{}", "memint.contact@gmail.com");
    let ok = to.clone();
    println!("{:?}", vec![ok]);
    sesclient
        .send_email(rusoto_ses::SendEmailRequest {
            destination: rusoto_ses::Destination {
                to_addresses: Some(vec![to]),
                ..rusoto_ses::Destination::default()
            },
            message: rusoto_ses::Message {
                subject: rusoto_ses::Content {
                    data: subject.to_string(),
                    ..rusoto_ses::Content::default()
                },
                body: rusoto_ses::Body {
                    text: Some(rusoto_ses::Content {
                        data: body.to_string(),
                        ..rusoto_ses::Content::default()
                    }),
                    ..rusoto_ses::Body::default()
                },
            },
            source: "hello@jenoh.dev".to_string(),
            ..rusoto_ses::SendEmailRequest::default()
        })
        .await?;
    // let email = EmailBuilder::new()
    //     // Addresses can be specified by the tuple (email, alias)
    //     .to(("memint.contact@gmail.com", "Firstname Lastname"))
    //     // ... or by an address only
    //     .from("contact@jenoh.dev")
    //     .subject("Hi, Hello world")
    //     .text("Hello world.")
    //     .build()
    //     .unwrap();

    // let raw_email = email.formatted();
    // let message = Message::new().body("<h1>Salut</h1>");
    // let message = Message {
    //     // subject: Body::new("fklsd"),
    //     // body: "<p>Salut tout le monde</p>"
    // }
    // let ses_request = SendEmailRequest {
    //     message: Message {
    //         subject: "Salut".to_string(),
    //         body: "Salut".to_string()
    //     }
    // }
    // let ses_request = SendRawEmailRequest {
    //     raw_message: RawMessage {
    //         data: base64::encode(email).into(),
    //     },
    //     ..Default::default()
    // };

    // ses_client.send_raw_email(ses_request).await?;

    Ok(())
}
