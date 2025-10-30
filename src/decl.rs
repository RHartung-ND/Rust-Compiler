use crate::expr::Expr;

pub struct Decl {
    name:String,
    Type:Type,
    value:Expr,
    code:Stmt,
    next:Decl
}

pub fn decl_create( name: String, Type: Type, value: Expr, code: Stmt, next: Decl ) -> Decl {
    
    let d = Decl {
        name:name,
        Type:Type,
        value:Expr,
        code:Stmt,
        next:Decl,
    };
	if (!d) {
        fprintf(stderr, "print error: unable to allocate memory for declaration node in AST.\n");
        exit(1);
    }
	
	return d;
}

pub fn decl_print( d: Decl, num_spaces: i8 ){
    if !d {
        return;
    }

	print_indent(num_spaces);

    printf("%s: ", d->name);
    
    type_print(d->type);
	
    if (d->value) {
        printf(" = ");
        if (d->value->kind == EXPR_LIST){
            printf("{");
            expr_print(d->value);
            printf("}");
        } else {
            expr_print(d->value);
            printf(";");
        }
    } else if (d->code) {
        printf(" = ");
        stmt_print(d->code, num_spaces);
    } else {
        printf(";");
    }
    printf("\n");

    decl_print(d->next, num_spaces);
}