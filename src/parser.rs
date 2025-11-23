use tree_sitter::Parser;
use tree_sitter_orgncf::LANGUAGE;

#[derive(Debug)]
pub struct Line<'a> {
  pub number: usize,
  pub _text: &'a str,
  pub mac: Option<&'a str>,
  pub ip: &'a str,
  pub names: Vec<&'a str>,
}

#[derive(Debug)]
pub struct ParsedInfo<'a> {
  pub ip_lines: Vec<Line<'a>>,
  pub domain: &'a str,
  pub dns_prefix: &'a str,
  pub dns_suffix: &'a str,
  pub dhcp_prefix: &'a str,
  pub dns_file_name: &'a str,
  pub reverse_dns_file_name: &'a str,
  pub dhcp_file_name: &'a str,
}

pub fn parser(content: &str) -> Result<ParsedInfo<'_>, Box<dyn std::error::Error + '_>> {
  let mut parser = Parser::new();
  parser
    .set_language(&LANGUAGE.into())
    .expect("Error loading orgncf grammar");
  let tree = parser.parse(content, None).expect("Failed to parse input");

  if tree.root_node().has_error() {
    return Err("The parse tree contains errors.".into());
  }

  let mut ip_lines = Vec::<Line>::new();
  let mut domain = "";
  let mut dns_file_name = "";
  let mut reverse_dns_file_name = "";
  let mut dhcp_file_name = "";
  let mut dns_prefix = "";
  let mut dns_suffix = "";
  let mut dhcp_prefix = "";

  let val = |node: tree_sitter::Node| &content[node.byte_range()];

  let val_child = |node: tree_sitter::Node, child_idx: usize| {
    &content[node
      .child(child_idx)
      .expect("child should exist")
      .byte_range()]
  };
  let val_grandchild = |node: tree_sitter::Node, child_idx: usize, grandchild_idx: usize| {
    &content[node
      .child(child_idx)
      .expect("child should exist")
      .child(grandchild_idx)
      .expect("grandchild should exist")
      .byte_range()]
  };

  let mut cursor = tree.walk();
  if cursor.goto_first_child() {
    loop {
      let node = cursor.node();
      match node.kind() {
        "ip_config_line" => {
          let line_text = &content[node.byte_range()];
          let line_number = node.start_position().row + 1;

          let mut mac: Option<&str> = None;
          let mut ip: &str = "";
          let mut names: Vec<&str> = Vec::new();

          let mut line_cursor = node.walk();
          if line_cursor.goto_first_child() {
            loop {
              let child_node = line_cursor.node();
              match child_node.kind() {
                "mac_address" => mac = Some(val(child_node)),
                "ip_address" => ip = val(child_node),
                "hostname" => {
                  let name = val(child_node);
                  names.push(name);
                },
                _ => {},
              }
              if !line_cursor.goto_next_sibling() {
                break;
              }
            }
          }

          ip_lines.push(Line {
            number: line_number,
            _text: line_text,
            mac,
            ip,
            names,
          });
        },
        "domain" => domain = val_child(node, 1),
        "dns_file_name" => dns_file_name = val_child(node, 1),
        "reverse_dns_file_name" => reverse_dns_file_name = val_child(node, 1),
        "dhcp_file_name" => dhcp_file_name = val_child(node, 1),
        "dns_prefix_section" => dns_prefix = val_grandchild(node, 1, 1).trim_start(),
        "dns_suffix_section" => dns_suffix = val_grandchild(node, 1, 1).trim_start(),
        "dhcp_prefix_section" => dhcp_prefix = val_grandchild(node, 1, 1).trim_start(),
        _ => {},
      }
      //println!("Node: {:?}, Type: {}", node, node.kind());
      if !cursor.goto_next_sibling() {
        break;
      }
    }
  }
  Ok(ParsedInfo {
    ip_lines,
    domain,
    dns_prefix,
    dns_suffix,
    dhcp_prefix,
    dns_file_name,
    reverse_dns_file_name,
    dhcp_file_name,
  })
  //  println!("Parsed IP Lines: \n{:?}", ip_lines);
  //  println!("Parsed Domain: 「{}」", domain);
  //  println!("DNS File Name: 「{}」", dns_file_name);
  //  println!("Reverse DNS File Name: 「{}」", reverse_dns_file_name);
  //  println!("DHCP File Name: 「{}」", dhcp_file_name);
  //  println!("DNS Prefix: \n「{}」", dns_prefix);
  //  println!("DNS Suffix: \n「{}」", dns_suffix);
  //  println!("DHCP Prefix: \n「{}」", dhcp_prefix);
}
