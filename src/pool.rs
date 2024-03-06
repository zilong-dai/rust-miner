use clap::Parser;

#[derive(Parser, Debug)]
pub struct Pool {
    #[clap(short, long, required = true)]
    pool: String,

    #[clap(short, long, required = true)]
    user: String,

    #[clap(short = 'w', long)]
    passwd: Option<String>,

    #[clap(short, long, default_value="iron")]
    algo: String,
}

#[test]
fn test_pool_parser() {
    // let args: Vec<String> = std::env::args().collect();
    // let args: Vec<String> = ["--pool", "pool0", "--user", "longer"].iter().map(|arg| arg.to_string() ).collect();
    let args = vec!["--pool=pool0", "--user=longer"];
    let pool = Pool::parse_from(args);
    println!("{:?}", pool);
}
