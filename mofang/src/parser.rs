use crate::core::{BaseMove, Command, Elem};
use nom::branch::alt;
use nom::character::complete::{char, one_of};
use nom::combinator::{all_consuming, map};
use nom::multi::{many0, many1, many_m_n};
use nom::sequence::{delimited, pair};
use nom::IResult;

/// 通过字符串转换成旋转命令
fn parse_move(i: &str) -> IResult<&str, BaseMove> {
    map(one_of("RLUDFBrludfbMESxyz"), |c| match c {
        'R' | 'r' => BaseMove::R,
        'L' | 'l' => BaseMove::L,
        'U' | 'u' => BaseMove::U,
        'D' | 'd' => BaseMove::D,
        'F' | 'f' => BaseMove::F,
        'B' | 'b' => BaseMove::B,
        'M' => BaseMove::M,
        'E' => BaseMove::E,
        'S' => BaseMove::S,
        'x' => BaseMove::X,
        'y' => BaseMove::Y,
        'z' => BaseMove::Z,
        _ => unreachable!(),
    })(i)
}

/// 判断是否需要转两次
fn parse_double(i: &str) -> IResult<&str, bool> {
    map(many_m_n(0, 1, char('2')), |v| !v.is_empty())(i)
}

/// 判断是否要逆时针旋转
fn parse_prime(i: &str) -> IResult<&str, bool> {
    map(many_m_n(0, 1, char('\'')), |v| !v.is_empty())(i)
}

/// 判断重复次数， 默认为1,倒转为-1
fn parse_rep(i: &str) -> IResult<&str, i8> {
    map(pair(parse_double, parse_prime), |(double, prime)| {
        let mut rep = 1;
        if double {
            rep = 2;
        }
        if prime {
            rep *= -1;
        }
        rep
    })(i)
}

/// 解析单个命令
fn parse_command(i: &str) -> IResult<&str, Command> {
    map(pair(parse_move, parse_rep), |(mv, rep)| Command(mv, rep))(i)
}

/// 解析一组命令
fn parse_group(i: &str) -> IResult<&str, (Vec<Command>, i8)> {
    let p1 = char('(');
    let p2 = many1(parse_command);
    let p3 = char(')');
    let f = delimited(p1, p2, p3);
    let f = pair(f, parse_rep);
    map(f, |(xs, rep)| (xs, rep))(i)
}

/// 解析多组命令
fn parse_elem(i: &str) -> IResult<&str, Elem> {
    let p1 = map(parse_command, Elem::One);
    let p2 = map(parse_group, |(xs, rep)| Elem::Group(xs, rep));
    alt((p1, p2))(i)
}

/// 解析字符串输入
pub fn parse(i: &str) -> IResult<&str, Vec<Elem>> {
    let p = many0(parse_elem);
    all_consuming(p)(i)
}

#[test]
fn test_parse() {
    use crate::core::BaseMove::*;
    use crate::core::Elem::*;
    assert_eq!(parse("").unwrap().1, vec![]);
    assert_eq!(parse("R").unwrap().1, vec![One(Command(R, 1))]);
    assert_eq!(parse("R2'").unwrap().1, vec![One(Command(R, -2))]);
    assert!(parse("R'2").is_err());
    assert_eq!(
        parse("(RR')'").unwrap().1,
        vec![Group(vec![Command(R, 1), Command(R, -1)], -1)]
    );
    assert!(parse("RUR'U'").is_ok());
    assert!(parse("R2D(R'U2R)D'(R'U2R')").is_ok());
    assert!(parse("(RU')(RU)2(RU')R'U'R2").is_ok());
    assert!(parse("(R2'U)(RUR')(U'R'U')(R'UR')").is_ok());
    assert!(parse("RNA").is_err());
}
