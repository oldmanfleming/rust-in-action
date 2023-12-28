const DATA: &str = "\
common name,length (cm)
Little penguin,33
Yellow-eyed penguin,65
Fiordland penguin,60
Invalid,data
";

fn main() {
    let lines = DATA.trim().lines();

    for (_i, line) in lines.skip(1).enumerate() {
        let fields: Vec<_> = line.split(',').map(|field| field.trim()).collect();

        let name = fields[0];
        if let Ok(len) = fields[1].parse::<f32>() {
            println!("{}, {}cm", name, len);
        }
    }
}
