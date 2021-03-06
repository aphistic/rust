// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

struct Foo {
    a: u32
}

impl Foo {
    fn x(&mut self) {
        self.a = 5;
    }
}

const FUNC: &'static Fn(&mut Foo) -> () = &Foo::x;

fn main() {
    let mut foo = Foo { a: 137 };
    FUNC(&mut foo); //~ ERROR bad
    assert_eq!(foo.a, 5);
}
