use std::fs;
use std::fs::OpenOptions;
use std::io::Write;

fn process_file() -> std::io::Result<String> {
    std::fs::write(
        "calvin_with_dad.txt",
        "Calvin: Dad, how come old photographs are always black and white? Didn't they have color film back then?
Dad: Sure they did. In fact, those photographs *are* in color. It's just the *world* was black and white then.
Calvin: Really?
Dad: Yep. The world didn't turn color until sometimes in the 1930s...",
    )?;

    let mut calvin_file = OpenOptions::new().append(true).read(true).open("calvin_with_dad.txt")?;
    calvin_file.write_all(b"And it was a pretty grainy color for a while too.\n")?;
    write!(&mut calvin_file, "That's really weird.\n")?;
    write!(&mut calvin_file, "Well, truth is stranger than fiction.")?;

    println!("{}", fs::read_to_string("calvin_with_dad.txt")?);
    Ok(("success".to_string()))
}

pub fn process_file_wrapper() {
    let res = process_file();
    match res {
        Ok(value) => println!("Result: {:?}", value),
        Err(message) => println!("failed. Message: {}", message),
    }
}
