macro_rules! parser_struct_alloc {
    (struct $name:ident { $($field:ident: $ty:ty),+, }, $parser:expr) => {
        pub struct $name<A: core::alloc::Allocator> {
            $(
                pub $field: $ty,
            )+
        }

        impl<A: core::alloc::Allocator> core::fmt::Debug for $name<A> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_struct(stringify!($name))
                    $(
                        .field(stringify!($field), &self.$field)
                    )+
                    .finish()
            }
        }

        impl<A: core::alloc::Allocator + Clone> $name<A> {
            pub fn p<I: parser::input::Input<Item = u8>, E: parser::error::ParseError<I, A>>(
                input: I,
                context: &mut crate::aml::Context<A>,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                #[allow(unused_parens)]
                $parser
                    .map(|($($field),+)| Self {
                        $(
                            $field,
                        )+
                    })
                .map(|a| {
                    let name = stringify!($name);
                    println!("{:width$} matched {:x?}, {:x?}", name,a, input.clone(), width = 20);
                    a
                })
                    .add_context(stringify!($name))
                    .parse(input.clone(), context, alloc)
            }
        }
    };
}

macro_rules! parser_struct {
    (struct $name:ident { $($field:ident: $ty:ty),+, }, $parser:expr) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                pub $field: $ty,
            )+
        }

        impl $name {
            pub fn p<I: parser::input::Input<Item = u8>, E: parser::error::ParseError<I, A>, A: core::alloc::Allocator + Clone>(
                input: I,
                context: &mut crate::aml::Context<A>,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                $parser
                    .map(|($($field),+)| Self {
                        $(
                            $field,
                        )+
                    })
                .map(|a| {
                    let name = stringify!($name);
                    println!("{:width$} matched {:x?}, {:x?}", name,a, input.clone(), width = 20);
                    a
                })
                    .add_context(stringify!($name))
                    .parse(input.clone(), context, alloc)
            }
        }
    };
}

macro_rules! parser_struct_wrapper_alloc {
    (struct $name:ident($ty:ty);, $parser:expr) => {
        pub struct $name<A: core::alloc::Allocator>(pub $ty);

        impl<A: core::alloc::Allocator> core::fmt::Debug for $name<A> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($name)).field(&self.0).finish()
            }
        }

        impl<A: core::alloc::Allocator + Clone> $name<A> {
            pub fn p<I: parser::input::Input<Item = u8>, E: parser::error::ParseError<I, A>>(
                input: I,
                context: &mut crate::aml::Context<A>,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                $parser
                    .map(Self)
                    .map(|a| {
                        let name = stringify!($name);
                        println!(
                            "{:width$} matched {:x?}, {:x?}",
                            name,
                            a,
                            input.clone(),
                            width = 20
                        );
                        a
                    })
                    .add_context(stringify!($name))
                    .parse(input.clone(), context, alloc)
            }
        }
    };
}

macro_rules! parser_struct_wrapper {
    (struct $name:ident($ty:ty);, $parser:expr) => {
        #[derive(Debug)]
        pub struct $name(pub $ty);

        impl $name {
            pub fn p<
                I: parser::input::Input<Item = u8>,
                E: parser::error::ParseError<I, A>,
                A: core::alloc::Allocator + Clone,
            >(
                input: I,
                context: &mut crate::aml::Context<A>,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                $parser
                    .map(Self)
                    .map(|a| {
                        let name = stringify!($name);
                        println!(
                            "{:width$} matched {:x?}, {:x?}",
                            name,
                            a,
                            input.clone(),
                            width = 20
                        );
                        a
                    })
                    .add_context(stringify!($name))
                    .parse(input.clone(), context, alloc)
            }
        }
    };
}

macro_rules! parser_struct_empty {
    (struct $name:ident;, $parser:expr) => {
        #[derive(Debug)]
        pub struct $name;

        impl $name {
            pub fn p<
                I: parser::input::Input<Item = u8>,
                E: parser::error::ParseError<I, A>,
                A: core::alloc::Allocator + Clone,
            >(
                input: I,
                context: &mut crate::aml::Context<A>,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                $parser
                    .map(|_| Self)
                    .map(|a| {
                        let name = stringify!($name);
                        println!(
                            "{:width$} matched {:x?}, {:x?}",
                            name,
                            a,
                            input.clone(),
                            width = 20
                        );
                        a
                    })
                    .add_context(stringify!($name))
                    .parse(input.clone(), context, alloc)
            }
        }
    };
}

macro_rules! parser_enum_alloc {
    (enum $name:ident { $($variant:ident($ty:ty)),+, }) => {
        pub enum $name<A: core::alloc::Allocator> {
            $(
                $variant($ty)
            ),+
        }

        impl<A: core::alloc::Allocator> core::fmt::Debug for $name<A> {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(
                      Self::$variant(inner) => f.debug_tuple(stringify!($variant)).field(&inner).finish(),
                    )+
                }
            }
        }

        impl<A: core::alloc::Allocator + Clone> $name<A> {
            pub fn p<I: parser::input::Input<Item = u8>, E: parser::error::ParseError<I, A>>(
                input: I,
                context: &mut crate::aml::Context<A>,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                (
                    $(
                        $variant::p.map(Self::$variant),
                    )+
                )
                    .alt()
                .map(|a| {
                    let name = stringify!($name);
                    println!("{:width$} matched {:x?}, {:x?}", name,a, input.clone(), width = 20);
                    a
                })
                    .add_context(stringify!($name))
                    .parse(input.clone(), context, alloc)
            }
        }
    };
}

macro_rules! parser_enum {
    (enum $name:ident { $($variant:ident($ty:ty)),+, }) => {
        #[derive(Debug)]
        pub enum $name {
            $(
                $variant($ty),
            )+
        }

        impl $name {
            pub fn p<I: parser::input::Input<Item = u8>, E: parser::error::ParseError<I, A>, A: core::alloc::Allocator + Clone>(
                input: I,
                context: &mut crate::aml::Context<A>,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                (
                    $(
                        <$ty>::p.map(Self::$variant),
                    )+
                )
                    .alt()
                .map(|a| {
                    let name = stringify!($name);
                    println!("{:width$} matched {:x?}, {:x?}", name,a, input.clone(), width = 20);
                    a
                })
                    .add_context(stringify!($name))
                    .parse(input.clone(), context, alloc)
            }
        }
    };
}

macro_rules! parser_fn {
    (fn $name:ident() -> $ret:ty $parser:block) => {
        pub fn $name<
            I: parser::input::Input<Item = u8>,
            E: parser::error::ParseError<I, A>,
            A: core::alloc::Allocator + Clone,
        >(
            input: I,
            context: &mut crate::aml::Context<A>,
            alloc: A,
        ) -> parser::error::ParseResult<I, $ret, E> {
            use parser::parser::Parser;
            $parser
                .map(|a| {
                    let name = stringify!($name);
                    println!(
                        "{:width$} matched {:x?}, {:x?}",
                        name,
                        a,
                        input.clone(),
                        width = 20
                    );
                    a
                })
                .add_context(stringify!($name))
                .parse(input.clone(), context, alloc)
        }
    };
}
