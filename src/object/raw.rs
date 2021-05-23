use crate::object::{*};

use std::fmt;

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let header = "#".repeat(self.level);
        let class = &self.class;
        write!(
            f,
            "{header} {class}\n",
            header = header,
            class = class,
        );
        for key in &self.keys {
            let value = match self.variables.get(&key.to_string()) {
                Some(v) =>  v,
                None => unreachable!(),
            };
            write!(
                f,
                "- {key} ({typ}): {value}\n",
                key = key,
                typ = value.typ,
                value = value,
            );
        }

        for subentry in &self.subentries {
            write!(f, "\n{}", subentry);
        }

        fmt::Result::Ok(())
    }

}

impl fmt::Display for ValueType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let typ = match &*self {
            Self::Str => "str".to_string(),
            Self::Num => "num".to_string(),
            Self::Bool => "bool".to_string(),
            Self::Ref => "ref".to_string(),
            Self::List(t) => format!("list {}", t.to_string()),
            Self::Unknown => unreachable!(),
        };
        write!(f, "{}", typ)
    }
}

impl fmt::Display for ValueBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match &self {
            Self::Str(s) =>  format!("{}", s),
            Self::Num(s) =>  format!("{}", s),
            Self::Bool(s) => format!("{}", s),
            Self::List(list) => format!("{}", list),
            Self::Unknown(_) => unreachable!(),
            Self::Ref(_, _) => unreachable!(),
        };
        write!(f, "{}", value)
    }
}

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.items {
            write!(
                f,
                "\n  * {item}",
                item = item,
            );
        }
        write!(f, "")
    }

}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value);
        match &self.annotation {
            Some(ann) => {
                let indent_num = match &self.typ {
                    ValueType::List(_) => 4,
                    _ => 2,
                };
                let indent = " ".repeat(indent_num);
                write!(
                    f,
                    "\n{indent}> {annotation}",
                    indent = indent,
                    annotation = ann,
                )
            },
            None => fmt::Result::Ok(()),
        }
    }
}
