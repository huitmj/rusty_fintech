use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    println!("Withdraw, Deposite, Send");
    io::stdin().read_line(&mut buffer)?;
    for n in 1..67 {
    	println!("{} Sophie says: {} !",n,buffer.trim());
    }
    Ok(())
}
