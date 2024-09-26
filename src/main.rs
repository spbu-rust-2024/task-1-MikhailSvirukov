use std::io;




fn main() {
    let mut array: Vec<i32>=Vec::new();
    let mut line=String::new();
    io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    let chars: Vec<&str> = line.trim().split(' ').collect();

    for i in 0..chars.len(){
        array.push(chars[i].parse::<i32>().unwrap());
    }

    let mut counter =1;
    while counter<array.len() {
        let mut i = counter;
        while i >0 {
            if array[i] < array[i - 1] {
                let t = array[i];
                array[i] = array[i - 1];
                array[i - 1] = t;
            }
            i -= 1;
        }
        counter+=1;
    }

    let strings:Vec<String>=array.into_iter().map(|x|x.to_string()).collect();
    print!("{}", strings.join(" "));


}
