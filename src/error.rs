error_chain! {
    foreign_links {
        Discord(::discord::Error);
        Regex(::regex::Error);
    }
}
