//Testing main
// fn main()->Result<(),io::Error>{
// file → file
// let mut input = File::open("test.txt")?;
// let mut output = File::create("out.txt")?;

// stdin → stdout
// let mut input=io::stdin();
// let mut output=io::stdout();

// file → stdout
// let mut input=File::open("test.txt")?;
// let mut output=io::stdout();

// stdin -> file
//     let mut input = io::stdin();
//     let mut output=File::create("out.txt")?;
//     pipe(&mut input, &mut output)?;
//     Ok(())
// }