use std::process::Command;
use serde_json::Value;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
   #[arg(short, long)]
   token: String,

   #[arg(short, long)]
   repo: String,

   /// Number of times to greet
   #[arg(short, long, default_value_t = 1)]
   number: u8,
}

fn main() {
    let args = Args::parse();

    let github_token = args.token;
    let repo_name = args.repo;
    let issue_number = args.number;
    let json_str = get_issues(&repo_name, github_token, issue_number);
    if let Ok(s) = json_str {
        // println!("{}", s);
        let clean_str = s.replace("--request GET", "");
        let json_data: Value = serde_json::from_str(&clean_str).unwrap();
        println!("{:?}", json_data.as_object().unwrap().get("body"));
        println!("{:?}", json_data.as_object().unwrap().get("labels"));

    }

}

fn get_issues(repo_name: &str, github_token: String, issue_number: u8) -> Result<String, std::string::FromUtf8Error> {
    // 构建请求URL
    let url = format!(
            "https://api.github.com/repos/{REPO_NAME}/issues/{ISSUE_NUMBER}",
            REPO_NAME = repo_name,
            ISSUE_NUMBER = issue_number,
        );
    let auth = format!(
        "Authorization: Bearer {}", github_token
    );
    // Run the cURL command and capture the output in a String variable
    let output = Command::new("curl")
        .arg("-w")
        .arg("--request GET")
        .arg("--url")
        .arg(url)
        .arg("--header")
        .arg("Accept: application/vnd.github+json")
        .arg("--header")
        .arg(auth)
        .output()
        .unwrap()
        .stdout;
    let json_str= String::from_utf8(output);
    json_str
}
