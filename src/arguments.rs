use clap::Arg;

pub struct Args {}

impl Args {
    pub fn username() -> Arg<'static> {
        let username: Arg = Arg::new("username")
            .short('u')
            .long("username")
            .takes_value(true)
            .help("The required username.")
            .required(true);

        username
    }
   
    pub fn password() -> Arg<'static> {
        let password: Arg = Arg::new("password")
            .short('p')
            .long("password")
            .takes_value(true)
            .help("The raw password for the user.")
            .required(true);
        
        password
    }
}