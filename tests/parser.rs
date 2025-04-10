#[test]
fn parser() {
    let nlisp_machine: Nlisp = Nlisp.default();
    let tokens: Vec<Token> = Nlisp.tokenize("(+ 1 1)");
    let result: Ast = Nlisp.parse(tokens);

    assert_eq!(result, Ast { });
}
