
#[derive(Clone)]
pub enum Command<'a> {
    Ehlo {
        domain: &'a str,
    },
    Helo {
        domain: &'a str,
    },
    Mail {
        reverse_path: &'a str,
        is8bit: bool,
        size: Option<usize>,
    },
    Rcpt {
        forward_path: &'a str,
    },
    Data,
    Rset,
    Noop,
    StartTls,
    Quit,
    Vrfy,
}