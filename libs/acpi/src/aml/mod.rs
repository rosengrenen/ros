#[macro_use]
pub mod macros;

pub mod data;
pub mod error;
pub mod input;
pub mod misc;
pub mod name;
pub mod ops;
pub mod pkg_len;
pub mod prefixed;
pub mod term;

use self::name::{NamePath, NameSeg};
use crate::aml::name::{DualNamePath, MultiNamePath, NameString, NullName};
use alloc::vec::Vec;
use core::alloc::Allocator;
use std::{
    collections::{HashMap, HashSet},
    fmt::Formatter,
    marker::PhantomData,
};

pub struct Context<A: Allocator> {
    root_scope: Scope<A>,
    current_scope: Vec<ScopePath<A>, A>,
    alloc: A,
}

impl<A: Allocator> core::fmt::Debug for Context<A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Context")
            .field("scopes", &self.root_scope)
            .field("current_scope", &self.current_scope)
            .finish()
    }
}

impl<A: Allocator + Clone> Context<A> {
    pub fn new(alloc: A) -> Self {
        Self {
            root_scope: Scope::new(),
            current_scope: Vec::new(alloc.clone()),
            alloc,
        }
    }

    pub fn push_scope(&mut self, name: &NameString<A>) {
        let new_scope = match self.current_scope.last() {
            Some(current_scope) => current_scope.clone().add_name_string(name),
            None => ScopePath::from_name_string(name, self.alloc.clone()),
        };
        println!("pushing {:?} {:?}", name, new_scope);
        self.current_scope.push(new_scope).unwrap();
    }

    pub fn pop_scope(&mut self) -> ScopePath<A> {
        println!("popping {:?}", self.current_scope);
        self.current_scope.pop().unwrap().unwrap()
    }

    pub fn add_field_seg(&mut self, field: NameSeg) {
        self.current_scope_mut().fields.insert(field);
    }

    pub fn add_field(&mut self, field: &NameString<A>) {
        let (scope, name) = field.split();
        let scope = self.current_scope_path().clone().add_name_string(&scope);
        self.get_scope_mut(&scope).fields.insert(name);
    }

    pub fn add_method(&mut self, method: &NameString<A>, args: usize) {
        let (scope, name) = method.split();
        let scope = self.current_scope_path().clone().add_name_string(&scope);
        self.get_scope_mut(&scope).methods.insert(name, args);
    }

    pub fn has_field(&mut self, field: &NameString<A>) -> bool {
        let (scope, name) = field.split();
        let search_scope = self.current_scope_path().clone().add_name_string(&scope);
        self.get_scope_mut(&search_scope)
            .fields
            .get(&name)
            .is_some()
    }

    pub fn method_args(&mut self, method: &NameString<A>) -> Option<usize> {
        let (scope, name) = method.split();
        let search_scope = self.current_scope_path().clone().add_name_string(&scope);
        self.get_scope_mut(&search_scope)
            .methods
            .get(&name)
            .map(|args| *args)
    }

    fn current_scope_mut(&mut self) -> &mut Scope<A> {
        let scope_path = self.current_scope_path().clone();
        self.get_scope_mut(&scope_path)
    }

    fn get_scope_mut(&mut self, path: &ScopePath<A>) -> &mut Scope<A> {
        let mut scope: &mut Scope<A> = &mut self.root_scope;
        for seg in &path.segments {
            scope = scope.scopes.entry(*seg).or_insert(Scope::new())
        }

        scope
    }

    fn current_scope_path(&self) -> &ScopePath<A> {
        self.current_scope.last().unwrap()
    }
}

pub struct Scope<A: Allocator> {
    scopes: HashMap<NameSeg, Scope<A>>,
    methods: HashMap<NameSeg, usize>,
    fields: HashSet<NameSeg>,
    alloc: PhantomData<A>,
}

impl<A: Allocator> core::fmt::Debug for Scope<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Scope")
            .field("scopes", &self.scopes)
            .field("methods", &self.methods)
            .field("variables", &self.fields)
            .finish()
    }
}

impl<A: Allocator> Scope<A> {
    fn new() -> Self {
        Self {
            scopes: HashMap::new(),
            methods: HashMap::new(),
            fields: HashSet::new(),
            alloc: PhantomData,
        }
    }
}

#[derive(Clone)]
struct ScopePath<A: Allocator> {
    segments: Vec<NameSeg, A>,
    alloc: A,
}

impl<A: Allocator> core::fmt::Debug for ScopePath<A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScopePath")
            .field("segments", &self.segments)
            .finish()
    }
}

impl<A: Allocator + Clone> ScopePath<A> {
    fn new(alloc: A) -> Self {
        Self {
            segments: Vec::new(alloc.clone()),
            alloc,
        }
    }

    fn from_name_string(name_string: &NameString<A>, alloc: A) -> Self {
        Self::new(alloc).add_name_string(name_string)
    }

    fn add_name_string(mut self, name_string: &NameString<A>) -> Self {
        let name_path = match name_string {
            NameString::Absolute(name_path) => {
                self.segments.clear();
                name_path
            }
            NameString::Relative(prefix, name_path) => {
                for _ in 0..*prefix {
                    self.segments.pop().unwrap();
                }

                name_path
            }
        };

        match name_path {
            NamePath::NameSeg(seg) => {
                self.segments.push(*seg).unwrap();
            }
            NamePath::DualNamePath(DualNamePath { first, second }) => {
                self.segments.push(*first).unwrap();
                self.segments.push(*second).unwrap();
            }
            NamePath::MultiNamePath(MultiNamePath(segments)) => {
                for segment in segments {
                    self.segments.push(*segment).unwrap();
                }
            }
            NamePath::NullName(_) => (),
        };

        self
    }
}

impl<A: Allocator + Clone> NameString<A> {
    fn split(&self) -> (Self, NameSeg) {
        match self {
            Self::Absolute(path) => {
                let (path, seg) = path.split();
                (Self::Absolute(path), seg)
            }
            Self::Relative(prefix, path) => {
                let (path, seg) = path.split();
                (Self::Relative(*prefix, path), seg)
            }
        }
    }
}

impl<A: Allocator + Clone> NamePath<A> {
    fn split(&self) -> (Self, NameSeg) {
        match self {
            Self::NameSeg(seg) => (Self::NullName(NullName), *seg),
            Self::DualNamePath(DualNamePath { first, second }) => (Self::NameSeg(*first), *second),
            Self::MultiNamePath(MultiNamePath(segments)) => {
                let mut segments = segments.clone();
                let segment = segments.pop().unwrap().unwrap();
                (Self::MultiNamePath(MultiNamePath(segments)), segment)
            }
            Self::NullName(_) => {
                unimplemented!()
            }
        }
    }
}
