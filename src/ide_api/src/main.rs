use tree_sitter::{Parser, Language};
use std::env;

extern "C" { fn tree_sitter_python() -> Language; }

fn main()  {
    println!("{:?}", env::var_os("OUT_DIR"));
    let mut parser = Parser::new();

    let python = unsafe { tree_sitter_python() };
    parser.set_language(python).unwrap();

    let source_code = "import os\n\ndef test() -> str:\n    teste = list()\n\nclass Basinga:\n    def __init__(self, bolo):\n        self.bolo = bolo";
    let tree = parser.parse(source_code, None).unwrap();
    let root_node = tree.root_node();

    let child_node = root_node.child(0);
    let child_node1 = root_node.child(1);

    //println!("{:#?}", tree);
    //println!("{:#?}", root_node.to_sexp());
    //println!("{:#?}", child_node);
    //println!("{:#?}", root_node);

    let hue = root_node.children().into_iter().for_each(|x| println!("{:?}", x));


}
