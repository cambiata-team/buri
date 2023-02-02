use typed_ast::ConcreteStringLiteralExpression;

// Helper function to convert a numeric value [0,15] to a hex digit [0,f].
// Undefined behavior for inputs outside the given domain.
fn hex_value_to_hex_digit(value: u32) -> char {
    char::from_u32(if value < 10 { value + 48 } else { value + 87 })
        .map_or_else(|| unreachable!(), |x| x)
}

pub fn print_string_literal(node: &ConcreteStringLiteralExpression) -> String {
    let mut result = String::new();
    result.push('\"');
    for character in node.value.chars() {
        match character {
            '\x08' => {
                result.push_str("\\b");
            }
            '\t' => {
                result.push_str("\\t");
            }
            '\n' => {
                result.push_str("\\n");
            }
            '\x0B' => {
                result.push_str("\\v");
            }
            '\x0C' => {
                result.push_str("\\f");
            }
            '\r' => {
                result.push_str("\\r");
            }
            '\"' => {
                result.push_str("\\\"");
            }
            '\'' => {
                result.push_str("\\\'");
            }
            '\\' => {
                result.push_str("\\\\");
            }
            '\0'..='\x1F' | '\x7F' => {
                let unicode_codepoint = character as u32;
                result.push_str("\\x");
                result.push(hex_value_to_hex_digit(unicode_codepoint >> 4));
                result.push(hex_value_to_hex_digit(unicode_codepoint & 0xF));
            }
            _ => {
                result.push(character);
            }
        }
    }
    result.push('\"');
    result
}

#[cfg(test)]
mod test {
    use super::*;

    use typed_ast::{ConcreteType, PrimitiveType};

    #[test]
    fn simple_string_literal() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "hello".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"hello\"");
    }

    #[test]
    fn newline_is_escaped() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "\n".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"\\n\"");
    }

    #[test]
    fn non_graphic_ascii_is_escaped() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "\x1F".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"\\x1f\"");
    }

    #[test]
    fn non_ascii_character_is_not_escaped() {
        let node = ConcreteStringLiteralExpression {
            expression_type: ConcreteType::Primitive(PrimitiveType::Str),
            value: "π".to_string(),
        };
        assert_eq!(print_string_literal(&node), "\"π\"");
    }
}
