use lettre::{
    message::Mailbox, transport::smtp::authentication::Credentials, AsyncSmtpTransport,
    AsyncTransport, Message, Tokio1Executor,
};
use tera::{Tera, Context};
use crate::config::AppConfig;
use crate::errors::AppError;
use crate::models::{Order, OrderItem, Address};
use std::collections::HashMap;
use chrono::Datelike;

pub struct EmailService {
    mailer: AsyncSmtpTransport<Tokio1Executor>,
    from_email: Mailbox,
    tera: Tera,
}

impl EmailService {
    pub fn new(config: &AppConfig) -> Result<Self, AppError> {
        let creds = Credentials::new(
            config.smtp_username.clone(),
            config.smtp_password.clone(),
        );
        
        let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay(&config.smtp_host)
            .map_err(|e| AppError::email_error(e.to_string()))?
            .credentials(creds)
            .build();
        
        let from_email: Mailbox = config.smtp_from.parse()
            .map_err(|e: lettre::address::AddressError| AppError::email_error(e.to_string()))?;
        
        let mut tera = Tera::default();
        tera.add_raw_template("welcome", WELCOME_TEMPLATE)
            .map_err(|e| AppError::email_error(e.to_string()))?;
        tera.add_raw_template("order_confirmation", ORDER_CONFIRMATION_TEMPLATE)
            .map_err(|e| AppError::email_error(e.to_string()))?;
        tera.add_raw_template("order_shipped", ORDER_SHIPPED_TEMPLATE)
            .map_err(|e| AppError::email_error(e.to_string()))?;
        tera.add_raw_template("password_reset", PASSWORD_RESET_TEMPLATE)
            .map_err(|e| AppError::email_error(e.to_string()))?;
        
        Ok(Self {
            mailer,
            from_email,
            tera,
        })
    }
    
    pub async fn send_welcome_email(
        &self,
        to_email: &str,
        user_name: &str,
    ) -> Result<(), AppError> {
        let mut context = Context::new();
        context.insert("name", user_name);
        context.insert("year", &chrono::Utc::now().year());
        
        let html = self.tera.render("welcome", &context)
            .map_err(|e| AppError::email_error(e.to_string()))?;
        
        self.send_email(to_email, "Welcome to Shopcore!", &html).await
    }
    
    pub async fn send_order_confirmation(
        &self,
        to_email: &str,
        order: &Order,
        items: &[OrderItem],
        shipping_address: &Address,
    ) -> Result<(), AppError> {
        let mut context = Context::new();
        context.insert("order_number", &order.order_number);
        context.insert("order_date", &order.created_at.format("%B %d, %Y").to_string());
        context.insert("subtotal", &format!("${}", order.subtotal));
        context.insert("tax", &format!("${}", order.tax));
        context.insert("shipping_cost", &format!("${}", order.shipping_cost));
        context.insert("total", &format!("${}", order.total));
        context.insert("shipping_address", &format!(
            "{}, {}, {} {}",
            shipping_address.address_line1,
            shipping_address.city,
            shipping_address.state,
            shipping_address.postal_code
        ));
        
        let items_json: Vec<HashMap<String, String>> = items
            .iter()
            .map(|item| {
                let mut map = HashMap::new();
                map.insert("name".to_string(), item.product_name.clone());
                map.insert("quantity".to_string(), item.quantity.to_string());
                map.insert("price".to_string(), format!("${}", item.price));
                map.insert("total".to_string(), format!("${}", item.total));
                map
            })
            .collect();
        
        context.insert("items", &items_json);
        
        let html = self.tera.render("order_confirmation", &context)
            .map_err(|e| AppError::email_error(e.to_string()))?;
        
        self.send_email(to_email, &format!("Order Confirmation #{}", order.order_number), &html).await
    }
    
    pub async fn send_order_shipped_email(
        &self,
        to_email: &str,
        order_number: &str,
        tracking_number: Option<&str>,
    ) -> Result<(), AppError> {
        let mut context = Context::new();
        context.insert("order_number", order_number);
        context.insert("tracking_number", &tracking_number.unwrap_or("Not available"));
        
        let html = self.tera.render("order_shipped", &context)
            .map_err(|e| AppError::email_error(e.to_string()))?;
        
        self.send_email(to_email, &format!("Your order #{} has been shipped!", order_number), &html).await
    }
    
    pub async fn send_password_reset_email(
        &self,
        to_email: &str,
        reset_token: &str,
    ) -> Result<(), AppError> {
        let mut context = Context::new();
        context.insert("reset_token", reset_token);
        context.insert("reset_url", &format!("https://shopcore.com/reset-password?token={}", reset_token));
        
        let html = self.tera.render("password_reset", &context)
            .map_err(|e| AppError::email_error(e.to_string()))?;
        
        self.send_email(to_email, "Password Reset Request", &html).await
    }
    
    async fn send_email(&self, to_email: &str, subject: &str, html: &str) -> Result<(), AppError> {
        let to: Mailbox = to_email.parse()
            .map_err(|e: lettre::address::AddressError| AppError::email_error(e.to_string()))?;
        
        let email = Message::builder()
            .from(self.from_email.clone())
            .to(to)
            .subject(subject)
            .header(lettre::message::header::ContentType::TEXT_HTML)
            .body(html.to_string())
            .map_err(|e| AppError::email_error(e.to_string()))?;
        
        self.mailer.send(email).await
            .map_err(|e: lettre::transport::smtp::Error| AppError::email_error(e.to_string()))?;
        
        Ok(())
    }
}

const WELCOME_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #4f46e5; color: white; padding: 20px; text-align: center; }
        .content { padding: 20px; background: #f9fafb; }
        .footer { text-align: center; padding: 20px; font-size: 12px; color: #6b7280; }
        .button { display: inline-block; padding: 10px 20px; background: #4f46e5; color: white; text-decoration: none; border-radius: 5px; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Welcome to Shopcore!</h1>
        </div>
        <div class="content">
            <h2>Hello {{ name }}!</h2>
            <p>Thank you for joining Shopcore. We're excited to have you on board!</p>
            <p>With Shopcore, you can:</p>
            <ul>
                <li>Browse thousands of products</li>
                <li>Track your orders in real-time</li>
                <li>Get exclusive deals and discounts</li>
                <li>Enjoy fast and secure checkout</li>
            </ul>
            <p>Start shopping now and discover amazing products!</p>
            <a href="https://shopcore.com/shop" class="button">Start Shopping</a>
        </div>
        <div class="footer">
            <p>&copy; {{ year }} Shopcore. All rights reserved.</p>
        </div>
    </div>
</body>
</html>
"#;

const ORDER_CONFIRMATION_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #10b981; color: white; padding: 20px; text-align: center; }
        .content { padding: 20px; background: #f9fafb; }
        .order-details { background: white; padding: 15px; border-radius: 5px; margin: 15px 0; }
        .item { border-bottom: 1px solid #e5e7eb; padding: 10px 0; }
        .total { font-size: 18px; font-weight: bold; color: #10b981; }
        .footer { text-align: center; padding: 20px; font-size: 12px; color: #6b7280; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Order Confirmed!</h1>
            <p>Order #{{ order_number }}</p>
        </div>
        <div class="content">
            <h2>Thank you for your order!</h2>
            <p>We've received your order and it's being processed.</p>
            
            <div class="order-details">
                <h3>Order Summary</h3>
                <p><strong>Date:</strong> {{ order_date }}</p>
                <p><strong>Shipping Address:</strong> {{ shipping_address }}</p>
                
                <h4>Items:</h4>
                {% for item in items %}
                <div class="item">
                    {{ item.name }} x {{ item.quantity }} - {{ item.price }} = {{ item.total }}
                </div>
                {% endfor %}
                
                <hr>
                <p><strong>Subtotal:</strong> {{ subtotal }}</p>
                <p><strong>Tax:</strong> {{ tax }}</p>
                <p><strong>Shipping:</strong> {{ shipping_cost }}</p>
                <p class="total"><strong>Total:</strong> {{ total }}</p>
            </div>
            
            <p>You can track your order status in your account dashboard.</p>
            <a href="https://shopcore.com/orders/{{ order_number }}" class="button">Track Order</a>
        </div>
        <div class="footer">
            <p>&copy; 2024 Shopcore. All rights reserved.</p>
        </div>
    </div>
</body>
</html>
"#;

const ORDER_SHIPPED_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #3b82f6; color: white; padding: 20px; text-align: center; }
        .content { padding: 20px; background: #f9fafb; }
        .tracking { background: white; padding: 15px; border-radius: 5px; margin: 15px 0; }
        .footer { text-align: center; padding: 20px; font-size: 12px; color: #6b7280; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Your Order Has Shipped!</h1>
            <p>Order #{{ order_number }}</p>
        </div>
        <div class="content">
            <h2>Great news!</h2>
            <p>Your order is on its way and should arrive soon.</p>
            
            <div class="tracking">
                <h3>Tracking Information</h3>
                <p><strong>Tracking Number:</strong> {{ tracking_number }}</p>
            </div>
            
            <a href="https://shopcore.com/orders/{{ order_number }}" class="button">Track Package</a>
        </div>
        <div class="footer">
            <p>&copy; 2024 Shopcore. All rights reserved.</p>
        </div>
    </div>
</body>
</html>
"#;

const PASSWORD_RESET_TEMPLATE: &str = r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        body { font-family: Arial, sans-serif; line-height: 1.6; color: #333; }
        .container { max-width: 600px; margin: 0 auto; padding: 20px; }
        .header { background: #ef4444; color: white; padding: 20px; text-align: center; }
        .content { padding: 20px; background: #f9fafb; }
        .warning { background: #fee2e2; padding: 15px; border-radius: 5px; margin: 15px 0; }
        .footer { text-align: center; padding: 20px; font-size: 12px; color: #6b7280; }
    </style>
</head>
<body>
    <div class="container">
        <div class="header">
            <h1>Password Reset Request</h1>
        </div>
        <div class="content">
            <h2>Hello!</h2>
            <p>We received a request to reset your password. Click the button below to create a new password:</p>
            
            <a href="{{ reset_url }}" class="button">Reset Password</a>
            
            <div class="warning">
                <p><strong>Security Note:</strong> This link will expire in 1 hour. If you didn't request this, please ignore this email.</p>
            </div>
            
            <p>Or use this token: <strong>{{ reset_token }}</strong></p>
        </div>
        <div class="footer">
            <p>&copy; 2024 Shopcore. All rights reserved.</p>
        </div>
    </div>
</body>
</html>
"#;