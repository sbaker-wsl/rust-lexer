pub fn help_display(command : &str) {
    clearscreen::clear().expect("failed to clear screen");
    match command {
        "" => help_help_display(),
        "help" => help_help_display(),
        "print" => help_print_display(),
        "list" => help_list_display(),
        "derive" => help_derive_display(),
        "tokenize" => help_tokenize_display(),
        _ => println!("working on it"),
    }
}

pub fn print_list() {
    clearscreen::clear().expect("failed to clear screen");
    println!("Commands:\n\thelp [command]\n\tprint <file> [--numbered]\n\tlist\nderive\ntokenize");
}

pub fn print_tokens_list() {
    clearscreen::clear().expect("failed to clear screen");
    println!("Tokens:");
    println!("PARENS_L, PARENS_R, BRACKET_L, BRACKET_R, BRACE_L, BRACE_R");
    println!("POINT, COMMA, COLON, SEMICOLON, ARROW_R");
    println!("ADD, SUB, MUL, DIV");
    println!("EQ, LT, GT, NEQ, NLT, NGT");
    println!("NOT, AND, OR");
    println!("ASSIGN");
    println!("FUNC, LET, IF, ELSE, WHILE, PRINT");
    println!("ID");
    println!("TYPE_INT32, TYPE_FLT32, TYPE_CHAR");
    println!("LIT_INT32, LIT_FLT32, LIT_CHAR, LIT_STRING");
    println!("EOI");
}

pub fn print_rules() {
    clearscreen::clear().expect("failed to clear screen");
    println!("Rules:\n\t(1)\tN->n+&N\n\t(2)\tN->E\n\t(3)\tE->(e/x)-E\n\t(4)\tE->z");
}

pub fn help_derive_display() {
    println!("<----------------------------- Welcome to the help page for the derive command! ------------------------------>\n");
    println!("The derive command allows you to derive from the starting symbol to a valid or invalid word\ngiven your supplied steps. Steps are given numbers assocaited with the rule you wish to apply to the\nsentenial form in your current derivation. Type your applied rules with spaces separating them, commas\nor other separators ARE NOT supported and will not be. See rules and their numbers via the 'list rules' command\n\nYou can also use the other optional command 'random' to generate a random word from the starting symbol\nand the given rules, which may either be a word or return invalid (case: there is a step limit that after\nn steps the derivation will give up on trying to generate a word)\n\n");
    println!("Usage: derive [random | int-list] (example for int-list : derive 0 0 0 1 2 3)\n");
    println!("<------------------------------------------------------------------------------------------------------------->");
}

/* meant to be called when help is called without a command or when 
'help help' is called */
pub fn help_help_display() {
    println!("<--- Welcome to the help page for the help command! --->\n");
    println!("The help command will take you to a help page associated \nwith your specified command\n");
    println!("Usage: help [command]\n");
    println!("Options for commands are:\n\thelp\n\tprint\n\tlist commands\n");
    println!("<------------------------------------------------------>");
}

/* called when 'help print' is called */
pub fn help_print_display() {
println!("<--- Welcome to the help page for the print command! --->\n");
    println!("The print command prints the contents of supplied file\nand has an optional flag --numbered which will print the\ncontents of the file where each line is numbered\n");
    println!("Usage: print <filename> [--numbered]\n");
    println!("<------------------------------------------------------->");
}    

pub fn help_list_display() {
    println!("<-------- Welcome to the help page for the list command! -------->\n");
    println!("The list commands command will list the commands available for this\ncommand line application");
    println!("It also can be used to list the rules for the implemented grammar,\nand list the tokens for the implemented lexer\n");
    println!("Usage: list commands; list rules; list tokens");
    println!("<---------------------------------------------------------------->");
}

pub fn help_tokenize_display() {
    println!("<--- Welcome to the help page for the tokenize command! --->\n");
    println!("The tokenize command will tokenize the contents of a file!\nUse list tokens to verify your results. Note that there are some");
    println!("conditions for your file, they must be valid for specific use \ncases(i.e. you cannot put more than one char inbetween ' ')\n");
    println!("Usage: tokenize <filename> (include extension!)");
    println!("<---------------------------------------------------------->");
}
