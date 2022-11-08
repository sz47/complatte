mod files;

fn main() {
    let filename = "dataset/bible.txt".to_string() + ".json";
    files::import_data(filename);
}
