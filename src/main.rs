use std::env;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        help();
        return;
    }
    std::fs::create_dir_all(get_memo_path()).unwrap();
    let command = &args[1];
    let extra_args = &args[2..];
    match command.as_str() {
        "help" | "-h" | "--help" => help(),
        "add" | "new" | "-a" | "-n" => add(extra_args),
        "show" | "see" | "open" | "-s" | "-o"  => show(extra_args),
        "delete" | "remove" | "-d" | "-r" => delete(extra_args),
        "list" | "all" | "-l" => list(extra_args),
        "clean" | "deleteall" | "removeall" | "-c" => clean(extra_args),
        "version" | "-v" | "--version" => version(extra_args),
        _ => print_unknown_command("", command)
    }
}

fn get_memo_path() -> PathBuf {
    dirs_next::data_dir().unwrap().join("memo")
}

fn version(args: &[String]) {
    if args.len() > 0 {
        print_unknown_command("version", &args.join(" "));
        return;
    }
    println!("memo v1.0.0");
}

fn help() {
    println!("Commands & Usage:");
    println!(" memo list - List all memos");
    println!(" memo clean - Delete all memos");
    println!(" memo add <memo> - Add memo");
    println!(" memo show <index> - Show memo");
    println!(" memo delete <index> - Delete memo");
}

fn print_unknown_command(command: &str, args: &str) {
    println!("Unknown command: 'memo {} {}', see help: 'memo help'", command, args);
}

fn get_memo_file_names() -> Vec<String> {
    let memo_path = get_memo_path();
    let mut file_names = Vec::new();
    for entry in std::fs::read_dir(memo_path).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        file_names.push(file_name.to_str().unwrap().split(".").collect::<Vec<&str>>()[0].to_string());
    }
    file_names
}

fn add(args: &[String]) {
    if args.len() < 1 {
        println!("Example: 'memo add New memo with text'");
        return;
    }
    let mut next_index = 1;
    let file_names = get_memo_file_names();
    if file_names.len() > 0 {
        let last_file_name = &file_names[file_names.len() - 1];
        if last_file_name.parse::<usize>().is_err() {
            std::fs::remove_file(get_memo_path().join(last_file_name)).unwrap();
            println!("Removed invalid memo: '{}'. Do not modify files by yourself.", last_file_name);
            return;
        }
        next_index = last_file_name.parse::<usize>().unwrap() + 1;
    }
    let memo_path = get_memo_path().join(format!("{}.txt", next_index));
    let memo = args.join(" ");

    let mut file = File::create(memo_path).unwrap();
    file.write_all(memo.as_bytes()).unwrap();

    println!("Added memo: {}", memo);
}

fn delete(args: &[String]) {
    if args.len() > 1 || args.len() == 0 {
        print_unknown_command("delete", &args.join(" "));
        return;
    }
    let index = match args[0].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            print_unknown_command("delete", &args[0]);
            return;
        }
    };
    let file_names = get_memo_file_names();    

    if index > file_names.len() || index < 1 {
        println!("Memo not found: {}", index);
        return;
    }

    let memo_path = get_memo_path().join(format!("{}.txt", file_names[index - 1]));
    std::fs::remove_file(memo_path).unwrap();

    println!("Deleted memo: {}", index);
}

fn show(args: &[String]) {
    if args.len() > 1 || args.len() == 0 {
        print_unknown_command("show", &args.join(" "));
        return;
    }
   
    let index = match args[0].parse::<usize>() {
        Ok(n) => n,
        Err(_) => {
            print_unknown_command("show", &args[0]);
            return;
        }
    };

    let file_names = get_memo_file_names();
    if index > file_names.len() || index < 1 {
        println!("Memo not found: {}", index);
        return;
    }

    let file_content = std::fs::read_to_string(get_memo_path().join(format!("{}.txt", file_names[index - 1]))).unwrap();
    println!("Memo {}: {}", index, file_content);
    
}

fn list(args: &[String]) {
    if args.len() > 0 {
        print_unknown_command("list", &args.join(" "));
        return;
    }

    let file_names = get_memo_file_names();
    if file_names.len() == 0 {
        println!("No memos");
        return;
    }
    println!("Memos:");
    for (index, file_name) in file_names.iter().enumerate() {
        let file_content = std::fs::read_to_string(get_memo_path().join(format!("{}.txt", file_name))).unwrap();
        println!(" {}. {}", index + 1, file_content);
    }
}

fn clean(args: &[String]) {
    if args.len() > 0 {
        print_unknown_command("clean", &args.join(" "));
        return;
    }

    let memo_path = get_memo_path();
    for entry in std::fs::read_dir(&memo_path).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        std::fs::remove_file(memo_path.join(file_name)).unwrap();
    }

    println!("Cleaned all memos");
}