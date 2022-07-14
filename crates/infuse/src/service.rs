pub trait Service {}

pub const fn is_service<T: ?Sized>() -> bool {
    trait IsServiceTest {
        const IS_SERVICE: bool;
    }
    impl<T: ?Sized> IsServiceTest for T {
        default const IS_SERVICE: bool = false;
    }
    impl<T: ?Sized + Service> IsServiceTest for T {
        const IS_SERVICE: bool = true;
    }

    <T as IsServiceTest>::IS_SERVICE
}
