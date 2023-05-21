// fn main(){
//    let mut line = String::new();
//    println!("Enter your name :");
//    std::io::stdin().read_line(&mut line).unwrap();
// //    let b1 = std::io::stdin().read_line(&mut line).unwrap();
//    println!("Hello , {}", line);
// //    println!("no of bytes read , {}", b1);
// }




use std::io;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    println!("Buffer Data: {}", buffer);
    Ok(())
}
