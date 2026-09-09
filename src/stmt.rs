use crate::{decl::Decl, expr::Expr};
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub enum stmt_t {
	STMT_DECL,
	STMT_EXPR,
	STMT_IF_ELSE,
	STMT_FOR,
	STMT_PRINT,
	STMT_RETURN,
	STMT_BLOCK
}

pub struct Stmt {
	pub kind:      stmt_t,
	pub decl:      Option<Box<Decl>>,
	pub init_expr: Option<Box<Expr>>,
	pub expr:      Option<Box<Expr>>,
	pub next_expr: Option<Box<Expr>>,
	pub body:      Option<Box<Stmt>>,
	pub else_body: Option<Box<Stmt>>,
	pub next:      Option<Box<Stmt>>
}

pub fn stmt_create( kind: stmt_t, decl: Option<Box<Decl>>, init_expr: Option<Box<Expr>>, expr: Option<Box<Expr>>, next_expr: Option<Box<Expr>>, body: Option<Box<Stmt>>, else_body: Option<Box<Stmt>>, next:Option<Box<Stmt>> ) -> Stmt{
    let s = Stmt{
        kind: kind,
        decl: decl,
        init_expr: init_expr,
        expr: expr,
        next_expr: next_expr,
        body: body,
        else_body: else_body,
        next: next,
    };

	return s;
}

// void stmt_print( struct stmt *s, int indent, bool skip_indent, bool skip_return ) {
//     if (!s) return;

//     if ((!s->init_expr) && (skip_indent == false)) {
//         print_indent(indent);
//     }

//     switch (s->kind) {
//         case STMT_DECL:
//             decl_print(s->decl, indent);
//             break;
//         case STMT_EXPR:
//             expr_print(s->expr);
//             printf(";");
//             if (skip_return){
//                 break;
//             }
//             printf("\n");
//             break;
//         case STMT_IF_ELSE:
//             printf("if (");
//             expr_print(s->expr);
//             printf(") ");

//             if (s->body){
//                 if (s->body->kind == STMT_BLOCK){
//                     stmt_print(s->body, indent, true, false);
//                 } else {
//                     stmt_print(s->body, indent + 1, true, true);
//                 }
//             }

//             if (s->else_body){
//                 printf(" else ");
//                 if (s->else_body->kind == STMT_BLOCK){
//                     stmt_print(s->else_body, indent, true, false);
//                 } else if (s->else_body->kind == STMT_IF_ELSE) {
//                     stmt_print(s->else_body, indent, true, false);
//                     break;
//                 } else {
//                     stmt_print(s->else_body, indent + 1, true, true);
//                 }
//             }
//             printf("\n");
//             break;
//         case STMT_FOR:
//             if (!skip_indent){
//                 print_indent(indent);
//             }
//             printf("for (");
//             if (s->init_expr){
//                 expr_print(s->init_expr);
//                 printf(";");
//             } else {
//                 printf(";");
//             }
//             if (s->expr){
//                 expr_print(s->expr);
//                 printf(";");
//             } else {
//                 printf(";");
//             }
//             if (s->next_expr){
//                 expr_print(s->next_expr);
//             }
//             printf(")");

//             if (s->body->kind == STMT_BLOCK) {
//                 stmt_print(s->body, indent, true, false);
//             } else {
//                 printf(" ");
//                 stmt_print(s->body, indent + 1, true, true);
//             }
//             printf("\n");
//             break;
//         case STMT_PRINT:
//             printf("print");
//             if (s->expr) printf(" ");
//             expr_print(s->expr);
//             printf(";");
//             if (skip_return){
//                 break;
//             }
//             printf("\n");
//             break;
//         case STMT_RETURN:
//             printf("return");
//             if (s->expr) printf(" ");
//             expr_print(s->expr);
//             printf(";");
//             if (skip_return){
//                 break;
//             }
//             printf("\n");
//             break;
//         case STMT_BLOCK:
//             if (s->body && s->body->kind == STMT_BLOCK) {
//                 stmt_print(s->body, indent, true, false);
//                 break;
//             }
//             printf("{\n");
//             print_indent(indent + 1);
//             stmt_print(s->body, indent + 1, true, false);
//             print_indent(indent);
//             printf("}");
//             break;
//         default:
//             fprintf(stderr, "print error: Invalid statement.\n");
//             exit(1);
//     }
//     stmt_print(s->next, indent, false, false);
// }