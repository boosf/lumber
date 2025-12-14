const DELIM: char = ';';
const MAX_HEADER_SIZE: usize = 10;

pub struct Header {
    pub start_idx: usize,
    pub size: usize,
}

pub fn parse_header(msg: &str) -> Result<Header, ()> {
    let mut msg_size = String::with_capacity(MAX_HEADER_SIZE);
    for (idx, c) in msg.chars().enumerate() {
        if c == DELIM {
            let size = msg_size.parse::<usize>().map_err(|_| ())?;
            let header = Header {
                size: size,
                start_idx: idx + 1,
            };
            return Ok(header);
        }
        msg_size.push(c);
    }
    Err(())
}
