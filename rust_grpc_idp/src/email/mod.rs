use crate::config::get_env_var;
use handlebars::Handlebars;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{message::header::ContentType, Message, SmtpTransport, Transport};
use log::{error, info};
use serde_json;

pub struct EmailAccount {
    pub first_name: String,
    pub email_address: String,
}

pub fn send_mail(to: EmailAccount, subject: &str, template_name: &str, end_point: &str) {
    let sender_first_name = get_env_var("BREVO_FIRSTNAME");
    let sender_email_address = get_env_var("BREVO_USER");
    let sender = format!("{} <{}>", sender_first_name, sender_email_address);
    let receiver = format!("{} <{}>", to.first_name, to.email_address);

    let html_template = render_template(
        template_name,
        TemplateData {
            first_name: &to.first_name,
            subject,
            url: end_point,
        },
    )
    .unwrap_or(format!(
        "Hi {}! Please verify your email <a href='{}'>{}</a>",
        to.first_name, end_point, template_name
    ));
    let email = Message::builder()
        .from(sender.parse().unwrap())
        .to(receiver.parse().unwrap())
        .subject(subject)
        .header(ContentType::TEXT_HTML)
        .body(html_template)
        .unwrap();

    let creds = Credentials::new(get_env_var("BREVO_USER"), get_env_var("BREVO_PASS"));

    // Open a remote connection to gmail
    let mailer = SmtpTransport::relay("smtp-relay.sendinblue.com")
        .unwrap()
        .credentials(creds)
        .build();

    // Send the email
    match mailer.send(&email) {
        Ok(_) => {
            info!(
                "Confimation Email is successfully sent to {}",
                to.email_address
            );
        }
        Err(e) => {
            error!("Could not send email: {:?}", e);
        }
    }
}

struct TemplateData<'a> {
    first_name: &'a str,
    subject: &'a str,
    url: &'a str,
}
fn render_template(
    template_name: &str,
    data: TemplateData,
) -> Result<String, handlebars::RenderError> {
    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_file(template_name, &format!("./templates/{}.hbs", template_name))?;
    handlebars.register_template_file("styles", "./templates/partials/styles.hbs")?;
    handlebars.register_template_file("base", "./templates/layouts/base.hbs")?;

    let data = serde_json::json!({
        "first_name": data.first_name.split_whitespace().next().unwrap(),
        "subject": data.subject,
        "url": &data.url
    });

    let content_template = handlebars.render(template_name, &data)?;

    Ok(content_template)
}
