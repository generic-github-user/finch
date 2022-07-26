use std::env;
use std::fs;
use std::any::Any;
use std::mem;

#[derive(PartialEq, Clone)]
enum Node {
    Character(Character),
    Expr(Expr),
    Statement(Statement),
    Block
}
