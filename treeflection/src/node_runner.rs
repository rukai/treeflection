use ::node_token::NodeToken;

#[derive(Clone)]
pub struct NodeRunner {
    pub tokens: Vec<NodeToken>
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
                    NodeTokenProgress::Action => {
                        NodeToken::Get
                    }
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
                Some("get") => NodeToken::Get,
                Some("set") => {
                    match action.next() {
                        Some(arg) => NodeToken::Set(arg.to_string()),
                        None => return Err (String::from("No argument given to set action"))
                    }
                }
                Some("copy")  => NodeToken::CopyFrom,
                Some("paste") => NodeToken::PasteTo,
                Some(&_)      => return Err (String::from("Action is invalid")), // TODO: Custom actions
                None          => return Err (String::from("This should be unreachable: No Action"))
            });
        }
        else {
            return Err (String::from("No action"));
        }

        tokens.reverse();
        println!("{:?}", tokens);

        Ok(NodeRunner {
            tokens: tokens
        })
    }

    pub fn step(&mut self) -> NodeToken {
        self.tokens.pop().unwrap()
    }
}

pub enum NodeTokenProgress {
    ChainProperty,
    ChainIndex,
    ChainKey,
    Action
}
