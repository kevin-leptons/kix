#[cfg(test)]
mod test {
    use kix::Result;

    #[test]
    fn file_must_be_empty() -> Result {
        let content = std::fs::read_to_string("/dev/nullx")?;
        assert!(content.is_empty());
        Ok(())
    }
}

fn main() {}
