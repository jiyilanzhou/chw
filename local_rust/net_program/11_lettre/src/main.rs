
// lettre[lɛtrə]n.(封口)信件    // mail[meɪl]v&n.邮件
// mailer[ˈmeɪlə(r)]n.邮件(程序/处理器)    // email[ˈiːmeɪl]n.电子邮件
// credential[krəˈdenʃl]n.证书,凭证
/*
0. SMTP (Simple Mail Transfer Protocol 简单邮件传输协议)
   a. 传输电子邮件的协议(主用于系统间邮件信息传递并提供来信通知)
   b. SMTP 工作在两种情况下:一种是电子邮件从客户机传输到服务器;
      一种是从某服务器传输到另外的服务器(即中继)
   c. SMTP 是请求响应协议，命令和响应都是基于 ASCII 文本目以 CR
      和 LE 符结尾，响应包括一个表示返回状态的三位数字代码
   d. SMTP 是基于 TCP 的请求、响应协议(在 25 号端口监听链接请求)


*/
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::{EmailBuilder, Mailbox};

fn main() {
    let email = EmailBuilder::new()
        //.from(Mailbox::new("xiaoming@163.com".to_string())) //发送者：xiaoming@163.com
        .from(Mailbox::new("发送者的邮箱地址".to_string()))
        //.to(Mailbox::new("xiaohong@126.com".to_string())) //接收者：xiaohong@126.com
        .to(Mailbox::new("接收者邮箱地址".to_string()))
        .subject("Test") //邮件标题
        .body("This is a test email!") //邮件内容
        .build()
        .unwrap();

    //for example: xiaoming@163.com, password: 123456
    //let creds = Credentials::new("xiaoming".to_string(), "123456".to_string());
    let creds = Credentials::new("你的邮箱用户名".to_string(), "你的邮箱密码".to_string());

    //如163的邮箱就是smtp.163.com, 126的邮箱就是smtp.126.com
    let mut mailer = SmtpClient::new_simple("邮箱服务器地址") 
        .unwrap()
        .credentials(creds)
        .transport();

    let result = mailer.send(email.into());

    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }

    assert!(result.is_ok());
}

