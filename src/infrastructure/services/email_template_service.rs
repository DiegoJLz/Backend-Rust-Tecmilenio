use crate::domain::services::email_service::EmailTemplateService;

#[derive(Clone)]
pub struct DefaultEmailTemplateService;

impl DefaultEmailTemplateService {
    pub fn new() -> Self {
        Self
    }
}

impl EmailTemplateService for DefaultEmailTemplateService {
    fn generate_email_verification_template(
        &self,
        user_name: &str,
        verification_token: &str,
    ) -> (String, String) {
        let verification_url = format!(
            "http://localhost:8080/api/v1/auth/verify-email?token={}",
            verification_token
        );

        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>Verifica tu Email - Marketplace Local</title>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #4CAF50; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .button {{ display: inline-block; background-color: #4CAF50; color: white; padding: 12px 24px; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>¡Bienvenido a Marketplace Local!</h1>
                    </div>
                    <div class="content">
                        <h2>Hola {},</h2>
                        <p>Gracias por registrarte en nuestro marketplace de experiencias locales. Para completar tu registro, necesitamos verificar tu dirección de email.</p>
                        <p>Haz clic en el siguiente botón para verificar tu email:</p>
                        <a href="{}" class="button">Verificar Email</a>
                        <p>Si el botón no funciona, copia y pega este enlace en tu navegador:</p>
                        <p style="word-break: break-all; background-color: #eee; padding: 10px; border-radius: 4px;">{}</p>
                        <p>Este enlace expirará en 24 horas por seguridad.</p>
                        <p>Si no creaste esta cuenta, puedes ignorar este email.</p>
                    </div>
                    <div class="footer">
                        <p>© 2024 Marketplace Local. Todos los derechos reservados.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            user_name, verification_url, verification_url
        );

        let text_content = format!(
            r#"
¡Bienvenido a Marketplace Local!

Hola {},

Gracias por registrarte en nuestro marketplace de experiencias locales. Para completar tu registro, necesitamos verificar tu dirección de email.

Haz clic en el siguiente enlace para verificar tu email:
{}

Este enlace expirará en 24 horas por seguridad.

Si no creaste esta cuenta, puedes ignorar este email.

© 2024 Marketplace Local. Todos los derechos reservados.
            "#,
            user_name, verification_url
        );

        (html_content, text_content)
    }

    fn generate_password_reset_template(
        &self,
        user_name: &str,
        reset_token: &str,
    ) -> (String, String) {
        let reset_url = format!(
            "http://localhost:8080/api/v1/auth/reset-password?token={}",
            reset_token
        );

        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>Restablecer Contraseña - Marketplace Local</title>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #FF9800; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .button {{ display: inline-block; background-color: #FF9800; color: white; padding: 12px 24px; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
                    .warning {{ background-color: #fff3cd; border: 1px solid #ffeaa7; padding: 15px; border-radius: 4px; margin: 20px 0; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>Restablecer Contraseña</h1>
                    </div>
                    <div class="content">
                        <h2>Hola {},</h2>
                        <p>Recibimos una solicitud para restablecer la contraseña de tu cuenta en Marketplace Local.</p>
                        <p>Haz clic en el siguiente botón para crear una nueva contraseña:</p>
                        <a href="{}" class="button">Restablecer Contraseña</a>
                        <p>Si el botón no funciona, copia y pega este enlace en tu navegador:</p>
                        <p style="word-break: break-all; background-color: #eee; padding: 10px; border-radius: 4px;">{}</p>
                        <div class="warning">
                            <strong>⚠️ Importante:</strong>
                            <ul>
                                <li>Este enlace expirará en 1 hora por seguridad</li>
                                <li>Solo puede ser usado una vez</li>
                                <li>Si no solicitaste este cambio, ignora este email</li>
                            </ul>
                        </div>
                    </div>
                    <div class="footer">
                        <p>© 2024 Marketplace Local. Todos los derechos reservados.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            user_name, reset_url, reset_url
        );

        let text_content = format!(
            r#"
Restablecer Contraseña - Marketplace Local

Hola {},

Recibimos una solicitud para restablecer la contraseña de tu cuenta en Marketplace Local.

Haz clic en el siguiente enlace para crear una nueva contraseña:
{}

IMPORTANTE:
- Este enlace expirará en 1 hora por seguridad
- Solo puede ser usado una vez
- Si no solicitaste este cambio, ignora este email

© 2024 Marketplace Local. Todos los derechos reservados.
            "#,
            user_name, reset_url
        );

        (html_content, text_content)
    }

    fn generate_welcome_template(&self, user_name: &str) -> (String, String) {
        let html_content = format!(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <meta charset="utf-8">
                <title>¡Bienvenido! - Marketplace Local</title>
                <style>
                    body {{ font-family: Arial, sans-serif; line-height: 1.6; color: #333; }}
                    .container {{ max-width: 600px; margin: 0 auto; padding: 20px; }}
                    .header {{ background-color: #4CAF50; color: white; padding: 20px; text-align: center; }}
                    .content {{ padding: 20px; background-color: #f9f9f9; }}
                    .button {{ display: inline-block; background-color: #4CAF50; color: white; padding: 12px 24px; text-decoration: none; border-radius: 4px; margin: 20px 0; }}
                    .footer {{ text-align: center; padding: 20px; color: #666; font-size: 12px; }}
                </style>
            </head>
            <body>
                <div class="container">
                    <div class="header">
                        <h1>¡Bienvenido a Marketplace Local!</h1>
                    </div>
                    <div class="content">
                        <h2>¡Hola {}! 🎉</h2>
                        <p>¡Tu email ha sido verificado exitosamente! Ahora puedes disfrutar de todas las funcionalidades de nuestro marketplace.</p>
                        <p>En Marketplace Local puedes:</p>
                        <ul>
                            <li>📅 Descubrir experiencias locales únicas</li>
                            <li>🎨 Reservar talleres y actividades</li>
                            <li>🌟 Calificar y reseñar experiencias</li>
                            <li>💰 Gestionar tus reservas y pagos</li>
                        </ul>
                        <p>¡Explora todas las experiencias disponibles y comienza tu aventura local!</p>
                        <a href="http://localhost:3000/experiences" class="button">Explorar Experiencias</a>
                    </div>
                    <div class="footer">
                        <p>© 2024 Marketplace Local. Todos los derechos reservados.</p>
                    </div>
                </div>
            </body>
            </html>
            "#,
            user_name
        );

        let text_content = format!(
            r#"
¡Bienvenido a Marketplace Local!

¡Hola {}! 🎉

¡Tu email ha sido verificado exitosamente! Ahora puedes disfrutar de todas las funcionalidades de nuestro marketplace.

En Marketplace Local puedes:
- 📅 Descubrir experiencias locales únicas
- 🎨 Reservar talleres y actividades
- 🌟 Calificar y reseñar experiencias
- 💰 Gestionar tus reservas y pagos

¡Explora todas las experiencias disponibles y comienza tu aventura local!

Visita: http://localhost:3000/experiences

© 2024 Marketplace Local. Todos los derechos reservados.
            "#,
            user_name
        );

        (html_content, text_content)
    }
}
