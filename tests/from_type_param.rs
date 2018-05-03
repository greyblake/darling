#[macro_use]
extern crate darling;
extern crate syn;

use darling::FromTypeParam;
use syn::{DeriveInput, GenericParam, Ident, TypeParam};

#[darling(attributes(lorem), from_ident)]
#[derive(FromTypeParam)]
struct Lorem {
    ident: Ident,
    foo: bool,
    bar: Option<String>,
}

impl From<Ident> for Lorem {
    fn from(ident: Ident) -> Self {
        Lorem {
            ident,
            foo: false,
            bar: None,
        }
    }
}

fn extract_type(param: &GenericParam) -> &TypeParam {
    match *param {
        GenericParam::Type(ref ty) => ty,
        _ => unreachable!("Not a type param"),
    }
}

#[test]
fn expand_many() {
    let di: DeriveInput = syn::parse_str(r#"
        struct Baz<
            #[lorem(foo)] T,
            #[lorem(bar = "x")] U: Eq + ?Sized
        >(T, U);
    "#).unwrap();
    let params = di.generics.params;

    {
        let ty = extract_type(&params[0]);
        let lorem = Lorem::from_type_param(ty).unwrap();
        assert_eq!(lorem.ident.as_ref(), "T");
        assert_eq!(lorem.foo, true);
        assert_eq!(lorem.bar, None);
    }

    {
        let ty = extract_type(&params[1]);
        let lorem = Lorem::from_type_param(ty).unwrap();
        assert_eq!(lorem.ident.as_ref(), "U");
        assert_eq!(lorem.foo, false);
        assert_eq!(lorem.bar, Some("x".to_string()));
    }
}
