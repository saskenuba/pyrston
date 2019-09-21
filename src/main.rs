use tree_sitter::{Parser, Language};

extern "C" { fn tree_sitter_python() -> Language; }

fn main()  {
    let mut parser = Parser::new();

    let python = unsafe { tree_sitter_python() };
    parser.set_language(python).unwrap();

    let source_code = "import os\n\ndef test():\n    teste = list()";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();


    println!("{:?}", tree);
    tree.walk();
    println!("{:?}", root_node.to_sexp());


}
