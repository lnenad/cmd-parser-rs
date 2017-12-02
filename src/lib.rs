use std::env;
use std::collections::HashMap;

type ArgumentValues = Vec<String>;
type ArgumentList = HashMap<String, ArgumentValues>;
type Flags = Vec<String>;

/**
* Parser structure
*/
pub struct Parser{
    prefixes: Option<Vec<String>>,
    merge: bool
}

/**
* Parser implementation
*/
impl Parser {

    /**
    * Constructor
    *
    * @return Parser
    */
    pub fn new() -> Parser {
        Parser {
            prefixes: None,
            merge: false,
        }
    }

    /**
    * Sets the prefix for the arguments
    *
    * @param prefixes Vec<String>
    * @return Parser
    */
    pub fn strict_prefix(&mut self, prefixes: Vec<String>) -> &Self {
        self.prefixes = Some(prefixes);
        self
    }

    /**
    * Disables overwriting of arguments and instead merges them
    *
    * @param prefixes Vec<String>
    * @return Parser
    */
    pub fn merge_values(&mut self, merge: bool) -> &Self {
        self.merge = merge;
        self
    }

    /**
    * Parses the given cmd arguments
    *
    * @return Touple(ArgumentList, Flags)
    */
    pub fn parse(&self) -> (ArgumentList, Flags) {
        let mut named_arguments: ArgumentList = ArgumentList::new();
        let mut flags: Flags = Flags::new();
        let mut argument_name: String;
        let received_arguments = env::args().collect::<Vec<String>>();
        let argument_prefixes = match self.prefixes {
            Some(ref prefixes) => prefixes.clone(),
            None => vec!("-".to_owned(), "--".to_owned())
        };

        for (index, argument) in received_arguments.iter().enumerate() {
            for prefix in &argument_prefixes {
                if argument.starts_with(&prefix[..]) {
                    let mut argument_values = vec![];
                    argument_name = argument[prefix.len()..argument.len()].to_owned();
                    for argument_value in &received_arguments[index+1..] {
                        if argument_value.starts_with(&prefix[..]) {
                            break;
                        }
                        argument_values.push(argument_value.to_owned());
                    }
                    if argument_values.len() == 0 {
                        flags.push(argument_name);
                        continue;
                    }

                    if self.merge == true {
                        if let Some(existing_values) = named_arguments.get(&argument_name) {
                            argument_values.append(&mut existing_values.clone());
                        }
                    }

                    named_arguments.insert(argument_name.clone(),argument_values);
                }
            }
        }

        (named_arguments, flags)
    }
}