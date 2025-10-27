use forgebase_core::Result;
use lettre::{
    Message, SmtpTransport, Transport,
    transport::smtp::authentication::Credentials,
};
use tera::{Context, Tera};

/// Email service
pub struct EmailService {
    smtp_transport: SmtpTransport,
    from_email: String,
    from_name: String,
    templates: Tera,
}

impl EmailService {
    pub fn new(
        smtp_host: String,
        smtp_port: u16,
        smtp_username: String,
        smtp_password: String,
        from_email: String,
        from_name: String,
    ) -> Result<Self> {
        let credentials = Credentials::new(smtp_username, smtp_password);
        let smtp_transport = SmtpTransport::relay(&smtp_host)
            .map_err(|e| forgebase_core::ForgeBaseError::Config(format!("SMTP config error: {}", e)))?
            .port(smtp_port)
            .credentials(credentials)
            .build();

        let mut templates = Tera::default();
        
        // Add email templates
        templates
            .add_raw_template("verification_email", include_str!("../templates/verification_email.html"))
            .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Template error: {}", e)))?;
        
        templates
            .add_raw_template("password_reset", include_str!("../templates/password_reset.html"))
            .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Template error: {}", e)))?;
        
        templates
            .add_raw_template("magic_link", include_str!("../templates/magic_link.html"))
            .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Template error: {}", e)))?;

        Ok(Self {
            smtp_transport,
            from_email,
            from_name,
            templates,
        })
    }

    /// Send verification email
    pub async fn send_verification_email(
        &self,
        to_email: &str,
        to_name: &str,
        verification_link: &str,
    ) -> Result<()> {
        let mut context = Context::new();
        context.insert("name", to_name);
        context.insert("verification_link", verification_link);

        let body = self
            .templates
            .render("verification_email", &context)
            .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Template render error: {}", e)))?;

        self.send_email(to_email, "Verify your email", &body).await
    }

    /// Send password reset email
    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        to_name: &str,
        reset_link: &str,
    ) -> Result<()> {
        let mut context = Context::new();
        context.insert("name", to_name);
        context.insert("reset_link", reset_link);

        let body = self
            .templates
            .render("password_reset", &context)
            .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Template render error: {}", e)))?;

        self.send_email(to_email, "Reset your password", &body).await
    }

    /// Send magic link email
    pub async fn send_magic_link_email(
        &self,
        to_email: &str,
        to_name: &str,
        magic_link: &str,
    ) -> Result<()> {
        let mut context = Context::new();
        context.insert("name", to_name);
        context.insert("magic_link", magic_link);

        let body = self
            .templates
            .render("magic_link", &context)
            .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Template render error: {}", e)))?;

        self.send_email(to_email, "Your sign-in link", &body).await
    }

    /// Send raw email
    async fn send_email(&self, to_email: &str, subject: &str, body: &str) -> Result<()> {
        let email = Message::builder()
            .from(
                format!("{} <{}>", self.from_name, self.from_email)
                    .parse()
                    .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Invalid from address: {}", e)))?,
            )
            .to(to_email
                .parse()
                .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Invalid to address: {}", e)))?)
            .subject(subject)
            .multipart(lettre::message::MultiPart::alternative()
                .singlepart(lettre::message::SinglePart::html(body.to_string())))
            .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Failed to build email: {}", e)))?;

        // Send email in a blocking task since SMTP is sync
        let transport = self.smtp_transport.clone();
        tokio::task::spawn_blocking(move || transport.send(&email))
            .await
            .map_err(|e| forgebase_core::ForgeBaseError::Internal(format!("Task error: {}", e)))?
            .map_err(|e| forgebase_core::ForgeBaseError::ExternalService(format!("Failed to send email: {}", e)))?;

        Ok(())
    }
}
