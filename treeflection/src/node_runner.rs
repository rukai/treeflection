use ::node_token::NodeToken;

#[derive(Clone)]
pub struct NodeRunner {
    pub tokens: Vec<NodeToken>,
}

impl NodeRunner {
    // TODO: Currently the command must begin with a ChainProperty.
    // However there is no reason this has to be the case.
    pub fn new(command: &str) -> Result<NodeRunner, String> {
        // add first identifier to token as property
        // get next identifier, could be:
        // *   ChainProperty - starts with '.'
        // *   ChainKey      - starts with '[0-9' ends with ']'
        // *   ChainIndex    - starts with '[a-z' ends with ']'
        // *   ChainContext  - is '[?]'
        // repeat until space found
        // then add identifier as action including any arguments seperated by spaces
        let mut tokens: Vec<NodeToken> = vec!();
        let mut token_progress = NodeTokenProgress::ChainProperty;
        let mut token_begin = 0;

        let chars: Vec<char> = command.chars().collect();
        for (i, c_ref) in chars.iter().enumerate() {
            let c = *c_ref;
            if c == '.' || c == ' ' || c == '[' {
                tokens.push(match token_progress {
                    NodeTokenProgress::ChainProperty => {
                        let token_str = &command[token_begin..i];
                        if token_str.len() == 0 {
                            return Err (String::from("Missing property"));
                        }
                        NodeToken::ChainProperty (token_str.to_string())
                    }

                    NodeTokenProgress::ChainIndex => {
                        let token_str = &command[token_begin..i-1];
                        if token_str.len() == 0 {
                            return Err (String::from("Missing index"));
                        }
                        match command[token_begin..i-1].parse() {
                            Ok (index) => NodeToken::ChainIndex (index),
                            Err (_)    => return Err (String::from("Not a valid index"))
                        }
                    }

                    NodeTokenProgress::ChainKey => {
                        let token_str = &command[token_begin..i-1];
                        if token_str.len() == 0 {
                            return Err (String::from("Missing index"));
                        }
                        NodeToken::ChainKey (token_str.to_string())
                    }

                    NodeTokenProgress::ChainContext => {
                        NodeToken::ChainContext
                    }

                    _ => { return Err(String::from("Whooops. This shouldnt happen.")) }
                });
                token_begin = i+1;
            }

            match c {
                '.' => {
                    token_progress = NodeTokenProgress::ChainProperty;
                }
                ' ' => {
                    token_progress = NodeTokenProgress::Action;
                    break;
                }
                '[' => {
                    if let Some(next_c) = chars.get(i+1) {
                        if next_c.is_digit(10) {
                            token_progress = NodeTokenProgress::ChainIndex;
                        }
                        else if next_c.is_alphabetic() {
                            token_progress = NodeTokenProgress::ChainKey;
                        }
                        else if *next_c == '?' {
                            token_progress = NodeTokenProgress::ChainContext;
                        }
                        else {
                            return Err (String::from("Not a valid key or index."));
                        }
                    }
                    else {
                        return Err (String::from("Unfinished key or index."));
                    }
                }
                _ => { }
            }
        }

        // add action
        if let NodeTokenProgress::Action = token_progress {
            let mut action = command[token_begin..].split_whitespace();
            tokens.push(match action.next() {
                Some("help")      => NodeToken::Help,
                Some("edit")      => NodeToken::Edit,
                Some("copy")      => NodeToken::CopyFrom,
                Some("paste")     => NodeToken::PasteTo,
                Some("get")       => NodeToken::Get,
                Some("set")    => {
                    // TODO: All groups of whitespace get converted into a single string. This could be an issue.
                    let mut set_value: Vec<&str> = vec!();
                    for token in action {
                        set_value.push(token);
                    }
                    NodeToken::Set(set_value.join(" "))
                }
                Some("insert") => {
                    match action.next() {
                        Some(arg) => {
                            match arg.parse() {
                                Ok(index) => NodeToken::Insert(index),
                                Err(_)    => return Err(String::from("Index must be an integer"))
                            }
                        }
                        None => {
                            NodeToken::Insert(0)
                        }
                    }
                }
                Some("remove") => {
                    match action.next() {
                        Some(arg) => {
                            match arg.parse() {
                                Ok(index) => NodeToken::Remove(index),
                                Err(_)    => return Err(String::from("Index must be an integer"))
                            }
                        }
                        None => {
                            NodeToken::Remove(0)
                        }
                    }
                }
                Some("variant") => {
                    NodeToken::SetVariant (action.next().unwrap().to_string())
                }
                Some("reset") => { NodeToken::SetDefault }
                Some(&_)      => return Err (String::from("Action is invalid")), // TODO: Custom actions
                None          => return Err (String::from("This should be unreachable: No Action"))
            });
        }
        else {
            return Err (String::from("No action"));
        }

        tokens.reverse();
        println!("{:?}", tokens); // TODO: This should be deleted, it is really useful for now though ...

        Ok(NodeRunner {
            tokens: tokens
        })
    }

    pub fn step(&mut self) -> NodeToken {
        self.tokens.pop().unwrap()
    }
}

pub enum NodeTokenProgress {
    ChainContext,
    ChainProperty,
    ChainIndex,
    ChainKey,
    Action
}
