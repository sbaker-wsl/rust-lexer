// Seth Baker
use rand::Rng;

#[allow(dead_code)]
#[derive(Clone,Debug)]
pub struct Rule {
    lhs: char,
    rhs: String,
}

#[allow(dead_code)]
impl Rule {
    pub fn new(lhs : char, rule : &str) -> Self {
        Self {
            lhs,
            rhs : rule.to_string(),
        }
    }

    pub fn is_valid(&self) -> bool {
        return self.lhs.is_ascii_uppercase();
    }

    pub fn is_right_regular(&self) -> bool {
        return self.rhs.chars().last().unwrap().is_ascii_uppercase();
    }

    pub fn is_left_regular(&self) -> bool {
        return self.rhs.chars().next().unwrap().is_ascii_uppercase();
    }

    pub fn print(&self) {
        println!("{} ; {}", self.lhs, self.rhs);
   }

}

#[allow(dead_code)]
pub struct Grammar {
    terms: String,
    non_terms: String,
    rules: Vec<Rule>,
    start: char,
}

#[allow(dead_code)]
impl Grammar {
    pub fn from_rules(rules : &Vec<Rule>) -> Self {
        let mut t = String::from("");
        let mut nt = String::from("");
        for rule in rules {
            for c in rule.rhs.chars() {
                if c.is_ascii_uppercase() {
                    nt.push(c);
                } else {
                    t.push(c);
                }
            }
        }

        Self {
            terms : t.to_string(),
            non_terms : nt.to_string(),
            rules : rules.to_vec(),
            start : rules[0].lhs,
        }
    }

    pub fn is_valid(&self) -> bool {
        for rule in &self.rules {
            if !rule.is_valid() {
                return false;
            }
        }
        return true;
    }

    pub fn is_regular(&self) -> bool {
        let mut right_reg = false;
        let mut left_reg = false;
        for rule in &self.rules {
            if rule.is_right_regular() {
                right_reg = true;
            } else if rule.is_left_regular() {
                left_reg = true;
            }
        }
        if right_reg == true && left_reg == true {
            return false;
        }
        return left_reg || right_reg;
    }

    pub fn rule_idxs_from_nt(&self, nt : char) -> Vec<usize> {
        let mut counter : usize = 0;
        let mut vec = Vec::<usize>::new();
        for rule in &self.rules {
            if rule.lhs == nt {
                vec.push(counter);
            }
            counter += 1;
        }
        vec
    }

    pub fn print(&self) {
        println!("terms {}\nnon_terms {}\nrules {:?}\nstart {}", self.terms, self.non_terms, self.rules, self.start);
    }
}

pub struct Sentenial {
    symbols : String,
    index : isize,
}

impl Sentenial {
    pub fn new_initial(grammar : &Grammar) -> Self {
        Self {
            symbols : grammar.start.to_string(),
            index : 0,
        }
    }

    pub fn new_next(&self, grammar : &Grammar, index : usize) -> Self {
        let mut new_syms = String::new();
        let mut position = 0;
        let mut new_index = -1;
        for c in self.symbols.chars() {
            if position != self.index as usize {
                new_syms.push(c);
            }
            position += 1;
        }
        new_syms.insert_str(self.index as usize, &grammar.rules[index].rhs);
        position = 0;
        for c in new_syms.chars() {
            if c.is_ascii_uppercase() {
                new_index = position as isize;
                break;
            }
            position += 1;
        }

        Self {
            symbols : new_syms,
            index : new_index,
        }

    }

    pub fn is_complete(&self) -> bool {
        return self.index == -1;
    }

    pub fn print(&self) {
        println!("symbols {}\nfirst nonterminal {}", self.symbols, self.index);
    }
}

#[allow(dead_code)]
pub struct Derivation {
    sent_form : Sentenial,
    applied_rule : isize,
    steps : Vec<(String, isize)>,
}

#[allow(dead_code)]
impl Derivation {
    pub fn new(grammar : &Grammar) -> Self {
        Self { 
            sent_form : Sentenial::new_initial(&grammar),
            applied_rule : -1,
            steps : Vec::<(String, isize)>::new(),
        }
    }

    pub fn derive_leftmost(&mut self, grammar : &Grammar, index : usize) {
        let sent_new = self.sent_form.new_next(&grammar, index);
        self.steps.push((self.sent_form.symbols.clone(),self.applied_rule));
        self.sent_form = sent_new;
        self.applied_rule = index as isize;
    }   

    pub fn is_complete(&self) -> bool {
        return self.sent_form.is_complete();
    }

    pub fn leftmost_nonterminal(&self) -> char {
        return self.sent_form.symbols.chars().nth(self.sent_form.index as usize).unwrap();
    }

    pub fn print(&self) {
        println!("sentenial {}\nrule {}\nsteps {:?}", self.sent_form.symbols, self.applied_rule, self.steps);
    }
}

#[allow(dead_code)]
pub fn example_manual() {
    let rules = vec!(
        Rule::new('N', "n+&N"),
        Rule::new('N', "E"),
        Rule::new('E', "(e/x)-E"),
        Rule::new('E', "z"),
    );
    let grammar = Grammar::from_rules(&rules);

    println!("grammar valid={:}", grammar.is_valid());
    println!("grammar regular={:}", grammar.is_regular());

    let mut derivation = Derivation::new(&grammar);
    derivation.derive_leftmost(&grammar,0);
    derivation.derive_leftmost(&grammar,0);
    derivation.derive_leftmost(&grammar,1);
    derivation.derive_leftmost(&grammar,2);
    derivation.derive_leftmost(&grammar,2);
    derivation.derive_leftmost(&grammar,2);
    derivation.derive_leftmost(&grammar,3);

    println!("derivation complete={:}", derivation.is_complete());
    println!("derivation word={:}", derivation.sent_form.symbols);
    derivation.print();
}

pub fn print_random(step_limit : usize) -> Option<Sentenial> {
    let rules = vec!(
        Rule::new('N', "n+&N"),
        Rule::new('N', "E"),
        Rule::new('E', "(e/x)-E"),
        Rule::new('E', "z"),
    );
    let mut rng = rand::thread_rng();
    let mut step_count = 0;
    let grammar = Grammar::from_rules(&rules);
    let mut derivation = Derivation::new(&grammar);
    let mut rule_track = 0; // 0 for first 2 rules, 1 for second 2 rules
    while step_count < step_limit && !derivation.is_complete() {
        let random_float : f64 = rng.r#gen();
        if random_float > 0.8 {
            match rule_track {
                0 => {
                    derivation.derive_leftmost(&grammar, 1);
                    rule_track += 1;
                },
                _ => {
                    derivation.derive_leftmost(&grammar, 3);
                }
            }
        } else {
            match rule_track {
                0 => {
                    derivation.derive_leftmost(&grammar, 0);
                }
                _ => {
                    derivation.derive_leftmost(&grammar, 2);
                }
            }
        }
        step_count += 1;
    }
    if derivation.is_complete() {
        return Some(derivation.sent_form);
    }
    return None
}

