use crate::node_token::NodeToken;
use std::slice::Iter;

#[derive(Clone)]
pub struct NodeRunner {
    pub tokens: Vec<NodeToken>,
}

impl NodeRunner {
    pub fn new(command: &str) -> Result<NodeRunner, String> {
        let mut tokens: Vec<NodeToken> = vec!();

        if command.len() == 0 {
            return Err(String::from("Empty command"));
        }

        let chars: Vec<char> = {
            let mut chars = vec!();
            if !command.starts_with('.') && !command.starts_with('[') && !command.starts_with(':') {
                chars.push('.');
            }
            chars.extend(command.chars());
            chars
        };

        // repeat:
        // *         if '.' then property, consume until before '.' or '[' or '>'
        // *    else if '[?]' then context, consume it.
        // *    else if '[*]' then all, consume it.
        // *    else if '["' then index, consume until '"]'
        // *    else if '[' then key, consume until ']'
        // *    else if '>' then action, consume arguments seperated by ' ' until end of string
        let mut i = 0;
        loop {
            if chars[i] == '.' {
                let mut prop_string = String::new();
                if i + 1 >= chars.len() {
                    return Err(String::from("Missing action"));
                }

                let mut next = chars[i+1];
                while next != '.' && next != '[' && next != ':' {
                    i += 1;
                    prop_string.push(chars[i]);
                    if i + 1 >= chars.len() {
                        return Err(String::from("Missing action"));
                    }
                    next = chars[i+1];
                }

                if i + 1 >= chars.len() {
                    return Err(String::from("Missing action"));
                }
                i += 1;

                if prop_string.len() == 0 {
                    return Err(String::from("Empty property"));
                }
                tokens.push(NodeToken::ChainProperty (prop_string));
            }
            else if i + 2 < chars.len() && chars[i] == '[' && chars[i+1] == '?' && chars[i+2] == ']' {
                if i + 3 >= chars.len() {
                    return Err(String::from("Missing action"));
                }
                i += 3;
                tokens.push(NodeToken::ChainContext);
            }
            else if i + 2 < chars.len() && chars[i] == '[' && chars[i+1] == '*' && chars[i+2] == ']' {
                if i + 3 >= chars.len() {
                    return Err(String::from("Missing action"));
                }
                i += 3;
                tokens.push(NodeToken::ChainAll);
            }
            else if i + 1 < chars.len() && chars[i] == '[' && chars[i+1] == '"' {
                let mut key_string = String::new();
                i += 1;
                if i + 4 >= chars.len() {
                    return Err(String::from("Missing action"));
                }
                let mut next1 = chars[i+1];
                let mut next2 = chars[i+2];
                while next1 != '"' || next2 != ']' {
                    i += 1;
                    key_string.push(chars[i]);
                    if i + 2 >= chars.len() {
                        return Err(String::from("Missing \"]"));
                    }
                    next1 = chars[i+1];
                    next2 = chars[i+2];
                }

                if i + 3 >= chars.len() {
                    return Err(String::from("Missing action"));
                }
                i += 3;

                tokens.push(NodeToken::ChainKey(key_string));
            }
            else if chars[i] == '[' {
                let mut index_string = String::new();
                if i + 1 >= chars.len() {
                    return Err(String::from("Missing action"));
                }

                let mut next = chars[i+1];
                while next != ']' {
                    i += 1;
                    index_string.push(chars[i]);
                    if i + 1 >= chars.len() {
                        return Err(String::from("Missing ]"));
                    }
                    next = chars[i+1];
                }

                if i + 2 >= chars.len() {
                    return Err(String::from("Missing action"));
                }
                i += 2;

                if index_string.len() == 0 {
                    return Err(String::from("Missing index"));
                }

                match index_string.parse() {
                    Ok (index) => tokens.push(NodeToken::ChainIndex (index)),
                    Err (_)    => return Err (format!("Invalid index: {}", index_string)),
                }
            }
            else if chars[i] == ':' {
                let tokenized = NodeRunner::tokenize_action(&chars[i+1..])?;
                tokens.push(NodeRunner::get_action(tokenized.iter())?);

                tokens.reverse();

                return Ok(NodeRunner {
                    tokens: tokens
                });
            }
            else {
                // This happens after a ] followed by a character that doesnt start a new property, key or index
                // So just assume the user forgot the dot on a property
                return Err(String::from("Missing ."));
            }
        }
    }

    // Split string into tokens by whitespace.
    // characters sorounded by quotes are considered one token regardless of whitespace
    fn tokenize_action(string: &[char]) -> Result<Vec<String>, String> {
        let mut tokens: Vec<String> = vec!();
        let mut current = String::new();
        let mut quoted = false;
        let mut escaped = false;
        for c in string {
            if escaped {
                match *c {
                    '"'  => { current.push('"') }
                    't'  => { current.push('\t') }
                    'n'  => { current.push('\n') }
                    ' '  => { current.push(' ') }
                    '\\' => { current.push('\\') }
                    _    => { }
                }
                escaped = false;
            }
            else if *c == '\\' {
                escaped = true;
            }
            else if !quoted && c.is_whitespace() {
                if current.len() > 0 {
                    tokens.push(current);
                    current = String::new();
                }
            }
            else if *c == '"' {
                if quoted {
                    tokens.push(current);
                    current = String::new();
                }
                quoted = !quoted;
            }
            else {
                current.push(*c);
            }
        }

        if current.len() > 0 {
            tokens.push(current);
        }

        if quoted {
            Err(String::from("Unterminated string"))
        } else {
            Ok(tokens)
        }
    }

    fn get_action(mut action: Iter<String>) -> Result<NodeToken, String> {
        match action.next().map(|x| x.as_ref()) {
            Some("help")    => Ok(NodeToken::Help),
            Some("reset")   => Ok(NodeToken::SetDefault),
            Some("edit")    => Ok(NodeToken::Edit),
            Some("copy")    => Ok(NodeToken::CopyFrom),
            Some("paste")   => Ok(NodeToken::PasteTo),
            Some("getkeys") => Ok(NodeToken::GetKeys),
            Some("get")     => Ok(NodeToken::Get),
            Some("set") => {
                let mut set_value: Vec<&str> = vec!();
                for token in action {
                    set_value.push(token);
                }
                Ok(NodeToken::Set(set_value.join(" ")))
            }
            Some("insert") => {
                match action.next() {
                    Some(arg0) => {
                        match action.next() {
                            Some(arg1) => {
                                match arg0.parse() {
                                    Ok(index) => {
                                        Ok(NodeToken::InsertIndexKey (index, arg1.to_string()))
                                    }
                                    Err(_) => Err(String::from("When two arguments are used, first must be a valid index."))
                                }
                            }
                            None => {
                                match arg0.parse() {
                                    Ok(index) => Ok(NodeToken::InsertIndex (index)),
                                    Err(_)    => Ok(NodeToken::InsertKey (arg0.to_string())),
                                }
                            }
                        }
                    }
                    None => {
                        Ok(NodeToken::Insert)
                    }
                }
            }
            Some("remove") => {
                match action.next() {
                    Some(arg) => {
                        match arg.parse() {
                            Ok(index) => Ok(NodeToken::RemoveIndex (index)),
                            Err(_)    => Ok(NodeToken::RemoveKey (arg.to_string())),
                        }
                    }
                    None => {
                        Ok(NodeToken::Remove)
                    }
                }
            }
            Some("variant") => {
                Ok(NodeToken::SetVariant (
                    match action.next() {
                        Some(value) => value.to_string(),
                        None        => String::new()
                    }
                ))
            }
            Some(action_name) => {
               let args: Vec<String> = action.cloned().collect();
               Ok(NodeToken::Custom(action_name.to_string(), args))
            }
            None     => Err (String::from("Missing action"))
        }
    }

    pub fn step(&mut self) -> NodeToken {
        self.tokens.pop().unwrap()
    }
}
