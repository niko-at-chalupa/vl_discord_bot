use scraper::{Html, Node};
use ego_tree::NodeRef;

pub fn html_to_discord_md(html: &str) -> String {
    let fragment = Html::parse_fragment(html);
    let mut out = String::new();
    walk(fragment.tree.root(), &mut out, false);
    // Collapse 3+ blank lines down to max 2 (just one blank line between blocks)
    let collapsed = out
        .split("\n\n\n")
        .collect::<Vec<_>>()
        .join("\n\n");
    collapsed.trim().to_string()
}

fn walk(node: NodeRef<Node>, out: &mut String, in_list: bool) {
    for child in node.children() {
        match child.value() {
            Node::Text(text) => {
                out.push_str(&text.text);
            }
            Node::Element(el) => {
                let tag = el.name();
                match tag {
                    "b" | "strong" => {
                        out.push_str("**");
                        walk(child, out, in_list);
                        out.push_str("**");
                    }
                    "i" | "em" => {
                        out.push('*');
                        walk(child, out, in_list);
                        out.push('*');
                    }
                    "u" => {
                        out.push_str("__");
                        walk(child, out, in_list);
                        out.push_str("__");
                    }
                    "s" | "strike" | "del" => {
                        out.push_str("~~");
                        walk(child, out, in_list);
                        out.push_str("~~");
                    }
                    "code" => {
                        out.push('`');
                        walk(child, out, in_list);
                        out.push('`');
                    }
                    "br" => {
                        out.push('\n');
                    }
                    "p" | "div" => {
                        walk(child, out, in_list);
                        out.push_str("\n\n");
                    }
                    "ul" | "ol" => {
                        walk(child, out, true);
                        out.push('\n');
                    }
                    "li" => {
                        out.push_str("- ");
                        walk(child, out, in_list);
                        out.push('\n');
                    }
                    "a" => {
                        let href = el.attr("href").unwrap_or("");
                        let mut label = String::new();
                        walk(child, &mut label, in_list);
                        if href.is_empty() {
                            out.push_str(&label);
                        } else {
                            out.push_str(&format!("[{label}]({href})"));
                        }
                    }
                    _ => {
                        // Unknown tag — just recurse into children, ignore the tag itself
                        walk(child, out, in_list);
                    }
                }
            }
            _ => {}
        }
    }
}