macro_rules! parser_struct_alloc {
    (struct $name:ident { $($field:ident: $ty:ty),+, }, $parser:expr) => {
        #[derive(Debug)]
        pub struct $name<A: core::alloc::Allocator> {
            $(
                pub $field: $ty,
            )+
        }

        impl<A: core::alloc::Allocator + Clone> $name<A> {
            pub fn p<I: parser::input::Input<Item = u8>, E: parser::error::ParseError<I, A>>(
                input: I,
                context: &mut crate::aml::Context,
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
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
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
                context: &mut crate::aml::Context,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                $parser
                    .map(|($($field),+)| Self {
                        $(
                            $field,
                        )+
                    })
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

macro_rules! parser_struct_wrapper_alloc {
    (struct $name:ident($ty:ty);, $parser:expr) => {
        #[derive(Debug)]
        pub struct $name<A: core::alloc::Allocator>($ty);

        impl<A: core::alloc::Allocator + Clone> $name<A> {
            pub fn p<I: parser::input::Input<Item = u8>, E: parser::error::ParseError<I, A>>(
                input: I,
                context: &mut crate::aml::Context,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                $parser
                    .map(Self)
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

macro_rules! parser_struct_wrapper {
    (struct $name:ident($ty:ty);, $parser:expr) => {
        #[derive(Debug)]
        pub struct $name($ty);

        impl $name {
            pub fn p<
                I: parser::input::Input<Item = u8>,
                E: parser::error::ParseError<I, A>,
                A: core::alloc::Allocator + Clone,
            >(
                input: I,
                context: &mut crate::aml::Context,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                $parser
                    .map(Self)
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
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
                context: &mut crate::aml::Context,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                $parser
                    .map(|_| Self)
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
            }
        }
    };
}

macro_rules! parser_enum_alloc {
    (enum $name:ident { $($variant:ident($ty:ty)),+, }) => {
        #[derive(Debug)]
        pub enum $name<A: core::alloc::Allocator> {
            $(
                $variant($ty)
            ),+
        }

        impl<A: core::alloc::Allocator + Clone> $name<A> {
            pub fn p<I: parser::input::Input<Item = u8>, E: parser::error::ParseError<I, A>>(
                input: I,
                context: &mut crate::aml::Context,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                (
                    $(
                        $variant::p.map(Self::$variant),
                    )+
                )
                    .alt()
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
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
                context: &mut crate::aml::Context,
                alloc: A,
            ) -> parser::error::ParseResult<I, Self, E> {
                use parser::parser::Parser;
                (
                    $(
                        <$ty>::p.map(Self::$variant),
                    )+
                )
                    .alt()
                    .add_context(stringify!($name))
                    .parse(input, context, alloc)
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
            context: &mut crate::aml::Context,
            alloc: A,
        ) -> parser::error::ParseResult<I, $ret, E> {
            use parser::parser::Parser;
            $parser
                .add_context(stringify!($name))
                .parse(input, context, alloc)
        }
    };
}
