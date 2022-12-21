use anyhow::{Result, anyhow};
use super::{Header, Pattern, Position};

#[allow(dead_code)]
pub static GOSPER_GLIDER_GUN_SYNTH: &str = r"#N Gosper glider gun_synth
#O Bill Gosper
#C Glider synthesis of Gosper glider gun.
#C www.conwaylife.com/wiki/index.php?title=Gosper_glider_gun
x = 47, y = 14, rule = b3/s23
16bo30b$16bobo16bo11b$16b2o17bobo9b$obo10bo21b2o10b$b2o11b2o31b$bo11b
2o32b3$10b2o20b2o13b$11b2o19bobo9b3o$10bo21bo11bo2b$27bo17bob$27b2o18b
$26bobo!";

///
/// RLE 编码格式说明：https://conwaylife.com/wiki/Run_Length_Encoded
///
/// 如 GOSPER_GLIDER_GUN_SYNTH 所示，`#` 开头的是注释信息，之后编码了大小信息：x y，以及采用的规则
///
/// 最后是 RLE 编码的细胞信息：
///   b 代表死细胞， o 代表活细胞， $ 代表行结束，! 代表编码结束
///   它们前面的数字代表数量，如 42b 代表 42 个死细胞， 2$ 代表有两行结束，即有一行全是死细胞
///
impl Pattern {
    pub fn decode_rle(input: &str) -> Result<Self> {
        let mut rle = Self::default();

        let mut prev_is_num = false;
        let (mut x, mut y, mut count) = (0usize, 0usize, 1usize);

        'outer: for line in input.trim_start().lines() {
            let line = line.trim();

            match &line[..1] {
                "#" | "x" if line.len() > 3 => rle.header.decode(line),
                _ => {
                    for c in line.chars() {
                        count = if prev_is_num { count } else { 1 };

                        match c {
                            '!' => break 'outer,
                            'b' => {
                                x += count;
                                prev_is_num = false;
                            }
                            'o' => {
                                for _ in 0..count {
                                    rle.cells.push(Position { x, y });
                                    x += 1;
                                }
                                prev_is_num = false;
                            }
                            '$' => {
                                y += count;
                                x = 0;
                                prev_is_num = false;
                            }
                            _ => {
                                if let Some(i) = c.to_digit(10) {
                                    count = i as usize + if prev_is_num { 10 * count } else { 0 };
                                    prev_is_num = true;
                                } else {
                                    return Err(anyhow!("parse failed: invalid char!"));
                                }
                            }
                        }
                    }
                }
            }
        }

        if rle.cells.is_empty() {
            return Err(anyhow!("parse failed: input is empty!"));
        }

        if rle.header.x == 0 || rle.header.y == 0 {
            let (x, y) = rle
                .cells
                .iter()
                .fold((0, 0), |(x, y), p| (x.max(p.x), y.max(p.y)));

            rle.header.x = x;
            rle.header.y = y;
        }

        Ok(rle)
    }

    pub fn encode_rle(&self) -> String {
        let mut rle = self.header.encode();

        let mut tag_rle = |len: usize, tag: &str| {
            if len != 0 {
                if len == 1 {
                    rle += tag;
                } else {
                    rle += &format!("{len}{tag}");
                }
            }
        };

        let mut prev: Option<&Position> = None;
        let mut alive_count = 1usize;

        for curr in &self.cells {
            if let Some(prev) = prev {
                let dead_count = if curr.y <= prev.y {
                    curr.x - prev.x - 1
                } else {
                    // 跨行，编码上一行的信息
                    tag_rle(alive_count, "o");
                    tag_rle(curr.y - prev.y, "$\n");

                    alive_count = 0;
                    curr.x
                };

                if dead_count == 0 {
                    alive_count += 1;
                } else {
                    // 间隔了死细胞，编码前一轮连续活细胞的信息
                    tag_rle(alive_count, "o");
                    tag_rle(dead_count, "b");

                    alive_count = 1;
                }
            } else {
                // 第一个细胞，编码起始信息
                tag_rle(curr.y, "$\n");
                tag_rle(curr.x, "b");

                alive_count = 1;
            }

            prev = Some(curr);
        }

        // 编码最后一轮活细胞信息
        tag_rle(alive_count, "o!");

        rle
    }
}

impl Header {
    fn decode(&mut self, line: &str) {
        match &line[..1] {
            "#" => match &line[1..2] {
                "N" => self.name = Some(line[3..].into()),
                "O" => self.owner = Some(line[3..].into()),
                "C" | "c" => {
                    if let Some(ref mut comment) = self.comment {
                        comment.push('\n');
                        comment.push_str(&line[3..]);
                    } else {
                        self.comment = Some(line[3..].into());
                    }
                }
                _ => todo!(),
            },
            "x" => {
                for kv in line.split(',') {
                    let mut key = None;
                    for v in kv.split('=') {
                        match key {
                            None => key = Some(v.trim()),
                            Some("rule") => self.rule = Some(v.trim().into()),
                            Some("x") => self.x = v.trim().parse().unwrap_or_default(),
                            Some("y") => self.y = v.trim().parse().unwrap_or_default(),
                            _ => todo!(),
                        }
                    }
                }
            }
            _ => todo!(),
        }
    }

    fn encode(&self) -> String {
        let mut header = String::new();

        if let Some(s) = &self.name {
            header += &format!("#N {s}\n");
        }

        if let Some(s) = &self.owner {
            header += &format!("#O {s}\n");
        }

        if let Some(s) = &self.comment {
            for s in s.lines() {
                header += &format!("#C {s}\n");
            }
        }

        header += &format!("x = {}, y = {}", self.x, self.y);

        if let Some(s) = &self.rule {
            header += &format!(", rule = {s}");
        }

        header + "\n"
    }
}

#[test]
fn decode() {
    assert_eq!(
        include!("gospers_glider_gun_synth.txt"),
        Pattern::decode_rle(GOSPER_GLIDER_GUN_SYNTH).unwrap()
    )
}

#[test]
fn encode() {
    let orig = Pattern::decode_rle(GOSPER_GLIDER_GUN_SYNTH).unwrap();
    let new = Pattern::decode_rle(&orig.encode_rle()).unwrap();

    assert_eq!(orig, new)
}
