use std::net::{IpAddr, Ipv4Addr};

use nom::branch::alt;
use nom::bytes::complete::{is_not, tag, tag_no_case, take_while1};
use nom::combinator::{map_res, opt};
use nom::{IResult, Parser};
use nom::sequence::terminated;

use crate::smtp::Command;


fn command(buf: &[u8]) -> IResult<&[u8], Command<'_>> {
    terminated(alt((helo,)), tag(&b"\r\n"[..])).parse(buf)
}

fn hello_domain(buf: &[u8]) -> IResult<&[u8], &str> {
    map_res(is_not(b" \t\r\n" as &[u8]), str::from_utf8).parse(buf)
}

fn helo(buf: &[u8]) -> IResult<&[u8], Command<'_>> {
    let (buf, _) = tag_no_case(&b"helo"[..]).parse(buf)?;
    let (buf, _) = take_while1(|b| b == b' ' || b == b'\t').parse(buf)?;
    let (buf, domain) = opt(hello_domain).parse(buf)?;

    Ok((
        buf,
        Command::Helo {
            domain: domain.unwrap_or(""),
        },
    ))
}


#[cfg(test)]
mod tests {
    use crate::parser::command;
    use crate::smtp::Command;

    #[test]
    fn parses_helo_with_domain() {
        let input = b"HELO example.com\r\n";
        let result = command(input);

        assert!(result.is_ok());
        let (rest, cmd) = result.expect("expected HELO command to parse");
        assert_eq!(rest, b"");
        assert!(matches!(
            cmd,
            Command::Helo {
                domain: "example.com"
            }
        ));
    }

    #[test]
    fn parses_helo_with_whitespace_only_domain() {
        let input = b"helo \t\r\n";
        let result = command(input);

        assert!(result.is_ok());
        let (rest, cmd) = result.expect("expected HELO command to parse");
        assert_eq!(rest, b"");
        assert!(matches!(
            cmd,
            Command::Helo {
                domain: ""
            }
        ));
    }
}