/////////////////////////////////////////////////////////////////////////////
//  Level 0 atomic function
/////////////////////////////////////////////////////////////////////////////
pub fn read_input(input: &str) -> anyhow::Result<Vec<u8>> {
    let mut reader: Box<dyn std::io::Read> = get_reader(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;
    Ok(buf)
}
pub fn trim_whitespace(buf: &[u8]) -> &[u8] {
    let len = buf
        .iter()
        .rposition(|&x| !x.is_ascii_whitespace())
        .map_or(0, |pos| pos + 1);
    &buf[..len]
}
/////////////////////////////////////////////////////////////////////////////
// private atomic function
/////////////////////////////////////////////////////////////////////////////
pub fn get_reader(input: &str) -> anyhow::Result<Box<dyn std::io::Read>> {
    Ok(if input == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(std::fs::File::open(input)?)
    })
}
