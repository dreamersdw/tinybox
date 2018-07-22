pub fn gen() {
    let aliases = r#"
# add these lines to your ~/.bashrc or ~/.zshrc

alias tlock="tinybox lock"
alias tloop="tinybox loop"
alias tsum="tinybox tsum"
alias twatch="tinybox watch""#;
    println!("{}", aliases);
}
