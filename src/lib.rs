pub trait ScopeFn {
    // simply `let` is a keyword.
    fn let_move<F: FnOnce(Self) -> R, R>(self, f: F) -> R
    where Self: Sized;
    fn let_ref<F: FnOnce(&Self) -> R, R>(&self, f: F) -> R;
    fn let_mut<F: FnOnce(&mut Self) -> R, R>(&mut self, f: F) -> R;
    // cannot use `DerefMove`, using `Box`.
    fn let_move_ptr<F: FnOnce(Box<Self>) -> R, R>(self: Box<Self>, f: F) -> R;
    fn let_ref_ptr<F: FnOnce(&Target) -> R, R, Target>(&self, f: F) -> R
    where Self: ::std::ops::Deref<Target = Target>;
    fn let_mut_ptr<F: FnOnce(&mut Target) -> R, R, Target>(&mut self, f: F) -> R
    where Self: ::std::ops::DerefMut<Target = Target>;

    // TODO: cannot impl kotlin's `this`

    // moving fn are must return `Self`
    fn also_move<F: FnOnce(Self) -> Self>(self, f: F) -> Self
    where Self: Sized;
    fn also_ref<F: FnOnce(&Self) -> R, R>(&self, f: F) -> &Self;
    fn also_mut<F: FnOnce(&mut Self) -> R, R>(&mut self, f: F) -> &mut Self;
    fn also_move_ptr<F: FnOnce(Box<Self>) -> Box<Self>>(self: Box<Self>, f: F) -> Box<Self>;
    fn also_ref_ptr<F: FnOnce(&Target) -> R, R, Target>(&self, f: F) -> &Self
    where Self: ::std::ops::Deref<Target = Target>;
    fn also_mut_ptr<F: FnOnce(&mut Target) -> R, R, Target>(&mut self, f: F) -> &mut Self
    where Self: ::std::ops::DerefMut<Target = Target>;
}

impl<T> ScopeFn for T {
    fn let_move<F: FnOnce(Self) -> R, R>(self, f: F) -> R
    where Self: Sized {
        f(self)
    }

    fn let_ref<F: FnOnce(&Self) -> R, R>(&self, f: F) -> R { f(self) }

    fn let_mut<F: FnOnce(&mut Self) -> R, R>(&mut self, f: F) -> R { f(self) }

    fn let_move_ptr<F: FnOnce(Box<Self>) -> R, R>(self: Box<Self>, f: F) -> R { f(self) }

    fn let_ref_ptr<F: FnOnce(&Target) -> R, R, Target>(&self, f: F) -> R
    where Self: ::std::ops::Deref<Target = Target> {
        f(self)
    }

    fn let_mut_ptr<F: FnOnce(&mut Target) -> R, R, Target>(&mut self, f: F) -> R
    where Self: ::std::ops::DerefMut<Target = Target> {
        f(self)
    }

    fn also_move<F: FnOnce(Self) -> Self>(self, f: F) -> Self
    where Self: Sized {
        f(self)
    }

    fn also_ref<F: FnOnce(&Self) -> R, R>(&self, f: F) -> &Self {
        f(self);
        self
    }

    fn also_mut<F: FnOnce(&mut Self) -> R, R>(&mut self, f: F) -> &mut Self {
        f(self);
        self
    }

    fn also_move_ptr<F: FnOnce(Box<Self>) -> Box<Self>>(self: Box<Self>, f: F) -> Box<Self> {
        f(self)
    }

    fn also_ref_ptr<F: FnOnce(&Target) -> R, R, Target>(&self, f: F) -> &Self
    where Self: std::ops::Deref<Target = Target> {
        f(self);
        self
    }

    fn also_mut_ptr<F: FnOnce(&mut Target) -> R, R, Target>(&mut self, f: F) -> &mut Self
    where Self: std::ops::DerefMut<Target = Target> {
        f(self);
        self
    }
}
