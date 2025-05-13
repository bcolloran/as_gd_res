pub mod extract;

// use godot::obj::{bounds, Bounds, Gd, GdRef, GodotClass};

// pub struct GdReadGuard<'a, T: GodotClass>(GdRef<'a, T>);

// pub trait GdBindSync {
//     type Ref<'a>;
//     fn read_sync<'a>(&self) -> Self::Ref<'a>;
// }

// Any data that is NOT copy and that is NOT a Gd<T> smart pointer
// // can be read via a normal rust `&T` reference.
// //
// // This reference's l
// impl<T> GdBindSync for T
// where
//     T: !Copy + !GodotClass,
// {
//     type Ref<'a> = &'a T;
//     fn read_sync(&'a self) -> Self::Ref {
//         self
//     }
// }

// pub trait GdBindSyncCopyable: Copy {}

// // Any data that implements Copy can be read via a copying dereference
// impl<T: GdBindSyncCopyable> GdBindSync for T {
//     type Ref<'a> = T;
//     fn read_sync<'a>(&self) -> Self::Ref<'a> {
//         *self
//     }
// }

// pub trait GdBindSyncGodoClass:
//     GodotClass + GdBindSync + Bounds<Declarer = bounds::DeclUser>
// {
// }

// // for any Gd<T> smart pointer for which the T type implements GdBindSync,
// // we can read the Gd<T> smart pointer via a GdReadGuard
// impl<T> GdBindSync for Gd<T>
// where
//     T: GdBindSyncGodoClass,
// {
//     type Ref<'a> = GdReadGuard<'a, T>;
//     fn read_sync<'a>(&self) -> Self::Ref<'a> {
//         self.bind()
//     }
// }
